use crate::primitives::helper_types::PixelColorTransformerFn;
use crate::primitives::plane::{Dimensions2d, RectArea};
use crate::primitives::point::Point;
use crate::utils::math::is_power_of_two;
use std::ptr;

// TODO:
// TODO:
// TODO: There is too much code in this file.
// TODO: Please refactor! Probably extract into multiple new files.
// TODO:
// TODO:

/// Additional options for modifying the copied image data (pixels).
pub struct ImageDataCopyProps<'a> {
    /// An optional color used to specify transparency.
    ///
    /// If provided, any pixel matching this color will be considered transparent
    /// and won't be copied to the destination buffer
    pub transparency_color: Option<u32>,

    /// An optional fill color to repaint the copied pixels.
    ///
    /// If provided, each copied pixel will end up in the destination in this color
    /// If `fill_color` is present, then `color_transformer` is ignored and not applied.
    pub fill_color: Option<u32>,

    /// An optional custom function for transforming pixel colors.
    ///
    /// If provided, this function will be applied to each pixel's color value
    /// to perform custom color transformations based on the original color, x and y of the pixel
    pub color_transformer: Option<PixelColorTransformerFn>,

    /// Any data that needs to be passed to `color_transformer()`
    pub data: Option<&'a Vec<u32>>,
}

/// Fast copying of image data without any transformations or checks (unsafe rust)
///
/// # Parameters
/// - `dst_buf`: The destination buffer for image data.
/// - `dst_buf_dimensions`: Dimensions of the destination buffer.
/// - `dst_point`: A `Point` specifying the starting point in the destination buffer where the image data will be copied to.
/// - `src_buf`: The source buffer.
/// - `src_buf_dimensions`: Dimensions of the source buffer.
/// - `src_region`: A `RectArea` specifying the rectangular area in the source buffer to copy.
pub fn copy_fast(
    dst_buf: &mut [u32],
    dst_buf_dimensions: &Dimensions2d,
    dst_point: &Point,
    src_buf: &[u32],
    src_buf_dimensions: &Dimensions2d,
    src_region: &RectArea,
) {
    // Calculate the effective width and height of the rectangle to be copied,
    // ensuring they do not exceed the source buffer's dimensions
    let rect_width = src_region
        .dimensions
        .w
        .min(src_buf_dimensions.w - src_region.top_left.x);
    let rect_height = src_region
        .dimensions
        .h
        .min(src_buf_dimensions.h - src_region.top_left.y);

    for y in 0..rect_height {
        let src_start =
            ((src_region.top_left.y + y) * src_buf_dimensions.w + src_region.top_left.x) as usize;
        let dest_start = ((dst_point.y + y) * dst_buf_dimensions.w + dst_point.x) as usize;
        let copy_len = rect_width as usize;

        unsafe {
            // Perform the copy for the current row
            ptr::copy_nonoverlapping(
                src_buf.as_ptr().add(src_start),
                dst_buf.as_mut_ptr().add(dest_start),
                copy_len,
            );
        }
    }
}

/// Fast copying of image data without any transformations or checks (unsafe rust)
///
/// # Parameters
/// - `dst_buf`: The destination buffer for image data.
/// - `dst_buf_dimensions`: Dimensions of the destination buffer.
/// - `dst_points`: An array of `Point` specifying the starting points in the destination buffer for each copy of the image data.
/// - `src_buf`: The source buffer.
/// - `src_buf_dimensions`: Dimensions of the source buffer.
/// - `src_region`: A `RectArea` specifying the rectangular area in the source buffer to copy.
pub fn copy_fast_multi_dst(
    dst_buf: &mut [u32],
    dst_buf_dimensions: &Dimensions2d,
    dst_points: &[Point],
    src_buf: &[u32],
    src_buf_dimensions: &Dimensions2d,
    src_region: &RectArea,
) {
    // Calculate the effective width and height of the rectangle to be copied,
    // ensuring they do not exceed the buffer's dimensions
    let rect_width = src_region
        .dimensions
        .w
        .min(src_buf_dimensions.w - src_region.top_left.x);
    let rect_height = src_region
        .dimensions
        .h
        .min(src_buf_dimensions.h - src_region.top_left.y);

    for y in 0..rect_height {
        let src_start =
            ((src_region.top_left.y + y) * src_buf_dimensions.w + src_region.top_left.x) as usize;
        let copy_len = rect_width as usize;

        for dst_point in dst_points {
            let dest_start = ((dst_point.y + y) * dst_buf_dimensions.w + dst_point.x) as usize;

            unsafe {
                // Perform the copy for the current row to each destination point
                ptr::copy(
                    src_buf.as_ptr().add(src_start),
                    dst_buf.as_mut_ptr().add(dest_start),
                    copy_len,
                );
            }
        }
    }
}

