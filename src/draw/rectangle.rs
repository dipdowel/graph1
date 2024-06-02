use crate::graph1_core::context::GraphContext;
use crate::primitives::primitives::RectArea;

pub fn rectangle_filled(ctx: &mut GraphContext, rect: &RectArea, color: u32) {
    // Dereference the options
    let start_x = rect.top_left.x;
    let start_y = rect.top_left.y;
    let width = rect.dimensions.w;
    let height = rect.dimensions.h;
    // let win_width = ctx.win.w
    // let win_height = ctx.win.h

    // Nothing to draw here
    if width == 0 || height == 0 {
        return;
    }

    let end_x = start_x + width;
    let end_y = start_y + height;

    let mut x = start_x;
    let mut y = start_y;

    // Which pixel in the vector should be filled in next.
    let mut pixel_index: usize;

    loop {
        pixel_index = (y * ctx.win.w + x) as usize;
        ctx.buf_view[pixel_index] = color;
        x += 1;

        if x == end_x || x == ctx.win.w {
            y += 1;
            x = start_x;
        };

        if y == end_y || y == ctx.win.h {
            break;
        }
    }
}
