#[derive(Debug)]
pub struct AdapterStatistics {
    /// Average color (ABGR) of the pixels that were converted
    pub average_color: u32,
    /// Average value of the red channel
    pub average_red: u32,
    /// Average value of the green channel
    pub average_green: u32,
    /// Average value of the blue channel
    pub average_blue: u32,
    /// Number of pixels that were converted
    pub num_pixels: u32,
}

/// Converts the source buffer `src` from RGBA to ABGR and writes the result to `dst`.
/// # Arguments
///
/// * `dst` - A mutable slice where the converted ABGR pixels will be stored.
/// * `src` - A slice of RGBA pixels to convert, where each pixel is a `u32`.
/// * `stats` - If `true`, calculate and return some statistics (makes the conversion just a little slower).
///
/// # Panics
///
/// Panics if `dst` and `src` have different lengths.
/// # Returns
/// An `AdapterStatistics` struct containing some basics statistics on the conversion.
pub fn rgba_to_abgr(dst: &mut [u32], src: &[u32], stats: bool) -> Option<AdapterStatistics> {
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

            // Reassemble ABGR
            *dst_pixel = (a << 24) | (b << 16) | (g << 8) | r;

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

        let average_color = (avg_r << 24) | (avg_g << 16) | (avg_b << 8) | 0xFF;

        Some(AdapterStatistics {
            average_color,
            average_red: avg_r & 0xFF,
            average_green: avg_g & 0xFF,
            average_blue: avg_b & 0xFF,
            num_pixels: num_pixels as u32,
        })
    } else {
        None
    }
}




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


/// Converts the source buffer `src` from RGBA to 0RGB and writes the result to `dst`.
/// 0RGB model is used by some rendering libraries, such as minifb.
/// # Arguments
///
/// * `dst` - A mutable slice where the converted 0RGB pixels will be stored.
/// * `src` - A slice of RGBA pixels to convert, where each pixel is a `u32`.
/// * `stats` - If `true`, calculate and return some statistics (makes the conversion just a little slower).
///
/// # Panics
///
/// Panics if `dst` and `src` have different lengths.
/// # Returns
/// An `AdapterStatistics` struct containing some basics statistics on the conversion.
pub fn rgba_to_0rgb(dst: &mut [u32], src: &[u32], stats: bool) -> Option<AdapterStatistics> {
    assert_eq!(
        dst.len(),
        src.len(),
        "Source and destination buffers must have the same length!"
    );

    // Faster version without the statistics
    if !stats {
        for (dst_pixel, &src_pixel) in dst.iter_mut().zip(src.iter()) {
            // Extract individual color channels from RGBA
            let r = (src_pixel >> 24) & 0xFF;
            let g = (src_pixel >> 16) & 0xFF;
            let b = (src_pixel >> 8) & 0xFF;

            // Reassemble in ABGR format and store in dst
            *dst_pixel = (0 << 24) | (r << 16) | (g << 8) | b;
        }
        return None;
    }

    // Slower version with the statistics
    let mut total_r = 0u64;
    let mut total_g = 0u64;
    let mut total_b = 0u64;
    let num_pixels = (src.len() / 4) as u64;

    for (dst_pixel, &src_pixel) in dst.iter_mut().zip(src.iter()) {
        // Extract individual color channels from RGBA
        let r = (src_pixel >> 24) & 0xFF;
        let g = (src_pixel >> 16) & 0xFF;
        let b = (src_pixel >> 8) & 0xFF;

        // Reassemble in ABGR format and store in dst
        *dst_pixel = (0 << 24) | (r << 16) | (g << 8) | b;

        // Accumulate color values for statistics
        total_r += r as u64;
        total_g += g as u64;
        total_b += b as u64;
    }

    // Calculate average color values
    let avg_r = (total_r / num_pixels) as u32;
    let avg_g = (total_g / num_pixels) as u32;
    let avg_b = (total_b / num_pixels) as u32;

    // Average ABGR color - using average alpha channel as 255 (opaque)
    let average_color = (avg_r << 24) | (avg_g << 16) | (avg_b << 8) | 0xff;

    Some(AdapterStatistics {
        average_color,
        average_red: avg_r & 0xff,
        average_green: avg_g & 0xff,
        average_blue: avg_b & 0xff,
        num_pixels: num_pixels as u32,
    })
}


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



/// Converts a given RGBA color to ABGR format.
pub fn rgba_color_to_abgr(rgba_color: u32) -> u32 {
    // Extract individual color channels from RGBA
    let r = (rgba_color >> 24) & 0xFF;
    let g = (rgba_color >> 16) & 0xFF;
    let b = (rgba_color >> 8) & 0xFF;
    let a = rgba_color & 0xFF;

    // Reassemble the color in ABGR format
    (a << 24) | (b << 16) | (g << 8) | r
}

/// Converts a given RGBA color to 0RGB format.
pub fn rgba_color_to_0rgb(rgba_color: u32) -> u32 {
    let r = (rgba_color >> 24) & 0xFF;
    let g = (rgba_color >> 16) & 0xFF;
    let b = (rgba_color >> 8) & 0xFF;

    // Reassemble in 0RGB format
    (0 << 24) | (r << 16) | (g << 8) | b
}
