# Color Transform Buffer - Design Discussion

This file contains a summary of discussion with an LLM on the fastest possible color transform chain.
The solution with the matrices looks promising but involves extra complexity and requires some benchmarking 
to confirm the performance benefits. The sequential per-pixel approach is simpler and may be sufficient for most use cases, 
especially with fewer transforms, so I'm sticking to that one for now. 

## Overview

This document captures the design discussion for implementing a unified `color_transform_buffer()` function that combines brightness, contrast, hue, and tint adjustments in a single pass.

---

## TODO Analysis (from `color_transform_buf.rs`)

1. Analyze `brightness_buf.rs`, `contrast_buf.rs`, `hue_buf.rs`, and `tint_buf.rs` to identify common patterns and opportunities for code reuse.
2. Implement `color_transform_buffer` with a single loop, applying transformations based on `ColorTransform` fields.
3. Keep individual functions (`brightness_buffer()`, `contrast_buffer()`, etc.) for single-adjustment use cases.
4. Consider adding color transformation matrices to `Brightness`, `Contrast`, and `Tint` (like `Hue` already has) to pre-compute contributions via matrix operations.

---

## Analysis & Suggestions

### 1. Common Patterns for Code Reuse

**Channel extraction** appears in all four functions. Create shared helpers:

```rust
#[inline(always)]
pub fn extract_rgba(pixel: u32) -> (i32, i32, i32, u32) {
    let r = ((pixel >> 24) & 0xFF) as i32;
    let g = ((pixel >> 16) & 0xFF) as i32;
    let b = ((pixel >> 8)  & 0xFF) as i32;
    let a =  (pixel        & 0xFF) as u32;
    (r, g, b, a)
}

#[inline(always)]
pub fn pack_rgba(r: u32, g: u32, b: u32, a: u32) -> u32 {
    (r << 24) | (g << 16) | (b << 8) | a
}
```

### 2. Single-Pass Combined Transform

Implement `color_transform_buffer` with one loop, applying transforms sequentially per-pixel:

```rust
pub fn color_transform_buffer(buffer: &mut [u32], transform: ColorTransform) {
    if transform.is_empty() {
        return;
    }

    for pixel in buffer.iter_mut() {
        let (mut r, mut g, mut b, a) = extract_rgba(*pixel);

        for t in &transform.transforms {
            match t {
                ColorTransformType::Brightness(br) => {
                    let f = br.factor_8_8();
                    r = ((r * f + 128) >> 8).clamp(0, 255);
                    g = ((g * f + 128) >> 8).clamp(0, 255);
                    b = ((b * f + 128) >> 8).clamp(0, 255);
                }
                ColorTransformType::Contrast(c) => {
                    let cf = c.fixed_multiplier();
                    r = (((r - 128) * cf) >> 8) + 128;
                    g = (((g - 128) * cf) >> 8) + 128;
                    b = (((b - 128) * cf) >> 8) + 128;
                    r = r.clamp(0, 255);
                    g = g.clamp(0, 255);
                    b = b.clamp(0, 255);
                }
                ColorTransformType::Hue(h) => {
                    let m = h.matrix_values();
                    let nr = (m[0][0]*r + m[0][1]*g + m[0][2]*b) >> 8;
                    let ng = (m[1][0]*r + m[1][1]*g + m[1][2]*b) >> 8;
                    let nb = (m[2][0]*r + m[2][1]*g + m[2][2]*b) >> 8;
                    r = nr.clamp(0, 255);
                    g = ng.clamp(0, 255);
                    b = nb.clamp(0, 255);
                }
                ColorTransformType::Tint(t) => {
                    r = ((r * t.r as i32) + 128) >> 8;
                    g = ((g * t.g as i32) + 128) >> 8;
                    b = ((b * t.b as i32) + 128) >> 8;
                }
            }
        }

        *pixel = pack_rgba(r as u32, g as u32, b as u32, a);
    }
}
```

### 3. Keep Individual Functions

The existing `brightness_buffer()`, `contrast_buffer()`, etc. should remain unchanged for single-operation use cases—they're already optimized for their specific purpose.

