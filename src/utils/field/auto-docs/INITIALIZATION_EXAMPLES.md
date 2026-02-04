# DiscreteRuleField Initialization Examples

This guide demonstrates various initialization patterns using the flexible `new()` constructor and the convenience `new_simple()` constructor.

## Type Alias for Generator Functions

The generator function signature is available as a type alias for easier usage:

```rust
pub type InitFn<CellState, InitialStateData> = fn(
    usize,
    &Dimensions2d<usize>,
    GridCoord,
    &InitialStateData,
) -> CellState;
```

This can be used when defining generator functions or in type annotations.

### Example: Using InitFn Type Alias

```rust
// Define a generator function using the type alias
fn gradient_generator(
    _index: usize,
    dims: &Dimensions2d<usize>,
    coords: GridCoord,
    _data: &(),
) -> CellState {
    let normalized_x = coords.x as f32 / dims.w as f32;
    CellState { value: (normalized_x * 255.0) as u8 }
}

// Use it in field construction
let field = DiscreteRuleField::new(
    Dimensions2d::new(100, 100),
    gradient_generator,
    &(),
    BoundaryPolicy::Clamp,
    rule_set,
    None,
    NeighborhoodType::Immediate,
)?;
```

## Table of Contents
1. [Uniform Initialization](#uniform-initialization)
2. [Position-Based Patterns](#position-based-patterns)
3. [Random Initialization](#random-initialization)
4. [Complex Patterns with Custom Data](#complex-patterns-with-custom-data)

---

## Uniform Initialization

### Using `new_simple()` (Recommended for uniform states)

```rust
use crate::utils::field::discrete_rule_field::*;
use crate::primitives::{plane::Dimensions2d, neighborhood::NeighborhoodType};

#[derive(Clone)]
struct CellState {
    value: u8,
}

fn my_rule(
    grid: &DiscreteRuleField<CellState>,
    cell_coords: GridCoord,
    neighborhood_type: NeighborhoodType,
    _boundary_policy: BoundaryPolicy,
) -> CellState {
    // Rule implementation...
    CellState { value: 0 }
}

let dimensions = Dimensions2d::new(100, 100);
let initial_state = CellState { value: 0 };
let rule_set = RuleSet::new(NeighborhoodType::Immediate, my_rule);

let field = DiscreteRuleField::new_simple(
    dimensions,
    initial_state,
    BoundaryPolicy::Clamp,
    rule_set,
    NeighborhoodType::Immediate,
)?;
```

### Using `new()` with closure

```rust
// Same result as new_simple(), but using the flexible generator
let field = DiscreteRuleField::new(
    dimensions,
    |_, _, _, _| CellState { value: 0 },
    &(),
    BoundaryPolicy::Clamp,
    rule_set,
    None,
    NeighborhoodType::Immediate,
)?;
```

---

## Position-Based Patterns

### Horizontal Gradient

```rust
let field = DiscreteRuleField::new(
    Dimensions2d::new(100, 100),
    |_index, dims, coords, _data| {
        // Value increases from left to right
        let normalized_x = coords.x as f32 / dims.w as f32;
        CellState { 
            value: (normalized_x * 255.0) as u8 
        }
    },
    &(),
    BoundaryPolicy::Clamp,
    rule_set,
    None,
    NeighborhoodType::Immediate,
)?;
```

### Vertical Gradient

```rust
let field = DiscreteRuleField::new(
    Dimensions2d::new(100, 100),
    |_index, dims, coords, _data| {
        // Value increases from top to bottom
        let normalized_y = coords.y as f32 / dims.h as f32;
        CellState { 
            value: (normalized_y * 255.0) as u8 
        }
    },
    &(),
    BoundaryPolicy::Clamp,
    rule_set,
    None,
    NeighborhoodType::Immediate,
)?;
```

### Radial Pattern (Distance from Center)

```rust
let field = DiscreteRuleField::new(
    Dimensions2d::new(100, 100),
    |_index, dims, coords, _data| {
        let center_x = dims.w as f32 / 2.0;
        let center_y = dims.h as f32 / 2.0;
        let dx = coords.x as f32 - center_x;
        let dy = coords.y as f32 - center_y;
        let distance = (dx * dx + dy * dy).sqrt();
        let max_distance = (center_x * center_x + center_y * center_y).sqrt();
        let normalized = (distance / max_distance).min(1.0);
        
        CellState { 
            value: (normalized * 255.0) as u8 
        }
    },
    &(),
    BoundaryPolicy::Clamp,
    rule_set,
    None,
    NeighborhoodType::Immediate,
)?;
```

### Checkerboard Pattern

```rust
let field = DiscreteRuleField::new(
    Dimensions2d::new(100, 100),
    |_index, _dims, coords, _data| {
        let is_even = (coords.x + coords.y) % 2 == 0;
        CellState { 
            value: if is_even { 255 } else { 0 }
        }
    },
    &(),
    BoundaryPolicy::Clamp,
    rule_set,
    None,
    NeighborhoodType::Immediate,
)?;
```

### Diagonal Stripes

```rust
let field = DiscreteRuleField::new(
    Dimensions2d::new(100, 100),
    |_index, _dims, coords, _data| {
        let stripe_width = 10;
        let diagonal_index = (coords.x + coords.y) / stripe_width;
        CellState { 
            value: if diagonal_index % 2 == 0 { 255 } else { 0 }
        }
    },
    &(),
    BoundaryPolicy::Clamp,
    rule_set,
    None,
    NeighborhoodType::Immediate,
)?;
```

---

## Random Initialization

### Using RNG with Custom Data

```rust
use crate::utils::math::rng::XorShiftRng;

struct InitData {
    rng: XorShiftRng,
}

let init_data = InitData {
    rng: XorShiftRng::from_seed(42),
};

let field = DiscreteRuleField::new(
    Dimensions2d::new(100, 100),
    |_index, _dims, _coords, data| {
        // Note: This is a simplified example. In practice, you'd need
        // to handle the fact that data.rng needs to be mutable.
        // Consider using RefCell or passing indices to a pre-generated array.
        CellState { 
            value: (data.rng.next_u32() % 256) as u8 
        }
    },
    &init_data,
    BoundaryPolicy::Clamp,
    rule_set,
    None,
    NeighborhoodType::Immediate,
)?;
```

### Pre-computed Random Values

```rust
// Better approach: pre-generate random values
let mut rng = XorShiftRng::from_seed(42);
let random_values: Vec<u8> = (0..10000)
    .map(|_| (rng.next_u32() % 256) as u8)
    .collect();

let field = DiscreteRuleField::new(
    Dimensions2d::new(100, 100),
    |index, _dims, _coords, data| {
        CellState { 
            value: data[index % data.len()] 
        }
    },
    &random_values,
    BoundaryPolicy::Clamp,
    rule_set,
    None,
    NeighborhoodType::Immediate,
)?;
```

---

## Complex Patterns with Custom Data

### Multi-Parameter Initialization

```rust
struct InitParams {
    base_value: u8,
    scale_factor: f32,
    noise_amplitude: f32,
    pattern_type: PatternType,
}

enum PatternType {
    Gradient,
    Radial,
    Checkerboard,
}

let params = InitParams {
    base_value: 100,
    scale_factor: 1.5,
    noise_amplitude: 20.0,
    pattern_type: PatternType::Radial,
};

let field = DiscreteRuleField::new(
    Dimensions2d::new(100, 100),
    |_index, dims, coords, data| {
        let pattern_value = match data.pattern_type {
            PatternType::Gradient => coords.x as f32 / dims.w as f32,
            PatternType::Radial => {
                let cx = dims.w as f32 / 2.0;
                let cy = dims.h as f32 / 2.0;
                let dx = coords.x as f32 - cx;
                let dy = coords.y as f32 - cy;
                let dist = (dx * dx + dy * dy).sqrt();
                let max_dist = (cx * cx + cy * cy).sqrt();
                dist / max_dist
            },
            PatternType::Checkerboard => {
                if (coords.x + coords.y) % 2 == 0 { 1.0 } else { 0.0 }
            },
        };
        
        let scaled = pattern_value * data.scale_factor * 255.0;
        let final_value = (data.base_value as f32 + scaled)
            .min(255.0)
            .max(0.0) as u8;
        
        CellState { value: final_value }
    },
    &params,
    BoundaryPolicy::Clamp,
    rule_set,
    None,
    NeighborhoodType::Immediate,
)?;
```

### Image-Based Initialization

```rust
struct ImageData {
    pixels: Vec<u8>,
    width: usize,
    height: usize,
}

let image = ImageData {
    pixels: load_image_pixels("pattern.png"),
    width: 100,
    height: 100,
};

let field = DiscreteRuleField::new(
    Dimensions2d::new(image.width, image.height),
    |_index, _dims, coords, data| {
        let pixel_index = coords.y * data.width + coords.x;
        CellState { 
            value: data.pixels[pixel_index] 
        }
    },
    &image,
    BoundaryPolicy::Clamp,
    rule_set,
    None,
    NeighborhoodType::Immediate,
)?;
```

### Time-Based or Frame-Based Initialization

```rust
struct TimeParams {
    frame_number: u32,
    time_offset: f32,
}

let time_params = TimeParams {
    frame_number: 0,
    time_offset: 0.0,
};

let field = DiscreteRuleField::new(
    Dimensions2d::new(100, 100),
    |_index, dims, coords, data| {
        let time = data.frame_number as f32 * 0.1 + data.time_offset;
        let wave = ((coords.x as f32 / 10.0 + time).sin() * 127.5 + 127.5) as u8;
        CellState { value: wave }
    },
    &time_params,
    BoundaryPolicy::Wrap,
    rule_set,
    None,
    NeighborhoodType::Immediate,
)?;
```

---

## Combining Initialization with Custom Rules

You can combine flexible initialization with per-cell custom rules:

```rust
// Define special rule for boundary cells
fn boundary_rule(
    _grid: &DiscreteRuleField<CellState>,
    _cell_coords: GridCoord,
    _neighborhood_type: NeighborhoodType,
    _boundary_policy: BoundaryPolicy,
) -> CellState {
    CellState { value: 255 } // Always maximum
}

let boundary_rule_set = RuleSet::new(NeighborhoodType::Immediate, boundary_rule);

// Set custom rules for all boundary cells
let dimensions = Dimensions2d::new(100, 100);
let mut custom_rules = Vec::new();

// Top and bottom edges
for x in 0..dimensions.w {
    custom_rules.push((GridCoord::new(x, 0), boundary_rule_set.clone()));
    custom_rules.push((GridCoord::new(x, dimensions.h - 1), boundary_rule_set.clone()));
}

// Left and right edges
for y in 1..dimensions.h - 1 {
    custom_rules.push((GridCoord::new(0, y), boundary_rule_set.clone()));
    custom_rules.push((GridCoord::new(dimensions.w - 1, y), boundary_rule_set.clone()));
}

// Initialize with gradient and custom boundary rules
let field = DiscreteRuleField::new(
    dimensions,
    |_index, dims, coords, _data| {
        let normalized_x = coords.x as f32 / dims.w as f32;
        CellState { value: (normalized_x * 255.0) as u8 }
    },
    &(),
    BoundaryPolicy::Clamp,
    default_rule_set,
    Some(custom_rules),
    NeighborhoodType::Immediate,
)?;
```

---

## Best Practices

1. **Use `new_simple()` for uniform initialization** - It's cleaner and more readable.

2. **Pre-compute expensive calculations** - If initialization requires expensive operations (random numbers, noise generation), compute them once and pass them in `init_data`.

3. **Keep generator functions simple** - Complex logic should be in helper functions, not inline in the closure.

4. **Use meaningful parameter names** - Create dedicated structs for `InitialStateData` instead of using tuples.

5. **Consider memory usage** - Large `init_data` structures are passed by reference, so there's no copying overhead.

6. **Validate dimensions early** - If your initialization depends on specific field dimensions, validate them before creating the field.

---

## Performance Considerations

- The generator function is called **once per cell** during construction
- For a 1000×1000 field, that's 1,000,000 function calls
- Keep the generator function as simple and fast as possible
- Pre-compute expensive operations and store them in `init_data`
- Consider using lookup tables for repeated calculations

---

**Last Updated**: January 25, 2026  
**Related**: `FIELD_CONSTRUCTOR_ENHANCEMENT.md`
