
// -------------------------------------------------------------------------------------------------
// These are the standard weights for calculating perceived luminance from RGB channels.
// The weights are chosen to match the sensitivity of the human eye to different colors.
const R_WEIGHT: f32 = 0.299;
const G_WEIGHT: f32 = 0.587;
const B_WEIGHT: f32 = 0.114;
// -------------------------------------------------------------------------------------------------


/// Calculates perceived luminance from separate RGB channels. <br />
/// Perceived luminance means the brightness of a color as perceived by the human eye.
/// # Arguments
/// * `red` - The red channel value (0-255).
/// * `green` - The green channel value (0-255).
/// * `blue` - The blue channel value (0-255).
///
/// # Returns
/// A `u8` representing the weighted luminance.
pub fn rgb_pixel_luminance(red: u8, green: u8, blue: u8) -> u8 {
    (R_WEIGHT * red as f32 + G_WEIGHT * green as f32 + B_WEIGHT * blue as f32).round() as u8
}

/// Calculates perceived luminance from an RGBA color represented as a `u32`. <br />
/// Perceived luminance means the brightness of a color as perceived by the human eye. <br />
/// Ignores the alpha channel and uses only RGB values for the calculation.
/// # Arguments
/// * `rgba` - The color as a `u32` in RGBA format.
///
/// # Returns
/// A `u8` representing the weighted luminance.
pub fn rgba_pixel_luminance(rgba: u32) -> u8 {
    let red = ((rgba >> 24) & 0xFF) as u8;
    let green = ((rgba >> 16) & 0xFF) as u8;
    let blue = ((rgba >> 8) & 0xFF) as u8;

    (R_WEIGHT * red as f32 + G_WEIGHT * green as f32 + B_WEIGHT * blue as f32).round() as u8
}


/// Calculates perceived luminance of each pixel from a buffer of `u32` with RGBA colors. <br />
/// Ignores the alpha channel and uses only RGB values for the calculation.
/// # Arguments
/// * `dst` - A mutable slice where the calculated luminance values will be stored.
/// * `src` - A slice of RGBA pixels to calculate the luminance from, where each pixel is a `u32`.
/// # Panics
/// Panics if `dst` and `src` have different lengths.
pub fn rgba_buffer_luminance(dst: &mut [u8], src: &[u32])  {
    assert_eq!(dst.len(), src.len(), "Source and destination buffers must have the same length!");

    for (dst_value, &src_color) in dst.iter_mut().zip(src.iter()) {
        let red = ((src_color >> 24) & 0xFF) as u8;
        let green = ((src_color >> 16) & 0xFF) as u8;
        let blue = ((src_color >> 8) & 0xFF) as u8;
        *dst_value = (R_WEIGHT * red as f32 + G_WEIGHT * green as f32 + B_WEIGHT * blue as f32).round() as u8;
    }
}