use crate::buffer_op;
use crate::core::context::GraphContext;
use crate::primitives::Pixel;


/// Fills a shape with a color using the paint bucket tool.
/// /// # Parameters
/// /// - `ctx`: The graph context
/// /// - `start_pixel`: The starting pixel (with fill color) where the fill operation begins.
pub fn paint_bucket<UserData>(ctx: &mut GraphContext<UserData>, start_pixel: &Pixel) {
    // let start = Instant::now();

    // TODO: See if we can use OpenCL GPU to scanline/wavefront fill a shape.
    // ==[ GPU OpenCL ]=======================================================================
    // let mut gpu_context: Option<&mut GpuContext> = None;
    // if ctx.gpu_context.enabled {
    //     gpu_context = Some(&mut ctx.gpu_context);
    // }

    buffer_op::scanline_wavefront(&mut ctx.frame_buf, &ctx.win.dimensions, start_pixel);
}
