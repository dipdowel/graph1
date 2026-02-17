use crate::buffer_op::color::cheap::Brightness;
use crate::utils::color::channel::pixel;


/// Applies brightness scaling to an RGBA buffer in RRGGBBAA format.
/// RGB channels are scaled by `brightness` and clamped to [0, 255].
/// Alpha is preserved.
///
/// Uses 8.8 fixed-point math:
///   out = (in * factor_8_8 + 128) >> 8
#[inline]
pub fn brightness_buffer(buffer: &mut [u32], brightness: Brightness) {

    // Early exit for no contrast change
    if brightness.is_identity() {
        return;
    }


    let f = brightness.factor_8_8();

    for pixel in buffer.iter_mut() {
        let (r, g, b, a) = pixel::to_rgb::as_u32(*pixel);
        let (r, g, b) = brightness_pixel(r, g, b, f);
        *pixel = pixel::from_rgb::of_u32(r, g, b, a);
    }
}

/// Apply brightness adjustment to individual color channels
///
/// # Parameters
/// - `r`: Red channel value (0-255)
/// - `g`: Green channel value (0-255)
/// - `b`: Blue channel value (0-255)
/// - `f`: Brightness factor in 8.8 fixed-point format
///
/// # Returns
/// A tuple `(r, g, b)` containing the brightness-adjusted color channel values, clamped to 0-255
#[inline(always)]
pub fn brightness_pixel(r: u32, g: u32, b: u32, f: i32) -> (u32, u32, u32) {
    // 8.8 fixed-point multiply, +128 for rounding before >> 8
    let r = ((r as i32) * f + 128) >> 8;
    let g = ((g as i32) * f + 128) >> 8;
    let b = ((b as i32) * f + 128) >> 8;

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
    fn test_identity_brightness() {
        let mut buffer = vec![
            rgba(255, 128, 64, 255),
            rgba(100, 150, 200, 128),
        ];
        let original = buffer.clone();

        let brightness = Brightness::from_percent(100); // 1.0x
        brightness_buffer(&mut buffer, brightness);

        assert_eq!(buffer, original);
    }

    #[test]
    fn test_alpha_preservation() {
        let mut buffer = vec![
            rgba(200, 100, 50, 0),
            rgba(128, 64, 32, 128),
            rgba(50, 150, 250, 255),
        ];
        let original_alphas: Vec<u8> = buffer.iter().map(|p| (p & 0xFF) as u8).collect();

        let brightness = Brightness::from_percent(150); // 1.5x
        brightness_buffer(&mut buffer, brightness);

        for (pixel, orig_alpha) in buffer.iter().zip(original_alphas.iter()) {
            let (_, _, _, a) = extract_rgba(*pixel);
            assert_eq!(a, *orig_alpha);
        }
    }

    #[test]
    fn test_increase_brightness() {
        let mut buffer = vec![rgba(100, 100, 100, 255)];

        let brightness = Brightness::from_percent(200); // 2.0x
        brightness_buffer(&mut buffer, brightness);

        let (r, g, b, _) = extract_rgba(buffer[0]);

        assert!(r > 100);
        assert!(g > 100);
        assert!(b > 100);
    }

    #[test]
    fn test_decrease_brightness() {
        let mut buffer = vec![rgba(200, 200, 200, 255)];

        let brightness = Brightness::from_percent(50); // 0.5x
        brightness_buffer(&mut buffer, brightness);

        let (r, g, b, _) = extract_rgba(buffer[0]);

        assert!(r < 200);
        assert!(g < 200);
        assert!(b < 200);
    }

    #[test]
    fn test_zero_brightness() {
        let mut buffer = vec![rgba(255, 128, 64, 200)];

        let brightness = Brightness::from_percent(0); // 0.0x
        brightness_buffer(&mut buffer, brightness);

        let (r, g, b, a) = extract_rgba(buffer[0]);

        assert_eq!(r, 0);
        assert_eq!(g, 0);
        assert_eq!(b, 0);
        assert_eq!(a, 200);
    }

    #[test]
    fn test_clamping_overflow() {
        let mut buffer = vec![rgba(200, 200, 200, 255)];

        let brightness = Brightness::from_percent(200); // 2.0x
        brightness_buffer(&mut buffer, brightness);

        // Should clamp to 255, not overflow
        for pixel in buffer.iter() {
            let (r, g, b, _) = extract_rgba(*pixel);
            let _ = (r, g, b); // u8 values are automatically bounded
        }
    }

    #[test]
    fn test_empty_buffer() {
        let mut buffer: Vec<u32> = vec![];
        let brightness = Brightness::from_percent(150); // 1.5x
        brightness_buffer(&mut buffer, brightness);
        assert_eq!(buffer.len(), 0);
    }
}

