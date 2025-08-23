use crate::primitives::plane::{Dimensions2d, RectArea};
use crate::primitives::point::Point;
use crate::primitives::math::Displacement;

use std::thread;
use crate::buffer_op::scale::scale_direction::ScaleDirection;

/// Worker: scale region into a destination buffer (single-threaded).
fn scale_to_another_buf_thread(
    src_buf: &[u32],
    src_dims: &Dimensions2d<u32>,
    src_area: &RectArea<u32>,
    dst_buf: &mut [u32],
    dst_dims: &Dimensions2d<u32>,
    dst_start: &Point<u32>,
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
                        let dst_y = dst_start.y + (sy * scale_factor + dy) + local_y_offset;
                        for dx in 0..scale_factor {
                            let dst_x = dst_start.x + (sx * scale_factor + dx);
                            if dst_x < dst_dims.w && dst_y < dst_dims.h {
                                dst_buf[(dst_dims.w * dst_y + dst_x) as usize] = pixel;
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
                let dst_y = dst_start.y + oy + local_y_offset;
                for ox in 0..out_w {
                    let src_x = src_area.top_left.x + ox * scale_factor + disp.dx as u32;
                    let src_y = src_area.top_left.y + oy * scale_factor + disp.dy as u32;
                    let pixel = src_buf[(src_y * src_dims.w + src_x) as usize];

                    let dst_x = dst_start.x + ox;
                    if dst_x < dst_dims.w && dst_y < dst_dims.h {
                        dst_buf[(dst_dims.w * dst_y + dst_x) as usize] = pixel;
                    }
                }
            }
        }
    }
}

/// Multithreaded scaling of a rectangular region from one buffer into another.
pub fn to_another_buf(
    src_buf: &[u32],
    src_dims: &Dimensions2d<u32>,
    src_area: &RectArea<u32>,
    dst_buf: &mut [u32],
    dst_dims: &Dimensions2d<u32>,
    dst_start: &Point<u32>,
    scale_factor: u32,
    src_pixel_displacement: Option<&Displacement<u8>>,
    direction: ScaleDirection,
    num_threads: usize,
) {
    if num_threads == 0 || scale_factor == 0 {
        return;
    }

    let region_height = match direction {
        ScaleDirection::Up => src_area.dimensions.h * scale_factor,
        ScaleDirection::Down => src_area.dimensions.h / scale_factor,
    };

    if num_threads == 1 || region_height < num_threads as u32 {
        scale_to_another_buf_thread(
            src_buf, src_dims, src_area,
            dst_buf, dst_dims, dst_start,
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
            let dst_slice = unsafe { &mut *dst_buf_ptr };
            let src_area = *src_area;

            s.spawn(move || {
                scale_to_another_buf_thread(
                    src_buf, src_dims, &src_area,
                    dst_slice, dst_dims, dst_start,
                    scale_factor, src_pixel_displacement,
                    dst_y_offset, direction
                );
            });

            dst_y_offset += lines;
        }
    });
}
