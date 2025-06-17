use crate::core::context::GraphContext;
use crate::utils::color::math::{rgba_operation, ColorOperation};

/// Fades the entire frame buffer by applying a color operation to each pixel.
/// # Parameters
/// - `ctx`: A mutable reference to the drawing context.
/// - `color_operand`: The color to be used in the fade operation (RGBA as u32)
/// - `op`: The color operation to apply (Add or Subtract).
/// - `use_alpha`: If true, the alpha channel is considered in the operation; otherwise, it is ignored.
pub fn fade<UserData>(ctx: &mut GraphContext<UserData>, color_operand:u32, op:ColorOperation, use_alpha:bool) {
    for i in 0..ctx.frame_buf.len() {
        ctx.frame_buf[i] = rgba_operation(
            ctx.frame_buf[i],
            color_operand,
            op,
            use_alpha,
        );
    }
}

// pub fn fade_region<UserData>(ctx: &mut GraphContext<UserData>, region: &RectArea, fade_out: bool) {
//
//
//     // Dereference the options
//     let start_x = region.top_left.x;
//     let start_y = region.top_left.y;
//     let width = region.dimensions.w;
//     let height = region.dimensions.h;
//
//     // Nothing to draw here
//     if width == 0 || height == 0 {
//         return;
//     }
//
// }