/// Fast copying of image data within the same buffer without any transformations or checks (unsafe rust)
///
/// # Parameters
/// - `buffer`: The buffer for image data.
/// - `buffer_dimensions`: Dimensions of the buffer.
/// - `dst_point`: A `Point` specifying the starting point in the buffer where the image data will be copied to.
/// - `src_region`: A `RectArea` specifying the rectangular area in the buffer to copy.
pub fn copy_fast_within_buffer(
    buffer: &mut [u32],
    buffer_dimensions: &Dimensions2d,
    dst_point: &Point,
    src_region: &RectArea,
) {
    // Calculate the effective width and height of the rectangle to be copied,
    // ensuring they do not exceed the buffer's dimensions
    let rect_width = src_region
        .dimensions
        .w
        .min(buffer_dimensions.w - src_region.top_left.x);
    let rect_height = src_region
        .dimensions
        .h
        .min(buffer_dimensions.h - src_region.top_left.y);

    for y in 0..rect_height {
        let src_start =
            ((src_region.top_left.y + y) * buffer_dimensions.w + src_region.top_left.x) as usize;
        let dest_start = ((dst_point.y + y) * buffer_dimensions.w + dst_point.x) as usize;
        let copy_len = rect_width as usize;

        unsafe {
            // Perform the copy for the current row
            ptr::copy(
                buffer.as_ptr().add(src_start),
                buffer.as_mut_ptr().add(dest_start),
                copy_len,
            );
        }
    }
}

/// Fast copying of image data within the same buffer to multiple destinations.
///
/// # Parameters
/// - `buffer`: The buffer for image data.
/// - `buffer_dimensions`: Dimensions of the buffer.
/// - `dst_points`: A vector of `Point` specifying the starting points in the buffer where the image data will be copied to.
/// - `src_region`: A `RectArea` specifying the rectangular area in the buffer to copy.
pub fn copy_fast_within_buffer_multi_dst(
    buffer: &mut [u32],
    buffer_dimensions: &Dimensions2d,
    dst_points: &[Point],
    src_region: &RectArea,
) {
    // Calculate the effective width and height of the rectangle to be copied,
    // ensuring they do not exceed the buffer's dimensions
    let rect_width = src_region
        .dimensions
        .w
        .min(buffer_dimensions.w - src_region.top_left.x);
    let rect_height = src_region
        .dimensions
        .h
        .min(buffer_dimensions.h - src_region.top_left.y);

    for y in 0..rect_height {
        let src_start =
            ((src_region.top_left.y + y) * buffer_dimensions.w + src_region.top_left.x) as usize;
        let copy_len = rect_width as usize;

        for dst_point in dst_points {
            let dest_start = ((dst_point.y + y) * buffer_dimensions.w + dst_point.x) as usize;

            unsafe {
                // Perform the copy for the current row to each destination point
                ptr::copy(
                    buffer.as_ptr().add(src_start),
                    buffer.as_mut_ptr().add(dest_start),
                    copy_len,
                );
            }
        }
    }
}

