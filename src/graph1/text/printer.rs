use crate::graph1::graph1_core::context::GraphContext;
use crate::graph1::primitives::primitives::{Dimensions2d, Point, RectArea};
use crate::graph1::text::font::PixelFont;

/// Copies image data from a source buffer to a destination buffer, within specified areas and dimensions.
///
/// # Parameters
///
/// - `dst_buf`: The destination buffer for image data.
/// - `dst_dimensions`: Dimensions of the destination buffer.
/// - `dest_point`: A `Point` specifying the starting point in the destination buffer where the image data will be copied to.
///
/// - `src_buf`: The source buffer
/// - `src_dimensions`: Dimensions of the destination buffer.
/// - `src_region`: A `RectArea` specifying the rectangular area in the source buffer to copy.
///
pub fn copy_image_data(
    dst_buf: &mut [u32],
    dst_dimensions: &Dimensions2d,
    dst_point: &Point,
    src_buf: &[u32],
    src_dimensions: &Dimensions2d,
    src_region: &RectArea,
) {
    // Ensure the dimensions and starting points are within bounds
    if dst_point.x >= dst_dimensions.w
        || dst_point.y >= dst_dimensions.h
        || src_region.top_left.x >= src_dimensions.w
        || src_region.top_left.y >= src_dimensions.h
    {
        return;
    }

    let rect_width = src_region.dimensions.w;
    let rect_height = src_region.dimensions.h;

    for y in 0..rect_height {
        for x in 0..rect_width {
            // Calculate source index
            let src_x = src_region.top_left.x + x;
            let src_y = src_region.top_left.y + y;

            // TODO: Find out whether this check actually works as expected
            // Ensure the source coordinates are within the image bounds
            if src_x >= src_dimensions.w || src_y >= src_dimensions.h {
                continue;
            }

            let src_index = (src_y * src_dimensions.w + src_x) as usize;

            // Calculate destination index
            let dest_x = dst_point.x + x;
            let dest_y = dst_point.y + y;

            // TODO: Find out whether this check actually works as expected
            // Ensure the destination coordinates are within the screen bounds
            if dest_x >= dst_dimensions.w || dest_y >= dst_dimensions.h {
                continue;
            }

            let dest_index = (dest_y * dst_dimensions.w + dest_x) as usize;

            // Copy the pixel
            dst_buf[dest_index] = src_buf[src_index];
        }
    }
}

pub fn print(ctx: &mut GraphContext, dst_position: &Point, font: &PixelFont, text: &str) {
    // FIXME: remove `.unwrap()`. If a non-existent char is requested, return the default char!




    let dst_dimensions: Dimensions2d = Dimensions2d {
        w: ctx.win.w,
        h: ctx.win.h,
    };

    let mut dst_point: Point = Point {
        x: dst_position.x,
        y: dst_position.y,
    };


    for text_char in text.chars(){
        let the_glyph = font.get_glyph(&text_char);

        copy_image_data(
            ctx.buf_view,
            &dst_dimensions,
            &dst_point,
            font.font_image_buf,
            &Dimensions2d {
                w: font.image_w,
                h: font.image_h,
            },
            &the_glyph
        );

        // dst_point.x += the_glyph.dimensions.w + font.kerning_px as u32;
        dst_point.x += the_glyph.dimensions.w + 2;

    }




}
