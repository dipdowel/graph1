use std::collections::HashMap;

use crate::graph1::primitives::primitives::{Dimensions2d, Point, RectArea};

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
pub struct PixelFont {
    /// Buffer with the font source image
    pub font_image_buf: Vec<u32>,

    /// Width of the source image with font
    pub image_w: u32,

    /// Height of the source image with font
    pub image_h: u32,

    /// Order in which characters appear in the font.
    /// @See e.g.: `DEFAULT_CHAR_ORDER`
    pub char_order: String,

    /// Character to display when a requested character is not present in the charset
    pub default_char: char,

    /// Font spacing properties (typography)
    pub spacing: Spacing,

    /// A map of a character to a glyph width
    glyph_widths_px: HashMap<char, u8>,

    /// Map of `chat` to where in `font_image_buf` its glyph can be found
    glyphs: HashMap<char, RectArea>,

    // FIXME: add some implementation for a dummy char that is shown when an unknown character is requested for rendering
    // pub default_char: PixelChar,

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
        image_w: u32,
        image_h: u32,
        char_order: String,
        default_char: char,
        spacing: Spacing,
        glyph_widths_px: HashMap<char, u8>,
        src_kerning_px: u8,
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
                    h: image_h,
                },
            };

            width_count += glyph.dimensions.w +  src_kerning_px;
            glyphs.insert(character, glyph);
        }

        return Self {
            font_image_buf,
            image_w,
            image_h,
            char_order,
            default_char,
            spacing,
            glyph_widths_px,
            glyphs,
        };
    }

    pub fn get_glyph(&self, character: &char) -> &RectArea {
        if self.glyphs.contains_key(character) {
            return self.glyphs.get(character).unwrap();
        }
        // Character not found, return glyph for the default character
        return self.glyphs.get(&self.default_char).unwrap();
    }
}
