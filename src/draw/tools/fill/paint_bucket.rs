use crate::buffer_op;
use crate::core::context::GraphContext;
use crate::primitives::Pixel;


/// Fills a shape with a color using the paint bucket tool.
/// /// # Parameters
/// /// - `ctx`: The graph context
/// /// - `start_pixel`: The starting pixel (with fill color) where the fill operation begins.
pub fn paint_bucket<UserData>(ctx: &mut GraphContext<UserData>, start_pixel: &Pixel) {
    buffer_op::scanline_wavefront(&mut ctx.frame_buf, &ctx.win.dimensions, start_pixel);
    
    // This is a slower alternative to `scanline_wavefront()`
    // Use it only if `scanline_wavefront()` fails for some reason.
    // buffer_op::fill::flood::flood(&mut ctx.frame_buf, &ctx.win.dimensions, start_pixel);
    
    // There is no GPU support planned for the paint bucket tool! 
    // Pain bucket is a sequential operation, so it's a bad fit for GPU.
}