### 4. Unified Matrix Approach

**Yes, this is possible but has tradeoffs:**

| Transform   | Matrix-based?     | Notes                          |
|-------------|-------------------|--------------------------------|
| Hue         | ✅ Already 3×3    | Works well                     |
| Brightness  | ✅ Diagonal 3×3   | `diag(f, f, f)`                |
| Contrast    | ⚠️ Affine         | Needs 3×4 or offset vector     |
| Tint        | ✅ Diagonal 3×3   | `diag(tr/255, tg/255, tb/255)` |

**Problem:** Contrast requires an affine transform (subtract 128, scale, add 128). This needs either:
- A 4×4 matrix with homogeneous coordinates
- Separate offset vectors alongside the 3×3 matrix

**Recommendation:** For simplicity and flexibility, the sequential per-pixel approach (option 2) is more maintainable. Matrix multiplication would only help if you have *many* transforms to combine, and even then the affine offset complicates things.

---

## Alternative for Contrast with 3×3 Matrix

### Bias-Free Contrast

Use a modified contrast that pivots around **0** instead of **128**, applied to **pre-centered** values:

```rust
/// Contrast as a diagonal 3×3 matrix (requires pre/post centering)
impl Contrast {
    pub fn as_matrix(&self) -> [[i32; 3]; 3] {
        let c = self.fixed_multiplier();
        [
            [c, 0, 0],
            [0, c, 0],
            [0, 0, c],
        ]
    }
}
```

Then in the combined transform, handle the offset separately:

```rust
// Per-pixel: subtract 128, apply matrix, add 128
let r_centered = r - 128;
let g_centered = g - 128;
let b_centered = b - 128;

// Matrix multiply (contrast is diagonal)
let r2 = (matrix[0][0] * r_centered) >> 8;
let g2 = (matrix[1][1] * g_centered) >> 8;
let b2 = (matrix[2][2] * b_centered) >> 8;

// Re-center
let r_out = (r2 + 128).clamp(0, 255);
```

---

## Better Option: 4×4 Affine Matrix

For true matrix composition, use homogeneous coordinates:

```rust
/// 4×4 affine color transform matrix (3×3 linear + offset column)
pub struct AffineColorMatrix {
    pub m: [[i32; 4]; 3],  // 3 rows × 4 cols, 8.8 fixed-point
}

impl Contrast {
    pub fn as_affine_matrix(&self) -> [[i32; 4]; 3] {
        let c = self.fixed_multiplier();
        let offset = ((256 - c) * 128) >> 8; // 128 * (1 - c)
        [
            [c, 0, 0, offset],
            [0, c, 0, offset],
            [0, 0, c, offset],
        ]
    }
}
```

**Recommendation:** The 4×4 affine approach is cleanest if you want full matrix composability. Otherwise, stick with the sequential per-pixel approach—it's simpler and equally performant for small transform chains.

---

## Combining 3×3 and 4×4 Matrices

You cannot directly multiply 3×3 and 4×4 matrices together. You need to **lift all transforms to 4×4 affine matrices**, then multiply them.

### Solution: Promote All Transforms to 4×4 Affine

A 4×4 affine color matrix has the form:
```
[m00 m01 m02 offset_r]
[m10 m11 m12 offset_g]
[m20 m21 m22 offset_b]
[  0   0   0        1]
```

The last row is always `[0, 0, 0, 1]` for affine transforms, so you only need to store 3×4.

### Affine Matrix Struct

