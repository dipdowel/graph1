use std::thread;
use crate::utils::color::adapters::AdapterStatistics;

// fn buffer_rgba_to_0rgb_thread(dst: &mut [u32], src: &[u32],  stats: bool) -> Option<AdapterStatistics> {
//     assert_eq!(
//         dst.len(),
//         src.len(),
//         "Source and destination buffers must have the same length!"
//     );
//
//
//     // ===[ FASTER CONVERSION, NO STATISTICS ]======================================================
//     if !stats {
//         for (dst_pixel, &src_pixel) in dst.iter_mut().zip(src.iter()) {
//             // Extract individual color channels from RGBA
//             let r = (src_pixel >> 24) & 0xFF;
//             let g = (src_pixel >> 16) & 0xFF;
//             let b = (src_pixel >> 8) & 0xFF;
//
//             // Reassemble in ABGR format and store in dst
//             *dst_pixel = (0 << 24) | (r << 16) | (g << 8) | b;
//         }
//         return None;
//     }
//
//     // ===[ SLOWER CONVERSION, WITH THE STATISTICS ]================================================
//     let mut total_r = 0u64;
//     let mut total_g = 0u64;
//     let mut total_b = 0u64;
//     let num_pixels = (src.len() / 4) as u64;
//
//     for (dst_pixel, &src_pixel) in dst.iter_mut().zip(src.iter()) {
//         // Extract individual color channels from RGBA
//         let r = (src_pixel >> 24) & 0xFF;
//         let g = (src_pixel >> 16) & 0xFF;
//         let b = (src_pixel >> 8) & 0xFF;
//
//         // Reassemble in ABGR format and store in dst
//         *dst_pixel = (0 << 24) | (r << 16) | (g << 8) | b;
//
//         // Accumulate color values for statistics
//         total_r += r as u64;
//         total_g += g as u64;
//         total_b += b as u64;
//     }
//
//     // Calculate average color values
//     let avg_r = (total_r / num_pixels) as u32;
//     let avg_g = (total_g / num_pixels) as u32;
//     let avg_b = (total_b / num_pixels) as u32;
//
//     // Average ABGR color - using average alpha channel as 255 (opaque)
//     let average_color = (avg_r << 24) | (avg_g << 16) | (avg_b << 8) | 0xff;
//
//     Some(AdapterStatistics {
//         average_color,
//         average_red: avg_r & 0xff,
//         average_green: avg_g & 0xff,
//         average_blue: avg_b & 0xff,
//         num_pixels: num_pixels as u32,
//     })
// }


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
pub fn rgba_to_0rgb(dst: &mut [u32], src: &[u32], num_threads:usize, stats: bool) -> Option<AdapterStatistics> {
    assert_eq!(
        dst.len(),
        src.len(),
        "Source and destination buffers must have the same length!"
    );

/*
    // Size of the chunk for each thread to process
    let chunk_size = usize::div_ceil(src.len(), num_threads);

    // Split the `src` and `dst` buffers into chunks to process in parallel threads
    let mut dst_chunks: Vec<&mut [u32]> = dst.chunks_mut(chunk_size).collect();
    let src_chunks: Vec<&[u32]> = src.chunks(chunk_size).collect();

    thread::scope(|s| {
        // Iterate over the chunks and process each in its own thread
        for (index, (dst_chunk, src_chunk)) in
            dst_chunks.iter_mut().zip(src_chunks.iter()).enumerate()
        {
            s.spawn(move || {
                for (src_elem_index, src_elem) in src_chunk.iter().enumerate() {
                    dst_chunk[src_elem_index] = buffer_rgba_to_0rgb_thread(&mut dst_chunk, &mut src_chunk, stats);
                    // thread::sleep(Duration::from_millis(10));
                    // println!( "[{:?}], in: {}, out: {},", thread::current().id(), src_chunk[src_elem_index], dst_chunk[src_elem_index],);
                }
            });
        }
    }); // The scope for the scoped threads ends here.
    return None;
*/

    // ===[ FASTER CONVERSION, NO STATISTICS ]======================================================
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

    // ===[ SLOWER CONVERSION, WITH THE STATISTICS ]================================================
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