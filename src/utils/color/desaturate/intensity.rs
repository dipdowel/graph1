use crate::graph1_core::context::GraphContext;
use crate::primitives::plane::RectArea;

/// Calculates intensity of an RGB color by taking a simple mean of the channels.
/// # Arguments
/// * `red` - The red channel value (0-255).
/// * `green` - The green channel value (0-255).
/// * `blue` - The blue channel value (0-255).
/// * `squared` - If `true`, calculates the intensity as the square root of the sum of squares of the channels. Slower but more accurate.
///
/// # Returns
/// A `u8` representing the average intensity.
pub fn rgb_pixel_intensity(red: u8, green: u8, blue: u8, squared: bool) -> u8 {
    if squared {
        return (red as f32 * red as f32
            + green as f32 * green as f32
            + blue as f32 * blue as f32 / 3f32)
            .sqrt() as u8;
    }

    ((red as u16 + green as u16 + blue as u16) / 3) as u8
}

/// Converts an RGBA color to IIIA . <br />
/// IIIA = Intensity, Intensity, Intensity, Alpha <br />
/// Ignores the alpha channel and uses only RGB values for the calculation.
/// # Arguments
/// * `color` - The color as a `u32` in RGBA format.
/// * `squared` - If `true`, calculates the intensity as the square root of the sum of squares of the channels. Slower but more accurate.
///
/// # Returns
/// A `u32` representing the average intensity.
pub fn rgba_pixel_intensity(color: u32, squared: bool) -> u32 {


    let red = (color >> 24) & 0xff;
    let green = (color >> 16) & 0xff;
    let blue = (color >> 8) & 0xff;

    if squared {
        return ((red * red + green * green + blue * blue) as f32 / 3f32).sqrt() as u32;
    }

    (red + green + blue) / 3
}

/// Calculates intensity of each pixel from a buffer of `u32` with RGBA colors. <br />
/// Ignores the alpha channel and uses only RGB values for the calculation.
/// The resulting intensity of each pixel
/// # Arguments
/// * `dst` - A mutable slice where the calculated intensities will be stored.
/// * `src` - A slice of RGBA pixels to calculate the intensity from, where each pixel is a `u32`.
/// * `squared` - If `true`, calculates the intensity as the square root of the sum of squares of the channels. Slower but more accurate.
///
/// # Panics
/// Panics if `dst` and `src` have different lengths.
pub fn rgba_buffer_intensity(dst: &mut [u32], src: &[u32], squared: bool) {
    assert_eq!(
        dst.len(),
        src.len(),
        "Source and destination buffers must have the same length!"
    );

    for (dst_value, &src_color) in dst.iter_mut().zip(src.iter()) {
        let red = src_color >> 24 & 0xff;
        let green = src_color >> 16 & 0xff;
        let blue = src_color >> 8 & 0xff;
        let original_alpha = src_color & 0x00_00_00_ff;

        let intensity: u32 = if squared {
            ((red * red + green * green + blue * blue) as f32 / 3f32).sqrt() as u32
        } else {
            (red + green + blue) / 3
        };

        *dst_value = intensity << 24 | intensity << 16 | intensity << 8 | original_alpha;
    }
}

/// Converts RGBA pixels in the frame buffer to IIIA pixels. <br /
/// IIIA = Intensity, Intensity, Intensity, Alpha <br />
/// The operation occurs in-place.
/// The alpha channel remains unchanged.
/// # Arguments
///
pub fn rgba_region_intensity<UserData>(
    ctx: &mut GraphContext<UserData>,
    region: &RectArea,
    squared: bool,
) {
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
        let intensity = rgba_pixel_intensity(ctx.frame_buf[pixel_index], squared);

        let original_alpha = ctx.frame_buf[pixel_index] & 0x00_00_00_ff;

        resulting_color = intensity << 24 | intensity << 16 | intensity << 8 | original_alpha;

        // resulting_color = 0x00_00_00_ff | intensity_level << 16 | intensity_level << 8 | intensity_level;
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
FIXME: Fix the tests!
FIXME: Fix the tests!
FIXME: Fix the tests!
FIXME: Fix the tests!

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_pixel_intensity_basic() {
        // Basic average of RGB values
        assert_eq!(rgb_pixel_intensity(100, 150, 200), 150);
    }

    #[test]
    fn test_rgb_pixel_intensity_edges() {
        // Test for all channels zero
        assert_eq!(rgb_pixel_intensity(0, 0, 0), 0);
        // Test for all channels max (255)
        assert_eq!(rgb_pixel_intensity(255, 255, 255), 255);
    }

    #[test]
    fn test_rgba_pixel_intensity_basic() {
        // Basic case with typical RGB values
        let color = (100 << 24) | (150 << 16) | (200 << 8) | 255;
        assert_eq!(rgba_pixel_intensity(color), 150);
    }

    #[test]
    fn test_rgba_pixel_intensity_ignore_alpha() {
        // Alpha channel should be ignored
        let color_with_alpha = (100 << 24) | (150 << 16) | (200 << 8) | 123;
        assert_eq!(rgba_pixel_intensity(color_with_alpha), 150);
    }

    #[test]
    fn test_rgba_pixel_intensity_edges() {
        // All channels zero
        let color = 0;
        assert_eq!(rgba_pixel_intensity(color), 0);
        // All channels max (ignoring alpha)
        let color = (255 << 24) | (255 << 16) | (255 << 8) | 255;
        assert_eq!(rgba_pixel_intensity(color), 255);
    }

    #[test]
    fn test_rgba_buffer_intensity_basic() {
        let src = [
            (100 << 24) | (150 << 16) | (200 << 8) | 255,
            (50 << 24) | (100 << 16) | (150 << 8) | 255,
        ];
        let mut dst = [0; 2];
        rgba_buffer_intensity(&mut dst, &src);
        assert_eq!(dst, [150, 100]);
    }

    #[test]
    #[should_panic(expected = "Source and destination buffers must have the same length!")]
    fn test_rgba_buffer_intensity_mismatched_lengths() {
        let src = [(100 << 24) | (150 << 16) | (200 << 8) | 255];
        let mut dst = [0; 2];
        rgba_buffer_intensity(&mut dst, &src);
    }

    #[test]
    fn test_rgba_buffer_intensity_empty() {
        let src: [u32; 0] = [];
        let mut dst: [u8; 0] = [];
        rgba_buffer_intensity(&mut dst, &src);
        assert_eq!(dst, []);
    }
}
*/
