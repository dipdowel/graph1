use crate::buffer_op::scale::scale_direction::ScaleDirection;
use crate::primitives::plane::{Dimensions2d, RectArea};
use crate::primitives::point::Point;
use crate::primitives::math::Displacement;

/// ## NB: This function is still experimental, might not work 100% correctly.
/// ==============================================================================
/// Scales a rectangular region inside a single buffer into another region
/// of the same buffer. Supports both nearest-neighbor upscaling and
/// downscaling.
///
/// When downscaling, the `src_pixel_displacement` parameter determines
/// which pixel inside each `scale_factor x scale_factor` block of the
/// source region is chosen for output.
///
/// This function does not use threading and runs in a single loop.
///
/// # Parameters
/// - `buf`: Buffer containing pixels (RGBA as `u32`), used as both source and destination.
/// - `dims`: Dimensions of the buffer.
/// - `src_area`: The rectangular region in the buffer to scale.
/// - `dst_start`: Top-left point where the scaled region should be written.
/// - `scale_factor`: Integer factor for scaling (≥1).
/// - `src_pixel_displacement`: Optional displacement `(dx, dy)` inside source
///   blocks (applies only for downscaling).
/// - `direction`: Whether to scale up or down (`ScaleDirection`).
pub fn within_buf(
    buf: &mut [u32],
    dims: &Dimensions2d<u32>,
    src_area: &RectArea<u32>,
    dst_start: &Point<u32>,
    scale_factor: u32,
    src_pixel_displacement: Option<&Displacement<u8>>,
    direction: ScaleDirection,
) {
    if scale_factor == 0 {
        return;
    }

    let rect_w = src_area.dimensions.w;
    let rect_h = src_area.dimensions.h;

    match direction {
        ScaleDirection::Up => {
            for sy in 0..rect_h {
                for sx in 0..rect_w {
                    let src_x = src_area.top_left.x + sx;
                    let src_y = src_area.top_left.y + sy;
                    let pixel = buf[(src_y * dims.w + src_x) as usize];

                    for dy in 0..scale_factor {
                        let dst_y = dst_start.y + sy * scale_factor + dy;
                        for dx in 0..scale_factor {
                            let dst_x = dst_start.x + sx * scale_factor + dx;
                            if dst_x < dims.w && dst_y < dims.h {
                                buf[(dims.w * dst_y + dst_x) as usize] = pixel;
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
                for ox in 0..out_w {
                    let src_x = src_area.top_left.x + ox * scale_factor + disp.dx as u32;
                    let src_y = src_area.top_left.y + oy * scale_factor + disp.dy as u32;
                    let pixel = buf[(src_y * dims.w + src_x) as usize];

                    let dst_x = dst_start.x + ox;
                    let dst_y = dst_start.y + oy;
                    if dst_x < dims.w && dst_y < dims.h {
                        buf[(dims.w * dst_y + dst_x) as usize] = pixel;
                    }
                }
            }
        }
    }
}