/// Copies image data from a source buffer to a destination buffer, within specified areas and dimensions.
///
/// # Parameters
///
/// - `dst_buf`: The destination buffer for image data.
/// - `dst_buf_dimensions`: Dimensions of the destination buffer.
/// - `dest_point`: A `Point` specifying the starting point in the destination buffer where the image data will be copied to.
///
/// - `src_buf`: The source buffer
/// - `src_buf_dimensions`: Dimensions of the destination buffer.
/// - `src_region`: A `RectArea` specifying the rectangular area in the source buffer to copy.
/// - `properties` options for modifying the copied pixels. @See `ImageDataCopyProps` for details
/// # Rules of how `properties` are applied
///  1. Pixels matching the value of `transparency_color` are not copied to the destination.
///  2. If no `transparency_color` provided, then only the `color_transformer` function will be applied
///     to each copied filter (if provided). `fill_color` is ignored.
///  3. If `transparency_color` is provided and both `fill_color` and `color_transformer` are provided,
///     then `fill_color` is applied to the copied pixels and `color_transformer` is ignored.
///
/// TODO: Add multiple destination points!
/// TODO: Add multiple destination points!
/// TODO: Add multiple destination points!
/// TODO: Add multiple destination points!
pub fn copy(
    dst_buf: &mut [u32],
    dst_buf_dimensions: &Dimensions2d,
    dst_point: &Point,
    src_buf: &[u32],
    src_buf_dimensions: &Dimensions2d,
    src_region: &RectArea,
    properties: Option<&ImageDataCopyProps>,
) {
    // Ensure the dimensions and starting points are within bounds
    if dst_point.x >= dst_buf_dimensions.w
        || dst_point.y >= dst_buf_dimensions.h
        || src_region.top_left.x >= src_buf_dimensions.w
        || src_region.top_left.y >= src_buf_dimensions.h
    {
        return;
    }

    // Calculate the effective width and height of the rectangle to be copied,
    // ensuring they do not exceed the source buffer's dimensions
    let rect_width = src_region
        .dimensions
        .w
        .min(src_buf_dimensions.w - src_region.top_left.x);
    let rect_height = src_region
        .dimensions
        .h
        .min(src_buf_dimensions.h - src_region.top_left.y);

    if properties.is_none() {
        // Call the fast path function if no properties are provided
        copy_fast(
            dst_buf,
            dst_buf_dimensions,
            dst_point,
            src_buf,
            src_buf_dimensions,
            src_region,
        );
        return;
    }

    // Extract properties or provide default values if none are provided
    let props = properties.unwrap_or(&ImageDataCopyProps {
        color_transformer: None,
        fill_color: None,
        transparency_color: None,
        data: None,
    });

    // Determine the transparency color (default to 0 if not provided)
    let transparency_color = props.transparency_color.unwrap_or(0);

    // Determine the fill color (default to 0 if not provided)
    let fill_color = props.fill_color.unwrap_or(0);

    // Determine the color transformer function (default to an identity function if not provided)
    let color_transformer = props
        .color_transformer
        .unwrap_or(|color, _, _, _, _, _| color);

    // Use unsafe block to allow unchecked memory access for performance
    unsafe {
        // Loop through each pixel in the specified rectangle area
        for y in 0..rect_height {
            for x in 0..rect_width {
                // Calculate the source coordinates for the current pixel
                let src_x = src_region.top_left.x + x;
                let src_y = src_region.top_left.y + y;

                // Calculate the source index in the buffer
                let src_index = (src_y * src_buf_dimensions.w + src_x) as usize;

                // Calculate the destination coordinates for the current pixel
                let dest_x = dst_point.x + x;
                let dest_y = dst_point.y + y;

                // Ensure the destination coordinates are within the destination buffer bounds
                if dest_x >= dst_buf_dimensions.w || dest_y >= dst_buf_dimensions.h {
                    continue;
                }

                // Calculate the destination index in the buffer
                let dest_index = (dest_y * dst_buf_dimensions.w + dest_x) as usize;

                // Read the source pixel color
                let src_pixel = *src_buf.get_unchecked(src_index);

                // If a transparency color is specified and the current pixel matches it, skip copying
                if props.transparency_color.is_some() && src_pixel == transparency_color {
                    continue;
                }

                // Determine the final pixel color to be copied to the destination buffer
                // Priority: fill color > color transformer > original source color
                let dest_pixel = if props.fill_color.is_some() {
                    fill_color
                } else {
                    color_transformer(src_pixel, x, y, rect_width, rect_height, props.data)
                };

                // Write the final pixel color to the destination buffer
                *dst_buf.get_unchecked_mut(dest_index) = dest_pixel;
            }
        }
    }
}

