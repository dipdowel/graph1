use crate::primitives::primitives::{Dimensions2d, RectArea};
/// Applies a Gaussian blur to a buffer of pixels within a specified area or the entire buffer if no area is specified.
///
/// # Arguments
///
/// * `buffer` - A mutable slice of u32 values representing the pixel buffer. Each pixel is a 0RGB value.
/// * `buffer_dimensions` - The dimensions of the pixel buffer.
/// * `kernel_size` - The size of the Gaussian kernel, must be an odd number. If even number provided, it gets increased by 1.
/// * `sigma` - The standard deviation for the Gaussian function.
/// * `area` - An optional area within the buffer where the Gaussian blur should be applied.
///
/// # Panics
///
/// Panics if the kernel_size is greater than the dimensions of the buffer.
pub fn gaussan(
    buffer: &mut [u32],
    buffer_dimensions: &Dimensions2d,
    kernel_size: u8,
    sigma: f32,
    area: Option<&RectArea>
) {


    // The kernel size must always be odd, let's not panic if it's even
    // and simply increase it by 1 :P
    let kernel_size = match kernel_size % 2 {
        0 => kernel_size + 1,
        _ => kernel_size
    };

    // assert_eq!(kernel_size % 2, 1, "Kernel size must be odd");

    let half_size = kernel_size as isize / 2;
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

    let mut kernel = vec![vec![0.0; kernel_size as usize]; kernel_size as usize];
    let mut sum = 0.0;

    // Generate Gaussian kernel
    for x in 0..kernel_size as isize {
        for y in 0..kernel_size as isize {
            let x_dist = x - half_size;
            let y_dist = y - half_size;
            kernel[x as usize][y as usize] = (-((x_dist * x_dist + y_dist * y_dist) as f32) / (2.0 * sigma * sigma)).exp() / (2.0 * std::f32::consts::PI * sigma * sigma);
            sum += kernel[x as usize][y as usize];
        }
    }

    // Normalize the kernel
    for x in 0..kernel_size as isize {
        for y in 0..kernel_size as isize {
            kernel[x as usize][y as usize] /= sum;
        }
    }

    // Create a copy of the buffer to read from
    let original_buffer = buffer.to_vec();

    // Apply Gaussian blur within the specified area or the entire buffer
    for y in area_top_left_y as isize..(area_top_left_y + area_height) as isize {
        for x in area_top_left_x as isize..(area_top_left_x + area_width) as isize {
            let mut r = 0.0;
            let mut g = 0.0;
            let mut b = 0.0;
            for ky in -half_size..=half_size {
                for kx in -half_size..=half_size {
                    let nx = x + kx;
                    let ny = y + ky;
                    if nx >= 0 && ny >= 0 && nx < width && ny < height {
                        let pixel = original_buffer[(ny * width + nx) as usize];
                        let weight = kernel[(kx + half_size) as usize][(ky + half_size) as usize];
                        r += weight * ((pixel >> 16) & 0xFF) as f32;
                        g += weight * ((pixel >> 8) & 0xFF) as f32;
                        b += weight * (pixel & 0xFF) as f32;
                    }
                }
            }
            let r = r.min(255.0) as u32;
            let g = g.min(255.0) as u32;
            let b = b.min(255.0) as u32;
            buffer[(y * width + x) as usize] = (r << 16) | (g << 8) | b;
        }
    }
}
