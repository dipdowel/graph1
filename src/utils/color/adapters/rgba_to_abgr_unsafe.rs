use crate::utils::color::adapters::AdapterStatistics;

/// Unsafe version of `rgba_to_abgr` for high performance.
///
/// # Safety
/// This function assumes that:
/// 1. `src` and `dst` are of the same length.
/// 2. `src` and `dst` slices do not overlap.
/// 3. The provided slices are properly aligned and valid for `u32` operations.
///
/// Incorrect usage may lead to undefined behavior.
///
/// # Arguments
/// - `dst`: Mutable slice to store converted ABGR pixels.
/// - `src`: Slice of RGBA pixels to convert.
/// - `stats`: If `true`, calculate and return conversion statistics.
///
/// # Returns
/// Option containing `AdapterStatistics` if `stats` is `true`; otherwise, `None`.
pub unsafe fn rgba_to_abgr_unsafe(
    dst: &mut [u32],
    src: &[u32],
    stats: bool,
) -> Option<AdapterStatistics> {
    assert_eq!(
        dst.len(),
        src.len(),
        "Source and destination buffers must have the same length!"
    );

    // Initialize variables for statistics if needed
    let mut total_r = 0u64;
    let mut total_g = 0u64;
    let mut total_b = 0u64;
    let num_pixels = src.len() as u64;

    // Get raw pointers
    let src_ptr = src.as_ptr();
    let dst_ptr = dst.as_mut_ptr();
    let len = src.len();

    for i in 0..len {
        // Read the RGBA pixel
        let src_pixel = *src_ptr.add(i);

        // Extract individual color channels
        let r = (src_pixel >> 24) & 0xFF;
        let g = (src_pixel >> 16) & 0xFF;
        let b = (src_pixel >> 8) & 0xFF;
        let a = src_pixel & 0xFF;

        // Reassemble in ABGR format
        let abgr_pixel = (a << 24) | (b << 16) | (g << 8) | r;

        // Write the ABGR pixel
        *dst_ptr.add(i) = abgr_pixel;

        // Accumulate channel values for statistics if enabled
        if stats {
            total_r += r as u64;
            total_g += g as u64;
            total_b += b as u64;
        }
    }

    // If statistics are enabled, calculate and return them
    if stats {
        let avg_r = (total_r / num_pixels) as u32;
        let avg_g = (total_g / num_pixels) as u32;
        let avg_b = (total_b / num_pixels) as u32;

        let average_color = (avg_r << 24) | (avg_g << 16) | (avg_b << 8) | 0xFF;

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
