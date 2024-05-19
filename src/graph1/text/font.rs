use std::collections::HashMap;
use crate::graph1::primitives::primitives::{Dimensions2d, ImageData0RGB, Point, RectArea};
use crate::graph1::text::types::HashMapCharDescriptions;

pub const DEFAULT_CHAR_ORDER: &str = " ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";
pub const DEFAULT_KERNING_PX: u8 = 1;

/// Location and vertical offset (margin-top) of a single renderable pixel-font character.
#[derive(Debug)]
pub struct PixelChar {
    /// Width of a character
    pub w: u8,

    /// Height of a character
    pub h: u8,

    /// How many pixels should this symbol be pushed down.
    /// Should be used for lowercase letters like `a` or `c` whose top-most pixel
    /// should be rendered a few pixels lower than the top-most pixel of chars such as `A` or `h`
    /// which are "taller"
    pub margin_top: u8,
}




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
    /// A map of a character to a set of character properties
    pub char_descriptions: HashMapCharDescriptions<'a>,

    glyphs: HashMap<char, RectArea>
    // pixel_char_bufs: HashMap<char, ImageBuffer>,
}

/*
    impl Person {
        fn new(name: &str, age: u16) -> Self {
            return Person {
                name: name.to_string(),
                age,
            };
        }

*/

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
    /// - `char_descriptions`: Pixel character properties, per character.
    ///
    /// # Returns
    ///
    /// A new instance of `PixelFont`.
    pub fn new (
        font_image_buf: &'a Vec<u32>,
        image_w: u32,
        image_h: u32,
        char_order: &'a str,
        kerning_px: u8,
        char_descriptions:  HashMapCharDescriptions<'a>,
    ) -> Self {
        //



        let mut glyphs: HashMap<char, RectArea> = HashMap::new();
        let mut char_count = 0;

        // let mut top_left:Point = Point{x:0,y:0};

        let mut width_count: u32 = 0;

        for character in char_order.chars() {
            let description = *char_descriptions.get(&character).unwrap();

            let glyph = RectArea {
                top_left: Point{
                    x: width_count,
                    y: 0
                },
                dimensions: Dimensions2d{
                    w: description.w as u32,
                    // h: description.h as u32
                    h: 9


                },
            };

            width_count += glyph.dimensions.w + kerning_px as u32;
            glyphs.insert(character, glyph);
            char_count += 1;

        }



        /*
        for (&char , &description) in  &char_descriptions {

            let glyph = RectArea {
                top_left: Point{
                    x: width_count,
                    y: 0
                },
                dimensions: Dimensions2d{
                    w: description.w as u32,
                    h: description.h as u32

            },
            };

            width_count = glyph.dimensions.w + kerning_px as u32;
            glyphs.insert(char, glyph);
            char_count += 1;
        }
*/
        // println!(">>> da glyphs! {:?}", glyphs);
        // println!(">>> da char_descriptions! {:?}", char_descriptions);

        return Self {
            font_image_buf,
            image_w,
            image_h,
            char_order,
            kerning_px,
            char_descriptions,
            glyphs
        };
    }

    pub fn get_glyph(&self, character: &char) -> &RectArea {
        // println!(">>> self.glyphs: {:?}",self.glyphs);

        if self.glyphs.contains_key(character) {

            // println!(">>> self.glyphs.get({character}).unwrap(): {:?}",self.glyphs.get(character).unwrap());

            return self.glyphs.get(character).unwrap();
        }
        panic!("TODO: Return a default character here!");
    }
}
