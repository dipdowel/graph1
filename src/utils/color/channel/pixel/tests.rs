#[cfg(test)]
mod tests {
    use super::super::{from_rgb, to_rgb};

    // Test pixel: RGBA(255, 128, 64, 32) = 0xFF80_4020
    const TEST_PIXEL: u32 = 0xff_80_40_20;
    const TEST_R: u8 = 0xff; // 255
    const TEST_G: u8 = 0x80; // 128
    const TEST_B: u8 = 0x40; // 64
    const TEST_A: u8 = 0x20; // 32

    // Test pixel: RGBA(0, 0, 0, 0) = 0x0000_0000
    const BLACK_TRANSPARENT: u32 = 0x0000_0000;

    // Test pixel: RGBA(255, 255, 255, 255) = 0xFFFF_FFFF
    const WHITE_OPAQUE: u32 = 0xFFFF_FFFF;

    #[test]
    fn test_to_u32() {
        let (r, g, b, a) = to_rgb::as_u32(TEST_PIXEL);
        assert_eq!(r, TEST_R as u32);
        assert_eq!(g, TEST_G as u32);
        assert_eq!(b, TEST_B as u32);
        assert_eq!(a, TEST_A as u32);
    }

    #[test]
    fn test_to_u8() {
        let (r, g, b, a) = to_rgb::as_u8(TEST_PIXEL);
        assert_eq!(r, TEST_R);
        assert_eq!(g, TEST_G);
        assert_eq!(b, TEST_B);
        assert_eq!(a, TEST_A);
    }

    #[test]
    fn test_to_u16() {
        let (r, g, b, a) = to_rgb::as_u16(TEST_PIXEL);
        assert_eq!(r, TEST_R as u16);
        assert_eq!(g, TEST_G as u16);
        assert_eq!(b, TEST_B as u16);
        assert_eq!(a, TEST_A as u16);
    }

    #[test]
    fn test_to_f32() {
        let (r, g, b, a) = to_rgb::as_f32(TEST_PIXEL);
        assert_eq!(r, TEST_R as f32);
        assert_eq!(g, TEST_G as f32);
        assert_eq!(b, TEST_B as f32);
        assert_eq!(a, TEST_A as f32);
    }

    #[test]
    fn test_to_f64() {
        let (r, g, b, a) = to_rgb::as_f64(TEST_PIXEL);
        assert_eq!(r, TEST_R as f64);
        assert_eq!(g, TEST_G as f64);
        assert_eq!(b, TEST_B as f64);
        assert_eq!(a, TEST_A as f64);
    }

    #[test]
    fn test_to_f32_normalized() {
        let (r, g, b, a) = to_rgb::as_f32_normalized(TEST_PIXEL);
        assert!((r - (TEST_R as f32 / 255.0)).abs() < 0.001);
        assert!((g - (TEST_G as f32 / 255.0)).abs() < 0.001);
        assert!((b - (TEST_B as f32 / 255.0)).abs() < 0.001);
        assert!((a - (TEST_A as f32 / 255.0)).abs() < 0.001);

        // Test edge cases
        let (r, g, b, a) = to_rgb::as_f32_normalized(BLACK_TRANSPARENT);
        assert_eq!(r, 0.0);
        assert_eq!(g, 0.0);
        assert_eq!(b, 0.0);
        assert_eq!(a, 0.0);

        let (r, g, b, a) = to_rgb::as_f32_normalized(WHITE_OPAQUE);
        assert_eq!(r, 1.0);
        assert_eq!(g, 1.0);
        assert_eq!(b, 1.0);
        assert_eq!(a, 1.0);
    }

    #[test]
    fn test_to_f64_normalized() {
        let (r, g, b, a) = to_rgb::as_f64_normalized(TEST_PIXEL);
        assert!((r - (TEST_R as f64 / 255.0)).abs() < 0.001);
        assert!((g - (TEST_G as f64 / 255.0)).abs() < 0.001);
        assert!((b - (TEST_B as f64 / 255.0)).abs() < 0.001);
        assert!((a - (TEST_A as f64 / 255.0)).abs() < 0.001);

        // Test edge cases
        let (r, g, b, a) = to_rgb::as_f64_normalized(BLACK_TRANSPARENT);
        assert_eq!(r, 0.0);
        assert_eq!(g, 0.0);
        assert_eq!(b, 0.0);
        assert_eq!(a, 0.0);

        let (r, g, b, a) = to_rgb::as_f64_normalized(WHITE_OPAQUE);
        assert_eq!(r, 1.0);
        assert_eq!(g, 1.0);
        assert_eq!(b, 1.0);
        assert_eq!(a, 1.0);
    }

