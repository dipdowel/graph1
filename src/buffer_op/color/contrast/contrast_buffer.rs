use crate::buffer_op::color::structs::Contrast;

/// Applies contrast adjustment to a buffer of RRGGBBAA pixels using a pre-computed multiplier.
///
/// This function adjusts the contrast of each pixel by scaling the difference between
/// each color channel and middle gray (128). The adjustment preserves the alpha channel.
///
/// The formula for each channel is:
/// ```text
/// adjusted = ((value - 128) * contrast) + 128
/// ```
///
/// # Performance
/// - Uses 8.8 fixed-point arithmetic for speed (avoids floating-point operations)
/// - Multiplier value is kept in a register
/// - Alpha channel is preserved unchanged
/// - Early exit for identity contrast (1.0)
///
/// # Arguments
/// * `buffer` - Mutable slice of RRGGBBAA pixels (R in MSB, A in LSB)
/// * `contrast` - Pre-computed contrast adjustment with cached fixed-point multiplier
#[inline]
pub fn contrast_buffer(buffer: &mut [u32], contrast: Contrast) {
    // Early exit for no contrast change
    if (contrast.as_f32() - 1.0).abs() < f32::EPSILON {
        return;
    }

    let contrast_fixed = contrast.fixed_multiplier();

    for pixel in buffer.iter_mut() {
        *pixel = adjust_contrast_pixel(*pixel, contrast_fixed);
    }
}


#[inline]
fn adjust_contrast_pixel(pixel: u32, contrast_fixed: i32) -> u32 {
    // Extract channels
    let r = ((pixel >> 24) & 0xFF) as i32;
    let g = ((pixel >> 16) & 0xFF) as i32;
    let b = ((pixel >> 8)  & 0xFF) as i32;
    let a =  (pixel        & 0xFF) as u32;

    // Adjust RGB
    let r = adjust_channel(r, contrast_fixed);
    let g = adjust_channel(g, contrast_fixed);
    let b = adjust_channel(b, contrast_fixed);

    // Repack
    ((r as u32) << 24)
        | ((g as u32) << 16)
        | ((b as u32) << 8)
        | a
}

#[inline]
fn adjust_channel(value: i32, contrast_fixed: i32) -> u8 {
    let centered = value - 128;
    let scaled = (centered * contrast_fixed) >> 8;
    let result = scaled + 128;
    result.clamp(0, 255) as u8
}
