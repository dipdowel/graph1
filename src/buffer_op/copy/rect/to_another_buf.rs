use crate::primitives::plane::{Dimensions2d, RectArea};
use crate::primitives::point::Point;
use std::thread;

/// Helper function that performs the actual buffer copy for a single region.
fn to_another_buf_thread(
    src_buf: &[u32],
    src_dims: &Dimensions2d<u32>,
    x_start: u32,
    x_end: u32,
    y_start: u32,
    y_end: u32,
    dst_buf: &mut [u32],
    dst_dims: &Dimensions2d<u32>,
    dst_start: &Point<u32>,
    use_absolute_alpha: bool,
    local_y_offset: u32,
) {
    let copy_all_mask: u32 = if use_absolute_alpha { 0 } else { 0xFFFF_FFFF };

    for y in y_start..y_end {
        let dst_y = dst_start.y + (y - y_start) + local_y_offset;
        for x in x_start..x_end {
            let dst_x = dst_start.x + (x - x_start);
            let src_index = (src_dims.w * y + x) as usize;
            let dst_index = (dst_dims.w * dst_y + dst_x) as usize;

            let src_alpha_nonzero = (((src_buf[src_index] & 0xFF) != 0) as u32).wrapping_neg();
            let mask = src_alpha_nonzero | copy_all_mask;
            dst_buf[dst_index] = (dst_buf[dst_index] & !mask) | (src_buf[src_index] & mask);
        }
    }
}

/// Multithreaded version of `to_another_buf` with optional thread count.
/// Falls back to single-threaded mode or no-op as required.
///
/// # Parameters
/// - `src_buf`: Source buffer with RGBA pixels.
/// - `src_dims`: Dimensions of the source buffer.
/// - `src_area`: Rectangle area in the source buffer to copy from.
/// - `dst_buf`: Destination buffer to copy pixels into.
/// - `dst_dims`: Dimensions of the destination buffer.
/// - `dst_start`: The top-left point of the destination where to copy pixels to.
/// - `use_absolute_alpha`: If true, skip pixels with alpha == 0.
/// - `num_threads`: Number of threads to use. 0 = no-op, 1 = single-threaded, >1 = scoped threads.
pub fn to_another_buf(
    src_buf: &[u32],
    src_dims: &Dimensions2d<u32>,
    src_area: &RectArea<u32>,
    dst_buf: &mut [u32],
    dst_dims: &Dimensions2d<u32>,
    dst_start: &Point<u32>,
    use_absolute_alpha: bool,
    num_threads: usize,
) {
    if num_threads == 0 {
        return;
    }

    let Point {
        x: x_start,
        y: y_start,
    } = src_area.top_left;
    let Point {
        x: mut x_end,
        y: mut y_end,
    } = src_area.get_bottom_right();

    // Clip source to buffer bounds
    x_end = x_end.min(src_dims.w);
    y_end = y_end.min(src_dims.h);

    let region_width = (  x_end.saturating_sub(x_start)).min(dst_dims.w.saturating_sub(dst_start.x));
    let region_height = (y_end.saturating_sub(y_start)).min(dst_dims.h.saturating_sub(dst_start.y));
    x_end = x_start + region_width;
    y_end = y_start + region_height;

    if num_threads == 1 || region_height < num_threads as u32 {
        to_another_buf_thread(
            src_buf,
            src_dims,
            x_start,
            x_end,
            y_start,
            y_end,
            dst_buf,
            dst_dims,
            dst_start,
            use_absolute_alpha,
            0,
        );
        return;
    }

    let lines_per_thread = region_height / num_threads as u32;
    let remainder = region_height % num_threads as u32;

    thread::scope(|s| {
        let mut src_y = y_start;
        let mut dst_y_offset = 0;

        for i in 0..num_threads {
            let extra_line = if i < remainder as usize { 1 } else { 0 };
            let lines = lines_per_thread + extra_line;
            let src_y_end = src_y + lines;

            let dst_buf_ptr = dst_buf as *mut [u32];
            // SAFETY: each thread is guaranteed exclusive mutable access to its own slice.
            let dst_slice = unsafe { &mut *dst_buf_ptr };

            s.spawn(move || {
                to_another_buf_thread(
                    src_buf,
                    src_dims,
                    x_start,
                    x_end,
                    src_y,
                    src_y_end,
                    dst_slice,
                    dst_dims,
                    dst_start,
                    use_absolute_alpha,
                    dst_y_offset,
                );
            });

            src_y = src_y_end;
            dst_y_offset += lines;
        }
    });
}
