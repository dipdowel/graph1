use crate::{buffer_op};
use crate::core::context::GraphContext;

#[cfg(feature = "gpu")]
use crate::core::context::gpu::GpuContext;
/// * A low-level buffer fill tool.
/// * Unsafely fills a buffer with a given color
/// * Multithreaded.
/// * @See `Multithreaded operations` in `README.md` for details on how `num_threads` is interpreted.
///
/// # Arguments
/// * `buffer` - A mutable buffer to fill
/// * `color` - The color to fill the buffer with
/// * `num_threads` - The number of threads to spawn.
pub fn frame_buffer<UserData>(ctx: &mut GraphContext<UserData>, color: Option<u32>) {
    // let start = Instant::now();

    let color = color.unwrap_or(ctx.win.background_color);

    // ==[ GPU / OpenCL ]=======================================================================
    #[cfg(feature = "gpu")]
    {
        let mut gpu_context: Option<&mut GpuContext> = None;
        if ctx.gpu_context.enabled {
            gpu_context = Some(&mut ctx.gpu_context);
            // Use GPU/OpenCL to fill the buffer
            buffer_op::fill(&mut ctx.frame_buf, color, ctx.num_threads, gpu_context);
        }
    }

    // ==[ CPU only ]=======================================================================
    #[cfg(not(feature = "gpu"))]
    buffer_op::fill(&mut ctx.frame_buf, color, ctx.num_threads);
}
