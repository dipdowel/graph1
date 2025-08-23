use crate::buffer_op::scale::scale_direction::ScaleDirection;
use crate::primitives::plane::{Dimensions2d, RectArea};
use crate::primitives::point::Point;
use crate::primitives::math::Displacement;


/// Scale a rectangular region within the same buffer.
/// Supports both upscaling and downscaling with nearest neighbor.
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
