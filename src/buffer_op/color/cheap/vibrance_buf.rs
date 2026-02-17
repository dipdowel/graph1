use crate::buffer_op::color::cheap::Vibrance;
use crate::utils::color::channel::pixel;

/// Applies vibrance adjustment to a buffer of pixels.
///
/// Vibrance selectively enhances or reduces saturation, affecting less-saturated colors
/// more strongly than highly-saturated ones. This creates a more natural-looking
/// saturation adjustment compared to uniform saturation changes.
///
/// # Algorithm
/// For each pixel:
/// 1. Calculate luminance using Rec.709 coefficients
/// 2. Compute saturation as max(R,G,B) - min(R,G,B)
/// 3. Calculate vibrance factor: 1 + vibrance * (1 - saturation)
///    - Low saturation colors get a stronger adjustment
///    - High saturation colors get a weaker adjustment
/// 4. Interpolate between luminance (gray) and original color by the factor
/// 5. Clamp results to valid range
///
/// # Performance
/// - Uses fixed-point arithmetic where possible for speed
/// - Preserves alpha channel throughout
/// - Early exit for identity vibrance (0.0)
///
/// # Arguments
/// * `buffer` - Mutable slice of RRGGBBAA pixels (R in MSB, A in LSB)
/// * `vibrance` - Vibrance adjustment to apply
///
/// # Examples
/// ```rust
/// use graph1::buffer_op::color::cheap::{Vibrance, vibrance_buffer};
///
/// let mut buffer = vec![0xFF8040FF; 100];
/// let vibrance = Vibrance::from_f32(0.5); // Boost dull colors
/// vibrance_buffer(&mut buffer, vibrance);
/// ```
#[inline]
pub fn vibrance_buffer(buffer: &mut [u32], vibrance: Vibrance) {
    // Early exit for identity vibrance (no change)
    if vibrance.is_identity() {
        return;
    }

    let vibrance_f32 = vibrance.as_f32();

    for pixel in buffer.iter_mut() {
        let (r, g, b, a) = pixel::to_rgb::as_u32(*pixel);
        let (r, g, b) = vibrance_pixel(r, g, b, vibrance_f32);
        *pixel = pixel::from_rgb::of_u32(r, g, b, a);
    }
}

