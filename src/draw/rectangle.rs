use crate::core::context::alpha::AlphaMethod;
use crate::core::context::GraphContext;
use crate::primitives::plane::RectArea;
use crate::utils::color::alpha::{blend_pixel_f32, blend_pixel_int};
use std::cmp::PartialEq;
use std::thread;

impl PartialEq for AlphaMethod {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (AlphaMethod::Int, AlphaMethod::Int) => true,
            (AlphaMethod::Float, AlphaMethod::Float) => true,
            _ => false,
        }
    }
}
/// A lower-level function that draws a part of a rectangle in a slice of the frame buffer.
/// Meant to be used in a thread, while parallelizing the drawing of a rectangle.
///
/// # Arguments
/// * `rectangle_slice` - A mutable slice of the frame buffer where a part of the rectangle will be drawn.
/// * `line_length` - The length of a horizontal line in the frame buffer (kind of like a scanline on a TV screen).
/// * `total_lines` - The total number of horizontal lines in the slice.
/// * `color_start` - The index of the first pixel to be colored in the 'scanline'
/// * `color_end` - The index of the last pixel to be colored in the 'scanline'
/// * `color` - The color to fill the rectangle with
/// * `alpha_method` - The method of alpha blending to use, or `None` for direct overwrite
fn draw_lines_of_rectangle_thread(
    rectangle_slice: &mut [u32],
    line_length: usize,
    total_lines: usize,
    color_start: usize,
    color_end: usize,
    color: u32,
    alpha_method: Option<AlphaMethod>,
) {
    match alpha_method {
        // No alpha blending — just overwrite pixels
        None => {
            for line in 0..total_lines {
                let base = line * line_length;
                for pixel in color_start..color_end {
                    rectangle_slice[base + pixel] = color;
                }
            }
        }
        // Integer-based alpha blending
        Some(AlphaMethod::Int) => {
            for line in 0..total_lines {
                let base = line * line_length;
                for pixel in color_start..color_end {
                    let idx = base + pixel;
                    rectangle_slice[idx] = blend_pixel_int(rectangle_slice[idx], color);
                }
            }
        }
        // Float-based alpha blending
        Some(AlphaMethod::Float) => {
            for line in 0..total_lines {
                let base = line * line_length;
                for pixel in color_start..color_end {
                    let idx = base + pixel;
                    rectangle_slice[idx] = blend_pixel_f32(rectangle_slice[idx], color);
                }
            }
        }
    }
}

/// Draws a rectangle with dimensions and filled with a color specified in the `RectArea` struct.
/// Multithreaded. Falls back to single-threaded rendering if threading is disabled or not feasible.
///
/// # Arguments
/// * `ctx` - The graph context
/// * `rect` - The rectangle to draw
pub fn filled<UserData>(ctx: &mut GraphContext<UserData>, rect: &RectArea) {
    // Do nothing if threading is not enabled
    if ctx.num_threads == 0 {
        return;
    }

    // Determine color: fallback to foreground if not set
    let color = rect.color.unwrap_or(ctx.win.foreground_color);

    // Determine if alpha blending is enabled
    let alpha_method = ctx.alpha.enabled.then_some(ctx.alpha.method);

    // Framebuffer and rectangle layout variables
    let line_length = ctx.win.w_usize;
    let total_lines = rect.dimensions.h as usize;
    let color_start = rect.top_left.x as usize;
    let color_end = (rect.top_left.x + rect.dimensions.w) as usize;

    // Slice start/end in framebuffer
    let first_line_start = (rect.top_left.y * ctx.win.w) as usize;
    let last_line_end = ((rect.top_left.y + rect.dimensions.h) * ctx.win.w) as usize;

    let rectangle_slice = &mut ctx.frame_buf[first_line_start..last_line_end];

    // Calculate how much work each thread should do
    let chunk_len = rectangle_slice.len();
    let mut chunk_size = usize::div_ceil(chunk_len, ctx.num_threads);

    // If chunks are smaller than a full line, it’s not worth threading
    let too_small_for_threads = chunk_size < line_length;
    if ctx.num_threads == 1 || too_small_for_threads {
        draw_lines_of_rectangle_thread(
            rectangle_slice,
            line_length,
            total_lines,
            color_start,
            color_end,
            color,
            alpha_method,
        );
        return;
    }

    // Ensure chunk size is line-aligned so each thread gets whole scanlines
    chunk_size = (chunk_size / line_length) * line_length;

    let mut chunks: Vec<&mut [u32]> = rectangle_slice.chunks_mut(chunk_size).collect();

    // Use thread::scope to safely spawn threads
    thread::scope(|s| {
        for chunk in chunks.iter_mut() {
            let lines = chunk.len() / line_length;

            s.spawn(move || {
                draw_lines_of_rectangle_thread(
                    chunk,
                    line_length,
                    lines,
                    color_start,
                    color_end,
                    color,
                    alpha_method,
                );
            });
        }
    }); // The scope for the scoped threads ends here. All the threads are expected to be joined automagically at this point.

}
