use crate::primitives::plane::{Dimensions2d, RectArea};
use crate::primitives::point::Point;
use crate::primitives::math::Displacement;
use std::thread;

/// Worker: scale region into a destination buffer (single-threaded, sparse upscaling).
fn scale_to_another_buf_sparse_thread(
    src_buf: &[u32],
    src_dims: &Dimensions2d<u32>,
    src_area: &RectArea<u32>,
    dst_buf: &mut [u32],
    dst_dims: &Dimensions2d<u32>,
    dst_start: &Point<u32>,
    scale_factor: u32,
    dst_pixel_displacement: &Displacement<u8>,
    local_y_offset: u32,
) {
    let rect_w = src_area.dimensions.w;
    let rect_h = src_area.dimensions.h;

    // Iterate over source pixels within the source area.
    for sy in 0..rect_h {
        for sx in 0..rect_w {
            let src_x = src_area.top_left.x + sx;
            let src_y = src_area.top_left.y + sy;
            let pixel = src_buf[(src_y * src_dims.w + src_x) as usize];

            // Compute the top-left corner of the corresponding destination rectangle.
            // Apply displacement (dot-matrix spacing) here.
            let base_dst_y = dst_start.y
                + sy * (scale_factor + dst_pixel_displacement.dy as u32)
                + local_y_offset;
            let base_dst_x = dst_start.x + sx * (scale_factor + dst_pixel_displacement.dx as u32);

            // Fill the destination rectangle for this source pixel.
            for dy in 0..scale_factor {
                let dst_y = base_dst_y + dy;
                if dst_y >= dst_dims.h {
                    continue;
                }
                for dx in 0..scale_factor {
                    let dst_x = base_dst_x + dx;
                    if dst_x < dst_dims.w {
                        dst_buf[(dst_dims.w * dst_y + dst_x) as usize] = pixel;
                    }
                }
            }
        }
    }
}

/// ==============================================================================
/// Sparse Upscaling Function
/// ------------------------------------------------------------------------------
/// Scales a rectangular region from the source buffer into a destination buffer,
/// using nearest-neighbor pixel replication. Unlike `scale::rect::to_another_buf`, this
/// function only supports **upscaling**. Additionally, each scaled pixel block
/// is placed in the destination buffer with extra spacing (displacement),
/// imitating a dot-matrix printer effect.
///
/// Supports multi-threading in the same way as `scale::rect::to_another_buf()`,
/// splitting the output region into horizontal slices across worker threads.
///
/// # Parameters
/// - `src_buf`: Source buffer containing pixels (RGBA packed as `u32`).
/// - `src_dims`: Dimensions of the source buffer.
/// - `src_area`: The rectangular region in the source buffer to scale.
/// - `dst_buf`: Destination buffer where the scaled region is written.
/// - `dst_dims`: Dimensions of the destination buffer.
/// - `dst_start`: Top-left point in the destination where scaling begins.
/// - `scale_factor`: Integer factor for scaling (≥1).
/// - `dst_pixel_displacement`: Horizontal/vertical displacement in destination
///   pixels between each scaled pixel block.
/// - `num_threads`: Number of worker threads to use.
///   - `0` = no-op
///   - `1` = single-threaded
///   - `>1` = multithreaded row partitioning
pub fn to_another_buf(
    src_buf: &[u32],
    src_dims: &Dimensions2d<u32>,
    src_area: &RectArea<u32>,
    dst_buf: &mut [u32],
    dst_dims: &Dimensions2d<u32>,
    dst_start: &Point<u32>,
    scale_factor: u32,
    dst_pixel_displacement: &Displacement<u8>,
    num_threads: usize,
) {
    if num_threads == 0 || scale_factor == 0 {
        return;
    }

    // Total vertical span of the output region (including displacement spacing).
    let region_height = src_area.dimensions.h * (scale_factor + dst_pixel_displacement.dy as u32);

    if num_threads == 1 || region_height < num_threads as u32 {
        scale_to_another_buf_sparse_thread(
            src_buf,
            src_dims,
            src_area,
            dst_buf,
            dst_dims,
            dst_start,
            scale_factor,
            dst_pixel_displacement,
            0,
        );
        return;
    }

    // Split the output region into horizontal slices per thread.
    let lines_per_thread = region_height / num_threads as u32;
    let remainder = region_height % num_threads as u32;

    thread::scope(|s| {
        let mut dst_y_offset = 0;
        for i in 0..num_threads {
            let extra = if i < remainder as usize { 1 } else { 0 };
            let lines = lines_per_thread + extra;

            let dst_buf_ptr = dst_buf as *mut [u32];
            let dst_slice = unsafe { &mut *dst_buf_ptr };
            let src_area = *src_area;

            let disp = *dst_pixel_displacement;

            s.spawn(move || {
                scale_to_another_buf_sparse_thread(
                    src_buf,
                    src_dims,
                    &src_area,
                    dst_slice,
                    dst_dims,
                    dst_start,
                    scale_factor,
                    &disp,
                    dst_y_offset,
                );
            });

            dst_y_offset += lines;
        }
    });
}
