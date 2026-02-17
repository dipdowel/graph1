use crate::buffer_op::color::cheap::Hue;
use crate::utils::color::channel::pixel;


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
        let (r, g, b, a) = pixel::to_rgb::as_u32(*pixel);
        let (r, g, b) = hue_pixel(r, g, b, m00, m01, m02, m10, m11, m12, m20, m21, m22);
        *pixel = pixel::from_rgb::of_u32(r, g, b, a);
    }
}

/// Apply hue rotation to individual color channels
///
/// # Parameters
/// - `r`: Red channel value (0-255)
/// - `g`: Green channel value (0-255)
/// - `b`: Blue channel value (0-255)
/// - `m00`: Matrix element [0,0] - red's contribution to output red (8.8 fixed-point)
/// - `m01`: Matrix element [0,1] - green's contribution to output red (8.8 fixed-point)
/// - `m02`: Matrix element [0,2] - blue's contribution to output red (8.8 fixed-point)
/// - `m10`: Matrix element [1,0] - red's contribution to output green (8.8 fixed-point)
/// - `m11`: Matrix element [1,1] - green's contribution to output green (8.8 fixed-point)
/// - `m12`: Matrix element [1,2] - blue's contribution to output green (8.8 fixed-point)
/// - `m20`: Matrix element [2,0] - red's contribution to output blue (8.8 fixed-point)
/// - `m21`: Matrix element [2,1] - green's contribution to output blue (8.8 fixed-point)
/// - `m22`: Matrix element [2,2] - blue's contribution to output blue (8.8 fixed-point)
///
/// # Returns
/// A tuple `(r, g, b)` containing the hue-rotated color channel values, clamped to 0-255
#[inline(always)]
pub fn hue_pixel(
    r: u32,
    g: u32,
    b: u32,
    m00: i32,
    m01: i32,
    m02: i32,
    m10: i32,
    m11: i32,
    m12: i32,
    m20: i32,
    m21: i32,
    m22: i32,
) -> (u32, u32, u32) {
    let r = r as i32;
    let g = g as i32;
    let b = b as i32;

    // 8.8 fixed-point multiply
    let r2 = (m00*r + m01*g + m02*b) >> 8;
    let g2 = (m10*r + m11*g + m12*b) >> 8;
    let b2 = (m20*r + m21*g + m22*b) >> 8;

    // Clamp to u8 range
    let r2 = r2.clamp(0, 255) as u32;
    let g2 = g2.clamp(0, 255) as u32;
    let b2 = b2.clamp(0, 255) as u32;

    (r2, g2, b2)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function to create RRGGBBAA pixel
    fn rgba(r: u8, g: u8, b: u8, a: u8) -> u32 {
        ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | (a as u32)
    }

    /// Helper function to extract RGBA components
    fn extract_rgba(pixel: u32) -> (u8, u8, u8, u8) {
        (
            ((pixel >> 24) & 0xFF) as u8,
            ((pixel >> 16) & 0xFF) as u8,
            ((pixel >> 8) & 0xFF) as u8,
            (pixel & 0xFF) as u8,
        )
    }

    #[test]
    fn test_identity_rotation_zero_degrees() {
        let mut buffer = vec![
            rgba(255, 0, 0, 255),    // Red
            rgba(0, 255, 0, 255),    // Green
            rgba(0, 0, 255, 255),    // Blue
            rgba(128, 128, 128, 200), // Gray
        ];
        let original = buffer.clone();

        let hue = Hue::from_degrees(0.0);
        hue_buffer(&mut buffer, hue);

        assert_eq!(buffer, original, "0° rotation should not change pixels");
    }

    #[test]
    fn test_identity_rotation_360_degrees() {
        let mut buffer = vec![
            rgba(255, 0, 0, 255),
            rgba(0, 255, 0, 255),
            rgba(0, 0, 255, 255),
        ];
        let original = buffer.clone();

        let hue = Hue::from_degrees(360.0);
        hue_buffer(&mut buffer, hue);

        // 360° should be normalized to ~0° (within floating point tolerance)
        // Colors should be very close to original
        for (result, orig) in buffer.iter().zip(original.iter()) {
            let (r1, g1, b1, a1) = extract_rgba(*result);
            let (r2, g2, b2, a2) = extract_rgba(*orig);

            // Allow small rounding differences due to fixed-point arithmetic
            assert!((r1 as i16 - r2 as i16).abs() <= 1, "Red channel differs too much");
            assert!((g1 as i16 - g2 as i16).abs() <= 1, "Green channel differs too much");
            assert!((b1 as i16 - b2 as i16).abs() <= 1, "Blue channel differs too much");
            assert_eq!(a1, a2, "Alpha should be preserved");
        }
    }

    #[test]
    fn test_alpha_preservation() {
        let mut buffer = vec![
            rgba(255, 0, 0, 0),      // Fully transparent
            rgba(0, 255, 0, 128),    // Semi-transparent
            rgba(0, 0, 255, 255),    // Fully opaque
            rgba(100, 150, 200, 42), // Arbitrary alpha
        ];

        let original_alphas: Vec<u8> = buffer.iter().map(|p| (p & 0xFF) as u8).collect();

        let hue = Hue::from_degrees(120.0);
        hue_buffer(&mut buffer, hue);

        for (pixel, original_alpha) in buffer.iter().zip(original_alphas.iter()) {
            let (_, _, _, a) = extract_rgba(*pixel);
            assert_eq!(a, *original_alpha, "Alpha channel must be preserved");
        }
    }

    #[test]
    fn test_empty_buffer() {
        let mut buffer: Vec<u32> = vec![];
        let hue = Hue::from_degrees(90.0);

        // Should not panic
        hue_buffer(&mut buffer, hue);

        assert_eq!(buffer.len(), 0);
    }

    #[test]
    fn test_single_pixel() {
        let mut buffer = vec![rgba(255, 128, 64, 200)];
        let hue = Hue::from_degrees(90.0);

        hue_buffer(&mut buffer, hue);

        assert_eq!(buffer.len(), 1);
        let (_, _, _, a) = extract_rgba(buffer[0]);
        assert_eq!(a, 200, "Alpha should be preserved");
    }

    #[test]
    fn test_grayscale_unchanged() {
        // Grayscale pixels should remain relatively unchanged since they have no hue
        let mut buffer = vec![
            rgba(0, 0, 0, 255),       // Black
            rgba(128, 128, 128, 255), // Mid gray
            rgba(255, 255, 255, 255), // White
        ];
        let original = buffer.clone();

        let hue = Hue::from_degrees(180.0);
        hue_buffer(&mut buffer, hue);

        // Grayscale should be mostly unchanged (within small tolerance due to rounding)
        for (result, orig) in buffer.iter().zip(original.iter()) {
            let (r1, g1, b1, _) = extract_rgba(*result);
            let (r2, g2, b2, _) = extract_rgba(*orig);

            // Allow some tolerance for rounding in fixed-point math
            assert!((r1 as i16 - r2 as i16).abs() <= 2);
            assert!((g1 as i16 - g2 as i16).abs() <= 2);
            assert!((b1 as i16 - b2 as i16).abs() <= 2);
        }
    }

    #[test]
    fn test_color_rotation_90_degrees() {
        // Test that primary colors shift as expected with 90° rotation
        let mut buffer = vec![
            rgba(255, 0, 0, 255), // Pure red
        ];

        let hue = Hue::from_degrees(90.0);
        hue_buffer(&mut buffer, hue);

        let (r, g, b, a) = extract_rgba(buffer[0]);

        // After 90° rotation, pure red should shift toward yellow/green
        // The exact values depend on the matrix, but we can verify:
        // - Alpha is preserved
        // - Color has changed
        assert_eq!(a, 255);
        assert!(r != 255 || g != 0 || b != 0, "Color should have changed");
    }

    #[test]
    fn test_double_rotation_180_degrees() {
        let mut buffer1 = vec![rgba(200, 100, 50, 255)];
        let mut buffer2 = buffer1.clone();

        // Apply 180° in one step
        let hue_180 = Hue::from_degrees(180.0);
        hue_buffer(&mut buffer1, hue_180);

        // Apply 90° twice
        let hue_90 = Hue::from_degrees(90.0);
        hue_buffer(&mut buffer2, hue_90);
        hue_buffer(&mut buffer2, hue_90);

        let (r1, g1, b1, _) = extract_rgba(buffer1[0]);
        let (r2, g2, b2, _) = extract_rgba(buffer2[0]);

        // Results should be close (within tolerance for cumulative rounding in fixed-point math)
        // Double application accumulates more rounding error, so we allow larger tolerance
        assert!((r1 as i16 - r2 as i16).abs() <= 30,
            "Red mismatch: {} vs {}", r1, r2);
        assert!((g1 as i16 - g2 as i16).abs() <= 30,
            "Green mismatch: {} vs {}", g1, g2);
        assert!((b1 as i16 - b2 as i16).abs() <= 30,
            "Blue mismatch: {} vs {}", b1, b2);
    }

    #[test]
    fn test_clamping() {
        // Test that values are properly clamped to 0-255 range
        // Even with extreme transformations, output should be valid u8 values
        let mut buffer = vec![
            rgba(255, 255, 255, 128), // White might overflow in some rotations
            rgba(0, 0, 0, 200),        // Black might underflow
        ];

        let hue = Hue::from_degrees(270.0);
        hue_buffer(&mut buffer, hue);

        // Verify that all pixels have valid RGB values (implicitly true since extract_rgba returns u8)
        for pixel in buffer.iter() {
            let (r, g, b, _) = extract_rgba(*pixel);
            // If these values exist, they're already in 0-255 range (u8 guarantees this)
            // This test mainly ensures no panic/overflow occurs during clamping
            let _ = (r, g, b); // Use the values to avoid unused variable warning
        }

        assert_eq!(buffer.len(), 2, "Buffer should still have 2 pixels");
    }

    #[test]
    fn test_multiple_pixels() {
        let mut buffer = vec![
            rgba(255, 0, 0, 255),
            rgba(0, 255, 0, 200),
            rgba(0, 0, 255, 150),
            rgba(255, 255, 0, 100),
            rgba(128, 64, 32, 50),
        ];

        let original_alphas: Vec<u8> = buffer.iter().map(|p| (p & 0xFF) as u8).collect();

        let hue = Hue::from_degrees(45.0);
        hue_buffer(&mut buffer, hue);

        assert_eq!(buffer.len(), 5);

        // Verify all alphas are preserved
        for (pixel, original_alpha) in buffer.iter().zip(original_alphas.iter()) {
            let (_, _, _, a) = extract_rgba(*pixel);
            assert_eq!(a, *original_alpha);
        }
    }
}

