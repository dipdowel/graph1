use crate::utils::color::adapters::AdapterStatistics;

/// Converts the source buffer `src` from RGBA to 0RGB and writes the result to `dst`.
/// # Arguments
///
/// * `dst` - A mutable slice where the converted 0RGB pixels will be stored.
/// * `src` - A slice of RGBA pixels to convert, where each pixel is a `u32`.
/// * `stats` - If `true`, calculate and return some statistics (makes the conversion just a little slower).
///
/// # Safety
/// This function uses unsafe code to bypass bounds checks. Ensure `dst` and `src`
/// have the same length and do not overlap before calling this function.
///
/// # Panics
/// Panics if `dst` and `src` have different lengths.
/// # Returns
/// An `AdapterStatistics` struct containing some basics statistics on the conversion.
pub fn rgba_to_0rgb_unsafe(dst: &mut [u32], src: &[u32], stats: bool) -> Option<AdapterStatistics> {
    assert_eq!(
        dst.len(),
        src.len(),
        "Source and destination buffers must have the same length!"
    );

    let len = src.len();

    unsafe {
        let src_ptr = src.as_ptr();
        let dst_ptr = dst.as_mut_ptr();

        if !stats {
            // Unsafe loop without bounds checks
            for i in 0..len {
                let src_pixel = *src_ptr.add(i);

                // Extract color channels
                let r = (src_pixel >> 24) & 0xFF;
                let g = (src_pixel >> 16) & 0xFF;
                let b = (src_pixel >> 8) & 0xFF;

                // Reassemble and write to dst
                *dst_ptr.add(i) = (0 << 24) | (r << 16) | (g << 8) | b;
            }
            return None;
        }

        // Slower version with statistics
        let mut total_r = 0u64;
        let mut total_g = 0u64;
        let mut total_b = 0u64;

        for i in 0..len {
            let src_pixel = *src_ptr.add(i);

            // Extract color channels
            let r = (src_pixel >> 24) & 0xFF;
            let g = (src_pixel >> 16) & 0xFF;
            let b = (src_pixel >> 8) & 0xFF;

            // Reassemble and write to dst
            *dst_ptr.add(i) = (0 << 24) | (r << 16) | (g << 8) | b;

            // Accumulate color values for statistics
            total_r += r as u64;
            total_g += g as u64;
            total_b += b as u64;
        }

        let num_pixels = len as u64;
        let avg_r = (total_r / num_pixels) as u32;
        let avg_g = (total_g / num_pixels) as u32;
        let avg_b = (total_b / num_pixels) as u32;

        let average_color = (avg_r << 24) | (avg_g << 16) | (avg_b << 8) | 0xff;

        Some(AdapterStatistics {
            average_color,
            average_red: avg_r & 0xff,
            average_green: avg_g & 0xff,
            average_blue: avg_b & 0xff,
            num_pixels: num_pixels as u32,
        })
    }
}