use crate::utils::color::adapters::AdapterStatistics;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, Copy, Clone)]
struct ColorTotals {
    red: u64,
    green: u64,
    blue: u64,
}

fn buffer_rgba_to_0rgb_thread(
    dst: &mut [u32],
    src: &[u32],
    color_totals_ref: Option<Arc<Mutex<ColorTotals>>>,
) {
    assert_eq!(
        dst.len(),
        src.len(),
        "Source and destination buffers must have the same length!"
    );

    // ===[ FASTER CONVERSION, NO STATISTICS ]======================================================
    if color_totals_ref.is_none() {
        for (dst_pixel, &src_pixel) in dst.iter_mut().zip(src.iter()) {
            // Extract individual color channels from RGBA
            let r = (src_pixel >> 24) & 0xFF;
            let g = (src_pixel >> 16) & 0xFF;
            let b = (src_pixel >> 8) & 0xFF;

            // Reassemble in ABGR format and store in dst
            *dst_pixel = (0 << 24) | (r << 16) | (g << 8) | b;
        }
        // println!(
        //     "thread id: {:?}, buf size: {:?} - No stats",
        //     thread::current().id(),
        //     dst.len()
        // );
        return;
    }

    // ===[ SLOWER CONVERSION, WITH THE STATISTICS ]================================================
    let mut total_r = 0u64;
    let mut total_g = 0u64;
    let mut total_b = 0u64;

    for (dst_pixel, &src_pixel) in dst.iter_mut().zip(src.iter()) {
        // Extract individual color channels from RGBA
        let r = (src_pixel >> 24) & 0xFF;
        let g = (src_pixel >> 16) & 0xFF;
        let b = (src_pixel >> 8) & 0xFF;

        // Reassemble in 0RGB format and write to dst
        *dst_pixel = (0 << 24) | (r << 16) | (g << 8) | b;

        // Accumulate color values for statistics
        total_r += r as u64;
        total_g += g as u64;
        total_b += b as u64;
    }

    // Lock the mutex and write the color totals of this thread to the shared results
    let mut color_totals_ref = color_totals_ref.unwrap();
    let mut color_totals_ref = color_totals_ref.lock().unwrap();
    color_totals_ref.red += total_r;
    color_totals_ref.green += total_g;
    color_totals_ref.blue += total_b;

    // Mutex is expected to unlock automatically when `color_totals_ref` goes out of scope.
}

/// Converts `ColorTotals` into `AdapterStatistics`.
fn prepare_stats(color_totals: ColorTotals, num_pixels: u64) -> AdapterStatistics {
    // Calculate average color values
    let avg_r = (color_totals.red / num_pixels) as u32;
    let avg_g = (color_totals.green / num_pixels) as u32;
    let avg_b = (color_totals.blue / num_pixels) as u32;

    // Average ABGR color - using average alpha channel as 255 (opaque)
    let average_color = (avg_r << 24) | (avg_g << 16) | (avg_b << 8) | 0xff;

    AdapterStatistics {
        average_color,
        average_red: avg_r & 0xff,
        average_green: avg_g & 0xff,
        average_blue: avg_b & 0xff,
        num_pixels: num_pixels as u32,
    }
}
// TODO: WRITE TESTS !!!
// TODO: WRITE TESTS !!!
// TODO: WRITE TESTS !!!
// TODO: WRITE TESTS !!!
// TODO: WRITE TESTS !!!
// TODO: WRITE TESTS !!!
// TODO: WRITE TESTS !!!
// TODO: WRITE TESTS !!!
// TODO: WRITE TESTS !!!
// TODO: WRITE TESTS !!!
// TODO: WRITE TESTS !!!
// TODO: WRITE TESTS !!!
// TODO: WRITE TESTS !!!


/// Converts the source buffer `src` from RGBA to 0RGB and writes the result to `dst`.
/// 0RGB model is used by some rendering libraries, such as `minifb`.
/// # Arguments
///
/// * `dst` - A mutable slice where the converted 0RGB pixels will be stored.
/// * `src` - A slice of RGBA pixels to convert, where each pixel is a `u32`.
/// * `num_threads` - How many threads to use. @See `Multithreaded operations` in `README.md`.
/// * `stats` - If `true`, calculate and return some statistics (makes the conversion just a little slower).
///
/// # Panics
///
/// Panics if `dst` and `src` have different lengths.
/// # Returns
/// An `AdapterStatistics` struct containing some basics statistics on the conversion.
pub fn rgba_to_0rgb(
    dst: &mut [u32],
    src: &[u32],
    num_threads: usize,
    stats: bool,
) -> Option<AdapterStatistics> {
    assert_eq!(
        dst.len(),
        src.len(),
        "Source and destination buffers must have the same length!"
    );

    let num_pixels = src.len() as u64;

    // TODO:    Handle the two cases below!
    // TODO: ==================================
    // if num_threads < 1 {
    //   // TODO: implement
    // }
    // if num_threads == 1 {
    // // TODO: implement
    // }

    // Thread-safe totals of color channels, per thread
    let color_totals: Arc<Mutex<ColorTotals>> = Arc::new(Mutex::new(ColorTotals {
        red: 0,
        green: 0,
        blue: 0,
    }));

    // Do nothing
    //==============================================================================================
    if num_threads < 1 {
        if stats {
            let color_totals = *(color_totals.lock().unwrap());
            return Some(prepare_stats(color_totals, num_pixels));
        }
        return None;
    }

    // Execute in the main thread
    //==============================================================================================
    if num_threads == 1 {
        // No statistics requested
        if !stats {
            buffer_rgba_to_0rgb_thread(dst, src, None);
            return None;
        }
        // Prepare statistics
        buffer_rgba_to_0rgb_thread(dst, src, Some(color_totals.clone()));
        let color_totals = *(color_totals.lock().unwrap());
        return Some(prepare_stats(color_totals, num_pixels));
    }

    // Spawn multiple threads
    //==============================================================================================

    // Size of the chunk for each thread to process
    let chunk_size = usize::div_ceil(src.len(), num_threads);

    // Split the `src` and `dst` buffers into chunks to process in parallel threads
    let mut dst_chunks: Vec<&mut [u32]> = dst.chunks_mut(chunk_size).collect();
    let src_chunks: Vec<&[u32]> = src.chunks(chunk_size).collect();

    thread::scope(|scope| {
        for (dst_chunk, src_chunk) in dst_chunks.iter_mut().zip(src_chunks.iter()) {
            if stats {
                let color_totals_ref = color_totals.clone();
                scope.spawn(move || {
                    buffer_rgba_to_0rgb_thread(dst_chunk, src_chunk, Some(color_totals_ref))
                });
            } else {
                scope.spawn(move || buffer_rgba_to_0rgb_thread(dst_chunk, src_chunk, None));
            }
        }
    }); // The scope for the scoped threads ends here.


    if !stats {
        return None;
    }

    let color_totals = *(color_totals.lock().unwrap());
    Some(prepare_stats(color_totals, num_pixels))
}

