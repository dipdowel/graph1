use crate::core::context::GraphContext;
use crate::primitives::plane::RectArea;

// -------------------------------------------------------------------------------------------------
// These are the standard weights for calculating perceived luminance from RGB channels.
// The weights are chosen to match the sensitivity of the human eye to different colors.
const R_WEIGHT: f32 = 0.299;
const G_WEIGHT: f32 = 0.587;
const B_WEIGHT: f32 = 0.114;
// -------------------------------------------------------------------------------------------------


/// Calculates perceived luminance from separate RGB channels. <br />
/// Perceived luminance means the brightness of a color as perceived by the human eye.
/// # Arguments
/// * `red` - The red channel value (0-255).
/// * `green` - The green channel value (0-255).
/// * `blue` - The blue channel value (0-255).
/// # Returns
/// A `u8` representing the weighted luminance.
pub fn rgb_pixel_luminance(red: u8, green: u8, blue: u8) -> u8 {
    (R_WEIGHT * red as f32 + G_WEIGHT * green as f32 + B_WEIGHT * blue as f32).round() as u8
}

/// Calculates perceived luminance from an RGBA color represented as a `u32`. <br />
/// Perceived luminance means the brightness of a color as perceived by the human eye. <br />
/// Ignores the alpha channel and uses only RGB values for the calculation.
/// # Arguments
/// * `rgba` - The color as a `u32` in RGBA format.
/// # Returns
/// A `u8` representing the weighted luminance.
pub fn rgba_pixel_luminance(rgba: u32) -> u8 {
    let red = ((rgba >> 24) & 0xFF) as u8;
    let green = ((rgba >> 16) & 0xFF) as u8;
    let blue = ((rgba >> 8) & 0xFF) as u8;

    (R_WEIGHT * red as f32 + G_WEIGHT * green as f32 + B_WEIGHT * blue as f32).round() as u8
}


/// Calculates perceived luminance of each pixel from a buffer of `u32` with RGBA colors. <br />
/// Ignores the alpha channel and uses only RGB values for the calculation.
/// # Arguments
/// * `dst` - A mutable slice where the calculated luminance values will be stored.
/// * `src` - A slice of RGBA pixels to calculate the luminance from, where each pixel is a `u32`.
/// # Panics
/// Panics if `dst` and `src` have different lengths.
pub fn rgba_buffer_luminance(dst: &mut [u8], src: &[u32])  {
    assert_eq!(dst.len(), src.len(), "Source and destination buffers must have the same length!");

    for (dst_value, &src_color) in dst.iter_mut().zip(src.iter()) {
        let red = ((src_color >> 24) & 0xFF) as u8;
        let green = ((src_color >> 16) & 0xFF) as u8;
        let blue = ((src_color >> 8) & 0xFF) as u8;
        *dst_value = (R_WEIGHT * red as f32 + G_WEIGHT * green as f32 + B_WEIGHT * blue as f32).round() as u8;
    }
}


pub fn rgba_region_luminance<UserData>(ctx: &mut GraphContext<UserData>, region: &RectArea) {
    // Dereference the options
    let start_x = region.top_left.x;
    let start_y = region.top_left.y;
    let width = region.dimensions.w;
    let height = region.dimensions.h;

    // Nothing to draw here
    if width == 0 || height == 0 {
        return;
    }

    let end_x = start_x + width;
    let end_y = start_y + height;
    let mut x = start_x;
    let mut y = start_y;

    // Which pixel in the vector should be filled in next.
    let mut pixel_index: usize;
    let mut resulting_color: u32 = 0;

    loop {
        pixel_index = (y * ctx.win.w + x) as usize;
        let intensity = rgba_pixel_luminance(ctx.frame_buf[pixel_index]);
        resulting_color =
            (intensity as u32) << 24 | (intensity as u32) << 16 | (intensity as u32) << 8 | 0xff;

        ctx.frame_buf[pixel_index] = resulting_color;

        x += 1;

        if x == end_x || x == ctx.win.w {
            y += 1;
            x = start_x;
        };

        if y == end_y || y == ctx.win.h {
            break;
        }
    }
}


/*
    // TODO: Fix the tests!

    // -------------------------------------------------------------------------------------------------
    // Unit tests for each function
    // -------------------------------------------------------------------------------------------------
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_rgb_pixel_luminance() {
            // Pure white
            assert_eq!(rgb_pixel_luminance(255, 255, 255), 255);
            // Pure black
            assert_eq!(rgb_pixel_luminance(0, 0, 0), 0);
            // Pure red
            assert_eq!(rgb_pixel_luminance(255, 0, 0), 76);
            // Pure green
            assert_eq!(rgb_pixel_luminance(0, 255, 0), 149);
            // Pure blue
            assert_eq!(rgb_pixel_luminance(0, 0, 255), 29);
        }

        #[test]
        fn test_rgba_pixel_luminance() {
            // Pure white with full alpha
            assert_eq!(rgba_pixel_luminance(0xFFFFFFFF), 255);
            // Pure black with full alpha
            assert_eq!(rgba_pixel_luminance(0xFF000000), 0);
            // Red channel only with full alpha
            assert_eq!(rgba_pixel_luminance(0xFF0000FF), 29);
            // Green channel only with full alpha
            assert_eq!(rgba_pixel_luminance(0x00FF00FF), 149);
            // Blue channel only with full alpha
            assert_eq!(rgba_pixel_luminance(0x0000FFFF), 76);
        }

        #[test]
        fn test_rgba_buffer_luminance() {
            let src = [0xFFFFFFFF, 0xFF000000, 0xFF0000FF, 0x00FF00FF, 0x0000FFFF];
            let mut dst = [0u8; 5];

            rgba_buffer_luminance(&mut dst, &src);

            assert_eq!(dst, [255, 0, 29, 149, 76]);
        }

        #[test]
        #[should_panic(expected = "Source and destination buffers must have the same length!")]
        fn test_rgba_buffer_luminance_panic() {
            let src = [0xFFFFFFFF, 0xFF000000];
            let mut dst = [0u8; 3]; // Mismatched length
            rgba_buffer_luminance(&mut dst, &src); // Should panic
        }
    }

     */