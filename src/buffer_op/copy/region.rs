use crate::primitives::plane::{Dimensions2d, RectArea};
use crate::primitives::point::Point;

pub fn to_another_buf(
    src_buf: &Vec<u32>,
    src_dims: &Dimensions2d<u32>,
    src_area: &RectArea<u32>,
    dst_buf: &mut Vec<u32>,
    dst_dims: &Dimensions2d<u32>,
    dst_start: &Point<u32>,
    use_absolute_alpha: bool,
) {
    let Point {
        x: x_start,
        y: y_start,
    } = src_area.top_left;
    let Point {
        x: mut x_end,
        y: mut y_end,
    } = src_area.get_bottom_right();

    if x_end > dst_dims.w {
        x_end = dst_dims.w - 1;
    }

    if y_end > dst_dims.h {
        y_end = dst_dims.h - 1;
    }

    let copy_all_mask: u32 = if use_absolute_alpha { 0 } else { 0xFFFF_FFFF };

    let mut src_index: usize;
    let mut dst_index: usize;
    let mut dst_x = dst_start.x;
    let mut dst_y = dst_start.y;

    for x in x_start..x_end {
        for y in y_start..y_end {
            src_index = (src_dims.w * y + x) as usize;
            dst_index = (dst_dims.w * dst_y + dst_x) as usize;
            let src_alpha_nonzero = (((src_buf[src_index] & 0xFF) != 0) as u32).wrapping_neg();
            let mask = src_alpha_nonzero | copy_all_mask;
            dst_buf[dst_index] = (dst_buf[dst_index] & !mask) | (src_buf[src_index] & mask);
            dst_y += 1;
        }
        dst_x += 1;
        dst_y = dst_start.y;
    }
}

pub fn within_buf(
    buf: &mut [u32],
    dims: &Dimensions2d<u32>,
    src_area: &RectArea<u32>,
    dst_start: &Point<u32>,
    use_absolute_alpha: bool,
) {
    let Point {
        x: x_start,
        y: y_start,
    } = src_area.top_left;
    let Point {
        x: mut x_end,
        y: mut y_end,
    } = src_area.get_bottom_right();

    if x_end > dims.w {
        x_end = dims.w - 1;
    }

    if y_end > dims.h {
        y_end = dims.h - 1;
    }

    let copy_all_mask: u32 = if use_absolute_alpha { 0 } else { 0xFFFF_FFFF };

    let mut src_index: usize;
    let mut dst_x = dst_start.x;
    let mut dst_y = dst_start.y;

    for y in y_start..y_end {
        for x in x_start..x_end {
            src_index = (dims.w * y + x) as usize;
            let dst_index: usize = (dims.w * dst_y + dst_x) as usize;

            let src_alpha_nonzero = (((buf[src_index] & 0xFF) != 0) as u32).wrapping_neg();
            let mask = src_alpha_nonzero | copy_all_mask;
            buf[dst_index] = (buf[dst_index] & !mask) | (buf[src_index] & mask);

            dst_y += 1;
        }
        dst_x += 1;
        dst_y = dst_start.y;
    }
}
