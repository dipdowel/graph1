# Learn:
- Lifetime elision: https://www.youtube.com/results?search_query=rust+lifetime+elision
- Lifetime: https://chatgpt.com/g/g-X6K3ISxr6-rusty/c/a2027242-b234-489c-8660-05533244faf0
- Lifetime finally explained! https://www.youtube.com/watch?v=juIINGuZyBc
- (!) How a Rust binary is executed by the OS: https://www.youtube.com/watch?v=7_o-YRxf_cc,  https://www.youtube.com/watch?v=rDoqT-a6UFg
# TODO:
- Refactor `graph1/primitives/primitives` into separate files per topic!
- Check if introducing primitive Line (Point, Point) and LineColor (Pixel, Point) would make sense
- Try to extract different experiments into scene functions instead of commenting them out in `main()`

## Bezier curves
+ See if `draw_bezier_curve()` can be deleted in favour of `draw_continuous_bezier_curve()`
1. Add mode for visualising the control points, that should help come up with shapes
2. Make the control points draggable by mouse!
3. Make it possible to dump the coordinates of all the start/end and control points into JSON into the console.
   That should make it easier creating cool curves and then hardcoding their points for animation, etc.

## Text rendering engine

- Modify header. We need to include the widths of characters into the header as well. 
  - Maybe the character order should also be included into the file?
  - Maybe the char-to-glyph map should also be included into the file? 
  - Is there a way to serialise a hashmap?
- Make a stand-alone convertor from PNG (and other formats too, maybe?) to RBF
Inputs:
- A pixel font.
- A location from the draft buffer of how a single pixel needs to be rendered. It can be a large animated cube or a single pixel!
- A buffer of text to render
- Destination on the frame buffer, where the first letter should appear. 
```
--------------------------------------------------
C&C Red Alert by N3tRunn3r
https://www.dafont.com/c-c-red-alert-inet.font
--------------------------------------------------
```



REPOS:
- graph1
- graph1_examples
- graph1_demo
- graph1_tools  // to creates assets like fonts?



## Tools
### Add "Fill shape" tool. 
- Consider the existing flood fill algorithm! 
- Consider a simplified and faster algorithm for symmetrical shapes which are have a guaranteed closed perimeter.

- Here's how my own fill algorithm can work: 
- There should be a starting `Pixel` for filling in an arbitrary shape with a provided colour
- Start scanning the row on which the pixel is to the left and right, till we reach pixels of a different colour.
- Start scanning the column in a similar way till we reach pixels of different colours.
- figure out how to go on (ask some AI maybe?)

#### Nice to haves:
- Tolerance: how much the scanned pixels can deviate from the colour of the starting Pixel to still be considered of the same colour.
- Think about playing and visualising audio 