    #[test]
    fn test_from_u32() {
        let pixel = from_rgb::from_u32(TEST_R as u32, TEST_G as u32, TEST_B as u32, TEST_A as u32);
        assert_eq!(pixel, TEST_PIXEL);

        // Test masking behavior - values > 255 should be masked
        let pixel = from_rgb::from_u32(0x1FF, 0x180, 0x140, 0x120);
        assert_eq!(pixel, TEST_PIXEL);
    }

    #[test]
    fn test_from_u8() {
        let pixel = from_rgb::from_u8(TEST_R, TEST_G, TEST_B, TEST_A);
        assert_eq!(pixel, TEST_PIXEL);

        let pixel = from_rgb::from_u8(0, 0, 0, 0);
        assert_eq!(pixel, BLACK_TRANSPARENT);

        let pixel = from_rgb::from_u8(255, 255, 255, 255);
        assert_eq!(pixel, WHITE_OPAQUE);
    }

    #[test]
    fn test_from_u16() {
        let pixel = from_rgb::from_u16(TEST_R as u16, TEST_G as u16, TEST_B as u16, TEST_A as u16);
        assert_eq!(pixel, TEST_PIXEL);

        // Test masking behavior - values > 255 should be masked
        let pixel = from_rgb::from_u16(0x1FF, 0x180, 0x140, 0x120);
        assert_eq!(pixel, TEST_PIXEL);
    }

    #[test]
    fn test_from_f32() {
        let pixel = from_rgb::from_f32(TEST_R as f32, TEST_G as f32, TEST_B as f32, TEST_A as f32);
        assert_eq!(pixel, TEST_PIXEL);

        let pixel = from_rgb::from_f32(0.0, 0.0, 0.0, 0.0);
        assert_eq!(pixel, BLACK_TRANSPARENT);

        let pixel = from_rgb::from_f32(255.0, 255.0, 255.0, 255.0);
        assert_eq!(pixel, WHITE_OPAQUE);
    }

    #[test]
    fn test_from_f64() {
        let pixel = from_rgb::from_f64(TEST_R as f64, TEST_G as f64, TEST_B as f64, TEST_A as f64);
        assert_eq!(pixel, TEST_PIXEL);

        let pixel = from_rgb::from_f64(0.0, 0.0, 0.0, 0.0);
        assert_eq!(pixel, BLACK_TRANSPARENT);

        let pixel = from_rgb::from_f64(255.0, 255.0, 255.0, 255.0);
        assert_eq!(pixel, WHITE_OPAQUE);
    }

    #[test]
    fn test_from_f32_normalized() {
        // Use exact normalized values
        let pixel = from_rgb::from_f32_normalized(1.0, 0.5, 0.25, 0.125);
        let (r, g, b, a) = to_rgb::as_u8(pixel);
        assert_eq!(r, 255); // 1.0 * 255 = 255
        assert_eq!(g, 127); // 0.5 * 255 = 127.5 -> 127
        assert_eq!(b, 63);  // 0.25 * 255 = 63.75 -> 63
        assert_eq!(a, 31);  // 0.125 * 255 = 31.875 -> 31

        let pixel = from_rgb::from_f32_normalized(0.0, 0.0, 0.0, 0.0);
        assert_eq!(pixel, BLACK_TRANSPARENT);

        let pixel = from_rgb::from_f32_normalized(1.0, 1.0, 1.0, 1.0);
        assert_eq!(pixel, WHITE_OPAQUE);

        // Test clamping
        let pixel = from_rgb::from_f32_normalized(2.0, -1.0, 0.5, 1.5);
        let (r, g, b, a) = to_rgb::as_u8(pixel);
        assert_eq!(r, 255); // clamped from 2.0
        assert_eq!(g, 0);   // clamped from -1.0
        assert_eq!(b, 127); // 0.5 * 255
        assert_eq!(a, 255); // clamped from 1.5
    }

