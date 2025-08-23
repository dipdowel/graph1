use crate::primitives::plane::{Dimensions2d, RectArea};
use crate::primitives::point::Point;
use crate::primitives::math::Displacement;

use std::thread;
use crate::buffer_op::scale::scale_direction::ScaleDirection;

/// Worker: scale region into multiple destinations (single-threaded).
fn scale_to_multi_dst_thread(
    src_buf: &[u32],
    src_dims: &Dimensions2d<u32>,
    src_area: &RectArea<u32>,
    dst_buf: &mut [u32],
    dst_dims: &Dimensions2d<u32>,
    dst_starts: &[Point<u32>],
    scale_factor: u32,
    src_pixel_displacement: Option<&Displacement<u8>>,
    local_y_offset: u32,
    direction: ScaleDirection,
) {
    let rect_w = src_area.dimensions.w;
    let rect_h = src_area.dimensions.h;

    match direction {
        ScaleDirection::Up => {
            for sy in 0..rect_h {
                for sx in 0..rect_w {
                    let src_x = src_area.top_left.x + sx;
                    let src_y = src_area.top_left.y + sy;
                    let pixel = src_buf[(src_y * src_dims.w + src_x) as usize];

                    for dy in 0..scale_factor {
                        let block_y = sy * scale_factor + dy + local_y_offset;
                        for dx in 0..scale_factor {
                            let block_x = sx * scale_factor + dx;
                            for dst_start in dst_starts {
                                let dst_x = dst_start.x + block_x;
                                let dst_y = dst_start.y + block_y;
                                if dst_x < dst_dims.w && dst_y < dst_dims.h {
                                    dst_buf[(dst_dims.w * dst_y + dst_x) as usize] = pixel;
                                }
                            }
                        }
                    }
                }
            }
        }
        ScaleDirection::Down => {
            let disp = src_pixel_displacement.unwrap_or(&Displacement { dx: 0, dy: 0 });
            let out_w = rect_w / scale_factor;
            let out_h = rect_h / scale_factor;

            for oy in 0..out_h {
                let block_y = oy + local_y_offset;
                for ox in 0..out_w {
                    let src_x = src_area.top_left.x + ox * scale_factor + disp.dx as u32;
                    let src_y = src_area.top_left.y + oy * scale_factor + disp.dy as u32;
                    let pixel = src_buf[(src_y * src_dims.w + src_x) as usize];

                    for dst_start in dst_starts {
                        let dst_x = dst_start.x + ox;
                        let dst_y = dst_start.y + block_y;
                        if dst_x < dst_dims.w && dst_y < dst_dims.h {
                            dst_buf[(dst_dims.w * dst_y + dst_x) as usize] = pixel;
                        }
                    }
                }
            }
        }
    }
}

/// ## NB: This function is still experimental, might not work 100% correctly.
/// ==============================================================================
/// Scales a rectangular region from the source buffer into multiple positions
/// in the same destination buffer, using nearest-neighbor scaling.
/// Supports both upscaling and downscaling.
///
/// Each destination point in `dst_starts` acts as a top-left placement
/// for the scaled result. When downscaling, the `src_pixel_displacement`
/// parameter allows selecting a pixel inside each block.
///
/// The scaling can be multithreaded across horizontal bands of the region.
///
/// # Parameters
/// - `src_buf`: Source buffer with pixels (RGBA as `u32`).
/// - `src_dims`: Dimensions of the source buffer.
/// - `src_area`: Rectangular region of the source to scale.
/// - `dst_buf`: Destination buffer (mutable).
/// - `dst_dims`: Dimensions of the destination buffer.
/// - `dst_starts`: A list of placement points for writing scaled results.
/// - `scale_factor`: Integer scaling factor (≥1).
/// - `src_pixel_displacement`: Optional `(dx, dy)` pixel offset used for
///   block sampling when downscaling.
/// - `direction`: Scaling mode (`Up` = enlarge, `Down` = shrink).
/// - `num_threads`: Number of threads.
///   - `0` = no-op
///   - `1` = single-threaded
///   - `>1` = parallel row partitioning
pub fn to_another_buf_multi_dst(
    src_buf: &[u32],
    src_dims: &Dimensions2d<u32>,
    src_area: &RectArea<u32>,
    dst_buf: &mut [u32],
    dst_dims: &Dimensions2d<u32>,
    dst_starts: &[Point<u32>],
    scale_factor: u32,
    src_pixel_displacement: Option<&Displacement<u8>>,
    direction: ScaleDirection,
    num_threads: usize,
) {
    if num_threads == 0 || dst_starts.is_empty() || scale_factor == 0 {
        return;
    }

    let region_height = match direction {
        ScaleDirection::Up => src_area.dimensions.h * scale_factor,
        ScaleDirection::Down => src_area.dimensions.h / scale_factor,
    };

    if num_threads == 1 || region_height < num_threads as u32 {
        scale_to_multi_dst_thread(
            src_buf, src_dims, src_area,
            dst_buf, dst_dims, dst_starts,
            scale_factor, src_pixel_displacement,
            0, direction
        );
        return;
    }

    let lines_per_thread = region_height / num_threads as u32;
    let remainder = region_height % num_threads as u32;

    thread::scope(|s| {
        let mut dst_y_offset = 0;
        for i in 0..num_threads {
            let extra = if i < remainder as usize { 1 } else { 0 };
            let lines = lines_per_thread + extra;

            let dst_buf_ptr = dst_buf as *mut [u32];
            let dst_starts = dst_starts.to_vec();
            let dst_slice = unsafe { &mut *dst_buf_ptr };
            let src_area = *src_area;

            s.spawn(move || {
                scale_to_multi_dst_thread(
                    src_buf, src_dims, &src_area,
                    dst_slice, dst_dims, &dst_starts,
                    scale_factor, src_pixel_displacement,
                    dst_y_offset, direction
                );
            });

            dst_y_offset += lines;
        }
    });
}
