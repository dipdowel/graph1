use crate::primitives::primitives::{Dimensions2d, Point, RectArea};

pub enum ImageFilter {
    /// Inverts the color of a buffer / pixel
    Invert,
    /// Desaturates (i.e. turns into grayscale) the color of a buffer / pixel
    Desaturate,
}

/// Transforms the colors of a region in a buffer using a specified operation.

/// # Arguments
/// * `buf` - A mutable slice of u32 values representing the pixel buffer. Each pixel is a 0RGB value.
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

    //----------------------------------------------------------------------------------------------
    // ENSURE COORDINATES ARE WITHIN THE BOUNDS OF THE BUFFER
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
    //----------------------------------------------------------------------------------------------

    // Iterate over each pixel in the specified source region
    for y in 0..dst_region_h {
        for x in 0..dst_region_w {
            // Calculate the destination pixel coordinates
            let dst_x = dst_point.x + x;
            let dst_y = dst_point.y + y;

            // Calculate the indices for the  destination pixel
            let dst_index = (dst_y * buf_dimensions.w + dst_x) as usize;

            // Get the destination pixel value
            let dst_pixel = buf[dst_index];

            // Blend the pixels using the specified blending mode
            let modified_pixel = match operation {
                ImageFilter::Invert => pixel_invert(dst_pixel),
                ImageFilter::Desaturate => pixel_desaturate(dst_pixel),
            };
            // Write the modified pixel back to the destination buffer
            buf[dst_index] = modified_pixel;
        }
    }
}

/// Desaturates a pixel by averaging its RGB components.
pub fn pixel_desaturate(color: u32) -> u32 {
    let mut r = (color >> 16) & 0xFF;
    let g = (color >> 8) & 0xFF;
    let b = color & 0xFF;
    r = (r + g + b) / 3;

    // Assign the average gray color to each RGB component
    (r << 16) | (r << 8) | r
}

/// Inverts the color of a pixel.
pub fn pixel_invert(pixel1: u32) -> u32 {
    // Extract the R, G, B components from the 0RGB color
    let r = (pixel1 >> 16) & 0xff;
    let g = (pixel1 >> 8) & 0xff;
    let b = pixel1 & 0xff;

    // Compute the opposite color components
    let r_opposite = 0xff - r;
    let g_opposite = 0xff - g;
    let b_opposite = 0xff - b;

    // Combine the components back into a single u32 color
    (r_opposite << 16) | (g_opposite << 8) | b_opposite
}
