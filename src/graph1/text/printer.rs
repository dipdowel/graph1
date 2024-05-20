use crate::graph1::graph1_core::context::GraphContext;
use crate::graph1::primitives::primitives::{Dimensions2d, Point, RectArea};
use crate::graph1::text::font::PixelFont;
use crate::graph1::utils::pixel_copy::image_data;

pub fn print(ctx: &mut GraphContext, dst_position: &Point, font: &PixelFont, text: &str) {
    let dst_dimensions: Dimensions2d = Dimensions2d {
        w: ctx.win.w,
        h: ctx.win.h,
    };

    let mut dst_point: Point = Point {
        x: dst_position.x,
        y: dst_position.y,
    };



    for text_char in text.chars() {
        let the_glyph = font.get_glyph(&text_char);

        fn transformer(color: u32, x: u32, y: u32, w:u32, h:u32) -> u32 {
            if y < (h /2) {
                return 0x00_ff_ff_ff;
            }
            return 0x00_ff_55_ff;
        }

        let props: image_data::ImageDataCopyProps = image_data::ImageDataCopyProps {
            // fill_color: Some(0x00_ff_77_ff),
            fill_color: None,
            // transparency_color: None,
            transparency_color: Some(0x00_ff_ff_ff),
            color_transformer: Some(transformer),
            // color_transformer: None
        };

        image_data::copy(
            ctx.buf_view,
            &dst_dimensions,
            &dst_point,
            font.font_image_buf,
            &Dimensions2d {
                w: font.image_w,
                h: font.image_h,
            },
            &the_glyph,
            Some(&props),
            // None
        );

        // dst_point.x += the_glyph.dimensions.w + ;
        dst_point.x += the_glyph.dimensions.w + font.kerning_px as u32;
    }
}
