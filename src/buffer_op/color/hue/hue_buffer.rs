use crate::buffer_op::color::hue::Hue;

/// Applies hue rotation to a buffer of RRGGBBAA pixels using a pre-computed matrix.
///
/// This function rotates the hue of each pixel using a 3x3 transformation matrix
/// that preserves luminance
/// (based on Rec. 601 coefficients: https://en.wikipedia.org/wiki/Rec._601).
/// The rotation is performed in RGB space using fixed-point arithmetic for performance.
///
/// # Performance
/// - Uses 8.8 fixed-point arithmetic for speed (avoids floating-point operations)
/// - Matrix values are kept in registers
/// - Alpha channel is preserved unchanged
/// - Early exit for identity rotation (0° or 360°)
///
/// # Arguments
/// * `buffer` - Mutable slice of RRGGBBAA pixels (R in MSB, A in LSB)
/// * `hue` - Pre-computed hue rotation with cached transformation matrix
#[inline]
pub fn hue_buffer(buffer: &mut [u32], hue: Hue) {
    // Early exit for identity rotation (no change needed)
    if hue.as_radians().abs() < f32::EPSILON {
        return;
    }

    let m = hue.matrix_values();

    // Unpack matrix once (helps compiler keep them in registers)
    let m00 = m[0][0];
    let m01 = m[0][1];
    let m02 = m[0][2];

    let m10 = m[1][0];
    let m11 = m[1][1];
    let m12 = m[1][2];

    let m20 = m[2][0];
    let m21 = m[2][1];
    let m22 = m[2][2];

    for pixel in buffer.iter_mut() {
        let p = *pixel;

        let r = ((p >> 24) & 0xFF) as i32;
        let g = ((p >> 16) & 0xFF) as i32;
        let b = ((p >> 8) & 0xFF) as i32;
        let a = p & 0xFF;

        // 8.8 fixed-point multiply
        let r2 = (m00*r + m01*g + m02*b) >> 8;
        let g2 = (m10*r + m11*g + m12*b) >> 8;
        let b2 = (m20*r + m21*g + m22*b) >> 8;

        // Clamp to u8 range
        let r2 = r2.clamp(0, 255) as u32;
        let g2 = g2.clamp(0, 255) as u32;
        let b2 = b2.clamp(0, 255) as u32;

        *pixel = (r2 << 24) | (g2 << 16) | (b2 << 8) | a;
    }
}
