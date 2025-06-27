use crate::buffer_op;
use crate::core::context::GraphContext;

pub fn clear_screen<UserData>(ctx: &mut GraphContext<UserData>) {
    let color = ctx.win.background_color;
    buffer_op::fill(&mut ctx.frame_buf, color, ctx.num_threads, &mut ctx.gpu_context);
}
