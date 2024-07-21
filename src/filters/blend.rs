use crate::primitives::primitives::{Dimensions2d, Point, RectArea};

pub enum BlendMode {
    Multiply,
    Add,
    Difference,
    Screen,
}

/// Blends a region from the source buffer into the destination buffer using a specified blending mode.
///
/// # Arguments
///
/// * `dst_buf` - A mutable slice of u32 values representing the destination pixel buffer. Each pixel is a 0RGB value.
/// * `dst_buf_dimensions` - The dimensions of the destination pixel buffer.
/// * `dst_point` - The point in the destination buffer where the source buffer will be applied.
/// * `src_buf` - A slice of u32 values representing the source pixel buffer. Each pixel is a 0RGB value.
/// * `src_buf_dimensions` - The dimensions of the source pixel buffer.
/// * `src_region` - The region of the source buffer that will be applied to the destination buffer.
/// * `mode` - The blending mode to use.
pub fn blend(
    dst_buf: &mut [u32],
    dst_buf_dimensions: &Dimensions2d,
    dst_point: &Point,
    src_buf: &[u32],
    src_buf_dimensions: &Dimensions2d,
    src_region: &RectArea,
    mode: &BlendMode,
) {
    let dst_width = dst_buf_dimensions.w;
    let dst_height = dst_buf_dimensions.h;

    let src_width = src_buf_dimensions.w;
    let src_height = src_buf_dimensions.h;

    // FIXME: check if the copy is at all needed. Maybe we can use `dst_buf` directly?
    // Create a copy of the destination buffer to read from, necessary for in-place blending.
    let original_dst_buf = dst_buf.to_vec();

    // Iterate over each pixel in the specified source region
    for y in 0..src_region.dimensions.h {
        for x in 0..src_region.dimensions.w {
            // Calculate the source pixel coordinates
            let src_x = src_region.top_left.x + x;
            let src_y = src_region.top_left.y + y;

            // Calculate the destination pixel coordinates
            let dst_x = dst_point.x + x;
            let dst_y = dst_point.y + y;

            // Ensure coordinates are within the bounds of both buffers
            if src_x < src_width && src_y < src_height && dst_x < dst_width && dst_y < dst_height {
                // Calculate the indices for source and destination pixels
                let src_index = (src_y * src_width + src_x) as usize;
                let dst_index = (dst_y * dst_width + dst_x) as usize;

                // Get the source and destination pixel values
                let src_pixel = src_buf[src_index];
                let dst_pixel = original_dst_buf[dst_index];

                // Blend the pixels using the specified blending mode
                let blended_pixel = match mode {
                    BlendMode::Multiply => blend_pixel_multiply(src_pixel, dst_pixel),
                    BlendMode::Add => blend_pixel_add(src_pixel, dst_pixel),
                    BlendMode::Difference => blend_pixel_difference(src_pixel, dst_pixel),
                    BlendMode::Screen => blend_pixel_screen(src_pixel, dst_pixel),
                };

                // Write the blended pixel back to the destination buffer
                dst_buf[dst_index] = blended_pixel;
            }
        }
    }
}

