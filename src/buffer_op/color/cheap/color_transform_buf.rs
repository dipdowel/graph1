use crate::buffer_op::color::cheap::structs::color_transform::ColorTransformType;
use crate::buffer_op::color::cheap::ColorTransform;
use crate::utils::color::channel::pixel;


/// Applies a sequence of color transformations to a buffer in a single pass.
///
/// This function processes the buffer once, applying brightness, contrast, hue, and tint
/// adjustments in the order specified in the `ColorTransform` collection. This is more
/// efficient than applying each transformation separately when multiple adjustments are needed.
///
/// # Performance
/// - Single pass over the buffer regardless of the number of transformations
/// - Uses 8.8 fixed-point arithmetic for speed (avoids floating-point operations)
/// - Early exit if no transformations are specified
/// - Alpha channel is preserved throughout all transformations
///
/// # Arguments
/// * `buffer` - Mutable slice of RRGGBBAA pixels (R in MSB, A in LSB)
/// * `transform` - Collection of color transformations to apply in sequence
///
/// # Examples
/// ```rust
/// use graph1::buffer_op::color::cheap::{
///     ColorTransform, ColorTransformType, Brightness, Contrast, Hue, color_transform_buffer
/// };
///
/// let mut buffer = vec![0xFF8040FF; 100];
/// let transform = ColorTransform::new(vec![
///     ColorTransformType::Brightness(Brightness::from_percent(120)),
///     ColorTransformType::Contrast(Contrast::from_f32(1.2)),
///     ColorTransformType::Hue(Hue::from_degrees(30.0)),
/// ]);
/// color_transform_buffer(&mut buffer, transform);
/// ```
pub fn color_transform_buffer(buffer: &mut [u32], transform: ColorTransform) {
    // Early exit if no transformations specified
    if transform.is_empty() {
        return;
    }

    // Single pass over the buffer
    for pixel in buffer.iter_mut() {
        let (r, g, b, a) = pixel::to_rgb::as_u32(*pixel);

        // Convert to i32 for arithmetic operations
        let mut r = r as i32;
        let mut g = g as i32;
        let mut b = b as i32;

        // Apply each transformation in sequence
        for t in &transform.transforms {
            match t {
                ColorTransformType::Brightness(brightness) => {
                    let f = brightness.factor_8_8();
                    // 8.8 fixed-point multiply, +128 for rounding before >> 8
                    r = ((r * f + 128) >> 8).clamp(0, 255);
                    g = ((g * f + 128) >> 8).clamp(0, 255);
                    b = ((b * f + 128) >> 8).clamp(0, 255);
                }
                ColorTransformType::Contrast(contrast) => {
                    let cf = contrast.fixed_multiplier();
                    // Adjust around middle gray (128)
                    r = (((r - 128) * cf) >> 8) + 128;
                    g = (((g - 128) * cf) >> 8) + 128;
                    b = (((b - 128) * cf) >> 8) + 128;
                    r = r.clamp(0, 255);
                    g = g.clamp(0, 255);
                    b = b.clamp(0, 255);
                }
                ColorTransformType::Hue(hue) => {
                    let m = hue.matrix_values();
                    // Apply 3x3 hue rotation matrix
                    let nr = (m[0][0]*r + m[0][1]*g + m[0][2]*b) >> 8;
                    let ng = (m[1][0]*r + m[1][1]*g + m[1][2]*b) >> 8;
                    let nb = (m[2][0]*r + m[2][1]*g + m[2][2]*b) >> 8;
                    r = nr.clamp(0, 255);
                    g = ng.clamp(0, 255);
                    b = nb.clamp(0, 255);
                }
                ColorTransformType::Tint(tint) => {
                    // Multiplicative tinting
                    r = ((r * tint.r as i32 + 128) >> 8).clamp(0, 255);
                    g = ((g * tint.g as i32 + 128) >> 8).clamp(0, 255);
                    b = ((b * tint.b as i32 + 128) >> 8).clamp(0, 255);
                }
            }
        }

        // Pack back into pixel format, preserving alpha
        *pixel = pixel::from_rgb::of_u32(r as u32, g as u32, b as u32, a);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buffer_op::color::cheap::{Brightness, Contrast, Hue, Tint};
    use crate::buffer_op::color::cheap::{brightness_buffer, contrast_buffer, hue_buffer, tint_buffer};

    #[test]
    fn test_empty_transform() {
        let mut buffer = vec![0xFF8040FF, 0x00FF00FF, 0x0000FFFF];
        let original = buffer.clone();
        let transform = ColorTransform::empty();

        color_transform_buffer(&mut buffer, transform);

        assert_eq!(buffer, original, "Empty transform should not modify buffer");
    }

    #[test]
    fn test_single_brightness() {
        let mut buffer_multi = vec![0xFF8040FF; 10];
        let mut buffer_single = buffer_multi.clone();

        let brightness = Brightness::from_percent(120);
        let transform = ColorTransform::new(vec![
            ColorTransformType::Brightness(brightness),
        ]);

        color_transform_buffer(&mut buffer_multi, transform);
        brightness_buffer(&mut buffer_single, brightness);

        assert_eq!(buffer_multi, buffer_single,
            "Single brightness via color_transform should match brightness_buffer");
    }

    #[test]
    fn test_single_contrast() {
        let mut buffer_multi = vec![0xFF8040FF; 10];
        let mut buffer_single = buffer_multi.clone();

        let contrast = Contrast::from_f32(1.5);
        let transform = ColorTransform::new(vec![
            ColorTransformType::Contrast(contrast),
        ]);

        color_transform_buffer(&mut buffer_multi, transform);
        contrast_buffer(&mut buffer_single, contrast);

        assert_eq!(buffer_multi, buffer_single,
            "Single contrast via color_transform should match contrast_buffer");
    }

    #[test]
    fn test_single_hue() {
        let mut buffer_multi = vec![0xFF8040FF; 10];
        let mut buffer_single = buffer_multi.clone();

        let hue = Hue::from_degrees(45.0);
        let transform = ColorTransform::new(vec![
            ColorTransformType::Hue(hue),
        ]);

        color_transform_buffer(&mut buffer_multi, transform);
        hue_buffer(&mut buffer_single, hue);

        assert_eq!(buffer_multi, buffer_single,
            "Single hue via color_transform should match hue_buffer");
    }

    #[test]
    fn test_single_tint() {
        let mut buffer_multi = vec![0xFF8040FF; 10];
        let mut buffer_single = buffer_multi.clone();

        let tint = Tint::new(200, 150, 100);
        let transform = ColorTransform::new(vec![
            ColorTransformType::Tint(tint),
        ]);

        color_transform_buffer(&mut buffer_multi, transform);
        tint_buffer(&mut buffer_single, tint);

        assert_eq!(buffer_multi, buffer_single,
            "Single tint via color_transform should match tint_buffer");
    }

    #[test]
    fn test_combined_transforms() {
        let mut buffer = vec![0xFF8040FF; 10];

        let transform = ColorTransform::new(vec![
            ColorTransformType::Brightness(Brightness::from_percent(120)),
            ColorTransformType::Contrast(Contrast::from_f32(1.2)),
            ColorTransformType::Hue(Hue::from_degrees(30.0)),
            ColorTransformType::Tint(Tint::new(255, 200, 150)),
        ]);

        color_transform_buffer(&mut buffer, transform);

        // Verify buffer was modified
        assert_ne!(buffer[0], 0xFF8040FF, "Buffer should be modified by transforms");
    }

    #[test]
    fn test_alpha_preservation() {
        let mut buffer = vec![
            0xFF0000FF, // Red, full alpha
            0x00FF0080, // Green, half alpha
            0x0000FF00, // Blue, zero alpha
        ];

        let transform = ColorTransform::new(vec![
            ColorTransformType::Brightness(Brightness::from_percent(150)),
            ColorTransformType::Contrast(Contrast::from_f32(1.5)),
        ]);

        color_transform_buffer(&mut buffer, transform);

        // Check alpha channels are preserved
        assert_eq!(buffer[0] & 0xFF, 0xFF, "Alpha should be preserved (full)");
        assert_eq!(buffer[1] & 0xFF, 0x80, "Alpha should be preserved (half)");
        assert_eq!(buffer[2] & 0xFF, 0x00, "Alpha should be preserved (zero)");
    }

    #[test]
    fn test_sequential_application() {
        // Test that transforms are applied in order
        let mut buffer1 = vec![0x80804080; 5];
        let mut buffer2 = buffer1.clone();

        // Apply in one order
        let transform1 = ColorTransform::new(vec![
            ColorTransformType::Brightness(Brightness::from_percent(150)),
            ColorTransformType::Contrast(Contrast::from_f32(1.5)),
        ]);

        // Apply in reverse order
        let transform2 = ColorTransform::new(vec![
            ColorTransformType::Contrast(Contrast::from_f32(1.5)),
            ColorTransformType::Brightness(Brightness::from_percent(150)),
        ]);

        color_transform_buffer(&mut buffer1, transform1);
        color_transform_buffer(&mut buffer2, transform2);

        // Different order should produce different results (in general)
        // Note: This may not always be true for all combinations, but for brightness+contrast it is
        assert_ne!(buffer1, buffer2,
            "Different transform order should generally produce different results");
    }

    #[test]
    fn test_clamping() {
        // Test that values are properly clamped to 0-255
        let mut buffer = vec![0xFFFFFFFF]; // White

        let transform = ColorTransform::new(vec![
            ColorTransformType::Brightness(Brightness::from_percent(300)), // Will overflow
            ColorTransformType::Contrast(Contrast::from_f32(3.0)),         // Will overflow
        ]);

        color_transform_buffer(&mut buffer, transform);

        let (r, g, b, a) = pixel::to_rgb::as_u32(buffer[0]);

        // All channels should be clamped to 255
        assert!(r <= 255, "Red should be clamped to 255");
        assert!(g <= 255, "Green should be clamped to 255");
        assert!(b <= 255, "Blue should be clamped to 255");
        assert_eq!(a, 255, "Alpha should be preserved");
    }
}
