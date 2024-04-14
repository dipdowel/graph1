use crate::init::init_window::WIN_WIDTH;
use crate::tools::primitives::{Point, RectArea};

pub fn copy_non_transparent_pixels(
    src_buf_view: &[u32],
    dst_buf_view: &mut [u32],
    src_area: RectArea,
    dst_start: Point,
    transparency_color: u32,
) {

    let x_start = src_area.top_left.x;
    let x_end = x_start + src_area.dimensions.w;

    let y_start = src_area.top_left.y;
    let y_end = y_start + src_area.dimensions.h;
    let mut index: usize;
    let mut pixel: u32;


    let mut dst_x = dst_start.x;
    let mut dst_y = dst_start.y;
    let dst_buf_view_len = dst_buf_view.len();
    for x in x_start..=x_end {
        for y in y_start..=y_end {
            index = (WIN_WIDTH * y + x) as usize;
            pixel = src_buf_view[index];
            if pixel != transparency_color {
                let dest_index: usize = (WIN_WIDTH * dst_y + dst_x) as usize;
                if dest_index < dst_buf_view_len {
                    dst_buf_view[dest_index] = pixel;
                }
            }
            dst_y += 1;
            // FIXME: It blows up at around here!!!
        }
        dst_x += 1;
        dst_y = dst_start.y;
    }
}
