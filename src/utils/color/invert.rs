use crate::primitives::plane::{Dimensions2d, RectArea};
use crate::primitives::point::Point;
use std::cmp::min;
use std::thread;

/// Meant to be used as a thread, while parallelizing the drawing of a rectangle.
/// # Arguments
/// * `buf_slice` - A mutable slice of u32 values representing the pixel buffer. Each pixel is a RGBA value.
/// * `buf_dimensions` - The dimensions of the pixel buffer.
/// * `area` - The region of the buffer to transform.

pub fn invert_colors_thread(buf_slice: &mut [u32], buf_dimensions: &Dimensions2d, area: &RectArea) {
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
            let dst_pixel = buf_slice[dst_index];

            // Inverse a pixel and write it back to the destination buffer
            buf_slice[dst_index] = invert_pixel(dst_pixel);
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

/// Inverts the colors of the buffer in a specified area (multi-threaded).
/// Each pixel is assumed to be in RGBA format (32-bit u32).
pub fn invert_colors(
    buf: &mut [u32],
    buf_dimensions: &Dimensions2d,
    area: &RectArea,
    num_threads: usize,
) {
    if num_threads < 2 {
        if num_threads == 0 {
            return;
        }
        invert_colors_thread(buf, buf_dimensions, area);
        return;
    }

    let dst_point: Point = area.top_left;

    let dst_area_w = min(
        area.dimensions.w,
        buf_dimensions.w.saturating_sub(dst_point.x),
    );
    let dst_area_h = min(
        area.dimensions.h,
        buf_dimensions.h.saturating_sub(dst_point.y),
    );

    if dst_area_w == 0 || dst_area_h == 0 {
        return;
    }

    let line_length = buf_dimensions.w as usize;
    let total_lines = dst_area_h as usize;
    let start_idx = (dst_point.y * buf_dimensions.w + dst_point.x) as usize;
    let slice_height = total_lines * line_length;

    // Get mutable slice of the region to process
    let buf_slice = &mut buf[start_idx..start_idx + slice_height];

    let mut chunk_size = usize::div_ceil(slice_height, num_threads);

    if chunk_size < line_length {
        // Not worth threading
        invert_colors_thread(buf, buf_dimensions, area);
        return;
    }

    // Ensure chunk size is line-aligned
    chunk_size = (chunk_size / line_length) * line_length;

    let mut chunks: Vec<&mut [u32]> = buf_slice.chunks_mut(chunk_size).collect();

    thread::scope(|s| {
        for chunk in chunks.iter_mut() {
            s.spawn(move || {
                let lines = chunk.len() / line_length;
                for line in 0..lines {
                    let base = line * line_length;
                    for x in 0..dst_area_w as usize {
                        let idx = base + x;
                        chunk[idx] = invert_pixel(chunk[idx]);
                    }
                }
            });
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::plane::{Dimensions2d, RectArea};
    use crate::primitives::point::Point;

    #[test]
    fn test_invert_pixel_basic() {
        // Input: R=0x20, G=0x40, B=0x80, A=0xFF
        let input = 0x20_40_80_ff;
        let expected = 0xdf_bf_7f_ff;
        assert_eq!(invert_pixel(input), expected);
    }

    #[test]
    fn test_invert_colors_whole_buffer() {
        let mut buf = vec![0x11_22_33_ff, 0x44_55_66_ff, 0x77_88_99_ff, 0xaa_bb_cc_ff];

        let dimensions = Dimensions2d { w: 2, h: 2 };
        let area = RectArea {
            top_left: Point { x: 0, y: 0 },
            dimensions,
            color: None,
        };

        invert_colors(&mut buf, &dimensions, &area, 1);

        let expected: Vec<u32> = vec![
            invert_pixel(0x11_22_33_ff),
            invert_pixel(0x44_55_66_ff),
            invert_pixel(0x77_88_99_ff),
            invert_pixel(0xaa_bb_cc_ff),
        ];

        assert_eq!(buf, expected);
    }

    #[test]
    fn test_invert_colors_partial_area() {
        let mut buf = vec![
            0x00_00_00_ff,
            0x00_00_00_ff,
            0x00_00_00_ff,
            0x00_00_00_ff,
            0x11_22_33_ff,
            0x00_00_00_ff,
            0x00_00_00_ff,
            0x00_00_00_ff,
            0x00_00_00_ff,
        ];

        let dimensions = Dimensions2d { w: 3, h: 3 };
        let area = RectArea {
            top_left: Point { x: 1, y: 1 },
            dimensions: Dimensions2d { w: 1, h: 1 },
            color: None,
        };

        invert_colors(&mut buf, &dimensions, &area, 1);

        let expected_pixel = invert_pixel(0x11_22_33_ff);
        assert_eq!(buf[4], expected_pixel);

        // All others should remain black
        assert!(buf.iter().enumerate().all(|(i, &p)| {
            if i == 4 {
                p == expected_pixel
            } else {
                p == 0x00_00_00_ff
            }
        }));
    }

    #[test]
    fn test_invert_colors_clamping() {
        let mut buf = vec![0xaa_bb_cc_dd; 4]; // 2x2 buffer
        let dimensions = Dimensions2d { w: 2, h: 2 };
        let area = RectArea {
            top_left: Point { x: 1, y: 1 },
            dimensions: Dimensions2d { w: 5, h: 5 },
            color: None,
        };

        invert_colors(&mut buf, &dimensions, &area, 1);

        // Only bottom-right pixel should be affected
        assert_eq!(buf[3], invert_pixel(0xaa_bb_cc_dd));
        assert_eq!(buf[0], 0xaa_bb_cc_dd);
        assert_eq!(buf[1], 0xaa_bb_cc_dd);
        assert_eq!(buf[2], 0xaa_bb_cc_dd);
    }
}
