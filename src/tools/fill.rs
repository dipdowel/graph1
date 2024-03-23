//
//
// /// Draws a rectangle.
// /// If rectangle bleeds beyond the window it gets truncated at window's width and height.
// /// * `options` - Config of the rectangle
// /// * `buffer` - A reference to the screen/window buffer
// pub fn rect(buffer: &mut Vec<u32>, color:&u32) {
//
//
//
//     let RectOptions {
//         x,
//         y,
//         w,
//         h,
//         color,
//         win_w,
//         win_h,
//     } = options;
//
//     // Dereference the options
//     let start_x = *x;
//     let start_y = *y;
//     let width = *w;
//     let height = *h;
//     let win_width = *win_w;
//     let win_height = *win_h;
//
//     // Nothing to draw here
//     if width == 0 || height == 0 {
//         return;
//     }
//
//     let end_x = start_x + width;
//     let end_y = start_y + height;
//
//     let mut x = start_x;
//     let mut y = start_y;
//
//     // Which pixel in the vector should be filled in next.
//     let mut pixel_index: usize;
//
//     loop {
//         pixel_index = (y * win_width + x) as usize;
//         buffer[pixel_index] = *color;
//         x += 1;
//
//         if x == end_x || x == win_width {
//             y += 1;
//             x = start_x;
//         };
//
//         if y == end_y || y == win_height {
//             break;
//         }
//     }
// }
