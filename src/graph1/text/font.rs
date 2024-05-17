use std::collections::HashMap;
use std::string::ToString;

pub const DEFAULT_CHAR_ORDER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";
pub const DEFAULT_KERNING_PX: u8 = 1;
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

type CharDescriptorTable<'a> = HashMap<char, &'a PixelChar>;

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
    pub default_kerning_px: u8,

    // FIXME: add some implementation for a dummy char that is shown when an unknown character is requested for rendering
    // pub default_char: PixelChar,

    /// A map of a character to a set of character properties
    // pub char_descriptions: HashMap<char, &'a PixelChar>,
    pub char_descriptions: CharDescriptorTable<'a>
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


/*
impl PixelFont {
    fn new(
        /// Buffer with the font source image
        font_image_buf: &Vec<u32>,

        /// Width of the source image with font
        image_w: u32,

        /// Height of the source image with font
        image_h: u32,

        /// Order in which characters appear in the font.
        /// @See e.g.: `DEFAULT_CHAR_ORDER`
        char_order: &str,

        /// Default horizontal spacing between characters in the file
        default_kerning_px: u8,

        /// Pixel character properties, per character
        char_map: HashMap<char, &'a PixelChar>,


    ) ->Self {
        return PixelFont{
            font_image_buf
        }
    }

}
*/