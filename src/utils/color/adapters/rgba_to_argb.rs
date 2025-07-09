use crate::utils::color::adapters::AdapterStatistics;

/// Converts the source buffer `src` from RGBA to ARGB and writes the result to `dst`.
/// # Arguments
///
/// * `dst` - A mutable slice where the converted ARGB pixels will be stored.
/// * `src` - A slice of RGBA pixels to convert, where each pixel is a `u32`.
/// * `stats` - If `true`, calculate and return some statistics (makes the conversion just a little slower).
///
/// # Panics
///
/// Panics if `dst` and `src` have different lengths.
/// # Returns
/// An `AdapterStatistics` struct containing some basics statistics on the conversion.
pub fn rgba_to_argb(dst: &mut [u32], src: &[u32], stats: bool) -> Option<AdapterStatistics> {
    assert_eq!(
        dst.len(),
        src.len(),
        "Source and destination buffers must have the same length!"
    );

    // Statistics tracking variables
    let (mut total_r, mut total_g, mut total_b, mut num_pixels) = (0u64, 0u64, 0u64, 0u64);

    dst.iter_mut()
        .zip(src.iter())
        .for_each(|(dst_pixel, &src_pixel)| {
            // Extract RGBA channels
            let (r, g, b, a) = (
                (src_pixel >> 24) & 0xFF,
                (src_pixel >> 16) & 0xFF,
                (src_pixel >> 8) & 0xFF,
                src_pixel & 0xFF,
            );

            // Reassemble ARGB
            *dst_pixel = (a << 24) | (r << 16) | (g << 8) | b;

            // Accumulate statistics if required
            if stats {
                total_r += r as u64;
                total_g += g as u64;
                total_b += b as u64;
                num_pixels += 1;
            }
        });

    // Return statistics if requested
    if stats && num_pixels > 0 {
        let avg_r = (total_r / num_pixels) as u32;
        let avg_g = (total_g / num_pixels) as u32;
        let avg_b = (total_b / num_pixels) as u32;

        let average_color = (0xFF << 24) | (avg_r << 16) | (avg_g << 8) | avg_b;

        return Some(AdapterStatistics {
            average_color,
            average_red: avg_r & 0xFF,
            average_green: avg_g & 0xFF,
            average_blue: avg_b & 0xFF,
            num_pixels: num_pixels as u32,
        });
    }

    None
}
