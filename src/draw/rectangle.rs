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
/// # Arguments
/// * `rectangle_slice` - A mutable slice of the frame buffer where a part of the rectangle will be drawn.
/// * `line_length` - The length of a horizontal line in the frame buffer (kind of like a scanline on a TV screen).
/// * `total_lines` - The total number of horizontal lines in the slice.
/// * `color_start` - The index of the first pixel to be colored in the 'scanline'
/// * `color_end` - The index of the last pixel to be colored in the 'scanline'
/// * `color` - The color to fill the rectangle with
/// * `alpha_method` - The method of alpha blending to use

fn draw_lines_of_rectangle_thread(
    rectangle_slice: &mut [u32],
    line_length: usize,
    total_lines: usize,
    color_start: usize,
    color_end: usize,
    color: u32,
    alpha_method: Option<AlphaMethod>,
) {
    let mut index: usize;

    // No alpha, just assign the provided color to the relevant pixels, as simple as that.
    if alpha_method.is_none() {
            for line in 0..total_lines {
                for pixel in color_start..color_end {
                    index = line * line_length + pixel;
                    rectangle_slice[index] = color;
                }
            }
        return;
    }

    let alpha_method = alpha_method.unwrap();

    match alpha_method {

        AlphaMethod::Int => {
            for line in 0..total_lines {
                for pixel in color_start..color_end {
                    index = line * line_length + pixel;
                    rectangle_slice[index] = blend_pixel_int(rectangle_slice[index], color);
                }
            }
        }
        AlphaMethod::Float => {
            for line in 0..total_lines {
                for pixel in color_start..color_end {
                    index = line * line_length + pixel;
                    rectangle_slice[index] = blend_pixel_f32(rectangle_slice[index], color);
                }
            }
        }
    }
}

/// Draws a rectangle with dimensions and filled with a color specified in the `RectArea` struct.
/// Multithreaded.
/// # Arguments
/// * `ctx` - The graph context
/// * `rect` - The rectangle to draw
pub fn filled<UserData>(ctx: &mut GraphContext<UserData>, rect: &RectArea) {
    // Do nothing...
    if ctx.num_threads == 0 {
        return;
    }

    let color = rect.color.unwrap_or(ctx.win.foreground_color);

    let alpha_method = if ctx.alpha.enabled {
        Some(ctx.alpha.method)
    } else {
        None
    };

    // ==[ SINGLE THREAD ]==========================================================================

    // index of the first pixel in the first horizontal line of the rectangle
    let first_line_start: usize = (rect.top_left.y * ctx.win.w) as usize;
    // index of the last pixel in the last horizontal line of the rectangle
    let last_line_end: usize = ((rect.top_left.y + rect.dimensions.h) * ctx.win.w) as usize;
    let line_length = ctx.win.w_usize;

    let mut chunk_size = usize::div_ceil(last_line_end - first_line_start, ctx.num_threads);

    // If chunk size is less than the line length, force single thread processing
    let forced_single_thread = chunk_size < line_length;

    if ctx.num_threads == 1 || forced_single_thread {
        let rectangle_slice = &mut ctx.frame_buf[first_line_start..last_line_end];

        let total_lines = rect.dimensions.h as usize;
        let color_start = rect.top_left.x as usize;
        let color_end = (rect.top_left.x + rect.dimensions.w) as usize;

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

    // ==[ MULTIPLE THREADS ]=======================================================================

    // index of the first pixel in the first horizontal line of the rectangle
    let first_line_start: usize = (rect.top_left.y * ctx.win.w) as usize;
    // index of the last pixel in the last horizontal line of the rectangle
    let last_line_end: usize = ((rect.top_left.y + rect.dimensions.h) * ctx.win.w) as usize;

    let rectangle_slice = &mut ctx.frame_buf[first_line_start..last_line_end];
    let line_length = ctx.win.w_usize;
    let color_start = rect.top_left.x as usize;
    let color_end = (rect.top_left.x + rect.dimensions.w) as usize;
    let mut chunk_size = usize::div_ceil(rectangle_slice.len(), ctx.num_threads);

    // Make sure the chunk size is a multiple of the line length, otherwise the rectangle might break :/
    chunk_size = chunk_size / line_length * line_length;

    let mut chunks: Vec<&mut [u32]> = rectangle_slice.chunks_mut(chunk_size).collect();

    thread::scope(|s| {
        // Iterate over the chunks and process each in its own thread
        for chunk in chunks.iter_mut() {

            s.spawn(move || {
                draw_lines_of_rectangle_thread(
                    chunk,
                    line_length,
                    chunk.len() / line_length,
                    color_start,
                    color_end,
                    color,
                    alpha_method,
                )
            });
        }
    }); // The scope for the scoped threads ends here. All the threads are expected to be joined automagically at this point.

}
