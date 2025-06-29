use crate::primitives::plane::Dimensions2d;
/*
/// Draws horizontal lines on a buffer. **Single-threaded!**
/// Each line is represented by  four consecutive elements: `x_start`, `x_end`, `y`, and `color`.
/// These 4 elements in exactly that order are expected in the `lines` vector.
///  **NB:** This function is CPU-only!
///  **NB:**  If you need GPU-powered line drawing, either check low-level functions under `src/buffer_op/gpu`,
///  **NB:**  Or use the high-level functions from `draw::lines_batches`
/// # Parameters
/// - `buf`: The buffer of pixels to draw the lines on
/// - `buf_dimensions`: Dimensions of the buffer (width, height)
/// - `lines`: A vector containing the line information (as described above)
pub fn horizontal_lines_x4(buf: &mut [u32], buf_dimensions: &Dimensions2d, lines: &Vec<u32>) {
    // Check whether the line information is incomplete
    if lines.len() % 4 != 0 {
        return;
    }

    let max_y = buf_dimensions.h as u32 - 1;

    for line_info in lines.chunks(4) {
        let mut x_start = line_info[0];
        let mut x_end = line_info[1];
        let y = line_info[2];
        let color = line_info[3];

        // Ensure the `y` coordinate is within bounds
        if y > max_y {
            continue;
        }

        // Clamp `x_start` and `x_end` to buffer bounds
        let max_x = buf_dimensions.w - 1;
        x_start = x_start.clamp(0, max_x);
        x_end = x_end.clamp(0, max_x);

        // Avoid empty or negative ranges
        if x_start > x_end {
            continue;
        }

        for x in x_start..=x_end {
            buf[(y * buf_dimensions.w + x) as usize] = color;
        }
    }
}

/// Draws horizontal lines on a buffer. **Single-threaded!**
/// Each line is represented by a vector of `u32` where the first element is the color,
/// followed by triplets of `x_start`, `x_end`, and `y` coordinates.
///  **NB:** This function is CPU-only!
///  **NB:**  If you need GPU-powered line drawing, either check low-level functions under `src/buffer_op/gpu`,
///  **NB:**  Or use the high-level functions from `draw::lines_batches`
/// # Parameters
/// - `buf`: The buffer of pixels to draw the lines on
/// - `buf_dimensions`: Dimensions of the buffer (width, height)
/// - `lines`: A vector of vectors, where each inner vector contains a color in RGBA, followed by triplets of `x_start`, `x_end`, and `y`
pub fn horizontal_lines_x3(buf: &mut [u32], buf_dimensions: &Dimensions2d, lines: &Vec<Vec<u32>>) {
    let max_y = buf_dimensions.h - 1;

    for color_batch in lines.iter() {
        // The first element is the color, the rest are triplets of `x_start, x_end, y`.
        // So without the color, the length of the batch should be a multiple of 3.
        if (color_batch.len() - 1) % 3 != 0 {
            continue;
        }

        let color = color_batch[0];

        for line_segment in color_batch[1..].chunks(3) {
            let mut x_start = line_segment[0];
            let mut x_end = line_segment[1];
            let y = line_segment[2];

            // Ensure the `y` coordinate is within bounds
            if y > max_y {
                continue;
            }
            let max_x = buf_dimensions.w - 1;
            x_start = x_start.clamp(0, max_x);
            x_end = x_end.clamp(0, max_x);

            if x_start > x_end {
                continue;
            }

            // Draw the horizontal line
            for x in x_start..=x_end {
                buf[(y * buf_dimensions.w + x) as usize] = color;
            }
        }
    }
}
*/
////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Draws horizontal lines on a buffer. **Single-threaded!**
/// TODO: write the docs!
pub fn horizontal_lines_y_grouped(
    buf: &mut [u32],
    buf_dimensions: &Dimensions2d,
    scanline_data: &Vec<u32>,
    scanline_pointers: &Vec<usize>,
) {
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
