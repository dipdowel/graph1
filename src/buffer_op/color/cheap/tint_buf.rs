use crate::buffer_op::color::cheap::Tint;

/// **NB:** Experimental! Use with caution.
/// Applies multiplicative tinting to a buffer of pixels.
///
/// This function multiplies each RGB channel by the corresponding tint channel value,
/// preserving the alpha channel (fixed-point arithmetics)
///
/// # Performance
/// - Uses inline hints for better optimization
/// - Optimized division approximation: `(x * y + 128) >> 8`
#[inline]
pub fn tint_buffer(buffer: &mut [u32], tint: Tint) {
    let tr = tint.r as u32;
    let tg = tint.g as u32;
    let tb = tint.b as u32;

    for pixel in buffer.iter_mut() {
        *pixel = tint_pixel(*pixel, tr, tg, tb);
    }
}

/// Tint a single pixel
#[inline(always)]
fn tint_pixel(pixel: u32, tr: u32, tg: u32, tb: u32) -> u32 {
    let r = (pixel >> 24) & 0xFF;
    let g = (pixel >> 16) & 0xFF;
    let b = (pixel >> 8) & 0xFF;
    let a = pixel & 0xFF;

    let r = (r * tr + 128) >> 8;
    let g = (g * tg + 128) >> 8;
    let b = (b * tb + 128) >> 8;

    (r << 24) | (g << 16) | (b << 8) | a
}
 