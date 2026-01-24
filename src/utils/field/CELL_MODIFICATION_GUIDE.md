# Cell State Modification Guide

This guide shows you how to modify cell states in a `DiscreteRuleField` from user code.

## Methods Available

### 1. `set_cell()` - Safe, bounds-checked modification

The safest and most straightforward way to modify a cell:

```ignore
use crate::utils::field::discrete_rule_field::*;
use crate::primitives::point::Point;

// Create your field
let mut field = DiscreteRuleField::<YourState>::new_uniform(/* ... */);

// Modify a cell
let coords = Point::new(5, 5);
match field.set_cell(coords, new_state) {
    Ok(()) => println!("Cell updated successfully"),
    Err(e) => println!("Error: {}", e),
}
```

**Pros:**
- Type-safe and bounds-checked
- Returns Result for error handling
- Respects boundary policy

**When to use:** Default choice for most use cases.

---

### 2. `get_cell_mut()` - Mutable reference for direct modification

Get a mutable reference to modify the cell directly:

```ignore
let coords = Point::new(5, 5);
if let Some(cell) = field.get_cell_mut(coords) {
    // Modify the cell state directly
    cell.state.value = 42;
    cell.state.flags |= SOME_FLAG;
} else {
    println!("Cell not found");
}
```

**Pros:**
- Allows multiple field modifications on the same cell
- No need to clone the entire state
- Still bounds-checked and safe

**When to use:** When you need to modify multiple fields of a complex state.

---

### 3. `get_cell_unchecked_mut()` - Fast, unchecked modification

For performance-critical loops where you've already verified bounds:

```ignore
let coords = Point::new(5, 5);
// IMPORTANT: Verify bounds first!
if coords.x < field.dimensions().w && coords.y < field.dimensions().h {
    let cell = field.get_cell_unchecked_mut(coords);
    cell.state = new_state;
}
```

**Pros:**
- Maximum performance
- No boundary checks overhead

**Cons:**
- Will panic if out of bounds
- Requires manual bounds verification

**When to use:** Hot loops processing many cells where performance matters.

---

### 4. `grid_mut()` - Bulk modification

For modifying many cells at once:

```ignore
let dimensions = field.dimensions();
for cell in field.grid_mut().iter_mut() {
    // Modify all cells
    cell.state.value *= 2;
}

// Or with enumeration for coordinates
for (index, cell) in field.grid_mut().iter_mut().enumerate() {
    let coords = field.index_to_coords(index);
    cell.state = compute_new_state(coords);
}
```

**When to use:** Batch operations on many/all cells.

---

## Rule Modification Methods

### 1. `set_cell_rule()` - Set custom rule for a cell

Override the default rule for a specific cell:
```ignore
```rust
// Define a custom rule function
fn custom_rule(
    grid: &DiscreteRuleField<MyState>,
    cell_coords: Point<usize>,
    neighborhood_type: NeighborhoodType,
    boundary_policy: BoundaryPolicy,
) -> MyState {
    // Custom logic
    MyState { value: 42 }
}

let custom_rule_set = RuleSet::new(NeighborhoodType::Orthogonal, custom_rule);
field.set_cell_rule(Point::new(5, 5), custom_rule_set)?;
```
```
**When to use:** Creating heat sources, obstacles, or special cells with different behavior.

---

### 2. `reset_cell_rule()` - Revert to default rule

Remove a cell's custom rule:

```rust
field.reset_cell_rule(Point::new(5, 5))?;
```

**When to use:** Removing temporary effects or resetting modified cells.

---

### 3. `get_cell_rule()` - Query a cell's rule

Check which rule applies to a cell:

```ignore
let rule = field.get_cell_rule(Point::new(5, 5));
println!("Neighborhood type: {:?}", rule.neighborhood_type);
```

**When to use:** Debugging or conditional logic based on cell rules.

---

## Complete Example

