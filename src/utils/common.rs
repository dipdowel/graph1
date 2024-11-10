use crate::core::context::GraphContext;

/// Fills the frame buffer with the background color of the window (`ctx.win.background_color`)
pub fn clear_screen<UserDataType>(ctx: &mut GraphContext<UserDataType>) {
    crate::draw::tools::fill::buffer(&mut ctx.frame_buf, ctx.win.background_color);
}
