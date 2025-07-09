use crate::buffer_op;
use crate::core::context::gpu::GpuContext;
use crate::primitives::plane::Dimensions2d;

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
pub fn horizontal_lines(
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

        buffer_op::gpu::horizontal_lines::horizontal_lines(
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
