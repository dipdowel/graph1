use crate::primitives::plane::Dimensions2d;

/// Draws horizontal lines on a buffer. **Single-threaded!**
/// TODO: write the docs!
pub fn horizontal_lines_y_grouped(
    buf: &mut [u32],
    buf_dimensions: &Dimensions2d,
    scanline_data: &Vec<u32>,
    scanline_pointers: &Vec<usize>,
) {

// TODO: Redirect to the GPU version if available
    // if ctx.gpu_context.is_enabled() {
    //     gpu::horizontal_lines_x3::horizontal_lines_x3_gpu(
    //         &mut ctx.frame_buf,
    //         &ctx.win.dimensions,
    //         &lines,
    //         &mut ctx.gpu_context
    //     ).expect("Failed to use the GPU for horizontal lines x4");
    //     return;
    // }
    
    
    
    let max_x = buf_dimensions.w - 1;

    for window in scanline_pointers.windows(2) {
        if window.get(0).is_none() || window.get(1).is_none() {
            continue;
        }

        let scanline_start = window[0];
        let scanline_end = window[1];

        let scanline = &scanline_data[scanline_start..scanline_end];

        let y = scanline[0];
        // println!("y: {y}");

        // Ensure the `y` coordinate is within bounds and there are enough elements in the line
        if y >= buf_dimensions.h || scanline.len() < 4 {
            println!("Skipping scanline at y: {y}, buffer height: {}", buf_dimensions.h);
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
