use std::collections::VecDeque;
use crate::primitives::Pixel;
use crate::primitives::plane::Dimensions2d;

/// A low-level fill tool.
/// Unsafely fills a buffer with a given color
/// This should be faster `draw::rectangle::fill()`
/// TODO: check the claim about the speed above!!!
/// TODO: check the claim about the speed above!!!
/// TODO: check the claim about the speed above!!!
pub fn buffer(buf_view: &mut [u32], color: u32) {
    unsafe {
        // Obtain a raw pointer
        let buffer_ptr = buf_view.as_mut_ptr();

        // Calculate the end pointer for our loop. This is safe because we are not dereferencing the pointer yet.
        let end_ptr = buffer_ptr.add(buf_view.len());

        // Initialize a mutable pointer to iterate through the buffer.
        let mut current_ptr = buffer_ptr;

        while current_ptr < end_ptr {
            // Directly write to the memory location pointed to by current_ptr.
            *current_ptr = color;

            // Advance the pointer.
            current_ptr = current_ptr.add(1);
        }
    }
}


/// Flood fills a shape with a color and starting at a position specified by `start`
pub fn flood(
    buf: &mut [u32],
    buf_dimensions: &Dimensions2d,
    start_pixel: &Pixel,
) {
    // Buffer dimensions
    let width = buf_dimensions.w;
    let height = buf_dimensions.h;

    // Get the color of the pixel where we start the flood fill
    let initial_color = buf[(start_pixel.y * width + start_pixel.x) as usize];

    // New color to be applied to all the pixels with `initial_color` inside the shape
    let fill_color = start_pixel.color;

    // Nothing to do if the initial and the fill colors match.
    if initial_color == fill_color {
        return;
    }

    // Queue to manage pixels to be filled
    let mut queue = VecDeque::new();
    // Add the starting pixel to the queue
    queue.push_back((start_pixel.x, start_pixel.y));

    // Process the queue until it's empty
    while let Some((x, y)) = queue.pop_front() {
        let index = (y * width + x) as usize;

        // skip attempts to fill outside the screen
        if x > width || y > height {
            continue;
        }

        // Check if the current pixel has the initial color
        if buf[index] == initial_color {
            // Change the color of the current pixel to the fill color
            buf[index] = fill_color;

            // Add neighboring pixels to the queue
            // Left neighbor
            if x > 0 {
                queue.push_back((x - 1, y));
            }
            // Right neighbor
            if x < width - 1 {
                queue.push_back((x + 1, y));
            }
            // Top neighbor
            if y > 0 {
                queue.push_back((x, y - 1));
            }
            // Bottom neighbor
            if y < height - 1 {
                queue.push_back((x, y + 1));
            }
        }
    }
}