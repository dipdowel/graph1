/// Calculates intensity of an RGB color by taking a simple mean of the channels.
/// # Arguments
/// * `red` - The red channel value (0-255).
/// * `green` - The green channel value (0-255).
/// * `blue` - The blue channel value (0-255).
///
/// # Returns
/// A `u8` representing the average intensity.
pub fn rgb_pixel_intensity(red: u8, green: u8, blue: u8) -> u8 {
    ((red as u16 + green as u16 + blue as u16) / 3) as u8
}


/// Calculates intensity from an RGBA color (`u32`) by taking a simple mean of the channels. <br />
/// Ignores the alpha channel and uses only RGB values for the calculation.
/// # Arguments
/// * `color` - The color as a `u32` in RGBA format.
///
/// # Returns
/// A `u8` representing the average intensity.
pub fn rgba_pixel_intensity(color: u32) -> u8 {
    let red = ((color >> 24) & 0xff) as u8;
    let green = ((color >> 16) & 0xff) as u8;
    let blue = ((color >> 8) & 0xff) as u8;

    ((red as u16 + green as u16 + blue as u16) / 3) as u8
}

/// Calculates intensity of each pixel from a buffer of `u32` with RGBA colors. <br />
/// Ignores the alpha channel and uses only RGB values for the calculation.
/// # Arguments
/// * `dst` - A mutable slice where the calculated intensities will be stored.
/// * `src` - A slice of RGBA pixels to calculate the intensity from, where each pixel is a `u32`.
/// # Panics
/// Panics if `dst` and `src` have different lengths.
pub fn rgba_buffer_intensity(dst: &mut [u8], src: &[u32])  {
    assert_eq!(dst.len(), src.len(), "Source and destination buffers must have the same length!");

    for (dst_value, &src_color) in dst.iter_mut().zip(src.iter()) {
        let red = ((src_color >> 24) & 0xff) as u8;
        let green = ((src_color >> 16) & 0xff) as u8;
        let blue = ((src_color >> 8) & 0xff) as u8;
        *dst_value = ((red as u16 + green as u16 + blue as u16) / 3) as u8;
    }
}


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
