# Field XY Module

Provides a 2D field system for managing and influencing a grid of points in space.

## Core Components

### `FieldXY`
A 2D field containing a grid of points that can be dynamically influenced. The field automatically distributes points in a grid pattern within a defined rectangular area and applies influence calculations through a customizable function.

### `FieldXYPoint`
Individual points in the field with the following properties:
- **Location**: On-screen position in 2D space
- **Index**: Both 1D and 2D indexing for efficient access
- **Sensitivity**: Controls how strongly the point responds to influences
- **State**: Generic state storage for custom per-point data

### `FieldXYInfluencer`
An entity that affects field points through a customizable influence function. Each influencer has:
- **Location**: Position in the field
- **Magnitude**: Strength and direction of influence (x, y components)
- **Function**: Custom logic defining how the influencer affects points

## Features

- Grid-based point distribution within a defined rectangular area
- Customizable influence functions to define point behavior
- Per-point state management with generic type support
- Configurable sensitivity for each point
- Window dimension awareness for relative calculations
- Interior mutability for efficient point updates during influence calculations

## Use Cases

- Interactive visualizations
- Particle systems
- Force fields and physics simulations
- Mouse cursor effects
- Spatial influence calculations
- Grid-based animations

## Example Workflow

1. Create a field with a rectangular area and desired number of points
2. Define an influencer with a custom influence function
3. Update the influencer's location (e.g., tracking mouse cursor)
4. Call `influence()` to apply the influence function to all points
5. Read point states and locations to render or process results
