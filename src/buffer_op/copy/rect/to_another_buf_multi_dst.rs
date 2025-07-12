use crate::primitives::plane::{Dimensions2d, RectArea};
use crate::primitives::point::Point;
use std::thread;


/// Helper function to copy a rectangular region from a source buffer
/// to multiple destination positions within the destination buffer.
/// This function is used internally and assumes single-threaded usage.
fn to_another_buf_multi_dest_thread(
    src_buf: &[u32],
    src_dims: &Dimensions2d<u32>,
    x_start: u32,
    x_end: u32,
    y_start: u32,
    y_end: u32,
    dst_buf: &mut [u32],
    dst_dims: &Dimensions2d<u32>,
    dst_starts: &[Point<u32>],
    use_absolute_alpha: bool,
    local_y_offset: u32,
) {
    let copy_all_mask: u32 = if use_absolute_alpha { 0 } else { 0xFFFF_FFFF };

    for y in y_start..y_end {
        for x in x_start..x_end {
            let src_index = (src_dims.w * y + x) as usize;
            let pixel = src_buf[src_index];

            let src_alpha_nonzero = (((pixel & 0xFF) != 0) as u32).wrapping_neg();
            let mask = src_alpha_nonzero | copy_all_mask;

            for dst_start in dst_starts {
                let dst_x = dst_start.x + (x - x_start);
                let dst_y = dst_start.y + (y - y_start) + local_y_offset;

                let dst_index = (dst_dims.w * dst_y + dst_x) as usize;
                dst_buf[dst_index] = (dst_buf[dst_index] & !mask) | (pixel & mask);
            }
        }
    }
}

/// Copies a rectangle from the source buffer to multiple destination positions in the destination buffer.
/// Optionally uses multiple threads to parallelize the operation across vertical slices of the source.
///
/// # Parameters
/// - `src_buf`: Source buffer with RGBA pixels.
/// - `src_dims`: Dimensions of the source buffer.
/// - `src_area`: Rectangle area in the source buffer to copy from.
/// - `dst_buf`: Destination buffer to copy pixels into.
/// - `dst_dims`: Dimensions of the destination buffer.
/// - `dst_starts`: List of top-left points where the source rectangle should be copied.
/// - `use_absolute_alpha`: If true, skip pixels with alpha == 0.
/// - `num_threads`: Number of threads to use. 0 = no-op, 1 = single-threaded, >1 = parallel copy.
pub fn to_another_buf_multi_dst(
    src_buf: &[u32],
    src_dims: &Dimensions2d<u32>,
    src_area: &RectArea<u32>,
    dst_buf: &mut [u32],
    dst_dims: &Dimensions2d<u32>,
    dst_starts: &[Point<u32>],
    use_absolute_alpha: bool,
    num_threads: usize,
) {
    if num_threads == 0 || dst_starts.is_empty() {
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

    let region_width = x_end - x_start;
    let region_height = y_end - y_start;

    // Filter only valid destination start points
    let clipped_starts: Vec<Point<u32>> = dst_starts
        .iter()
        .cloned()
        .filter(|p| {
            p.x <= dst_dims.w.saturating_sub(region_width)
                && p.y <= dst_dims.h.saturating_sub(region_height)
        })
        .collect();

    if clipped_starts.is_empty() {
        return;
    }

    if num_threads == 1 || region_height < num_threads as u32 {
        to_another_buf_multi_dest_thread(
            src_buf,
            src_dims,
            x_start,
            x_end,
            y_start,
            y_end,
            dst_buf,
            dst_dims,
            &clipped_starts,
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
            let dst_starts = clipped_starts.clone();
            // SAFETY: writes are disjoint by region; overlaps within dst_starts are user-managed.
            let dst_slice = unsafe { &mut *dst_buf_ptr };

            s.spawn(move || {
                to_another_buf_multi_dest_thread(
                    src_buf,
                    src_dims,
                    x_start,
                    x_end,
                    src_y,
                    src_y_end,
                    dst_slice,
                    dst_dims,
                    &dst_starts,
                    use_absolute_alpha,
                    dst_y_offset,
                );
            });

            src_y = src_y_end;
            dst_y_offset += lines;
        }
    });
}
