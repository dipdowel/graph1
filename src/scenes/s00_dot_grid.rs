use crate::graph1::graph1_core::context::GraphContext;

pub fn render(ctx: &mut GraphContext) {
    // Draw kinda dotted grid
    let grid_factor: usize = 20;
    for i in 0..ctx.win.w_usize {
        if i % grid_factor == 0 {
            ctx.buf_view[i] = 0x00_55_55_aa;
        }
    }
    for i in 1..ctx.win.h_usize / grid_factor {
        ctx.buf_view
            .copy_within(0..ctx.win.w_usize, i * grid_factor * ctx.win.w_usize);
    }
}
