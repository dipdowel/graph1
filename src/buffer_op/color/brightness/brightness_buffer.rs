use crate::utils::color::Brightness;

/// Applies brightness scaling to an RGBA buffer in RRGGBBAA format.
/// RGB channels are scaled by `brightness` and clamped to [0, 255].
/// Alpha is preserved.
///
/// Uses 8.8 fixed-point math:
///   out = (in * factor_8_8 + 128) >> 8
pub fn brightness_buffer(buffer: &mut [u32], brightness: Brightness) {
    let f = brightness.factor_8_8();

    for px in buffer.iter_mut() {
        let p = *px;

        let r = ((p >> 24) & 0xFF) as i32;
        let g = ((p >> 16) & 0xFF) as i32;
        let b = ((p >> 8)  & 0xFF) as i32;
        let a =  (p        & 0xFF);

        // 8.8 fixed-point multiply, +128 for rounding before >> 8
        let r2 = (r * f + 128) >> 8;
        let g2 = (g * f + 128) >> 8;
        let b2 = (b * f + 128) >> 8;

        let r2 = clamp_u8_i32(r2);
        let g2 = clamp_u8_i32(g2);
        let b2 = clamp_u8_i32(b2);

        *px = (r2 << 24) | (g2 << 16) | (b2 << 8) | a;
    }
}


#[inline(always)]
fn clamp_u8_i32(v: i32) -> u32 {
    if v < 0 { 0 } else if v > 255 { 255 } else { v as u32 }
}