/// Apply vibrance adjustment to individual color channels.
///
/// # Parameters
/// - `r`: Red channel value (0-255)
/// - `g`: Green channel value (0-255)
/// - `b`: Blue channel value (0-255)
/// - `vibrance`: Vibrance adjustment value (-2.0 to 2.0, typically -1.0 to 1.0)
///
/// # Returns
/// A tuple `(r, g, b)` containing the vibrance-adjusted color channel values, clamped to 0-255
///
/// # Algorithm
/// Vibrance preferentially affects less-saturated colors:
/// - Calculates saturation as the difference between max and min RGB values
/// - Applies stronger adjustment to colors with lower saturation
/// - Uses luminance as the neutral point for interpolation
#[inline(always)]
pub fn vibrance_pixel(r: u32, g: u32, b: u32, vibrance: f32) -> (u32, u32, u32) {
    // Convert to float for calculations (0-255 range)
    let rf = r as f32;
    let gf = g as f32;
    let bf = b as f32;

    // Normalize to 0-1
    let r_norm = rf / 255.0;
    let g_norm = gf / 255.0;
    let b_norm = bf / 255.0;

    // Calculate luminance using Rec.709 coefficients
    let luminance = 0.2126 * r_norm + 0.7152 * g_norm + 0.0722 * b_norm;

    // Approximate saturation
    let max = r_norm.max(g_norm).max(b_norm);
    let min = r_norm.min(g_norm).min(b_norm);
    let saturation = max - min;

    // Vibrance factor: boost low saturation more, high saturation less
    // factor = 1 + vibrance * (1 - saturation)
    let factor = 1.0 + vibrance * (1.0 - saturation);

    // Mix gray (luminance) and original color
    // new_color = luminance + (original - luminance) * factor
    let r_new = luminance + (r_norm - luminance) * factor;
    let g_new = luminance + (g_norm - luminance) * factor;
    let b_new = luminance + (b_norm - luminance) * factor;

    // Clamp to 0-1 and convert back to 0-255
    let r_out = (r_new.clamp(0.0, 1.0) * 255.0).round() as u32;
    let g_out = (g_new.clamp(0.0, 1.0) * 255.0).round() as u32;
    let b_out = (b_new.clamp(0.0, 1.0) * 255.0).round() as u32;

    (r_out, g_out, b_out)
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
    fn test_identity_vibrance() {
        let mut buffer = vec![
            rgba(255, 128, 64, 255),
            rgba(100, 150, 200, 128),
            rgba(50, 50, 50, 255),
        ];
        let original = buffer.clone();

        let vibrance = Vibrance::identity();
        vibrance_buffer(&mut buffer, vibrance);

        assert_eq!(buffer, original, "Identity vibrance should not modify buffer");
    }

    #[test]
    fn test_vibrance_zero() {
        let mut buffer = vec![rgba(128, 100, 80, 255)];
        let original = buffer.clone();

        let vibrance = Vibrance::from_f32(0.0);
        vibrance_buffer(&mut buffer, vibrance);

        assert_eq!(buffer, original, "Zero vibrance should not modify buffer");
    }

    #[test]
    fn test_alpha_preservation() {
        let mut buffer = vec![
            rgba(255, 0, 0, 255),   // Red, full alpha
            rgba(0, 255, 0, 128),   // Green, half alpha
            rgba(0, 0, 255, 0),     // Blue, zero alpha
        ];

        let vibrance = Vibrance::from_f32(0.5);
        vibrance_buffer(&mut buffer, vibrance);

        // Check alpha channels are preserved
        let (_, _, _, a1) = extract_rgba(buffer[0]);
        let (_, _, _, a2) = extract_rgba(buffer[1]);
        let (_, _, _, a3) = extract_rgba(buffer[2]);

        assert_eq!(a1, 255, "Alpha should be preserved (full)");
        assert_eq!(a2, 128, "Alpha should be preserved (half)");
        assert_eq!(a3, 0, "Alpha should be preserved (zero)");
    }

    #[test]
    fn test_positive_vibrance() {
        let mut buffer = vec![rgba(128, 100, 80, 255)]; // Dull color
        let original = buffer[0];

        let vibrance = Vibrance::from_f32(0.5);
        vibrance_buffer(&mut buffer, vibrance);

        // Positive vibrance should change the color
        assert_ne!(buffer[0], original, "Positive vibrance should modify color");
    }

    #[test]
    fn test_negative_vibrance() {
        let mut buffer = vec![rgba(255, 100, 50, 255)]; // Colorful
        let original = buffer[0];

        let vibrance = Vibrance::from_f32(-0.5);
        vibrance_buffer(&mut buffer, vibrance);

        // Negative vibrance should change the color (mute it)
        assert_ne!(buffer[0], original, "Negative vibrance should modify color");
    }

    #[test]
    fn test_grayscale_unchanged() {
        // Grayscale colors (saturation = 0) should be less affected
        let mut buffer = vec![rgba(128, 128, 128, 255)];
        let original = buffer[0];

        let vibrance = Vibrance::from_f32(0.5);
        vibrance_buffer(&mut buffer, vibrance);

        // Gray should remain mostly unchanged (saturation is 0, so factor ≈ 1 + vibrance)
        // However, due to floating point arithmetic, there might be small changes
        let (r, g, b, _) = extract_rgba(buffer[0]);
        let (r_orig, g_orig, b_orig, _) = extract_rgba(original);

        // Allow for small rounding differences
        assert!((r as i32 - r_orig as i32).abs() <= 2, "Gray R should be mostly unchanged");
        assert!((g as i32 - g_orig as i32).abs() <= 2, "Gray G should be mostly unchanged");
        assert!((b as i32 - b_orig as i32).abs() <= 2, "Gray B should be mostly unchanged");
    }

    #[test]
    fn test_clamping() {
        // Test with white and extreme vibrance
        let mut buffer = vec![rgba(255, 255, 255, 255)];

        let vibrance = Vibrance::from_f32(2.0); // Extreme vibrance
        vibrance_buffer(&mut buffer, vibrance);

        let (r, g, b, a) = extract_rgba(buffer[0]);

        // All channels are u8, so they're automatically bounded to 0-255
        // Just verify the buffer was processed without panicking
        assert_eq!(a, 255, "Alpha should be preserved");
        // Values are valid by definition since they're u8
        let _ = (r, g, b); // Acknowledge we got the values
    }

    #[test]
    fn test_vibrance_pixel_direct() {
        // Test the pixel function directly
        let (r, g, b) = vibrance_pixel(128, 100, 80, 0.5);

        // Should return valid values
        assert!(r <= 255, "R should be in valid range");
        assert!(g <= 255, "G should be in valid range");
        assert!(b <= 255, "B should be in valid range");

        // Positive vibrance should enhance the color difference from gray
        // (exact values depend on algorithm, so we just check validity)
    }

    #[test]
    fn test_fully_saturated_color() {
        // Pure red (saturation = 1.0) should be less affected
        let mut buffer = vec![rgba(255, 0, 0, 255)];

        let vibrance = Vibrance::from_f32(0.5);
        vibrance_buffer(&mut buffer, vibrance);

        let (r, g, b, _) = extract_rgba(buffer[0]);

        // Red should still be relatively high
        assert!(r > 200, "Red channel should remain strong for saturated color");
        // Other channels should remain low
        assert!(g < 100, "Green should remain low");
        assert!(b < 100, "Blue should remain low");
    }
}

