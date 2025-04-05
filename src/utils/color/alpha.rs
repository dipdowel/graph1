/// Blends foreground and background colors with integer alpha for the foreground
/// - `bg` is the background color in RGBA format as u32
/// - `fg` is the foreground color in RGBA format as u32 (alpha included in the last 8 bits)
#[inline(always)]
pub fn blend_pixel_int(bg: u32, fg: u32) -> u32 {
    // Extract alpha from the foreground color
    let alpha = (fg & 0xff) as u8;
    let inv_alpha = 255 - alpha;

    // Extract RGBA channels from background and foreground
    let bg_r = ((bg >> 24) & 0xff) as u16;
    let bg_g = ((bg >> 16) & 0xff) as u16;
    let bg_b = ((bg >> 8) & 0xff) as u16;
    let bg_a = (bg & 0xff) as u16;

    let fg_r = ((fg >> 24) & 0xff) as u16;
    let fg_g = ((fg >> 16) & 0xff) as u16;
    let fg_b = ((fg >> 8) & 0xff) as u16;
    let fg_a = (fg & 0xff) as u16;

    // Blend each color channel
    let blended_r = (fg_r * alpha as u16 + bg_r * inv_alpha as u16) / 255;
    let blended_g = (fg_g * alpha as u16 + bg_g * inv_alpha as u16) / 255;
    let blended_b = (fg_b * alpha as u16 + bg_b * inv_alpha as u16) / 255;

    // Blend the alpha channel itself (to handle partial transparency)
    let blended_a = (fg_a * alpha as u16 + bg_a * inv_alpha as u16) / 255;

    // Reassemble channels into a single u32 RGBA value
    ((blended_r as u32) << 24)
        | ((blended_g as u32) << 16)
        | ((blended_b as u32) << 8)
        | (blended_a as u32)
}

/// Blends foreground and background colors using `f32` for higher precision.
/// - `bg` is the background color in RGBA format as `u32`
/// - `fg` is the foreground color in RGBA format as `u32` (alpha included in the last 8 bits)
/// Returns a `u32` representing the blended color.
#[inline(always)]
pub fn blend_pixel_f32(bg: u32, fg: u32) -> u32 {
    // Extract the alpha from the foreground color and normalize it to a range of 0.0 - 1.0
    let alpha = ((fg & 0xff) as f32) / 255.0;
    let inv_alpha = 1.0 - alpha;

    // Extract RGBA channels from background and foreground and normalize to `f32`
    let bg_r = ((bg >> 24) & 0xff) as f32;
    let bg_g = ((bg >> 16) & 0xff) as f32;
    let bg_b = ((bg >> 8) & 0xff) as f32;
    let bg_a = (bg & 0xff) as f32;

    let fg_r = ((fg >> 24) & 0xff) as f32;
    let fg_g = ((fg >> 16) & 0xff) as f32;
    let fg_b = ((fg >> 8) & 0xff) as f32;
    let fg_a = (fg & 0xff) as f32;

    // Blend each color channel with `f32` precision
    let blended_r = fg_r * alpha + bg_r * inv_alpha;
    let blended_g = fg_g * alpha + bg_g * inv_alpha;
    let blended_b = fg_b * alpha + bg_b * inv_alpha;

    // Blend the alpha channel as well, resulting in final transparency
    let blended_a = fg_a * alpha + bg_a * inv_alpha;

    // Convert back to `u32` and assemble into a single RGBA value
    ((blended_r.round() as u32) << 24)
        | ((blended_g.round() as u32) << 16)
        | ((blended_b.round() as u32) << 8)
        | (blended_a.round() as u32)
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fully_opaque_foreground() {
        let bg = 0x64_96_c8_ff; // Background color (100, 150, 200, fully opaque)
        let fg = 0xff_00_00_ff; // Foreground color (255, 0, 0, fully opaque)

        // Since foreground is fully opaque, result should be equal to foreground
        assert_eq!(blend_pixel_int(bg, fg), fg);
    }

    #[test]
    fn test_fully_transparent_foreground() {
        let bg = 0x64_96_c8_ff; // Background color (100, 150, 200, fully opaque)
        let fg = 0xff_00_00_00; // Foreground color (255, 0, 0, fully transparent)

        // Since foreground is fully transparent, result should be equal to background
        assert_eq!(blend_pixel_int(bg, fg), bg);
    }



    // FIXME: THIS TEST BREAKS!!!!
    //====================================================================================================
        #[test]
        fn test_half_transparent_foreground() {
            let bg = 0x64_96_c8_ff; // Background color (100, 150, 200, fully opaque)
            let fg = 0xff_00_00_80; // Foreground color (255, 0, 0, half transparent)

            // Expected color manually calculated for this blend
            let expected = 0xc8_64_64_7f; // Blended color

            assert_eq!(blend_pixel_int(bg, fg), expected);
        }

            #[test]
            fn test_partially_transparent_background() {
                let bg = 0x64_96_c8_88; // Background color (100, 150, 200, partially transparent)
                let fg = 0x00_ff_00_88; // Foreground color (0, 255, 0, partially transparent)

                // Expected color manually calculated for this blend
                let expected = 0x32_99_c4_88; // Blended color

                assert_eq!(blend_pixel_int(bg, fg), expected);
            }

            #[test]
            fn test_both_semi_transparent() {
                let bg = 0x64_96_c8_88; // Background color (100, 150, 200, semi-transparent)
                let fg = 0xff_00_00_88; // Foreground color (255, 0, 0, semi-transparent)

                // Expected color manually calculated for this blend
                let expected = 0xc8_4c_64_88; // Blended color

                assert_eq!(blend_pixel_int(bg, fg), expected);
            }
}*/