    #[test]
    fn test_from_f64_normalized() {
        // Use exact normalized values
        let pixel = from_rgb::from_f64_normalized(1.0, 0.5, 0.25, 0.125);
        let (r, g, b, a) = to_rgb::as_u8(pixel);
        assert_eq!(r, 255); // 1.0 * 255 = 255
        assert_eq!(g, 127); // 0.5 * 255 = 127.5 -> 127
        assert_eq!(b, 63);  // 0.25 * 255 = 63.75 -> 63
        assert_eq!(a, 31);  // 0.125 * 255 = 31.875 -> 31

        let pixel = from_rgb::from_f64_normalized(0.0, 0.0, 0.0, 0.0);
        assert_eq!(pixel, BLACK_TRANSPARENT);

        let pixel = from_rgb::from_f64_normalized(1.0, 1.0, 1.0, 1.0);
        assert_eq!(pixel, WHITE_OPAQUE);

        // Test clamping
        let pixel = from_rgb::from_f64_normalized(2.0, -1.0, 0.5, 1.5);
        let (r, g, b, a) = to_rgb::as_u8(pixel);
        assert_eq!(r, 255); // clamped from 2.0
        assert_eq!(g, 0);   // clamped from -1.0
        assert_eq!(b, 127); // 0.5 * 255
        assert_eq!(a, 255); // clamped from 1.5
    }

    #[test]
    fn test_roundtrip_u8() {
        let pixel = from_rgb::from_u8(TEST_R, TEST_G, TEST_B, TEST_A);
        let (r, g, b, a) = to_rgb::as_u8(pixel);
        assert_eq!(r, TEST_R);
        assert_eq!(g, TEST_G);
        assert_eq!(b, TEST_B);
        assert_eq!(a, TEST_A);
    }

    #[test]
    fn test_roundtrip_u32() {
        let pixel = from_rgb::from_u32(TEST_R as u32, TEST_G as u32, TEST_B as u32, TEST_A as u32);
        let (r, g, b, a) = to_rgb::as_u32(pixel);
        assert_eq!(r, TEST_R as u32);
        assert_eq!(g, TEST_G as u32);
        assert_eq!(b, TEST_B as u32);
        assert_eq!(a, TEST_A as u32);
    }

    #[test]
    fn test_roundtrip_f32() {
        let pixel = from_rgb::from_f32(TEST_R as f32, TEST_G as f32, TEST_B as f32, TEST_A as f32);
        let (r, g, b, a) = to_rgb::as_f32(pixel);
        assert_eq!(r, TEST_R as f32);
        assert_eq!(g, TEST_G as f32);
        assert_eq!(b, TEST_B as f32);
        assert_eq!(a, TEST_A as f32);
    }

    #[test]
    fn test_roundtrip_f32_normalized() {
        // Use exact values that roundtrip perfectly
        let pixel = from_rgb::from_u8(100, 150, 200, 250);
        let (r, g, b, a) = to_rgb::as_f32_normalized(pixel);
        let pixel2 = from_rgb::from_f32_normalized(r, g, b, a);
        assert_eq!(pixel, pixel2);
    }

    #[test]
    fn test_roundtrip_f64_normalized() {
        // Use exact values that roundtrip perfectly
        let pixel = from_rgb::from_u8(100, 150, 200, 250);
        let (r, g, b, a) = to_rgb::as_f64_normalized(pixel);
        let pixel2 = from_rgb::from_f64_normalized(r, g, b, a);
        assert_eq!(pixel, pixel2);
    }

    #[test]
    fn test_pixel_format() {
        // Verify that the pixel format is RGBA in big-endian order
        let pixel = from_rgb::from_u8(0xAA, 0xBB, 0xCC, 0xDD);
        assert_eq!(pixel, 0xAABB_CCDD);

        // Extract and verify
        assert_eq!((pixel >> 24) & 0xFF, 0xAA); // R
        assert_eq!((pixel >> 16) & 0xFF, 0xBB); // G
        assert_eq!((pixel >> 8) & 0xFF, 0xCC);  // B
        assert_eq!(pixel & 0xFF, 0xDD);         // A
    }

    #[test]
    fn test_all_channels_independent() {
        // Test that each channel is independent
        let pixel_r = from_rgb::from_u8(255, 0, 0, 0);
        assert_eq!(pixel_r, 0xFF00_0000);

        let pixel_g = from_rgb::from_u8(0, 255, 0, 0);
        assert_eq!(pixel_g, 0x00FF_0000);

        let pixel_b = from_rgb::from_u8(0, 0, 255, 0);
        assert_eq!(pixel_b, 0x0000_FF00);

        let pixel_a = from_rgb::from_u8(0, 0, 0, 255);
        assert_eq!(pixel_a, 0x0000_00FF);
    }
}

