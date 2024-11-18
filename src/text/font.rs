use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use crate::primitives::plane::{Dimensions2d, RectArea};
use crate::primitives::point::Point;

pub const DEFAULT_CHAR_ORDER: &str = " ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";

/// Space between glyphs in the font source image file
pub const DEFAULT_KERNING_PX: u8 = 1;

#[derive(Debug)]
/// Font spacing properties (typography)
pub struct Spacing {
    /// Horizontal spacing between characters
    pub kerning_px: u8,
    /// Vertical spacing between lines of characters
    pub leading_px: u8,
}

#[derive(Debug)]
pub struct PixelFontMeta {
    pub font_ver: u16,
    pub date_year: u16,
    pub date_month: u8,
    pub date_day: u8,
    pub font_name: String,
    pub author_signature: String,
}

#[derive(Debug)]
pub struct PixelFont {
    /// Buffer with the font source image
    pub font_image_buf: Vec<u32>,
    /// Width and height of the source image with font
    pub img_dimensions: Dimensions2d,
    /// Order in which characters appear in the font.
    /// @See e.g.: `DEFAULT_CHAR_ORDER`
    pub char_order: String,
    /// Character to display when a requested character is not present in the charset
    pub default_char: char,
    /// Font spacing properties (typography)
    pub spacing: Spacing,
    /// Map of `char` to where in `font_image_buf` its glyph can be found
    glyphs: HashMap<char, RectArea>,
    // FIXME: add some implementation for a dummy char that is shown when an unknown character is requested for rendering
    // pub default_char: PixelChar,
    meta: PixelFontMeta,
}

impl PixelFont {
    /// Creates a new `PixelFont` instance.
    ///
    /// # Parameters
    ///
    /// - `font_image_buf`: Buffer with the font source image.
    /// - `image_w`: Width of the source image with font.
    /// - `image_h`: Height of the source image with font.
    /// - `char_order`: Order in which characters appear in the font. See `DEFAULT_CHAR_ORDER` for an example.
    /// - `spacing`: Font spacing properties (typography)
    /// - `glyph_widths_px`: Pixel character glyph width, in pixels
    /// - `src_kerning_px`: Distance between characters in the source font image.
    ///
    /// # Returns
    ///
    /// A new instance of `PixelFont`.
    pub fn new(
        font_image_buf: Vec<u32>,
        img_dimensions: Dimensions2d,
        char_order: String,
        default_char: char,
        spacing: Spacing,
        glyph_widths_px: HashMap<char, u8>,
        src_kerning_px: u8,
        meta: PixelFontMeta,
    ) -> Self {
        let mut src_kerning_px: u32 = src_kerning_px as u32;
        if src_kerning_px == 0 {
            src_kerning_px = DEFAULT_KERNING_PX as u32;
        }

        let mut glyphs: HashMap<char, RectArea> = HashMap::new();

        let mut width_count: u32 = 0;

        // collect and save information about each glyph of the charset (x,y,w,h)
        for character in char_order.chars() {
            let width = *glyph_widths_px.get(&character).unwrap();

            let glyph = RectArea {
                top_left: Point {
                    x: width_count,
                    y: 0,
                },
                dimensions: Dimensions2d {
                    w: width as u32,
                    h: img_dimensions.h,
                },
                color: None,
            };

            width_count += glyph.dimensions.w + src_kerning_px;
            glyphs.insert(character, glyph);
        }

        Self {
            font_image_buf,
            img_dimensions,
            char_order,
            default_char,
            spacing,
            glyphs,
            meta,
        }
    }

    pub fn get_glyph(&self, character: &char) -> &RectArea {
        if self.glyphs.contains_key(character) {
            return self.glyphs.get(character).unwrap();
        }
        // Character not found, return glyph for the default character
        return self.glyphs.get(&self.default_char).unwrap();
    }
}

impl Display for PixelFont {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let PixelFontMeta {
            font_ver,
            date_year,
            date_month,
            date_day,
            font_name,
            author_signature,
        } = &self.meta;

        let output = format!(
            "[PixelFont] {} ver. {} | Author: {} | Created: {}-{}-{} ",
            font_name, font_ver, author_signature, date_day, date_month, date_year
        );

        f.write_str(&output)
    }
}