/// Copies image data within the same buffer, within specified areas and dimensions.
///
/// # Parameters
///
/// - `buffer`: The buffer for image data, which acts both as the source and destination.
/// - `buffer_dimensions`: Dimensions of the buffer.
/// - `src_region`: A `RectArea` specifying the rectangular area in the buffer to copy from.
/// - `dst_point`: A `Point` specifying the starting point in the buffer where the image data will be copied to.
/// - `properties`: Options for modifying the copied pixels. See `ImageDataCopyProps` for details.
///
/// # Rules of how `properties` are applied
///  1. Pixels matching the value of `transparency_color` are not copied to the destination.
///  2. If no `transparency_color` is provided, then only the `color_transformer` function will be applied
///     to each copied filter (if provided). `fill_color` is ignored.
///  3. If `transparency_color` is provided and both `fill_color` and `color_transformer` are provided,
///     then `fill_color` is applied to the copied pixels and `color_transformer` is ignored.
///
pub fn copy_within_buffer(
    buffer: &mut [u32],
    buffer_dimensions: &Dimensions2d,
    dst_point: &Point,
    src_region: &RectArea,
    properties: Option<&ImageDataCopyProps>,
) {
    // Ensure the dimensions and starting points are within bounds
    if dst_point.x >= buffer_dimensions.w
        || dst_point.y >= buffer_dimensions.h
        || src_region.top_left.x >= buffer_dimensions.w
        || src_region.top_left.y >= buffer_dimensions.h
    {
        return;
    }

    // Calculate the effective width and height of the rectangle to be copied,
    // ensuring they do not exceed the buffer's dimensions
    let rect_width = src_region
        .dimensions
        .w
        .min(buffer_dimensions.w - src_region.top_left.x);
    let rect_height = src_region
        .dimensions
        .h
        .min(buffer_dimensions.h - src_region.top_left.y);

    // Extract properties or provide default values if none are provided
    let props = properties.unwrap_or(&ImageDataCopyProps {
        color_transformer: None,
        fill_color: None,
        transparency_color: None,
        data: None,
    });

    // Determine the transparency color (default to 0 if not provided)
    let transparency_color = props.transparency_color.unwrap_or(0);

    // Determine the fill color (default to 0 if not provided)
    let fill_color = props.fill_color.unwrap_or(0);

    // Determine the color transformer function (default to an identity function if not provided)
    let color_transformer = props
        .color_transformer
        .unwrap_or(|color, _, _, _, _, _| color);

    // Use unsafe block to allow unchecked memory access for performance
    unsafe {
        // Loop through each pixel in the specified rectangle area
        for y in 0..rect_height {
            for x in 0..rect_width {
                // Calculate the source coordinates for the current pixel
                let src_x = src_region.top_left.x + x;
                let src_y = src_region.top_left.y + y;

                // Calculate the source index in the buffer
                let src_index = (src_y * buffer_dimensions.w + src_x) as usize;

                // Calculate the destination coordinates for the current pixel
                let dest_x = dst_point.x + x;
                let dest_y = dst_point.y + y;

                // Ensure the destination coordinates are within the buffer bounds
                if dest_x >= buffer_dimensions.w || dest_y >= buffer_dimensions.h {
                    continue;
                }

                // Calculate the destination index in the buffer
                let dest_index = (dest_y * buffer_dimensions.w + dest_x) as usize;

                // Read the source pixel color
                let src_pixel = *buffer.get_unchecked(src_index);

                // If a transparency color is specified and the current pixel matches it, skip copying
                if props.transparency_color.is_some() && src_pixel == transparency_color {
                    continue;
                }

                // Determine the final pixel color to be copied to the destination buffer
                // Priority: fill color > color transformer > original source color
                let dest_pixel = if props.fill_color.is_some() {
                    fill_color
                } else {
                    color_transformer(src_pixel, x, y, rect_width, rect_height, props.data)
                };

                // Write the final pixel color to the destination buffer
                *buffer.get_unchecked_mut(dest_index) = dest_pixel;
            }
        }
    }
}

/// Scales up a given image data and saves the result to a destination buffer
///
/// # Parameters
///
/// - `dst_buf`: The destination buffer for image data.
/// - `dst_buf_dimensions`: Dimensions of the destination buffer.
/// - `dest_point`: A `Point` specifying the starting point in the destination buffer where the image data will be copied to.
///
/// - `src_buf`: The source buffer
/// - `src_buf_dimensions`: Dimensions of the source buffer.
/// - `src_region`: A `RectArea` specifying the rectangular area in the source buffer to copy.
/// - `scale_factor` How much to scale the image up, must be a power of two, otherwise the function panics.
pub fn scale_up(
    dst_buf: &mut [u32],
    dst_buf_dimensions: &Dimensions2d,
    dst_point: &Point,
    src_buf: &[u32],
    src_buf_dimensions: &Dimensions2d,
    src_region: &RectArea,
    scale_factor: u8,
) {
    let scale_factor: u32 = scale_factor as u32;
    if !is_power_of_two(scale_factor) {
        panic!("`scale_factor` must be a power of 2");
    }

    for y in 0..src_region.dimensions.h {
        for x in 0..src_region.dimensions.w {
            let input_x = src_region.top_left.x + x;
            let input_y = src_region.top_left.y + y;
            if input_x < src_buf_dimensions.w && input_y < src_buf_dimensions.h {
                let pixel = src_buf[(input_y * src_buf_dimensions.w + input_x) as usize];
                for dy in 0..scale_factor {
                    for dx in 0..scale_factor {
                        let out_x = dst_point.x + x * scale_factor + dx;
                        let out_y = dst_point.y + y * scale_factor + dy;
                        if out_x < dst_buf_dimensions.w && out_y < dst_buf_dimensions.h {
                            dst_buf[(out_y * dst_buf_dimensions.w + out_x) as usize] = pixel;
                        }
                    }
                }
            }
        }
    }
}
