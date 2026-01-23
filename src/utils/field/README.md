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

#### `RuleAssignment<CellState>`
Strategy for applying rules to the field:
- **Uniform**: All cells use the same rule
- **PerCell**: Each cell can have its own unique rule

#### `BoundaryPolicy`
Defines edge behavior:
- **Clamp**: Edge cells repeat (coordinates clamped to grid bounds)
- **Wrap**: Toroidal topology (coordinates wrap around)
- **Mirror**: Reflect at boundaries

### Features

- **Double buffering**: Ensures synchronous updates (all cells update based on the same previous state)
- **Flexible neighborhoods**: Immediate (Moore), Orthogonal (Von Neumann), Diagonal, Square, Circle, Diamond
- **Customizable rules**: User-defined functions with full access to grid state
- **Boundary policies**: Clamp, Wrap, or Mirror for handling edge cases
- **Per-cell or uniform rules**: Different cells can follow different rules
- **Generic cell state**: Store any cloneable data type
- **Cache-friendly**: Row-major storage and scanline processing

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
4. Initialize a `DiscreteRuleField` with dimensions and initial state
5. Call `update()` repeatedly to evolve the field over time
6. Read cell states to visualize or process results

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

