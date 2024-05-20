use crate::graph1::graph1_core::context::GraphContext;
use crate::graph1::primitives::primitives::{Dimensions2d, PixelColorTransformerFn, Point};
use crate::graph1::text::font::PixelFont;
use crate::graph1::utils::pixel_copy::image_data;

const DEFAULT_TRANSPARENCY_COLOR: u32 = 0x00_ff_ff_ff;

pub struct ColorProperties {
    /// Text color. If `color` provided, `color_transformer` is ignored
    pub color: Option<u32>,
    /// A custom function for transforming color of each pixel of a printed character
    /// @See `PixelColorTransformerFn`
    pub color_transformer: Option<PixelColorTransformerFn>,
}

///
/// Prints a line of text to the screen at a specified on-screen position using a specified font.
/// # Parameters
///
/// - `ctx`: A context to which where the text will be rendered.
/// - `dst_position`: A position where the text rendering should start. This determines the top-left corner of the text.
/// - `font`: A font to be used for rendering the text.
/// - `color_props`: Defines the color(s) of the printed text
/// - `text_str`: A line of text to be rendered.
pub fn print_line(
    ctx: &mut GraphContext,
    dst_position: &Point,
    font: &PixelFont,
    color_props: &ColorProperties,
    text_str: &str,
) {
    let dst_dimensions: Dimensions2d = Dimensions2d {
        w: ctx.win.w,
        h: ctx.win.h,
    };

    let mut dst_point: Point = Point {
        x: dst_position.x,
        y: dst_position.y,
    };

    for text_char in text_str.chars() {
        let the_glyph = font.get_glyph(&text_char);

        let props: image_data::ImageDataCopyProps = image_data::ImageDataCopyProps {
            transparency_color: Some(DEFAULT_TRANSPARENCY_COLOR),
            fill_color: color_props.color,
            color_transformer: color_props.color_transformer,
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

        dst_point.x += the_glyph.dimensions.w + font.kerning_px as u32;
    }
}

/// Prints an array of strings to the screen at a specified on-screen position using a specified font.
/// # Parameters
///
/// - `ctx`: A context to which where the text will be rendered.
/// - `dst_position`: A position where the text rendering should start. This determines the top-left corner of the text.
/// - `font`: A font to be used for rendering the text.
/// - `color_props`: Defines the color(s) of the printed text
/// - `text`: The lines of text to be rendered. Each element in the array is printer on a new line.
pub fn print(
    ctx: &mut GraphContext,
    dst_position: &Point,
    font: &PixelFont,
    color_props: &ColorProperties,
    text: &[&str],
) {
    let mut position = *dst_position;

    for text_line in text {
        print_line(ctx, &position, font, color_props, text_line);
        position.y += (font.image_h + font.leading_px as u32);
    }
}
