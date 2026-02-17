use crate::buffer_op::color::cheap::Tint;
use crate::utils::color::channel::pixel;


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
        let (r, g, b, a) = pixel::to_rgb::as_u32(*pixel);
        let (r, g, b) = tint_pixel(r, g, b, tr, tg, tb);
        *pixel = pixel::from_rgb::of_u32(r, g, b, a);
    }
}

/// Tint individual color channels
#[inline(always)]
fn tint_pixel(r: u32, g: u32, b: u32, tr: u32, tg: u32, tb: u32) -> (u32, u32, u32) {
    let r = (r * tr + 128) >> 8;
    let g = (g * tg + 128) >> 8;
    let b = (b * tb + 128) >> 8;

    (r, g, b)
}
 