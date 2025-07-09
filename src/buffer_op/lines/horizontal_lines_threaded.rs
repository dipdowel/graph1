use crate::buffer_op;
use crate::core::context::gpu::GpuContext;
use crate::primitives::plane::Dimensions2d;

/// Draws scanlines onto a buffer in **parallel** using multiple threads.
/// Each scanline is defined as a sequence of (x_start, x_end, color) triplets for a specific y-coordinate.
/// This function partitions the set of occupied scanlines (y-indices) into bands distributed across threads,
/// such that each thread writes to a unique (non-overlapping) set of buffer rows for maximum performance.
///
/// # Parameters
/// - `buf`: Mutable slice of `u32`, representing the pixel buffer. Length must be at least `buf_dimensions.w * buf_dimensions.h`.
/// - `buf_dimensions`: The width and height of the buffer.
/// - `flat_scanline_data`: Flattened data of all scanlines, in triplets `[x_start, x_end, color, ...]`.
/// - `occupied_scanline_indices`: List of y-coordinates (scanline indices) that contain at least one line segment.
/// - `flat_data_ptrs`: For each scanline (indexed by y), the starting index into `flat_scanline_data`.
/// - `scanline_sizes`: For each scanline (indexed by y), the size (number of elements) in `flat_scanline_data`.
/// - `num_threads`: How many threads to use (0 = no-op, 1 = single-threaded fallback).
/// - `gpu_context`: For GPU path; if enabled, GPU implementation is called instead.
///
/// # Panics
/// Panics if provided slices are inconsistent or buffer is too small.
///
/// # Safety
/// Uses `unsafe` internally to partition `buf` for parallel writes; safe because rows are not shared.
///
/// # Example usage
/// ```ignore
/// scanlines_threaded(buf, &buf_dimensions, flat_scanline_data, occupied_indices, flat_ptrs, scanline_sizes, 4, &mut gpu_context);
/// ```
pub fn horizontal_lines_threaded(
    buf: &mut [u32],
    buf_dimensions: &Dimensions2d,
    flat_scanline_data: Vec<u32>,
    occupied_scanline_indices: Vec<u32>,
    flat_data_ptrs: Vec<u32>,
    scanline_sizes: Vec<u32>,
    num_threads: usize,
    gpu_context: &mut GpuContext,
) {
    // ==[ Early outs / fallback ]==
    if occupied_scanline_indices.is_empty() || buf.is_empty() || num_threads == 0 {
        return;
    }

    // ==[ GPU OpenCL path (if available) ]==
    if gpu_context.is_enabled() {
        println!(">>>>>>>>>>>>>. scanlines_threaded() Scanlines using GPU OpenCL");

        buffer_op::gpu::horizontal_lines::horizontal_lines(
            buf,
            buf_dimensions,
            // &occupied_scanline_indices,
            &flat_scanline_data,
            &flat_data_ptrs,
            &scanline_sizes,
            gpu_context,
        )
        .expect("Failed to draw horizontal lines using GPU OpenCL");
        return;
    }

    // ==[ Single-threaded fallback ]==
    if num_threads == 1 {
        crate::buffer_op::lines::horizontal_lines::horizontal_lines(
            buf,
            buf_dimensions,
            &flat_scanline_data,
            &occupied_scanline_indices,
            &flat_data_ptrs,
            &scanline_sizes,
            gpu_context,
        );
        return;
    }

    let width = buf_dimensions.w as usize;
    let buf_ptr = buf.as_mut_ptr();

    // ==[ Partition occupied_scanline_indices into bands for each thread ]==

    let mut occupied_scanline_indices = occupied_scanline_indices;
    occupied_scanline_indices.sort_unstable();

    let n_bands = num_threads.min(occupied_scanline_indices.len());
    let mut bands = Vec::with_capacity(n_bands);

    let band_size = (occupied_scanline_indices.len() + n_bands - 1) / n_bands;
    for i in 0..n_bands {
        let start = i * band_size;
        let end = ((i + 1) * band_size).min(occupied_scanline_indices.len());
        if start < end {
            bands.push(&occupied_scanline_indices[start..end]);
        }
    }

    // ==[ Spawn threads; each thread writes only to its band's rows ]==
    std::thread::scope(|scope| {
        for band in bands {
            if band.is_empty() {
                continue;
            }
            // Get the Y-range for this band
            let min_y = *band.first().unwrap() as usize;
            let max_y = *band.last().unwrap() as usize;
            let n_rows = max_y - min_y + 1;

            // Safety: Each thread gets a unique, non-overlapping region of the buffer for its rows
            let band_buf = unsafe {
                std::slice::from_raw_parts_mut(buf_ptr.add(min_y * width), n_rows * width)
            };

            // Needed data for drawing (move/copy as needed)
            let band_flat_scanline_data = &flat_scanline_data;
            let band_flat_data_ptrs = &flat_data_ptrs;
            let band_scanline_sizes = &scanline_sizes;
            let band_width = width;
            let band_buf_dimensions = *buf_dimensions;

            scope.spawn(move || {
                // For each scanline (y) in this band
                for &y in band {
                    let y_usize = y as usize;
                    // Skip Y values outside our band's rows or outside the buffer
                    if y_usize < min_y
                        || y_usize > max_y
                        || y_usize >= band_buf_dimensions.h as usize
                    {
                        continue;
                    }

                    let start = band_flat_data_ptrs[y_usize] as usize;
                    let size = band_scanline_sizes[y_usize] as usize;
                    if size == 0 {
                        continue;
                    }
                    let end = start + size;
                    if end > band_flat_scanline_data.len() {
                        continue;
                    }
                    let scanline: &[u32] = &band_flat_scanline_data[start..end];

                    let row_in_band = y_usize - min_y;
                    let row_buf =
                        &mut band_buf[row_in_band * band_width..(row_in_band + 1) * band_width];

                    // Each scanline is made of triplets: [x_start, x_end, color]
                    for segment in scanline.chunks_exact(3) {
                        let x_start = segment[0].clamp(0, (band_width - 1) as u32) as usize;
                        let x_end = segment[1].clamp(0, (band_width - 1) as u32) as usize;
                        let color = segment[2];
                        if x_start > x_end {
                            continue;
                        }
                        for x in x_start..=x_end {
                            row_buf[x] = color;
                        }
                    }
                }
            });
        }
    });
}
