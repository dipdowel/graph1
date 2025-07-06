use crate::primitives::plane::{Dimensions2d, RectArea};
use crate::primitives::point::Point;

/// Copies a rectangular region from a source buffer to a destination buffer.
///
/// This function copies a subrectangle (`src_area`) from `src_buf` into a corresponding
/// position in `dst_buf`, starting at `dst_start`. It supports optional alpha-masking so
/// that it's possible to skip copying pixels with alpha channel == `00`.
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
/// Will panic if indices are out of bounds for the provided buffers.
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

    // Clip rectangle if it would exceed destination buffer dimensions
    if x_end > dst_dims.w {
        x_end = dst_dims.w - 1;
    }

    if y_end > dst_dims.h {
        y_end = dst_dims.h - 1;
    }

    // If use_absolute_alpha is true, copy all pixels (mask = 0)
    // Otherwise, copy only pixels that have their alpha channel greater than 0 (mask = 0xFFFF_FFFF)
    let copy_all_mask: u32 = if use_absolute_alpha { 0 } else { 0xFFFF_FFFF };

    let mut src_index: usize;
    let mut dst_index: usize;
    let mut dst_x = dst_start.x;
    let mut dst_y = dst_start.y;

    for x in x_start..x_end {
        for y in y_start..y_end {
            src_index = (src_dims.w * y + x) as usize;
            dst_index = (dst_dims.w * dst_y + dst_x) as usize;

            // Calculate if source pixel's alpha is nonzero
            // (The lowest byte is alpha, i.e. it is RGBA)
            let src_alpha_nonzero = (((src_buf[src_index] & 0xFF) != 0) as u32).wrapping_neg();
            let mask = src_alpha_nonzero | copy_all_mask;
            // Bitwise blend: keeps destination if mask == 0, copies source if mask == 0xFFFF_FFFF
            dst_buf[dst_index] = (dst_buf[dst_index] & !mask) | (src_buf[src_index] & mask);

            dst_y += 1;
        }
        dst_x += 1;
        dst_y = dst_start.y;
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
/// Will panic if indices are out of bounds for the provided buffer.
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

    // Clip rectangle if it would exceed buffer dimensions
    if x_end > dims.w {
        x_end = dims.w - 1;
    }

    if y_end > dims.h {
        y_end = dims.h - 1;
    }

    // If use_absolute_alpha is true, copy all pixels (mask = 0)
    // Otherwise, copy only pixels that have their alpha channel greater than 0 (mask = 0xFFFF_FFFF)
    let copy_all_mask: u32 = if use_absolute_alpha { 0 } else { 0xFFFF_FFFF };

    let mut src_index: usize;
    let mut dst_x = dst_start.x;
    let mut dst_y = dst_start.y;

    for y in y_start..y_end {
        for x in x_start..x_end {
            src_index = (dims.w * y + x) as usize;
            let dst_index: usize = (dims.w * dst_y + dst_x) as usize;

            // See comment in `to_another_buf()`: nonzero alpha test, create mask
            let src_alpha_nonzero = (((buf[src_index] & 0xFF) != 0) as u32).wrapping_neg();
            let mask = src_alpha_nonzero | copy_all_mask;
            buf[dst_index] = (buf[dst_index] & !mask) | (buf[src_index] & mask);

            dst_y += 1;
        }
        dst_x += 1;
        dst_y = dst_start.y;
    }
}
