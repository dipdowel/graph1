# Field Utilities

This module provides two complementary 2D field systems for spatial computations and simulations.

---

## Field XY Module (`field_xy.rs`)

Provides a continuous 2D field system for managing and influencing a grid of points in space with real-time influence calculations.

### Core Components

#### `FieldXY`
A 2D field containing a grid of points that can be dynamically influenced. The field automatically distributes points in a grid pattern within a defined rectangular area and applies influence calculations through a customizable function.

#### `FieldXYPoint`
Individual points in the field with the following properties:
- **Location**: On-screen position in 2D space
- **Index**: Both 1D and 2D indexing for efficient access
- **Sensitivity**: Controls how strongly the point responds to influences
- **State**: Generic state storage for custom per-point data

#### `FieldXYInfluencer`
An entity that affects field points through a customizable influence function. Each influencer has:
- **Location**: Position in the field
- **Magnitude**: Strength and direction of influence (x, y components)
- **Function**: Custom logic defining how the influencer affects points

### Features

- Grid-based point distribution within a defined rectangular area
- Customizable influence functions to define point behavior
- Per-point state management with generic type support
- Configurable sensitivity for each point
- Window dimension awareness for relative calculations
- Interior mutability for efficient point updates during influence calculations

### Use Cases

- Interactive visualizations
- Particle systems
- Force fields and physics simulations
- Mouse cursor effects
- Spatial influence calculations
- Grid-based animations

### Example Workflow

1. Create a field with a rectangular area and desired number of points
2. Define an influencer with a custom influence function
3. Update the influencer's location (e.g., tracking mouse cursor)
4. Call `influence()` to apply the influence function to all points
5. Read point states and locations to render or process results

---

## Discrete Rule Field Module (`discrete_rule_field.rs`)

Provides a discrete 2D cellular automaton system for rule-based grid simulations with double buffering.

### Core Components

#### `DiscreteRuleField<CellState>`
A 2D grid of cells that evolve according to user-defined rules. The field uses double buffering to ensure all cells update simultaneously based on the previous state.

#### `Cell<CellState>`
Individual cells in the grid with customizable state that can store any cloneable data (colors, brightness, flags, composite structures, etc.).

#### `RuleSet<CellState>`
Defines how cells should update, including:
- **Neighborhood Type**: Which surrounding cells to inspect
- **Rule Function**: Custom logic that computes the next state based on neighbors

#### Rule Management (Index-Based System)
The field uses an efficient index-based system for managing rules:
- **Default Rule**: Applied to all cells by default
- **Override Rules**: Sparse storage for cells with custom rules
- **Rule Indices**: Fast O(1) lookup mapping each cell to its rule

This allows efficient memory usage: only cells with custom rules consume extra memory, while maintaining O(1) access performance.

#### `BoundaryPolicy`
Defines edge behavior:
- **Clamp**: Edge cells repeat (coordinates clamped to grid bounds)
- **Wrap**: Toroidal topology (coordinates wrap around)
- **Mirror**: Reflect at boundaries

#### `UpdateConfig`
Controls which cells get updated during each time step:
- **update_whole_field**: When true, all cells are processed
- **update_neighborhood**: The neighborhood type around the center to update
- **update_neighborhood_center**: The focal point of the update region

This allows efficient localized updates when only a portion of the field needs to be recalculated (e.g., around a user interaction point).

### Features

- **Double buffering**: Ensures synchronous updates (all cells update based on the same previous state)
- **Localized updates**: Option to update only cells within a specific neighborhood (performance optimization)
- **Flexible neighborhoods**: Immediate (Moore), Orthogonal (Von Neumann), Diagonal, Square, Circle, Diamond
- **Customizable rules**: User-defined functions with full access to grid state
- **Dynamic per-cell rules**: Set custom rules for individual cells at runtime with `set_cell_rule()`
- **Boundary policies**: Clamp, Wrap, or Mirror for handling edge cases
- **Efficient rule storage**: Index-based system with O(1) access and sparse override storage
- **Generic cell state**: Store any cloneable data type
- **Cache-friendly**: Row-major storage and scanline processing
- **Cell iteration and modification**: Multiple methods for reading and modifying cell states

