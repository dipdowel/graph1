use crate::primitives::primitives::{Dimensions2d, RectArea};



/// Applies Fast Approximate Anti-Aliasing (FXAA) to a buffer of pixels within a specified area or the entire buffer if no area is specified.
///
/// # Arguments
///
/// * `buffer` - A mutable slice of u32 values representing the pixel buffer. Each pixel is a 0RGB value.
/// * `buffer_dimensions` - The dimensions of the pixel buffer.
/// * `area` - An optional area within the buffer where FXAA should be applied.
pub fn fxaa(
    buffer: &mut [u32],
    buffer_dimensions: &Dimensions2d,
    area: Option<RectArea>
) {
    let width = buffer_dimensions.w as isize;
    let height = buffer_dimensions.h as isize;

    // Adjust area to fit within buffer dimensions
    let (area_top_left_x, area_top_left_y, area_width, area_height) = if let Some(area) = area {
        let area_top_left_x = area.top_left.x.min(buffer_dimensions.w);
        let area_top_left_y = area.top_left.y.min(buffer_dimensions.h);
        let area_width = (area_top_left_x + area.dimensions.w).min(buffer_dimensions.w) - area_top_left_x;
        let area_height = (area_top_left_y + area.dimensions.h).min(buffer_dimensions.h) - area_top_left_y;
        (area_top_left_x, area_top_left_y, area_width, area_height)
    } else {
        (0, 0, buffer_dimensions.w, buffer_dimensions.h)
    };

    // Create a copy of the buffer to read from
    let original_buffer = buffer.to_vec();

    // Apply FXAA within the specified area or the entire buffer
    for y in area_top_left_y as isize..(area_top_left_y + area_height) as isize {
        for x in area_top_left_x as isize..(area_top_left_x + area_width) as isize {
            let index = (y * width + x) as usize;
            let pixel = original_buffer[index];

            // Convert pixel to luminance
            let luma = |p: u32| -> f32 {
                let r = ((p >> 16) & 0xFF) as f32;
                let g = ((p >> 8) & 0xFF) as f32;
                let b = (p & 0xFF) as f32;
                0.299 * r + 0.587 * g + 0.114 * b
            };

            // Get neighboring pixels
            let get_pixel = |dx: isize, dy: isize| -> u32 {
                let nx = (x + dx).clamp(0, width - 1);
                let ny = (y + dy).clamp(0, height - 1);
                original_buffer[(ny * width + nx) as usize]
            };

            let luma_center = luma(pixel);
            let luma_left = luma(get_pixel(-1, 0));
            let luma_right = luma(get_pixel(1, 0));
            let luma_up = luma(get_pixel(0, -1));
            let luma_down = luma(get_pixel(0, 1));

            let edge_horizontal = (luma_left - luma_center).abs() + (luma_right - luma_center).abs();
            let edge_vertical = (luma_up - luma_center).abs() + (luma_down - luma_center).abs();

            if edge_horizontal > edge_vertical {
                let luma_blend = (luma_left + luma_right) / 2.0;
                let r = (pixel >> 16) & 0xFF;
                let g = (pixel >> 8) & 0xFF;
                let b = pixel & 0xFF;
                let r = (r as f32 * 0.5 + luma_blend * 0.5) as u32;
                let g = (g as f32 * 0.5 + luma_blend * 0.5) as u32;
                let b = (b as f32 * 0.5 + luma_blend * 0.5) as u32;
                buffer[index] = (r << 16) | (g << 8) | b;
            } else {
                let luma_blend = (luma_up + luma_down) / 2.0;
                let r = (pixel >> 16) & 0xFF;
                let g = (pixel >> 8) & 0xFF;
                let b = pixel & 0xFF;
                let r = (r as f32 * 0.5 + luma_blend * 0.5) as u32;
                let g = (g as f32 * 0.5 + luma_blend * 0.5) as u32;
                let b = (b as f32 * 0.5 + luma_blend * 0.5) as u32;
                buffer[index] = (r << 16) | (g << 8) | b;
            }
        }
    }
}

