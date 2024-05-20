use crate::graph1::primitives::primitives::{
    Dimensions2d, PixelColorTransformerFn, Point, RectArea,
};
use crate::graph1::utils::misc::is_power_of_two;

/// Additional options for modifying the copied image data (pixels).
pub struct ImageDataCopyProps {
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

    let rect_width = src_region.dimensions.w;
    let rect_height = src_region.dimensions.h;

    //--------------------------------------------------------------------------------------------------
    //   FIXME: the way of extracting properties used below is somewhat dumb. Do it in a more idiomatic way!

    let props = properties.unwrap_or_else(|| &ImageDataCopyProps {
        color_transformer: None,
        fill_color: None,
        transparency_color: None,
    });

    // Figure out if transparency needs to be applied and for what color
    let check_transparency = props.transparency_color.is_some();
    let mut transparency_color: u32 = 0;
    if check_transparency {
        transparency_color = props.transparency_color.unwrap();
    }

    // Figure out if we need to set needs to be applied and for what color
    let check_fill_color = props.fill_color.is_some();
    let mut fill_color: u32 = 0;
    if check_fill_color {
        fill_color = props.fill_color.unwrap();
    }

    // Figure out if we need to execute `color_transformer` per pixel
    let check_color_transformer = props.color_transformer.is_some();
    let mut color_transformer: PixelColorTransformerFn =
        |color: u32, x: u32, y: u32, w: u32, h: u32| -> u32 { color };
    if check_color_transformer {
        color_transformer = props.color_transformer.unwrap();
    }
    //--------------------------------------------------------------------------------------------------

    for y in 0..rect_height {
        for x in 0..rect_width {
            // Calculate source index
            let src_x = src_region.top_left.x + x;
            let src_y = src_region.top_left.y + y;

            // Ensure the source coordinates are within the image bounds
            if src_x >= src_buf_dimensions.w || src_y >= src_buf_dimensions.h {
                continue;
            }

            let src_index = (src_y * src_buf_dimensions.w + src_x) as usize;

            // Calculate destination index
            let dest_x = dst_point.x + x;
            let dest_y = dst_point.y + y;

            // Ensure the destination coordinates are within the screen bounds
            if dest_x >= dst_buf_dimensions.w || dest_y >= dst_buf_dimensions.h {
                continue;
            }

            let dest_index = (dest_y * dst_buf_dimensions.w + dest_x) as usize;

            // No transparency. Only the color transformer makes sense. Fill color does not!
            if !check_transparency {
                // Color transformer: transform the color and copy the pixel
                if check_color_transformer {
                    dst_buf[dest_index] =
                        color_transformer(src_buf[src_index], x, y, rect_width, rect_height);
                    continue;
                }
                // No color transformer: just copy the pixel
                dst_buf[dest_index] = src_buf[src_index];
                continue;
            }

            // Transparency color is set and the current pixel is transparent,
            // skip the copying altogether!
            if check_transparency && src_buf[src_index] == transparency_color {
                continue;
            }

            // The pixel neither needs to be transformed nor filled, so just copy it
            if !check_fill_color && !check_color_transformer {
                dst_buf[dest_index] = src_buf[src_index];
                continue;
            }

            // Fill color has a higher priority than the transformer.
            // Simply replace the original pixel with the provided fill color and that's it
            if check_fill_color {
                dst_buf[dest_index] = fill_color;
                continue;
            }

            // Transformer has the lowest priority.
            // If no fill color is provided but the transformer is provided, then apply the transformer
            if check_color_transformer {
                dst_buf[dest_index] =
                    color_transformer(src_buf[src_index], x, y, rect_width, rect_height);
                continue;
            }

            panic!(
                "Image data copy function got confused with the input! \
            Please read the function inline documentation!"
            );
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
/// - `src_buf_dimensions`: Dimensions of the destination buffer.
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
