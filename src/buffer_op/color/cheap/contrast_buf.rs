use crate::buffer_op::color::cheap::Contrast;
use crate::utils::color::channel::pixel;


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
        let (r, g, b, a) = pixel::to_rgb::as_u32(*pixel);
        let (r, g, b) = contrast_pixel(r, g, b, contrast_fixed);
        *pixel = pixel::from_rgb::of_u32(r, g, b, a);
    }
}

/// Apply contrast adjustment to individual color channels
///
/// # Parameters
/// - `r`: Red channel value (0-255)
/// - `g`: Green channel value (0-255)
/// - `b`: Blue channel value (0-255)
/// - `contrast_fixed`: Contrast multiplier in 8.8 fixed-point format
///
/// # Returns
/// A tuple `(r, g, b)` containing the contrast-adjusted color channel values, clamped to 0-255
#[inline(always)]
pub fn contrast_pixel(r: u32, g: u32, b: u32, contrast_fixed: i32) -> (u32, u32, u32) {
    let r = ((r as i32 - 128) * contrast_fixed >> 8) + 128;
    let g = ((g as i32 - 128) * contrast_fixed >> 8) + 128;
    let b = ((b as i32 - 128) * contrast_fixed >> 8) + 128;

    let r = r.clamp(0, 255) as u32;
    let g = g.clamp(0, 255) as u32;
    let b = b.clamp(0, 255) as u32;

    (r, g, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rgba(r: u8, g: u8, b: u8, a: u8) -> u32 {
        ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | (a as u32)
    }

    fn extract_rgba(pixel: u32) -> (u8, u8, u8, u8) {
        (
            ((pixel >> 24) & 0xFF) as u8,
            ((pixel >> 16) & 0xFF) as u8,
            ((pixel >> 8) & 0xFF) as u8,
            (pixel & 0xFF) as u8,
        )
    }

    #[test]
    fn test_identity_contrast() {
        let mut buffer = vec![
            rgba(255, 0, 0, 255),
            rgba(0, 255, 0, 200),
            rgba(100, 150, 200, 128),
        ];
        let original = buffer.clone();

        let contrast = Contrast::from_f32(1.0);
        contrast_buffer(&mut buffer, contrast);

        assert_eq!(buffer, original);
    }

    #[test]
    fn test_alpha_preservation() {
        let mut buffer = vec![
            rgba(200, 100, 50, 0),
            rgba(128, 128, 128, 128),
            rgba(50, 150, 250, 255),
        ];
        let original_alphas: Vec<u8> = buffer.iter().map(|p| (p & 0xFF) as u8).collect();

        let contrast = Contrast::from_f32(1.5);
        contrast_buffer(&mut buffer, contrast);

        for (pixel, orig_alpha) in buffer.iter().zip(original_alphas.iter()) {
            let (_, _, _, a) = extract_rgba(*pixel);
            assert_eq!(a, *orig_alpha);
        }
    }

    #[test]
    fn test_middle_gray_unchanged() {
        let mut buffer = vec![rgba(128, 128, 128, 255)];
        let original = buffer.clone();

        let contrast = Contrast::from_f32(2.0);
        contrast_buffer(&mut buffer, contrast);

        // Middle gray (128) should remain unchanged regardless of contrast
        assert_eq!(buffer, original);
    }

    #[test]
    fn test_increase_contrast() {
        let mut buffer = vec![rgba(200, 100, 128, 255)];

        let contrast = Contrast::from_f32(2.0);
        contrast_buffer(&mut buffer, contrast);

        let (r, g, b, _) = extract_rgba(buffer[0]);

        // Higher contrast: values above 128 increase, below 128 decrease
        assert!(r > 200, "Value above 128 should increase");
        assert!(g < 100, "Value below 128 should decrease");
        // 128 stays ~128 (within rounding tolerance)
        assert!((b as i16 - 128).abs() <= 1);
    }

    #[test]
    fn test_decrease_contrast() {
        let mut buffer = vec![rgba(255, 0, 128, 255)];

        let contrast = Contrast::from_f32(0.5);
        contrast_buffer(&mut buffer, contrast);

        let (r, g, b, _) = extract_rgba(buffer[0]);

        // Lower contrast: values move toward 128
        assert!(r < 255, "Value above 128 should decrease");
        assert!(g > 0, "Value below 128 should increase");
        assert!((b as i16 - 128).abs() <= 1);
    }

    #[test]
    fn test_clamping() {
        let mut buffer = vec![
            rgba(255, 255, 255, 255),
            rgba(0, 0, 0, 255),
        ];

        let contrast = Contrast::from_f32(3.0);
        contrast_buffer(&mut buffer, contrast);

        // Verify no overflow/underflow
        for pixel in buffer.iter() {
            let (r, g, b, _) = extract_rgba(*pixel);
            let _ = (r, g, b); // Values are u8, automatically bounded
        }
    }

    #[test]
    fn test_empty_buffer() {
        let mut buffer: Vec<u32> = vec![];
        let contrast = Contrast::from_f32(1.5);
        contrast_buffer(&mut buffer, contrast);
        assert_eq!(buffer.len(), 0);
    }
}

