use crate::buffer_op;
use crate::primitives::plane::Dimensions2d;
use std::collections::BTreeMap;
use std::thread;
use crate::core::context::gpu::GpuContext;

/// Draws horizontal lines on a buffer. **Multi-threaded!**
/// Ownership of scanline_data is passed in for efficient moves (not copies).
/// 
/// # Parameters
/// - `buf`: A mutable slice of `u32` representing the pixel buffer.
/// - `buf_dimensions`: Dimensions of the buffer.
/// - `scanline_data`: A vector of `u32` where the 0th element is the `y` coordinate,
///   the 1st element is the color in RGBA, and the rest are pairs of `x_start` and `x_end` values
///   of line segments on that scanline.
/// - `scanline_pointers`: A vector  pointers into `scanline_data` that mark the start and end of each scanline.
/// - `scanline_dict`: A speed look-up table:
///       - `[y][0]` - how many scanlines are there for `y`
///       - `[y][1]` - the first pointer into `scanline_data` for that `y`
///       - `[y][n]` - the nth pointer into `scanline_data` for that `y`
pub fn horizontal_lines_y_grouped_threaded(
    buf: &mut [u32],
    buf_dimensions: &Dimensions2d,
    scanline_data: Vec<u32>,   // TAKE OWNERSHIP
    scanline_pointers: Vec<usize>, // TAKE OWNERSHIP
    scanline_dict: Vec<Vec<usize>>, // TAKE OWNERSHIP
    num_threads: usize,
    gpu_context: &mut GpuContext,
) {

    if scanline_data.is_empty() || scanline_pointers.len() < 2 {
        return;
    }

    // ==[ GPU OpenCL ]=======================================================================

    if gpu_context.is_enabled() {
        let len = buf.len();
        buffer_op::gpu::horizontal_lines_y_grouped::horizontal_lines_y_grouped(
            buf,
            buf_dimensions,
            &scanline_dict,
            &scanline_data,
            gpu_context,
        )
            .expect("Failed to draw horizontal lines using GPU OpenCL");
    }

    // ==[ CPU ]=======================================================================

    if  num_threads == 0 {
        return;
    }


    // If only one thread is available, just use the single-threaded version
    if num_threads == 1 {

        buffer_op::lines::horizontal_batch::horizontal_lines_y_grouped(
            buf,
            buf_dimensions,
            &scanline_data,
            &scanline_pointers,
            &scanline_dict,
            gpu_context,
        );
        return;
    }


    let width = buf_dimensions.w;
    let height = buf_dimensions.h;
    let buf_ptr = buf.as_mut_ptr();

    // 1. Group scanlines by y, preserving order, moving data (NO COPY)
    // BTreeMap for sorted y order
    let mut y_groups: BTreeMap<u32, Vec<Vec<u32>>> = BTreeMap::new();
    for w in scanline_pointers.windows(2) {
        let scanline_start = w[0];
        let scanline_end = w[1];
        if scanline_end <= scanline_start { continue; }

        // Move out the slice (no copy)
        let scanline: Vec<u32> = scanline_data[scanline_start..scanline_end].to_vec();
        if scanline.len() < 4 { continue; }
        let y = scanline[0];
        if y >= height { continue; }
        y_groups.entry(y).or_default().push(scanline);
    }

    // 2. Distribute y-groups to threads in balanced bands
    let mut bands: Vec<Vec<(u32, Vec<Vec<u32>>)>> = vec![Vec::new(); num_threads];
    let mut band_load: Vec<usize> = vec![0; num_threads];
    for (y, group) in y_groups {
        // Greedily assign this y-group to the least loaded thread
        let mut min_band = 0;
        for i in 1..num_threads {
            if band_load[i] < band_load[min_band] {
                min_band = i;
            }
        }
        band_load[min_band] += group.len();
        bands[min_band].push((y, group));
    }

    // 3. Each thread: flatten its scanlines into a Vec<u32> + Vec<usize> pointers
    thread::scope(|scope| {
        for band in bands.into_iter() {
            if band.is_empty() { continue; }
            // Calculate band min/max y for efficient buffer slicing
            let min_y = band.first().unwrap().0;
            let max_y = band.last().unwrap().0;
            let row_offset = (min_y * width) as usize;
            let n_rows = (max_y - min_y + 1) as usize;
            let band_buf = unsafe {
                std::slice::from_raw_parts_mut(buf_ptr.add(row_offset), n_rows * width as usize)
            };

            // Flatten all scanlines for this band into contiguous vector
            let mut band_scanline_data = Vec::new();
            let mut band_pointers = Vec::new();
            for (_y, scanlines) in &band {
                for scanline in scanlines {
                    band_pointers.push(band_scanline_data.len());
                    band_scanline_data.extend_from_slice(scanline);
                }
            }
            band_pointers.push(band_scanline_data.len());

            let band_width = width;
            let band_min_y = min_y;
            let band_buf_dimensions = *buf_dimensions;
            scope.spawn(move || {
                for window in band_pointers.windows(2) {
                    let scanline_start = window[0];
                    let scanline_end = window[1];
                    let scanline = &band_scanline_data[scanline_start..scanline_end];
                    let y = scanline[0];
                    let color = scanline[1];
                    if y >= band_buf_dimensions.h || scanline.len() < 4 { continue; }
                    let row_in_band = (y - band_min_y) as usize;
                    let row_buf = &mut band_buf[row_in_band * band_width as usize .. (row_in_band + 1) * band_width as usize];
                    for x_pair in scanline[2..].chunks(2) {
                        if x_pair.len() != 2 { continue; }
                        let x_start = x_pair[0].clamp(0, band_width - 1);
                        let x_end = x_pair[1].clamp(0, band_width - 1);
                        if x_start > x_end { continue; }
                        for x in x_start..=x_end {
                            row_buf[x as usize] = color;
                        }
                    }
                }
            });
        }
    });
}



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
pub fn scanlines_threaded(
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

        buffer_op::gpu::scanlines::scanlines_gpu(
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
        crate::buffer_op::lines::horizontal_batch::scanlines(
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
            if band.is_empty() { continue; }
            // Get the Y-range for this band
            let min_y = *band.first().unwrap() as usize;
            let max_y = *band.last().unwrap() as usize;
            let n_rows = max_y - min_y + 1;

            // Safety: Each thread gets a unique, non-overlapping region of the buffer for its rows
            let band_buf = unsafe {
                std::slice::from_raw_parts_mut(
                    buf_ptr.add(min_y * width),
                    n_rows * width,
                )
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
                    if y_usize < min_y || y_usize > max_y || y_usize >= band_buf_dimensions.h as usize {
                        continue;
                    }

                    let start = band_flat_data_ptrs[y_usize] as usize;
                    let size = band_scanline_sizes[y_usize] as usize;
                    if size == 0 { continue; }
                    let end = start + size;
                    if end > band_flat_scanline_data.len() { continue; }
                    let scanline: &[u32] = &band_flat_scanline_data[start..end];

                    let row_in_band = y_usize - min_y;
                    let row_buf = &mut band_buf[row_in_band * band_width .. (row_in_band + 1) * band_width];


                    // Each scanline is made of triplets: [x_start, x_end, color]
                    for segment in scanline.chunks_exact(3) {
                        let x_start = segment[0].clamp(0, (band_width - 1) as u32) as usize;
                        let x_end   = segment[1].clamp(0, (band_width - 1) as u32) as usize;
                        let color   = segment[2];
                        if x_start > x_end { continue; }
                        for x in x_start..=x_end {
                            row_buf[x] = color;
                        }
                    }
                }
            });
        }
    });
}
