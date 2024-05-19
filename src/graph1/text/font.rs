use std::collections::HashMap;

use crate::graph1::primitives::primitives::{Dimensions2d, Point, RectArea};

pub const DEFAULT_CHAR_ORDER: &str = " ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";
pub const DEFAULT_KERNING_PX: u8 = 1;


#[derive(Debug)]
pub struct PixelFont<'a> {
    /// Buffer with the font source image
    pub font_image_buf: &'a Vec<u32>,

    /// Width of the source image with font
    pub image_w: u32,

    /// Height of the source image with font
    pub image_h: u32,

    /// Order in which characters appear in the font.
    /// @See e.g.: `DEFAULT_CHAR_ORDER`
    pub char_order: &'a str,

    /// Default horizontal spacing between characters in the file
    pub kerning_px: u8,

    // FIXME: add some implementation for a dummy char that is shown when an unknown character is requested for rendering
    // pub default_char: PixelChar,
    /// A map of a character to a glyph width
    char_descriptions: HashMap<char, u8>,

    glyphs: HashMap<char, RectArea>,
}

impl<'a> PixelFont<'a> {
    /// Creates a new `PixelFont` instance.
    ///
    /// # Parameters
    ///
    /// - `font_image_buf`: Buffer with the font source image.
    /// - `image_w`: Width of the source image with font.
    /// - `image_h`: Height of the source image with font.
    /// - `char_order`: Order in which characters appear in the font. See `DEFAULT_CHAR_ORDER` for an example.
    /// - `default_kerning_px`: Default horizontal spacing between characters in the file.
    /// - `glyph_widths_px`: Pixel character glyph width, in pixels
    ///
    /// # Returns
    ///
    /// A new instance of `PixelFont`.
    pub fn new(
        font_image_buf: &'a Vec<u32>,
        image_w: u32,
        image_h: u32,
        char_order: &'a str,
        kerning_px: u8,
        glyph_widths_px: HashMap<char, u8>,
    ) -> Self {
        let mut glyphs: HashMap<char, RectArea> = HashMap::new();
        let mut char_count = 0;

        let mut width_count: u32 = 0;

        for character in char_order.chars() {
            let width = *glyph_widths_px.get(&character).unwrap();

            let glyph = RectArea {
                top_left: Point {
                    x: width_count,
                    y: 0,
                },
                dimensions: Dimensions2d {
                    w: width as u32,
                    // h: description.h as u32
                    h: 9,
                },
            };

            width_count += glyph.dimensions.w + kerning_px as u32;
            glyphs.insert(character, glyph);
            char_count += 1;
        }

        return Self {
            font_image_buf,
            image_w,
            image_h,
            char_order,
            kerning_px,
            char_descriptions: glyph_widths_px,
            glyphs,
        };
    }

    pub fn get_glyph(&self, character: &char) -> &RectArea {
        if self.glyphs.contains_key(character) {
            return self.glyphs.get(character).unwrap();
        }
        panic!("TODO: Return a default glyph here if a requested character does not have a glyph!!");
    }
}
