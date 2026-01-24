# DiscreteRuleField Constructor Quick Reference

## API at a Glance

### `new_simple()` - For Uniform Initialization ⭐
```rust
pub fn new_simple(
    dimensions: Dimensions2d<usize>,
    initial_state: CellState,
    boundary_policy: BoundaryPolicy,
    rule_set: RuleSet<CellState>,
    update_neighborhood: NeighborhoodType,
) -> Result<Self, String>
```

**Use when**: All cells have the same initial state and use the same rule.

---

### `new()` - For Flexible Initialization 🎨
```rust
pub fn new<F, InitialStateData>(
    dimensions: Dimensions2d<usize>,
    init_fn: F,
    init_data: &InitialStateData,
    boundary_policy: BoundaryPolicy,
    default_rule_set: RuleSet<CellState>,
    custom_rule_sets: Option<Vec<(GridCoord, RuleSet<CellState>)>>,
    update_neighborhood: NeighborhoodType,
) -> Result<Self, String>
where
    F: Fn(usize, &Dimensions2d<usize>, GridCoord, &InitialStateData) -> CellState
```

**Generator Function Signature**:
```rust
|index, dimensions, coords, init_data| -> CellState
```

**Use when**: Cells need different initial states based on position, index, or custom data.

---

## Quick Examples

### Uniform (All cells same)
```rust
DiscreteRuleField::new_simple(
    Dimensions2d::new(100, 100),
    MyState { value: 0 },
    BoundaryPolicy::Clamp,
    rule_set,
    NeighborhoodType::Immediate,
)?
```

### Gradient (Position-based)
```rust
DiscreteRuleField::new(
    Dimensions2d::new(100, 100),
    |_, dims, coords, _| MyState { 
        value: (coords.x * 255 / dims.w) as u8 
    },
    &(),
    BoundaryPolicy::Clamp,
    rule_set,
    None,
    NeighborhoodType::Immediate,
)?
```

### Checkerboard
```rust
DiscreteRuleField::new(
    Dimensions2d::new(100, 100),
    |_, _, coords, _| MyState { 
        value: if (coords.x + coords.y) % 2 == 0 { 255 } else { 0 }
    },
    &(),
    BoundaryPolicy::Wrap,
    rule_set,
    None,
    NeighborhoodType::Immediate,
)?
```

### With Custom Data
```rust
struct InitData { scale: f32, offset: u8 }
let data = InitData { scale: 2.0, offset: 100 };

DiscreteRuleField::new(
    Dimensions2d::new(100, 100),
    |idx, _, _, data| MyState { 
        value: ((idx as f32 * data.scale) as u8).saturating_add(data.offset)
    },
    &data,
    BoundaryPolicy::Clamp,
    rule_set,
    None,
    NeighborhoodType::Immediate,
)?
```

---

## Generator Function Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `index` | `usize` | Linear index (0..capacity-1), row-major order |
| `dimensions` | `&Dimensions2d<usize>` | Grid dimensions (width, height) |
| `coords` | `GridCoord` | Cell coordinates (x, y) |
| `init_data` | `&InitialStateData` | User-provided data (any type) |

**Returns**: `CellState` - The initial state for the cell

---

## Common Patterns Cheat Sheet

```rust
// Horizontal gradient
|_, dims, coords, _| value_from_x(coords.x, dims.w)

// Vertical gradient  
|_, dims, coords, _| value_from_y(coords.y, dims.h)

// Radial (distance from center)
|_, dims, coords, _| {
    let cx = dims.w as f32 / 2.0;
    let cy = dims.h as f32 / 2.0;
    let dist = ((coords.x as f32 - cx).powi(2) + (coords.y as f32 - cy).powi(2)).sqrt();
    value_from_distance(dist)
}

// Checkerboard
|_, _, coords, _| if (coords.x + coords.y) % 2 == 0 { even_value } else { odd_value }

// Diagonal stripes
|_, _, coords, _| {
    let stripe = (coords.x + coords.y) / STRIPE_WIDTH;
    if stripe % 2 == 0 { value_a } else { value_b }
}

// Linear index-based
|index, _, _, _| value_from_index(index)

// Random (pre-computed)
|index, _, _, random_array| random_array[index % random_array.len()]

// Uniform (same as new_simple)
|_, _, _, _| INITIAL_STATE.clone()
```

---

## Decision Tree

```
Need to create a DiscreteRuleField?
│
├─ All cells same initial state?
│  └─ YES → Use new_simple() ⭐
│
└─ Different initial states needed?
   │
   ├─ Based on position/index only?
   │  └─ YES → Use new() with simple closure
   │
   └─ Need custom data (RNG, config, etc.)?
      └─ YES → Use new() with InitialStateData struct
```

---

## See Also

- `FIELD_CONSTRUCTOR_ENHANCEMENT.md` - Detailed documentation
- `INITIALIZATION_EXAMPLES.md` - Comprehensive examples
- `README.md` - Module overview
- `RULE_MANAGEMENT_QUICK_REF.md` - Rule management API

---

**Quick Tip**: Start with `new_simple()`. Only use `new()` when you need different initial states per cell.
