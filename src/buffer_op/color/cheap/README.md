# Cheap Color Operations

Fast color transformations for RRGGBBAA pixel buffers using 8.8 fixed-point arithmetic.

## Overview

High-performance color operations on `Vec<u32>` buffers (pixel format: `0xRRGGBBAA`). All operations preserve alpha and avoid floating-point math in tight loops.

## Quick Start

```rust
use graph1::buffer_op::color::cheap::*;

let mut buffer = vec![0xFF8040FF; 1000];

// Individual operations
brightness_buffer(&mut buffer, Brightness::from_percent(120));
contrast_buffer(&mut buffer, Contrast::from_f32(1.5));
hue_buffer(&mut buffer, Hue::from_degrees(45.0));
tint_buffer(&mut buffer, Tint::new(255, 200, 150));
vibrance_buffer(&mut buffer, Vibrance::from_f32(0.5));
```

## Operations

### Brightness
Scales RGB channels uniformly.

```rust
brightness_buffer(&mut buffer, Brightness::from_percent(120)); // 20% brighter
```

- `Brightness::from_percent(100)` - No change
- `Brightness::from_percent(50)` - Half brightness
- `Brightness::identity()` - Same as 100%

### Contrast
Adjusts difference from middle gray (128).

```rust
contrast_buffer(&mut buffer, Contrast::from_f32(1.5)); // 50% more contrast
```

- `Contrast::from_f32(1.0)` - No change
- `Contrast::from_f32(0.0)` - Pure gray
- `Contrast::default()` - Same as 1.0

### Hue Rotation
Rotates colors around the color wheel (preserves luminance via Rec. 601).

```rust
hue_buffer(&mut buffer, Hue::from_degrees(120.0)); // Red → Green
```

- `Hue::from_degrees(0.0)` - No change
- `Hue::from_degrees(120.0)` - Rotate 120°
- `Hue::from_radians(PI)` - Rotate 180°

### Tint
⚠️ **Experimental!** Multiplies RGB channels by tint values.

```rust
tint_buffer(&mut buffer, Tint::new(255, 128, 64)); // Orange tint
```

- `Tint::white()` - No change (255, 255, 255)
- `Tint::from((200, 150, 100))` - From tuple
- `Tint::from(0xFF8040FF)` - From pixel color

### Vibrance
Selective saturation adjustment (affects dull colors more than vibrant ones).

```rust
vibrance_buffer(&mut buffer, Vibrance::from_f32(0.5)); // Boost dull colors
```

- `Vibrance::from_f32(0.0)` - No change
- `Vibrance::from_f32(0.5)` - Enhance less-saturated colors
- `Vibrance::from_f32(-0.5)` - Mute colors
- `Vibrance::identity()` - Same as 0.0

**Note:** Unlike uniform saturation, vibrance protects already-saturated colors from oversaturation.

## Combined Transformations

Apply multiple operations in **one pass**:

```rust
let transform = ColorTransform::new(vec![
    ColorTransformType::Brightness(Brightness::from_percent(120)),
    ColorTransformType::Vibrance(Vibrance::from_f32(0.4)),
    ColorTransformType::Contrast(Contrast::from_f32(1.2)),
    ColorTransformType::Hue(Hue::from_degrees(30.0)),
]);
color_transform_buffer(&mut buffer, transform);
```

**Benefits:** Single iteration, order preserved, auto-skips identity operations.

## Performance

- **Fixed-point**: 8.8 format (256 = 1.0) avoids floating-point overhead
- **Single pass**: Combined transforms iterate once
- **Early exits**: Identity operations skipped via `is_identity()`
- **Clamping**: All outputs → [0, 255]