/// Blends a region in the destination buffer with a given color using a specified blending mode.
///
/// # Arguments
///
/// * `dst_buf` - A mutable slice of u32 values representing the destination pixel buffer. Each pixel is a 0RGB value.
/// * `dst_buf_dimensions` - The dimensions of the destination pixel buffer.
/// * `dst_region` - The region of the destination buffer that will blended with the color.
/// * `color` - Color to apply
/// * `mode` - The blending mode to use.
pub fn blend_with_color(
    dst_buf: &mut [u32],
    dst_buf_dimensions: &Dimensions2d,
    dst_region: &RectArea,
    color: u32,
    mode: &BlendMode,
) {
    let dst_point: Point = dst_region.top_left;

    //----------------------------------------------------------------------------------------------
    // ENSURE COORDINATES ARE WITHIN THE BOUNDS OF THE BUFFER
    let rightmost_x = dst_region.top_left.x + dst_region.dimensions.w;
    let bottommost_y = dst_region.top_left.y + dst_region.dimensions.h;

    let dst_region_w = if rightmost_x > dst_buf_dimensions.w {
        println!("Rightmost x coordinate exceeds buffer width, clamping to buffer width");
        dst_buf_dimensions.w - dst_region.top_left.x
    } else {
        dst_region.dimensions.w
    };

    let dst_region_h = if bottommost_y > dst_buf_dimensions.h {
        println!("Bottommost y coordinate exceeds buffer height, clamping to buffer height");
        dst_buf_dimensions.h - dst_region.top_left.y
    } else {
        dst_region.dimensions.h
    };
    //----------------------------------------------------------------------------------------------

    // Iterate over each pixel in the specified source region
    for y in 0..dst_region_h {
        for x in 0..dst_region_w {
            // Calculate the destination pixel coordinates
            let dst_x = dst_point.x + x;
            let dst_y = dst_point.y + y;

            // Calculate the indices for the  destination pixel
            let dst_index = (dst_y * dst_buf_dimensions.w + dst_x) as usize;

            // Get the destination pixel value
            let dst_pixel = dst_buf[dst_index];

            // Blend the pixels using the specified blending mode
            let blended_pixel = match mode {
                BlendMode::Multiply => blend_pixel_multiply(color, dst_pixel),
                BlendMode::Add => blend_pixel_add(color, dst_pixel),
                BlendMode::Difference => blend_pixel_difference(color, dst_pixel),
                BlendMode::Screen => blend_pixel_screen(color, dst_pixel),
            };

            // Write the blended pixel back to the destination buffer
            dst_buf[dst_index] = blended_pixel;
        }
    }
}

/// Multiplies two pixels.
pub fn blend_pixel_multiply(pixel1: u32, pixel2: u32) -> u32 {
    // Extract and multiply each color component, then combine them back into a single u32 pixel value
    let r =
        (((pixel1 >> 16) & 0xFF) as f32 * ((pixel2 >> 16) & 0xFF) as f32 / 255.0).min(255.0) as u32;
    let g =
        (((pixel1 >> 8) & 0xFF) as f32 * ((pixel2 >> 8) & 0xFF) as f32 / 255.0).min(255.0) as u32;
    let b = ((pixel1 & 0xFF) as f32 * (pixel2 & 0xFF) as f32 / 255.0).min(255.0) as u32;
    (r << 16) | (g << 8) | b
}

/// Adds two pixels.
pub fn blend_pixel_add(pixel1: u32, pixel2: u32) -> u32 {
    // Add the components, ensuring we don't exceed 255
    let r = (((pixel1 >> 16) & 0xFF) + ((pixel2 >> 16) & 0xFF)).min(255);
    let g = (((pixel1 >> 8) & 0xFF) + ((pixel2 >> 8) & 0xFF)).min(255);
    let b = ((pixel1 & 0xFF) + (pixel2 & 0xFF)).min(255);

    // Combine the components back into a single u32 value in ARGB format, with alpha set to 0x00
    (0x00 << 24) | (r << 16) | (g << 8) | b
}

/// Screens two pixels.
pub fn blend_pixel_screen(pixel1: u32, pixel2: u32) -> u32 {
    // Extract and screen each color component, then combine them back into a single u32 pixel value
    let r = (255 - ((255 - ((pixel1 >> 16) & 0xFF)) * (255 - ((pixel2 >> 16) & 0xFF)) / 255))
        .min(255) as u32;
    let g = (255 - ((255 - ((pixel1 >> 8) & 0xFF)) * (255 - ((pixel2 >> 8) & 0xFF)) / 255)).min(255)
        as u32;
    let b = (255 - ((255 - (pixel1 & 0xFF)) * (255 - (pixel2 & 0xFF)) / 255)).min(255) as u32;
    (r << 16) | (g << 8) | b
}

/// Blends two pixels using the Difference mode.
fn blend_pixel_difference(pixel1: u32, pixel2: u32) -> u32 {
    // Extract each color component from the two pixels
    let (r1, g1, b1) = ((pixel1 >> 16) & 0xFF, (pixel1 >> 8) & 0xFF, pixel1 & 0xFF);
    let (r2, g2, b2) = ((pixel2 >> 16) & 0xFF, (pixel2 >> 8) & 0xFF, pixel2 & 0xFF);

    // Calculate the difference for each color component
    let r = (r1 as i32 - r2 as i32).abs() as u32;
    let g = (g1 as i32 - g2 as i32).abs() as u32;
    let b = (b1 as i32 - b2 as i32).abs() as u32;

    // Combine the components back into a single u32 value
    (r << 16) | (g << 8) | b
}
