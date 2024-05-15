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

#[derive(Debug)]
pub struct PixelFont<'a> {

    /// Order in which characters appear in the font.
    /// @See e.g.: `DEFAULT_CHAR_ORDER`
    pub char_order: &'a str,

    /// Width of the source image with font
    pub image_w: u32,

    /// Height of the source image with font
    pub image_h: u32,

    /// Default horizontal spacing between characters in the file
    pub default_kerning_px: u8,

    // FIXME
    // pub default_char: PixelChar,

    /// Pixel character properties, per character
    pub char_map: HashMap<char, &'a PixelChar>,
}