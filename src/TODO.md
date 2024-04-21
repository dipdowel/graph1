
## Bezier curves
1. See if `draw_bezier_curve()` can be deleted in favour of `draw_continuous_bezier_curve()`
2. Add mode for visualising the control points, that should help come up with shapes
3. Make the control points draggable by mouse!
4. Make it possible to dump the coordinates of all the start/end and control points into JSON into the console.
   That should make it easier creating cool curves and then hardcoding their points for animation, etc.

## Tools

### Add "Fill shape" tool.
Here's how it can work: 
- There should be a starting `Pixel` for filling in an arbitrary shape with a provided colour
- Start scanning the row on which the pixel is to the left and right, till we reach pixels of a different colour.
- Start scanning the column in a similar way till we reach pixels of different colours.
- figure out how to go on (ask some AI maybe?)

#### Nice to haves:
- Tolerance: how much the scanned pixels can deviate from the colour of the starting Pixel to still be considered of the same colour.
 