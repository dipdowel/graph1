use crate::graph_core::context::ContextWindow;
use crate::tools::primitives::{Point, RectArea};

/// Transparency-aware copy -- copies non-transparent pixels from the source memory buf to the destination memory buf.
/// - `src_buf_view` -- source memory buffer
/// - `dst_buf_view` -- destination memory buffer
/// - `src_area` -- A rectangular area that will be copied
/// - `dst_start` -- Top-left point of the destination area. The area itself matches the source.
/// - `transparency_color` -- Pixels of this color will not be copied to the destination
/// - `win` -- a `ContextWindow` instance with information on the window size
pub fn trans_copy(
    src_buf_view: &[u32],
    dst_buf_view: &mut [u32],
    src_area: &RectArea,
    dst_start: &Point,
    transparency_color: &u32,
    win:&ContextWindow
) {
    let x_start = src_area.top_left.x;
    let x_end = x_start + src_area.dimensions.w;

    let y_start = src_area.top_left.y;
    let y_end = y_start + src_area.dimensions.h;
    let mut index: usize;
    let mut pixel: u32;
    let transparency_color = *transparency_color;

    let mut dst_x = dst_start.x;
    let mut dst_y = dst_start.y;
    let dst_buf_view_len = dst_buf_view.len();
    for x in x_start..=x_end {
        for y in y_start..=y_end {
            index = (win.w * y + x) as usize;
            pixel = src_buf_view[index];
            if pixel != transparency_color {
                let dest_index: usize = (win.w * dst_y + dst_x) as usize;
                if dest_index < dst_buf_view_len {
                    dst_buf_view[dest_index] = pixel;
                }
            }
            dst_y += 1;
        }
        dst_x += 1;
        dst_y = dst_start.y;
    }
}
