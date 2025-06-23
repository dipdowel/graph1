use crate::buffer_op;
use crate::core::context::GraphContext;

#[cfg(feature = "gpu")]
use crate::core::context::gpu::GpuContext;
pub fn clear_screen<UserData>(ctx: &mut GraphContext<UserData>) {
    let color = ctx.win.background_color;

    // ==[ GPU OpenCL ]=======================================================================
    #[cfg(feature = "gpu")]{
    let mut gpu_context: Option<&mut GpuContext> = None;
    if ctx.gpu_context.enabled {
        gpu_context = Some(&mut ctx.gpu_context);
    }
    // Use GPU/OpenCL to fill the buffer
        buffer_op::fill(&mut ctx.frame_buf, color, ctx.num_threads, gpu_context);
        return;
    }

    #[cfg(not(feature = "gpu"))]
    buffer_op::fill(&mut ctx.frame_buf, color, ctx.num_threads);
}