```ignore
use crate::utils::field::discrete_rule_field::*;
use crate::primitives::point::Point;
use crate::primitives::plane::Dimensions2d;
use crate::primitives::neighborhood::NeighborhoodType;

#[derive(Clone, Copy, Debug)]
struct MyState {
    temperature: f32,
    color: u32,
}

fn my_rule(
    grid: &DiscreteRuleField<MyState>,
    cell_coords: Point<usize>,
    neighborhood_type: NeighborhoodType,
    _boundary_policy: BoundaryPolicy,
) -> MyState {
    // Rule implementation
    MyState { temperature: 0.0, color: 0 }
}

fn main() {
    let dimensions = Dimensions2d::new(100, 100);
    let initial_state = MyState { temperature: 20.0, color: 0xFF0000 };
    let rule_set = RuleSet::new(NeighborhoodType::Immediate, my_rule);
    
    let mut field = DiscreteRuleField::new_uniform(
        dimensions,
        initial_state,
        BoundaryPolicy::Wrap,
        rule_set,
        NeighborhoodType::Immediate,
    );
    
    // Method 1: Set a specific cell
    field.set_cell(
        Point::new(50, 50),
        MyState { temperature: 100.0, color: 0x00FF00 }
    ).unwrap();
    
    // Method 2: Get mutable reference and modify
    if let Some(cell) = field.get_cell_mut(Point::new(25, 25)) {
        cell.state.temperature += 10.0;
        cell.state.color = 0x0000FF;
    }
    
    // Method 3: Fast unchecked modification in a loop
    for y in 0..10 {
        for x in 0..10 {
            let coords = Point::new(x, y);
            let cell = field.get_cell_unchecked_mut(coords);
            cell.state.temperature = 50.0;
        }
    }
    
    // Method 4: Bulk modification
    for (index, cell) in field.grid_mut().iter_mut().enumerate() {
        let coords = field.index_to_coords(index);
        if coords.x < 10 && coords.y < 10 {
            cell.state.color = 0xFFFFFF;
        }
    }
    
    // Update the entire field (traditional CA evolution)
    field.update(None);
    
    // Or do a localized update around a point of interest
    let interaction_point = Point::new(50, 50);
    let localized_config = UpdateConfig::new(
        NeighborhoodType::Circle { radius: 15 },
        interaction_point,
        false,
    );
    field.update(Some(localized_config));
}
```

## Update Modes

The `DiscreteRuleField` supports two update modes:

### Full Field Update
```ignore
// Update all cells in the field
field.update(None);
```

### Localized Update (One-time)

```ignore
// Update only cells in a specific neighborhood for this single update
let update_config = UpdateConfig::new(
    NeighborhoodType::Circle { radius: 10 },
    Point::new(50, 50), // center point
    false, // don't update whole field
);
field.update(Some(update_config));
```

### Localized Update (Persistent)
```ignore
// Set a persistent update configuration
let persistent_config = UpdateConfig::new(
    NeighborhoodType::Circle { radius: 10 },
    Point::new(50, 50), // center point
    false,
);
field.set_update_config(persistent_config);

// Now all subsequent calls to update(None) will use this configuration
field.update(None); // Uses persistent config
field.update(None); // Still uses persistent config

// You can override it temporarily with a one-time config
field.update(Some(other_config));

// Or update the persistent config
field.set_update_config(new_persistent_config);
```

**Performance tip:** Use localized updates when only a small region changes (e.g., around user interaction) to avoid recalculating the entire field.

---

## Performance Tips

1. **Use `set_cell()` for individual updates** - Clear and safe
2. **Use `get_cell_mut()` for complex state modifications** - Avoids cloning
3. **Use `get_cell_unchecked_mut()` in verified loops** - Maximum performance
4. **Use `grid_mut()` for bulk operations** - Most efficient for many cells
5. **Use localized updates** - When only a region needs updating, use `UpdateConfig` to avoid processing the entire field

## Thread Safety

Note that `DiscreteRuleField` is not thread-safe by default. If you need concurrent modification:
- Use external synchronization (Mutex, RwLock)
- Or implement a parallel update strategy with separate read/write phases
