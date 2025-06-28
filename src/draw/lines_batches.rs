use crate::buffer_op::gpu;
use crate::core::context::GraphContext;

use crate::buffer_op::lines::horizontal_batch_threaded;
use crate::buffer_op::lines::horizontal_batch;


/// Fast, integer-only line drawing without thickness or anti-aliasing.



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