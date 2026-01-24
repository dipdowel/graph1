# Quick Reference: DiscreteRuleField Rule Management API

## Creating a Field

```rust
use crate::utils::field::discrete_rule_field::*;
use crate::primitives::point::Point;
use crate::primitives::plane::Dimensions2d;
use crate::primitives::neighborhood::NeighborhoodType;

// Create field with default rule
let dimensions = Dimensions2d::new(100, 100);
let initial_state = MyState { value: 0 };
let default_rule = RuleSet::new(NeighborhoodType::Immediate, my_rule_fn);

let mut field = DiscreteRuleField::new_uniform(
    dimensions,
    initial_state,
    BoundaryPolicy::Wrap,
    default_rule,
    NeighborhoodType::Immediate,
);
```

## Managing Rules

### Set Custom Rule for a Cell
```rust
fn heat_source_rule(...) -> MyState {
    MyState { temperature: 100.0 } // Always hot
}

let custom_rule = RuleSet::new(NeighborhoodType::Orthogonal, heat_source_rule);
field.set_cell_rule(Point::new(50, 50), custom_rule)?;
```

### Reset Cell to Default Rule
```rust
field.reset_cell_rule(Point::new(50, 50))?;
```

### Query Which Rule a Cell Uses
```rust
let rule = field.get_cell_rule(Point::new(50, 50));
println!("Neighborhood: {:?}", rule.neighborhood_type);
```

## Modifying Cell States

### Individual Cell Modification
```rust
// Safe, bounds-checked
field.set_cell(Point::new(10, 10), new_state)?;

// Mutable reference
if let Some(cell) = field.get_cell_mut(Point::new(10, 10)) {
    cell.state.value += 1;
}

// Fast, unchecked (requires manual bounds verification)
if coords.x < field.dimensions().w && coords.y < field.dimensions().h {
    let cell = field.get_cell_unchecked_mut(coords);
    cell.state = new_state;
}
```

### Bulk Modification
```rust
// Iterate over all cells
for cell in field.grid_mut().iter_mut() {
    cell.state.value *= 2;
}

// With coordinates
for (index, cell) in field.grid_mut().iter_mut().enumerate() {
    let coords = field.index_to_coords(index);
    cell.state = compute_new_state(coords);
}
```

## Updating the Field

### Full Field Update
```rust
field.update(None);
```

### Localized Update (One-time)
```rust
let config = UpdateConfig::new(
    NeighborhoodType::Circle { radius: 10 },
    Point::new(50, 50),  // center
    false,               // don't update whole field
);
field.update(Some(config));
```

### Localized Update (Persistent)
```rust
let config = UpdateConfig::new(
    NeighborhoodType::Circle { radius: 15 },
    Point::new(50, 50),
    false,
);
field.set_update_config(config);

// All subsequent updates use this config
field.update(None);
field.update(None);
```

## Common Patterns

### Heat Source Simulation
```rust
// Set heat sources with custom rules
fn heat_source_rule(...) -> ThermalState {
    ThermalState { temperature: 100.0 }
}

let heat_rule = RuleSet::new(NeighborhoodType::Immediate, heat_source_rule);
field.set_cell_rule(Point::new(25, 25), heat_rule.clone())?;
field.set_cell_rule(Point::new(75, 75), heat_rule)?;

// Rest of the field uses default thermal diffusion
field.update(None);
```

### Interactive Cellular Automata
```rust
// User clicks at (x, y) to toggle special behavior
fn handle_click(field: &mut DiscreteRuleField<State>, click_pos: Point<usize>) {
    let current_rule = field.get_cell_rule(click_pos);
    
    if is_default_rule(current_rule) {
        // Add special rule
        let special = RuleSet::new(NeighborhoodType::Orthogonal, special_rule);
        field.set_cell_rule(click_pos, special).unwrap();
    } else {
        // Reset to default
        field.reset_cell_rule(click_pos).unwrap();
    }
}
```

### Wave Propagation with Obstacles
```rust
// Create obstacles that don't propagate
fn obstacle_rule(...) -> WaveState {
    WaveState { amplitude: 0.0 } // No propagation
}

let obstacle = RuleSet::new(NeighborhoodType::Immediate, obstacle_rule);

// Place obstacles
for y in 40..60 {
    field.set_cell_rule(Point::new(50, y), obstacle.clone())?;
}

// Rest of the field propagates waves normally
field.update(None);
```

## Performance Tips

1. **Use `set_cell_rule()` sparingly** - Best for < 25% of cells needing custom rules
2. **Use localized updates** - When only a region changes (mouse interaction, etc.)
3. **Use `get_cell_unchecked_mut()` in hot loops** - After verifying bounds
4. **Batch rule assignments** - Set all custom rules before starting simulation
5. **Avoid frequent reset/set cycles** - Keep rule assignments stable when possible

## Error Handling

All rule management methods return `Result<(), String>`:

```rust
match field.set_cell_rule(coords, rule) {
    Ok(()) => println!("Rule set successfully"),
    Err(e) => eprintln!("Failed to set rule: {}", e),
}
```

Common errors:
- "Coordinates out of bounds" - Invalid cell coordinates
- "Invalid cell index" - Internal error (shouldn't happen in normal use)