```rust
/// 3×4 affine color matrix (3×3 linear + offset column), 8.8 fixed-point
#[derive(Clone, Copy, Debug)]
pub struct AffineColorMatrix {
    pub m: [[i32; 4]; 3],
}

impl AffineColorMatrix {
    /// Identity matrix (no transformation)
    pub const fn identity() -> Self {
        Self {
            m: [
                [256, 0, 0, 0],
                [0, 256, 0, 0],
                [0, 0, 256, 0],
            ],
        }
    }

    /// Multiply two affine matrices (self * other)
    pub fn multiply(&self, other: &AffineColorMatrix) -> AffineColorMatrix {
        let mut result = [[0i32; 4]; 3];

        for i in 0..3 {
            for j in 0..3 {
                result[i][j] = (self.m[i][0] * other.m[0][j]
                    + self.m[i][1] * other.m[1][j]
                    + self.m[i][2] * other.m[2][j]) >> 8;
            }
            // Offset column: linear part * other's offset + self's offset
            result[i][3] = ((self.m[i][0] * other.m[0][3]
                + self.m[i][1] * other.m[1][3]
                + self.m[i][2] * other.m[2][3]) >> 8)
                + self.m[i][3];
        }

        AffineColorMatrix { m: result }
    }
}
```

### Convert Each Transform to 4×4

```rust
impl Brightness {
    pub fn as_affine_matrix(&self) -> AffineColorMatrix {
        let f = self.factor_8_8;
        AffineColorMatrix {
            m: [
                [f, 0, 0, 0],
                [0, f, 0, 0],
                [0, 0, f, 0],
            ],
        }
    }
}

impl Contrast {
    pub fn as_affine_matrix(&self) -> AffineColorMatrix {
        let c = self.fixed;
        let offset = 128 * (256 - c) >> 8; // 128 * (1 - c)
        AffineColorMatrix {
            m: [
                [c, 0, 0, offset],
                [0, c, 0, offset],
                [0, 0, c, offset],
            ],
        }
    }
}

impl Hue {
    pub fn as_affine_matrix(&self) -> AffineColorMatrix {
        let m = self.matrix;
        AffineColorMatrix {
            m: [
                [m[0][0], m[0][1], m[0][2], 0],
                [m[1][0], m[1][1], m[1][2], 0],
                [m[2][0], m[2][1], m[2][2], 0],
            ],
        }
    }
}

impl Tint {
    pub fn as_affine_matrix(&self) -> AffineColorMatrix {
        AffineColorMatrix {
            m: [
                [self.r as i32, 0, 0, 0],
                [0, self.g as i32, 0, 0],
                [0, 0, self.b as i32, 0],
            ],
        }
    }
}
```

### Compose and Apply

```rust
pub fn compute_combined_matrix(transform: &ColorTransform) -> AffineColorMatrix {
    let mut result = AffineColorMatrix::identity();

    for t in &transform.transforms {
        let m = match t {
            ColorTransformType::Brightness(b) => b.as_affine_matrix(),
            ColorTransformType::Contrast(c) => c.as_affine_matrix(),
            ColorTransformType::Hue(h) => h.as_affine_matrix(),
            ColorTransformType::Tint(t) => t.as_affine_matrix(),
        };
        result = result.multiply(&m);
    }

    result
}
```

---

## Performance Impact of `compute_combined_matrix()`

### Positive:
- Matrix computed **once per transform chain**, not per-pixel
- Per-pixel work reduced to **one matrix-vector multiply** (9 multiplies + 9 adds) regardless of transform count
- Better for **3+ transforms** — cost is constant vs. linear with sequential approach

### Negative:
- **Fixed overhead** of matrix multiplications during setup (~3-4 matrix mults for typical chains)
- Per-pixel still needs **offset addition** (3 adds) after the 3×3 multiply
- For **1-2 transforms**, sequential approach may be faster (less setup, simpler per-pixel ops)

### Break-even Point
~2-3 transforms, depending on buffer size.

### Recommendation
Use matrix approach for large buffers with multiple transforms; keep individual functions for single-transform cases.

---

## Summary

| Approach | Best For | Complexity |
|----------|----------|------------|
| Individual functions | Single transforms | Low |
| Sequential per-pixel | 1-2 transforms, small buffers | Medium |
| Pre-computed affine matrix | 3+ transforms, large buffers | Higher setup, lower per-pixel |

The **4×4 affine matrix approach** provides the most flexibility and best performance for complex transform chains, while the **sequential approach** is simpler to implement and maintain for simpler use cases.

