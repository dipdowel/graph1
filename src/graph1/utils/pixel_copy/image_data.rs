use crate::graph1::primitives::primitives::{Dimensions2d, PixelColorTransformerFn, Point, RectArea};
use crate::graph1::text::printer::print;

/// Copies image data from a source buffer to a destination buffer, within specified areas and dimensions.
///
/// # Parameters
///
/// - `dst_buf`: The destination buffer for image data.
/// - `dst_dimensions`: Dimensions of the destination buffer.
/// - `dest_point`: A `Point` specifying the starting point in the destination buffer where the image data will be copied to.
///
/// - `src_buf`: The source buffer
/// - `src_dimensions`: Dimensions of the destination buffer.
/// - `src_region`: A `RectArea` specifying the rectangular area in the source buffer to copy.
///
pub fn copy(
    dst_buf: &mut [u32],
    dst_dimensions: &Dimensions2d,
    dst_point: &Point,
    src_buf: &[u32],
    src_dimensions: &Dimensions2d,
    src_region: &RectArea,
) {
    // Ensure the dimensions and starting points are within bounds
    if dst_point.x >= dst_dimensions.w
        || dst_point.y >= dst_dimensions.h
        || src_region.top_left.x >= src_dimensions.w
        || src_region.top_left.y >= src_dimensions.h
    {
        return;
    }

    let rect_width = src_region.dimensions.w;
    let rect_height = src_region.dimensions.h;

    for y in 0..rect_height {
        for x in 0..rect_width {
            // Calculate source index
            let src_x = src_region.top_left.x + x;
            let src_y = src_region.top_left.y + y;

            // TODO: Find out whether this check actually works as expected
            // Ensure the source coordinates are within the image bounds
            if src_x >= src_dimensions.w || src_y >= src_dimensions.h {
                continue;
            }

            let src_index = (src_y * src_dimensions.w + src_x) as usize;

            // Calculate destination index
            let dest_x = dst_point.x + x;
            let dest_y = dst_point.y + y;

            // TODO: Find out whether this check actually works as expected
            // Ensure the destination coordinates are within the screen bounds
            if dest_x >= dst_dimensions.w || dest_y >= dst_dimensions.h {
                continue;
            }

            let dest_index = (dest_y * dst_dimensions.w + dest_x) as usize;

            // Copy the pixel
            dst_buf[dest_index] = src_buf[src_index];
        }
    }
}

/// Copies image data from a source buffer to a destination buffer, within specified areas and dimensions.
/// Pixels matching the `transparency_color` are ignored and not copied to the destination
///
/// # Parameters
///
/// - `dst_buf`: The destination buffer for image data.
/// - `dst_dimensions`: Dimensions of the destination buffer.
/// - `dest_point`: A `Point` specifying the starting point in the destination buffer where the image data will be copied to.
///
/// - `src_buf`: The source buffer
/// - `src_dimensions`: Dimensions of the destination buffer.
/// - `src_region`: A `RectArea` specifying the rectangular area in the source buffer to copy.
/// - `src_region`: A `RectArea` specifying the rectangular area in the source buffer to copy.
/// - `transparency_color`: Pixels of this color are considered transparent and won't be copied to the destination.
///
pub fn copy_non_transparent(
    dst_buf: &mut [u32],
    dst_dimensions: &Dimensions2d,
    dst_point: &Point,
    src_buf: &[u32],
    src_dimensions: &Dimensions2d,
    src_region: &RectArea,
    transparency_color: u32,
) {
    // Ensure the dimensions and starting points are within bounds
    if dst_point.x >= dst_dimensions.w
        || dst_point.y >= dst_dimensions.h
        || src_region.top_left.x >= src_dimensions.w
        || src_region.top_left.y >= src_dimensions.h
    {
        return;
    }

    let rect_width = src_region.dimensions.w;
    let rect_height = src_region.dimensions.h;

    for y in 0..rect_height {
        for x in 0..rect_width {
            // Calculate source index
            let src_x = src_region.top_left.x + x;
            let src_y = src_region.top_left.y + y;

            // TODO: Find out whether this check actually works as expected
            // Ensure the source coordinates are within the image bounds
            if src_x >= src_dimensions.w || src_y >= src_dimensions.h {
                continue;
            }

            let src_index = (src_y * src_dimensions.w + src_x) as usize;

            // Calculate destination index
            let dest_x = dst_point.x + x;
            let dest_y = dst_point.y + y;

            // TODO: Find out whether this check actually works as expected
            // Ensure the destination coordinates are within the screen bounds
            if dest_x >= dst_dimensions.w || dest_y >= dst_dimensions.h {
                continue;
            }

            let dest_index = (dest_y * dst_dimensions.w + dest_x) as usize;

            // Copy the pixel if it is not of a transparent color
             if src_buf[src_index] != transparency_color {
                dst_buf[dest_index] = src_buf[src_index];
            }
        }
    }
}

