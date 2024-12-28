/// Converts a given RGBA color to ABGR format.
pub fn rgba_color_to_abgr(rgba_color: u32) -> u32 {
    // Extract individual color channels from RGBA
    let r = (rgba_color >> 24) & 0xFF;
    let g = (rgba_color >> 16) & 0xFF;
    let b = (rgba_color >> 8) & 0xFF;
    let a = rgba_color & 0xFF;

    // Reassemble the color in ABGR format
    (a << 24) | (b << 16) | (g << 8) | r
}

/// Converts a given RGBA color to 0RGB format.
pub fn rgba_color_to_0rgb(rgba_color: u32) -> u32 {
    let r = (rgba_color >> 24) & 0xFF;
    let g = (rgba_color >> 16) & 0xFF;
    let b = (rgba_color >> 8) & 0xFF;

    // Reassemble in 0RGB format
    (0 << 24) | (r << 16) | (g << 8) | b
}
