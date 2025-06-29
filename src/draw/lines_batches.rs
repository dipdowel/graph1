use crate::buffer_op::gpu;
use crate::core::context::GraphContext;

use crate::buffer_op::lines::horizontal_batch_threaded;
use crate::buffer_op::lines::horizontal_batch;


// Fast, integer-only line drawing without thickness or anti-aliasing.

/*

/// Draws horizontal lines on `ctx.frame_buf`. **Single-threaded!**
/// Each line is represented by  four consecutive elements: `x_start`, `x_end`, `y`, and `color`.
/// These 4 elements in exactly that order are expected in the `lines` vector.
/// # Parameters
/// - `ctx` : A mutable reference to the Graph1 context.
/// - `lines`: A vector containing the line information (as described above)
pub fn horizontal_lines_x4<UserData>(
    ctx: &mut GraphContext<UserData>,
    lines: &Vec<u32>
) {
    if ctx.gpu_context.is_enabled() {
        gpu::horizontal_lines_x4::horizontal_lines_x4_gpu(
            &mut ctx.frame_buf,
            &ctx.win.dimensions,
            &lines,
            &mut ctx.gpu_context
        ).expect("Failed to use the GPU for horizontal lines x4");
        return;
    }
    horizontal_batch::horizontal_lines_x4(&mut ctx.frame_buf, &ctx.win.dimensions, lines)
}


/// Draws horizontal lines on `ctx.frame_buf`. **Multi-threaded!**
/// Each line is represented by  four consecutive elements: `x_start`, `x_end`, `y`, and `color`.
/// These 4 elements in exactly that order are expected in the `lines` vector.
/// # Parameters
/// - `ctx` : A mutable reference to the Graph1 context.
/// - `lines`: A vector containing the line information (as described above)
pub fn horizontal_lines_x4_threaded<UserData>(
    ctx: &mut GraphContext<UserData>,
    lines: &Vec<u32>
) {
    if ctx.gpu_context.is_enabled() {
        gpu::horizontal_lines_x4::horizontal_lines_x4_gpu(
            &mut ctx.frame_buf,
            &ctx.win.dimensions,
            &lines,
            &mut ctx.gpu_context
        ).expect("Failed to use the GPU for horizontal lines x4");
        return;
    }

    horizontal_batch_threaded::horizontal_lines_x4_threaded(&mut ctx.frame_buf, &ctx.win.dimensions, lines, ctx.num_threads)
}


/// Draws horizontal lines on a buffer. **Single-threaded!**
/// Each line is represented by a vector of `u32` where the first element is the color,
/// followed by triplets of `x_start`, `x_end`, and `y` coordinates.
///
/// /// # Parameters
/// - `ctx` : A mutable reference to the Graph1 context.
/// /// - `buf_dimensions`: Dimensions of the buffer (width, height)
/// /// - `lines`: A vector of vectors, where each inner vector contains a color in RGBA, followed by triplets of `x_start`, `x_end`, and `y`
pub fn horizontal_lines_x3<UserData>(
    ctx: &mut GraphContext<UserData>,
    lines: &Vec<Vec<u32>>
) {

    if ctx.gpu_context.is_enabled() {
        gpu::horizontal_lines_x3::horizontal_lines_x3_gpu(
            &mut ctx.frame_buf,
            &ctx.win.dimensions,
            &lines,
            &mut ctx.gpu_context
        ).expect("Failed to use the GPU for horizontal lines x4");
        return;
    }

    horizontal_batch::horizontal_lines_x3(&mut ctx.frame_buf, &ctx.win.dimensions, lines)
}

/// Draws horizontal lines on a buffer. **Multi-threaded!**
/// Each line is represented by a vector of `u32` where the first element is the color,
/// followed by triplets of `x_start`, `x_end`, and `y` coordinates.
///
/// /// # Parameters
/// - `ctx` : A mutable reference to the Graph1 context.
/// /// - `buf_dimensions`: Dimensions of the buffer (width, height)
/// /// - `lines`: A vector of vectors, where each inner vector contains a color in RGBA, followed by triplets of `x_start`, `x_end`, and `y`
pub fn horizontal_lines_x3_threaded<UserData>(
    ctx: &mut GraphContext<UserData>,
    lines: &Vec<Vec<u32>>
) {

    if ctx.gpu_context.is_enabled() {
        gpu::horizontal_lines_x3::horizontal_lines_x3_gpu(
            &mut ctx.frame_buf,
            &ctx.win.dimensions,
            &lines,
            &mut ctx.gpu_context
        ).expect("Failed to use the GPU for horizontal lines x4");
        return;
    }

    horizontal_batch_threaded::horizontal_lines_x3_threaded(&mut ctx.frame_buf, &ctx.win.dimensions, lines, ctx.num_threads)


}
*/


/// Draws horizontal lines on a buffer.
/// Each scanline is represented by a vector of `u32` where the 0th element is the `y` coordinate,
/// the 1st element is the color in RGBA, and the rest are pairs of `x_start` and  `x_end` values 
/// of line segments on that scanline.
///
/// # Parameters
/// - `ctx` : A mutable reference to the Graph1 context.
/// - `scanlines`: A vector where each element describes line segments on one `y` scanline as follows
///   - `[0]` - the `y` coordinate
///   - `[1]` - the color in RGBA,
///   - `[2]` - `x_start` of the 1st line segment
///   - `[3]` - `x_end` of the 1st line segment
///   - `[4]` - `x_start` of the 2nd line segment
///   - `[5]` - `x_end` of the 2nd line segment
///   - Etc.
pub fn horizontal_lines_y_grouped<UserData>(
    ctx: &mut GraphContext<UserData>,
    scanlines: &Vec<Vec<u32>>
) {
    
    // Convert `scanlines` into a flat Vec<u32> (`scanline_data`)
    // and a Vec<usize> (`scanline_pointers`) with the start index of each scanline in `scanline_data`.
    let mut scanline_data = Vec::new();
    let mut scanline_pointers = Vec::with_capacity(scanlines.len() + 1);

    for scanline in scanlines {
        scanline_pointers.push(scanline_data.len());
        scanline_data.extend(scanline.iter());
    }
    // Final pointer marks the end of the last scanline
    scanline_pointers.push(scanline_data.len());


    horizontal_batch::horizontal_lines_y_grouped(&mut ctx.frame_buf, &ctx.win.dimensions, &scanline_data, &scanline_pointers);
}





// if ctx.gpu_context.is_enabled() {
//     gpu::horizontal_lines_x3::horizontal_lines_x3_gpu(
//         &mut ctx.frame_buf,
//         &ctx.win.dimensions,
//         &lines,
//         &mut ctx.gpu_context
//     ).expect("Failed to use the GPU for horizontal lines x4");
//     return;
// }