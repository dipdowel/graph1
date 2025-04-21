use crate::primitives::plane::{Dimensions2d, RectArea};
use crate::primitives::point::Point;


pub enum ImageFilter {
    /// Inverts the color of a buffer / pixel
    Invert,
    /// Desaturates (i.e. turns into grayscale) the color of a buffer / pixel
    Desaturate,
}

/// Transforms the colors of a region in a buffer using a specified operation.
///
/// # Arguments
/// * `buf` - A mutable slice of u32 values representing the pixel buffer. Each pixel is an RGBA value (0xRRGGBBAA).
/// * `buf_dimensions` - The dimensions of the pixel buffer.
/// * `region` - The region of the buffer to transform.
/// * `operation` - The operation to apply to the region.
pub fn transform_colors(
    buf: &mut [u32],
    buf_dimensions: &Dimensions2d,
    region: &RectArea,
    operation: &ImageFilter,
) {
    let dst_point: Point = region.top_left;

    let rightmost_x = region.top_left.x + region.dimensions.w;
    let bottommost_y = region.top_left.y + region.dimensions.h;

    let dst_region_w = if rightmost_x > buf_dimensions.w {
        println!("Rightmost x coordinate exceeds buffer width, clamping to buffer width");
        buf_dimensions.w - region.top_left.x
    } else {
        region.dimensions.w
    };

    let dst_region_h = if bottommost_y > buf_dimensions.h {
        println!("Bottommost y coordinate exceeds buffer height, clamping to buffer height");
        buf_dimensions.h - region.top_left.y
    } else {
        region.dimensions.h
    };

    for y in 0..dst_region_h {
        for x in 0..dst_region_w {
            let dst_x = dst_point.x + x;
            let dst_y = dst_point.y + y;
            let dst_index = (dst_y * buf_dimensions.w + dst_x) as usize;

            let dst_pixel = buf[dst_index];

            let modified_pixel = match operation {
                ImageFilter::Invert => pixel_invert(dst_pixel),
                ImageFilter::Desaturate => pixel_desaturate(dst_pixel),
            };

            buf[dst_index] = modified_pixel;
        }
    }
}

/// Desaturates a pixel in RGBA format by averaging RGB components and preserving alpha.
///
/// # Parameters
/// * `color` - A 32-bit unsigned int in RGBA format (0xRRGGBBAA)
///
/// # Returns
/// A desaturated grayscale pixel in RGBA format.
pub fn pixel_desaturate(color: u32) -> u32 {
    let r = (color >> 24) & 0xFF;
    let g = (color >> 16) & 0xFF;
    let b = (color >> 8) & 0xFF;
    let a = color & 0xFF;

    let gray = ((r + g + b) / 3) as u32;
    (gray << 24) | (gray << 16) | (gray << 8) | a
}

/// Inverts a pixel in RGBA format while preserving alpha.
///
/// # Parameters
/// * `pixel1` - A 32-bit unsigned int in RGBA format (0xRRGGBBAA)
///
/// # Returns
/// An inverted color in RGBA format.
pub fn pixel_invert(pixel1: u32) -> u32 {
    let r = (pixel1 >> 24) & 0xFF;
    let g = (pixel1 >> 16) & 0xFF;
    let b = (pixel1 >> 8) & 0xFF;
    let a = pixel1 & 0xFF;

    let r_inv = 0xFF - r;
    let g_inv = 0xFF - g;
    let b_inv = 0xFF - b;

    (r_inv << 24) | (g_inv << 16) | (b_inv << 8) | a
}
