use crate::primitives::plane::{Dimensions2d, RectArea};
use crate::primitives::point::Point;

/// Copies a rectangular region from a source buffer to a destination buffer.
///
/// This function copies a subrectangle (`src_area`) from `src_buf` into a corresponding
/// position in `dst_buf`, starting at `dst_start`. It supports optional alpha-masking so
/// that pixels with alpha channel == `00` can be skipped.
///
/// # Parameters
/// - `src_buf`: Source buffer with RGBA pixels.
/// - `src_dims`: Dimensions of the source buffer.
/// - `src_area`: Rectangle area in the source buffer to copy from.
/// - `dst_buf`: Destination buffer to copy pixels into.
/// - `dst_dims`: Dimensions of the destination buffer.
/// - `dst_start`: The top-left point of the destination where to copy pixels to.
/// - `use_absolute_alpha`: if true, copy only pixels with alpha channel greater than `00`;
///
/// # Panics
/// Will panic if indices are out of bounds for the provided buffers. However,
/// clipping logic guarantees that all writes remain within bounds.
pub fn to_another_buf(
    src_buf: &Vec<u32>,
    src_dims: &Dimensions2d<u32>,
    src_area: &RectArea<u32>,
    dst_buf: &mut Vec<u32>,
    dst_dims: &Dimensions2d<u32>,
    dst_start: &Point<u32>,
    use_absolute_alpha: bool,
) {
    let Point { x: x_start, y: y_start } = src_area.top_left;
    let Point { x: mut x_end, y: mut y_end } = src_area.get_bottom_right();

    // 1. Clip source rectangle to source buffer bounds.
    if x_end > src_dims.w {
        x_end = src_dims.w;
    }
    if y_end > src_dims.h {
        y_end = src_dims.h;
    }

    // 2. Clip region width/height so we never write out of destination buffer bounds.
    let region_width = (x_end - x_start).min(dst_dims.w.saturating_sub(dst_start.x));
    let region_height = (y_end - y_start).min(dst_dims.h.saturating_sub(dst_start.y));
    x_end = x_start + region_width;
    y_end = y_start + region_height;

    let copy_all_mask: u32 = if use_absolute_alpha { 0 } else { 0xFFFF_FFFF };
    let mut src_index: usize;
    let mut dst_index: usize;
    let mut dst_x;
    let mut dst_y;

    // Main loop: copy all eligible pixels, now guaranteed to be within buffer bounds.
    for y in y_start..y_end {
        dst_y = dst_start.y + (y - y_start);
        for x in x_start..x_end {
            dst_x = dst_start.x + (x - x_start);

            src_index = (src_dims.w * y + x) as usize;
            dst_index = (dst_dims.w * dst_y + dst_x) as usize;

            // Alpha channel masking (RGBA: lowest byte is alpha)
            let src_alpha_nonzero = (((src_buf[src_index] & 0xFF) != 0) as u32).wrapping_neg();
            let mask = src_alpha_nonzero | copy_all_mask;
            // Bitwise blend: keeps destination if mask == 0, copies source if mask == 0xFFFF_FFFF
            dst_buf[dst_index] = (dst_buf[dst_index] & !mask) | (src_buf[src_index] & mask);
        }
    }
}

/// Copies a rectangular region within a single buffer, possibly overlapping.
///
/// This function copies a subrectangle (`src_area`) within the same buffer to another
/// region, starting at `dst_start`. Alpha masking is supported just like in `to_another_buf`.
///
/// # Parameters
/// - `buf`: Buffer containing RGBA pixels.
/// - `dims`: Dimensions of the buffer.
/// - `src_area`: Rectangle area in buffer to copy from.
/// - `dst_start`: The top-left point of the destination where to copy pixels to.
/// - `use_absolute_alpha`: if true, copy only pixels with alpha channel greater than `00`;
///
/// # Panics
/// Will panic if indices are out of bounds for the provided buffer. However,
/// clipping logic guarantees that all writes remain within bounds.
pub fn within_buf(
    buf: &mut [u32],
    dims: &Dimensions2d<u32>,
    src_area: &RectArea<u32>,
    dst_start: &Point<u32>,
    use_absolute_alpha: bool,
) {
    let Point { x: x_start, y: y_start } = src_area.top_left;
    let Point { x: mut x_end, y: mut y_end } = src_area.get_bottom_right();

    // 1. Clip source rectangle to buffer bounds.
    if x_end > dims.w {
        x_end = dims.w;
    }
    if y_end > dims.h {
        y_end = dims.h;
    }

    // 2. Clip region width/height to not write out of buffer bounds.
    let region_width = (x_end - x_start).min(dims.w.saturating_sub(dst_start.x));
    let region_height = (y_end - y_start).min(dims.h.saturating_sub(dst_start.y));
    x_end = x_start + region_width;
    y_end = y_start + region_height;

    let copy_all_mask: u32 = if use_absolute_alpha { 0 } else { 0xFFFF_FFFF };
    let mut src_index: usize;
    let mut dst_index: usize;
    let mut dst_x;
    let mut dst_y;

    // Main loop: copy all eligible pixels, now guaranteed to be within buffer bounds.
    for y in y_start..y_end {
        dst_y = dst_start.y + (y - y_start);
        for x in x_start..x_end {
            dst_x = dst_start.x + (x - x_start);

            src_index = (dims.w * y + x) as usize;
            dst_index = (dims.w * dst_y + dst_x) as usize;

            // Alpha channel masking (RGBA: lowest byte is alpha)
            let src_alpha_nonzero = (((buf[src_index] & 0xFF) != 0) as u32).wrapping_neg();
            let mask = src_alpha_nonzero | copy_all_mask;
            buf[dst_index] = (buf[dst_index] & !mask) | (buf[src_index] & mask);
        }
    }
}
