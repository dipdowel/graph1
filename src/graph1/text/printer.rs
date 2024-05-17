use crate::graph1::graph1_core::context::GraphContext;
use crate::graph1::primitives::primitives::{Dimensions2d, Point, RectArea};
use crate::graph1::text::font::PixelFont;

/// Copies a rectangular area from an image buffer to a screen buffer.
///
/// # Parameters
///
/// - `buf_view`: Mutable reference to the screen buffer.
/// - `screen_w`: Width of the screen.
/// - `screen_h`: Height of the screen.
/// - `buf_image`: Reference to the image buffer.
/// - `img_w`: Width of the image.
/// - `img_h`: Height of the image.
/// - `dest_point`: The point on the screen where the rectangle will be copied.
/// - `rect_area`: The rectangular area to copy from the image.
///
/// # Examples
///
/// ```
/// let mut screen_buffer = vec![0; screen_width * screen_height];
/// let image_buffer = vec![0; image_width * image_height];
/// let destination = Point { x: 10, y: 10 };
/// let area = RectArea {
///     top_left: Point { x: 5, y: 5 },
///     dimensions: Dimensions2d { w: 20, h: 20 },
/// };
///
/// copy_rect_to_screen(&mut screen_buffer, screen_width, screen_height, &image_buffer, image_width, image_height, destination, area);
/// ```
pub fn copy_rect_to_screen(
    buf_view: &mut [u32],      // Mutable reference to the screen buffer
    screen_w: u32,            // Width of the screen
    screen_h: u32,            // Height of the screen
    buf_image: &[u32],        // Reference to the image buffer
    img_w: u32,               // Width of the image
    img_h: u32,               // Height of the image
    dest_point: Point,        // The point on the screen where the rectangle will be copied
    rect_area: RectArea,      // The rectangular area to copy from the image
) {
    // Ensure the dimensions and starting points are within bounds
    if dest_point.x >= screen_w || dest_point.y >= screen_h ||
        rect_area.top_left.x >= img_w || rect_area.top_left.y >= img_h {
        return;
    }

    let rect_width = rect_area.dimensions.w;
    let rect_height = rect_area.dimensions.h;

    for y in 0..rect_height {
        for x in 0..rect_width {
            // Calculate source index
            let src_x = rect_area.top_left.x + x;
            let src_y = rect_area.top_left.y + y;

            // Ensure the source coordinates are within the image bounds
            if src_x >= img_w || src_y >= img_h {
                continue;
            }

            let src_index = (src_y * img_w + src_x) as usize;

            // Calculate destination index
            let dest_x = dest_point.x + x;
            let dest_y = dest_point.y + y;

            // Ensure the destination coordinates are within the screen bounds
            if dest_x >= screen_w || dest_y >= screen_h {
                continue;
            }

            let dest_index = (dest_y * screen_w + dest_x) as usize;

            // Copy the pixel
            buf_view[dest_index] = buf_image[src_index];
        }
    }
}
pub fn print(
    ctx: &mut GraphContext,
    dst_position: &Point,
    font:&PixelFont,
    text:&str
){

    // FIXME: remove `.unwrap()`. If a non-existent char is requested, return the default char!
    let text_char = text.chars().next().unwrap();

    // if font.char_map.contains_key(&text_char) { }

        let char_descriptor = *font.char_descriptions.get(&text_char).unwrap();
        // char_descriptor.w
        // char_descriptor.h
        // char_descriptor.margin_top

    let char_descriptor = *font.char_descriptions.get(&'A').unwrap();

        // ctx.win.w
        //  ctx.win.h_usize

        // let a = ctx.buf_view[0];

        copy_rect_to_screen(
            ctx.buf_view,
            ctx.win.w,
            ctx.win.h,
            font.font_image_buf,
            font.image_w,
            font.image_h,
            *dst_position,

            RectArea{
                dimensions:Dimensions2d{
                    w: char_descriptor.w as u32,
                    h: char_descriptor.h as u32,
                },
                top_left:Point{
                    x:6,
                    y:0
                }
            }

        )


    }