/// Copies image data from a source buffer to a destination buffer, within specified areas and dimensions.
/// Pixels matching the `transparency_color` are ignored and not copied to the destination.
/// Each pixel can be transformed
///
/// # Parameters
///
/// - `dst_buf`: The destination buffer for image data.
/// - `dst_dimensions`: Dimensions of the destination buffer.
/// - `dest_point`: A `Point` specifying the starting point in the destination buffer where the image data will be copied to.
///
/// - `src_buf`: The source buffer
/// - `src_dimensions`: Dimensions of the destination buffer.
/// - `src_region`: A `RectArea` specifying the rectangular area in the source buffer to copy.
/// - `src_region`: A `RectArea` specifying the rectangular area in the source buffer to copy.
/// - `transparency_color`: Pixels of this color are considered transparent and won't be copied to the destination.
/// - `color_transformer`: your custom function for modifying a color of each copied pixel @See `PixelColorTransformerFn`
///
pub fn copy_with_transform(
    dst_buf: &mut [u32],
    dst_dimensions: &Dimensions2d,
    dst_point: &Point,
    src_buf: &[u32],
    src_dimensions: &Dimensions2d,
    src_region: &RectArea,
    transparency_color: u32,
    color_transformer: PixelColorTransformerFn
) {
    // Ensure the dimensions and starting points are within bounds
    if dst_point.x >= dst_dimensions.w
        || dst_point.y >= dst_dimensions.h
        || src_region.top_left.x >= src_dimensions.w
        || src_region.top_left.y >= src_dimensions.h
    {
        return;
    }

    let rect_width = src_region.dimensions.w;
    let rect_height = src_region.dimensions.h;

    for y in 0..rect_height {
        for x in 0..rect_width {
            // Calculate source index
            let src_x = src_region.top_left.x + x;
            let src_y = src_region.top_left.y + y;

            // TODO: Find out whether this check actually works as expected
            // Ensure the source coordinates are within the image bounds
            if src_x >= src_dimensions.w || src_y >= src_dimensions.h {
                continue;
            }

            let src_index = (src_y * src_dimensions.w + src_x) as usize;

            // Calculate destination index
            let dest_x = dst_point.x + x;
            let dest_y = dst_point.y + y;

            // TODO: Find out whether this check actually works as expected
            // Ensure the destination coordinates are within the screen bounds
            if dest_x >= dst_dimensions.w || dest_y >= dst_dimensions.h {
                continue;
            }

            let dest_index = (dest_y * dst_dimensions.w + dest_x) as usize;

            // Copy the pixel if it is not of a transparent color
            if src_buf[src_index] != transparency_color {
                // Apply a provided transformation function to modify the original pixel color
                dst_buf[dest_index] = color_transformer(src_buf[src_index], x, y);
            }
        }
    }
}


pub struct ImageDataCopyProps {
    pub transparency_color: Option<u32>,
    pub replacement_color: Option<u32>,
    pub color_transformer: Option<PixelColorTransformerFn>,
}


pub fn copy_universal(
    dst_buf: &mut [u32],
    dst_dimensions: &Dimensions2d,
    dst_point: &Point,
    src_buf: &[u32],
    src_dimensions: &Dimensions2d,
    src_region: &RectArea,
    properties: &ImageDataCopyProps
) {
    // Ensure the dimensions and starting points are within bounds
    if dst_point.x >= dst_dimensions.w
        || dst_point.y >= dst_dimensions.h
        || src_region.top_left.x >= src_dimensions.w
        || src_region.top_left.y >= src_dimensions.h
    {
        return;
    }

    let rect_width = src_region.dimensions.w;
    let rect_height = src_region.dimensions.h;

    for y in 0..rect_height {
        for x in 0..rect_width {
            // Calculate source index
            let src_x = src_region.top_left.x + x;
            let src_y = src_region.top_left.y + y;

            // TODO: Find out whether this check actually works as expected
            // Ensure the source coordinates are within the image bounds
            if src_x >= src_dimensions.w || src_y >= src_dimensions.h {
                continue;
            }

            let src_index = (src_y * src_dimensions.w + src_x) as usize;

            // Calculate destination index
            let dest_x = dst_point.x + x;
            let dest_y = dst_point.y + y;

            // TODO: Find out whether this check actually works as expected
            // Ensure the destination coordinates are within the screen bounds
            if dest_x >= dst_dimensions.w || dest_y >= dst_dimensions.h {
                continue;
            }

            let dest_index = (dest_y * dst_dimensions.w + dest_x) as usize;
            /*
            if properties.transparency_color.is_some() {

            }


            // Copy the pixel if it is not of a transparent color
            if src_buf[src_index] != transparency_color {
                // Apply a provided transformation function to modify the original pixel color
                dst_buf[dest_index] = color_transformer(src_buf[src_index], x, y);
            }

             */
        }
    }
}