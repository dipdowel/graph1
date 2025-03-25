use crate::primitives::plane::{Dimensions2d, RectArea};
use crate::primitives::point::Point;
use std::cmp::min;



/// # Arguments
/// * `buf` - A mutable slice of u32 values representing the pixel buffer. Each pixel is a 0RGB value.
/// * `buf_dimensions` - The dimensions of the pixel buffer.
/// * `area` - The region of the buffer to transform.
/// * `operation` - The operation to apply to the region.

pub fn invert_colors(buf: &mut [u32], buf_dimensions: &Dimensions2d, area: &RectArea) {
    let dst_point: Point = area.top_left;

    //----------------------------------------------------------------------------------------------
    // ENSURE COORDINATES ARE WITHIN THE BOUNDS OF THE BUFFER

    let dst_area_w = min(
        area.dimensions.w,
        buf_dimensions.w.saturating_sub(dst_point.x),
    );
    let dst_area_h = min(
        area.dimensions.h,
        buf_dimensions.h.saturating_sub(dst_point.y),
    );

    //----------------------------------------------------------------------------------------------

    // Iterate over each pixel in the specified source region
    for y in 0..dst_area_h {
        for x in 0..dst_area_w {
            // Calculate the destination pixel coordinates
            let dst_x = dst_point.x + x;
            let dst_y = dst_point.y + y;

            // Calculate the indices for the  destination pixel
            let dst_index = (dst_y * buf_dimensions.w + dst_x) as usize;

            // Get the destination pixel value
            let dst_pixel = buf[dst_index];

            // Inverse a pixel and write it back to the destination buffer
            buf[dst_index] = invert_pixel(dst_pixel);
        }
    }
}

/// Inverts the color of an RGBA pixel.
#[inline(always)]

pub fn invert_pixel(pixel: u32) -> u32 {
    // Color as channels
    let r = (pixel >> 24) & 0xFF;
    let g = (pixel >> 16) & 0xFF;
    let b = (pixel >> 8) & 0xFF;
    let a = pixel & 0xFF;
    ((0xFF - r) << 24) | ((0xFF - g) << 16) | ((0xFF - b) << 8) | a
}