### Use Cases

- Conway's Game of Life and other cellular automata
- Procedural texture generation
- Wave propagation simulations
- Image filters and stencil operations
- Fire, fluid, or particle simulations
- Reaction-diffusion systems
- Grid-based game mechanics

### Example Workflow

1. Define a cell state type (e.g., `struct State { alive: bool }`)
2. Create a rule function that examines neighbors and returns new state
3. Create a `RuleSet` with your rule and desired neighborhood type
4. Initialize a `DiscreteRuleField` with `new_uniform()` and dimensions
5. Optionally modify individual cells using `set_cell()` or `get_cell_mut()`
6. Optionally set custom rules for specific cells using `set_cell_rule()`
7. Call `update()` to evolve the entire field, or `update(Some(UpdateConfig::new(...)))` for localized updates
8. Iterate over cells using `grid()` or `grid_mut()` to read/modify states
9. Read cell states to visualize or process results

### Example: Simple Averaging Rule

```rust
fn averaging_rule(
    grid: &DiscreteRuleField<MyState>,
    cell_coords: Point<usize>,
    neighborhood_type: NeighborhoodType,
    boundary_policy: BoundaryPolicy,
) -> MyState {
    let neighbors = grid.get_neighbors(cell_coords, neighborhood_type);
    let sum: u32 = neighbors
        .iter()
        .filter_map(|&coord| grid.get_cell(coord))
        .map(|cell| cell.state.value as u32)
        .sum();
    let avg = sum / neighbors.len() as u32;
    MyState { value: avg as u8 }
}
```

### Example: Localized Updates

```ignore
// Update the entire field
field.update(None);

// Or update only a specific region (e.g., around user interaction)
let mouse_position = Point::new(50, 50);
let localized_config = UpdateConfig::new(
    NeighborhoodType::Circle { radius: 10 },
    mouse_position,
    false, // don't update whole field
);
field.update(Some(localized_config));

// You can also set a persistent update configuration
let persistent_config = UpdateConfig::new(
    NeighborhoodType::Circle { radius: 15 },
    Point::new(75, 75),
    false,
);
field.set_update_config(persistent_config);
// Now all subsequent calls to field.update(None) will use this configuration
field.update(None);
```

### Example: Dynamic Per-Cell Rules

```ignore
// Create field with default rule
let default_rule = RuleSet::new(NeighborhoodType::Immediate, averaging_rule);
let mut field = DiscreteRuleField::new_simple(
    dimensions,
    initial_state,
    BoundaryPolicy::Wrap,
    default_rule,
    NeighborhoodType::Immediate,
)?;

// Set custom rule for specific cells (e.g., heat sources)
fn heat_source_rule(...) -> MyState {
    MyState { temperature: 100.0 } // Always hot
}

let heat_source_rule_set = RuleSet::new(NeighborhoodType::Immediate, heat_source_rule);
field.set_cell_rule(Point::new(10, 10), heat_source_rule_set)?;

// Reset a cell back to default rule
field.reset_cell_rule(Point::new(10, 10))?;

// Query which rule a cell uses
let rule = field.get_cell_rule(Point::new(10, 10));
```

---

## Choosing Between Field Types

- **Use `FieldXY`** when you need:
  - Continuous spatial influence (mouse effects, force fields)
  - Real-time interactive effects
  - Points with individual sensitivity
  - Non-uniform point distribution

- **Use `DiscreteRuleField`** when you need:
  - Cellular automata or grid-based simulations
  - Synchronous state updates (double buffering)
  - Rule-based evolution over time
  - Neighborhood-dependent computations
  - Conway's Game of Life-style systems

