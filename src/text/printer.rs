use crate::core::context::GraphContext;
use crate::primitives::helper_types::PixelColorTransformerFn;
use crate::primitives::plane::Dimensions2d;
use crate::primitives::point::Point;
use crate::text::font::PixelFont;
use crate::utils::pixel_copy::image_data;

const DEFAULT_TRANSPARENCY_COLOR: u32 = 0xff_ff_ff_ff;

#[derive(Debug, Clone, Copy)]
pub struct ColorProperties<'a> {
    /// Text color. If `color` provided, `color_transformer` is ignored
    pub color: Option<u32>,
    /// A custom function for transforming color of each pixel of a printed character
    /// @See `PixelColorTransformerFn`
    pub color_transformer: Option<PixelColorTransformerFn>,
    /// Any data that needs to be passed to `color_transformer()`
    pub data: Option<&'a Vec<u32>>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Align {
    Left,
    Right,
    Center,
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
///
/// # Returns
/// Dimensions of the printed line of text, in pixels
///
pub fn print_line<UserData>(
    ctx: &mut GraphContext<UserData>,
    dst_position: &Point,
    font: &PixelFont,
    color_props: &ColorProperties,
    text_str: &str,
) -> Dimensions2d {
    let mut result: Dimensions2d = Dimensions2d { w: 0, h: 0 };
    if text_str.len() == 0 {
        return result;
    }

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
            data: color_props.data,
        };

        image_data::copy(
            &mut ctx.frame_buf,
            &ctx.win.dimensions,
            &dst_point,
            &font.font_image_buf,
            &font.img_dimensions,
            &the_glyph,
            Some(&props),
        );

        dst_point.x += the_glyph.dimensions.w + font.spacing.kerning_px as u32;
        result.w += the_glyph.dimensions.w + font.spacing.kerning_px as u32;
    }

    // The last kerning is extra and should not be counted
    result.w -= font.spacing.kerning_px as u32;

    result.h = font.img_dimensions.h;
    return result;
}

/// Returns a vector of widths in the same order in which lines are ordered in the `text`
/// NB: The last element of the vector is special, it's an extra copy of the width of the widest line!
fn get_line_widths(font: &PixelFont, text: &[&str]) -> Vec<usize> {
    // Initialize the resulting vector
    let mut line_widths: Vec<usize> = vec![0; text.len() + 1];

    let kerning = font.spacing.kerning_px;

    let mut line_count: usize = 0;
    let mut max_found_width: usize = 0; // width (in pixels) of the widest (longest) line in the text

    let mut text_line_chars_count: usize;

    for text_line in text {
        for ch in text_line.chars() {
            line_widths[line_count] += font.get_glyph(&ch).dimensions.w as usize;
        }
        text_line_chars_count = text_line.chars().count();
        line_widths[line_count] += if text_line_chars_count > 0 {
            kerning as usize * (text_line_chars_count - 1)
        } else {
            0
        };

        // Figure out the longest line
        if line_widths[line_count] > max_found_width {
            max_found_width = line_widths[line_count];
        }

        line_count += 1;
    }

    // write the longest line to the last element of the vector
    line_widths[text.len()] = max_found_width;

    return line_widths;
}

/// Prints an array of strings to the screen at a specified on-screen position using a specified font.
/// # Parameters
///
/// - `ctx`: A context to which where the text will be rendered.
/// - `dst_position`: A position where the text rendering should start. This determines the top-left corner of the text.
/// - `font`: A font to be used for rendering the text.
/// - `color_props`: Defines the color(s) of the printed text
/// - `text`: The lines of text to be rendered. Each element in the array is printer on a new line.
/// - `alignment`: The alignment of the text. Can be `Align::Left`, `Align::Right`, or `Align::Center`.
///
/// # Returns
/// Dimensions of the printed line of text, in pixels
///
pub fn print<UserData>(
    ctx: &mut GraphContext<UserData>,
    dst_position: &Point,
    font: &PixelFont,
    color_props: &ColorProperties,
    text: &[&str],
    alignment: Align,
) -> Dimensions2d {
    // Do nothing if there was no text provided
    if text.len() < 1 {
        return Dimensions2d { w: 0, h: 0 };
    };

    let mut position = *dst_position;
    let original_position = position.clone();

    let mut result: Dimensions2d = Dimensions2d { w: 0, h: 0 };

    // height of the line of text + leading
    let full_line_height = font.img_dimensions.h + font.spacing.leading_px as u32;

    let mut line_index: usize = 0;

    let mut line_widths = get_line_widths(font, text);
    let longest_line_width = line_widths.pop().unwrap_or(0);

    for text_line in text {
        position.x = match alignment {
            Align::Right => {
                original_position.x + (longest_line_width - line_widths[line_index]) as u32
            }
            Align::Center => {
                original_position.x + ((longest_line_width - line_widths[line_index]) / 2) as u32
            }
            Align::Left => original_position.x,
        };

        if text_line.chars().count() > 0 {
            print_line(ctx, &position, font, color_props, text_line);
        }
        position.y += full_line_height;
        result.h += full_line_height;
        line_index += 1;
    }
    result.w = longest_line_width as u32;

    return result;
}
