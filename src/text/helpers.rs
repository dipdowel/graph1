use crate::text::font::PixelFont;

/// Check if a font is monospaced (all glyphs have the same width)
/// # Arguments
/// * `font` - The pixel font to be checked
/// # Returns
/// `true` if the font is monospaced, `false` otherwise
pub fn is_font_monospaced(font: &PixelFont) -> bool {
    if font.char_order.len() == 0 {
        return false;
    }

    let first_char = font.char_order.chars().nth(0).unwrap();
    let first_glyph = font.get_glyph(&first_char);
    let first_width = first_glyph.dimensions.w;

    for c in font.char_order.chars() {
        let glyph = font.get_glyph(&c);
        if glyph.dimensions.w != first_width {
            return false;
        }
    }
    true
}