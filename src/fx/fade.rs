#[cfg(feature = "gpu")]
use crate::buffer_op::gpu::fade::fade_gpu;
use crate::core::context::GraphContext;
use crate::utils::color::math::{rgba_operation, ColorOperation};
use std::thread;
/// Fades the entire frame buffer by applying a color operation to each pixel.
/// Multithreaded, if possible (per `ctx.num_threads`)
///
/// # Parameters
/// - `ctx`: A mutable reference to the drawing context.
/// - `color_operand`: The color to be used in the fade operation (RGBA as u32)
/// - `op`: The color operation to apply (Add or Subtract).
/// - `use_alpha`: If true, the alpha channel is considered in the operation; otherwise, it is ignored.
pub fn fade<UserData>(
    ctx: &mut GraphContext<UserData>,
    color_operand: u32,
    op: ColorOperation,
    use_alpha: bool,
) {
    #[cfg(feature = "gpu")]
    if ctx.gpu_context.is_enabled() {
        fade_gpu(
            &mut ctx.frame_buf,
            ctx.win.w,
            ctx.win.h,
            color_operand,
            op,
            use_alpha,
            &mut ctx.gpu_context,
        )
        .expect("GPU fade failed");
        return;
    }

    // Nothing to do...
    if ctx.num_threads == 0 {
        return;
    }

    // If threading is disabled, do single-threaded fade
    if ctx.num_threads == 1 {
        for px in ctx.frame_buf.iter_mut() {
            *px = rgba_operation(*px, color_operand, op, use_alpha);
        }
        return;
    }

    let line_length = ctx.win.w_usize; // Framebuffer width in pixels
    let buf_len = ctx.frame_buf.len();
    let chunk_size = {
        // Divide the framebuffer into roughly equal scanline-aligned chunks
        let raw_chunk_size = usize::div_ceil(buf_len, ctx.num_threads);
        // Align to scanlines (avoid splitting scanlines between threads)
        (raw_chunk_size / line_length) * line_length
    };

    // If chunk is too small to bother with threads, fall back
    if chunk_size < line_length {
        for px in ctx.frame_buf.iter_mut() {
            *px = rgba_operation(*px, color_operand, op, use_alpha);
        }
        return;
    }

    // Split framebuffer into mutable chunks
    let mut chunks: Vec<&mut [u32]> = ctx.frame_buf.chunks_mut(chunk_size).collect();

    // Use thread::scope to spawn threads that each fade their chunk
    thread::scope(|s| {
        for chunk in chunks.iter_mut() {
            s.spawn(move || {
                for px in chunk.iter_mut() {
                    *px = rgba_operation(*px, color_operand, op, use_alpha);
                }
            });
        }
    });
    // All threads joined here.
}
