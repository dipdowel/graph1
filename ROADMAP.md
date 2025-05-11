# The Roadmap of Graph1: what to development next

**NB:** The roadmap is a work in progress.

## Fonts and text rendering
- Rework scaling, so that a [CBF](https://github.com/dipdowel/compact-bitmap-font) font can be scaled by 3x, not only by a power of 2
- Add mode [CBF](https://github.com/dipdowel/compact-bitmap-font) fonts to the library

## Fast memory copying

### One source, one destination

- Copy a specified rectangular region of the window to another location in the window
- Copy a specified section of the framebuffer to another location in the same framebuffer
- Copy a section of memory between the framebuffer and any other buffer (in both directions)
- Etc.

### One source, multiple destinations

- Copy a specified rectangular region of the window to multiple other locations in the window
- Copy a specified section of the framebuffer to multiple other locations in the same framebuffer
- Copy a section of memory from any buffer to multiple destinations in the framebuffer
- Etc.

### Copying with transformation

Apply a transformation function to the copied data in order to:

- Shift colors
- Apply an effect such as blur, desaturation, etc.
- Any other on-the-fly modifications

## `Grid`

Introduction of `Grid` is planned to simplify access to preconfigured rectangular areas on the screen.

### Use cases:

- UI layouts
- Game Tiles / Tilemaps
- Animation Paths and Control
- Bitmap scale-up
- Design Alignment and Snapping
- Procedural Content Placement
- Etc.

[WindowContext::quadrants](https://github.com/dipdowel/graph1/blob/develop/src/core/context/window/window_quadrants.rs)
is essentially a 2*2 grid, but it's not flexible and resizes only together with the whole window.
Introduction of an m*n `Grid` should provide more flexibility and customization.

## Transformations

Implement the classic matrix transformations on the 2D plane:

- Rotation
- Reflection
- Shear / skew
- Scaling
- Translation

Consider allowing the transformations to be applied during the fast memory copying.

### Advanced transformations:

- Projection
- Perspective transformation

## Coordinate systems

In order to facilitate the transformations, animation paths, precise object positioning, etc., a set of coordinate
systems should be introduced.

### Types

- Cartesian coordinate system with 4 quadrants
- Polar coordinate system
- Conversions between the Cartesian and Polar systems

### Locality

- Local coordinate system (within a specified rectangle in the window
- Global coordinate system (the whole window)
- Conversion between the local and the global systems

It should be possible to animate a local coordinate system (both Cartesian and Polar).

(A configurable location of the origin is needed)

## Effects and filters

### Perlin noise

Perlin noise is currently a PoC. It should be improved and finalized

### Blur

- Box blur
- Gaussan blur

### Blend

- Multiply
- Add
- Difference
- Screen
- Etc.

### Colors / Image

- Invert colors
- Downscale the palette ("posterize")
- Etc.

## Bézier curves
Check if the Bézier curves drawing can be multithreaded. If it's possible, implement it!


## Parametric curves
- TBD

## Multithreading

- See where else multithreading can be implemented
- Multithreaded color adapters
- Filters / effects
- Flood fill
- Etc.

## Documentation

- Improve
- Keep up to date

## Derive macros inspection
Check whether all the structs have all the needed derive macros for the essential traits (Default, Debug, Clone, etc).


## Native TTF-support (good luck with that)
- TBD

## Procedural shape generation
- TBD