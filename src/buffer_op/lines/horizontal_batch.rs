use crate::buffer_op;
use crate::core::context::gpu::GpuContext;
use crate::primitives::plane::Dimensions2d;

/// Draws horizontal lines on a buffer. **Single-threaded!**
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
pub fn horizontal_lines_y_grouped(
    buf: &mut [u32],
    buf_dimensions: &Dimensions2d,
    scanline_data: &Vec<u32>,
    scanline_pointers: &Vec<usize>,
    scanline_dict: &Vec<Vec<usize>>,
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
            scanline_dict,
            scanline_data,
            gpu_context,
        )
        .expect("Failed to draw horizontal lines using GPU OpenCL");
    }

    // ==[ CPU ]=======================================================================

    let max_x = buf_dimensions.w - 1;

    for window in scanline_pointers.windows(2) {
        if window.get(0).is_none() || window.get(1).is_none() {
            continue;
        }

        let scanline_start = window[0];
        let scanline_end = window[1];

        let scanline = &scanline_data[scanline_start..scanline_end];

        let y = scanline[0];

        // Ensure the `y` coordinate is within bounds and there are enough elements in the line
        if y >= buf_dimensions.h || scanline.len() < 4 {
            println!(
                "Skipping scanline at y: {y}, buffer height: {}",
                buf_dimensions.h
            );
            return;
        }

        let color = scanline[1];

        for x_start_end in scanline[2..].chunks_exact(2) {
            if let [x_start, x_end] = x_start_end {
                // Clamp `x_start` and `x_end` to buffer bounds
                let x_start = (*x_start).clamp(0, max_x);
                let x_end = (*x_end).clamp(0, max_x);

                // Avoid negative ranges
                if x_start > x_end {
                    continue;
                }

                for x in x_start..=x_end {
                    buf[(y * buf_dimensions.w + x) as usize] = color;
                }
            }
        }
    }
}

/// Draws horizontal lines on a buffer. **Single-threaded!**
///
/// # Parameters
/// - `buf`: A mutable slice of `u32` representing the pixel buffer.
/// - `buf_dimensions`: Dimensions of the buffer.
/// - `flat_scanline_data`: A vector of triplets `x_start, x_end, color`. Each triplet represents a line segment on a scanline.
/// - `occupied_scanline_indices`: Numbers of scanlines (their `y` coordinates) which have at least one line segment.
/// - `flat_data_ptrs`: A vector of pointers into `flat_scanline_data` that mark the start of each scanline.
/// - `scanline_sizes`: A vector of sizes of each scanline in `flat_scanline_data`.
/// - `gpu_context`: A mutable reference to the GPU context.
///
pub fn scanlines(
    buf: &mut [u32],
    buf_dimensions: &Dimensions2d,
    flat_scanline_data: &Vec<u32>,
    occupied_scanline_indices: &Vec<u32>,
    flat_data_ptrs: &Vec<u32>,
    scanline_sizes: &Vec<u32>,
    gpu_context: &mut GpuContext,
) {

    // ==[ GPU OpenCL ]=======================================================================

    if gpu_context.is_enabled() {

        // println!(">>>>>>>>>>>>> scanlines (no threads) Scanlines using GPU OpenCL");

        buffer_op::gpu::scanlines::scanlines_gpu(
            buf,
            buf_dimensions,
            // occupied_scanline_indices,
            flat_scanline_data,
            flat_data_ptrs,
            scanline_sizes,
            gpu_context,
        )
            .expect("Failed to draw horizontal lines using GPU OpenCL");
        return;
    }


    let x_max = buf_dimensions.w;
    let y_max = buf_dimensions.h - 1;

    for i in occupied_scanline_indices {
        let y = *i;

        if y > y_max {
            continue;
        }

        let i = y as usize;
        let start = flat_data_ptrs[i] as usize;
        let end = start + scanline_sizes[i] as usize;
        let scanline: &[u32] = &flat_scanline_data[start..end];

        for line_segment in scanline.chunks_exact(3) {
            let x_start = line_segment[0];
            let x_end = line_segment[1];
            let color = line_segment[2];

            // Clamp `x_start` and `x_end` to buffer bounds
            let x_start = x_start.clamp(0, x_max - 1);
            let x_end = x_end.clamp(0, x_max - 1);

            for x in x_start..=x_end {
                buf[(y * buf_dimensions.w + x) as usize] = color;
            }
        }
    }
}
