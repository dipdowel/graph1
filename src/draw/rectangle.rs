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

///
fn draw_lines_of_rectangle_thread(
    rectangle_slice: &mut [u32],
    line_length: usize,
    total_lines: usize,
    color_start: usize,
    color_end: usize,
    color: u32,
    alpha_method: AlphaMethod,
) {
    let mut index: usize;

    match alpha_method {
        AlphaMethod::None => {
            for line in 0..total_lines {
                for pixel in color_start..color_end {
                    index = line * line_length + pixel;
                    rectangle_slice[index] = color;
                }
            }
        }
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
pub fn filled<UserData>(ctx: &mut GraphContext<UserData>, rect: &RectArea) {

    // Do nothing...
    if ctx.num_threads == 0 {
        return;
    }


    let color = rect.color.unwrap_or(ctx.win.foreground_color);

    let alpha_method = if ctx.alpha.enabled {
        ctx.alpha.method
    } else {
        AlphaMethod::None
    };


    // ==[ SINGLE THREAD ]==========================================================================
    if ctx.num_threads == 1 {
        // index of the first pixel in the first horizontal line of the rectangle
        let first_line_start: usize = (rect.top_left.y * ctx.win.w) as usize;
        // index of the last pixel in the last horizontal line of the rectangle
        let last_line_end: usize = ((rect.top_left.y + rect.dimensions.h) * ctx.win.w) as usize;

        let rectangle_slice = &mut ctx.frame_buf[first_line_start..last_line_end];
        let line_length = ctx.win.w_usize;
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


    // rect.dimensions.h /
    //
    // // ctx.num_threads

    // ==[ MULTIPLE THREADS ]=======================================================================
    // let num_lines: u32 = rect.dimensions.h;
    // let num_columns: u32 = rect.dimensions.w;

    // TODO: 1. Let's simplify it to the state where `y` does not matter, i.e.
    // TODO:    the function receives a buffer and the line length (i.e. window width).
    // TODO:    The buffer contains only lines that need to be handled.
    // TODO:    So we only need the buffer, the line length, the color, `x_start` and `x_end`.
    // TODO:
    // TODO:    So yeah, extract the logic into a lower level function that would not need to know
    // TODO:    anything about the context and window.

    // TODO: 2. That way we can chunkify the buffer and process each chunk in a parallel thread


    // index of the first pixel in the first horizontal line of the rectangle
    let first_line_start: usize = (rect.top_left.y * ctx.win.w) as usize;
    // index of the last pixel in the last horizontal line of the rectangle
    let last_line_end: usize = ((rect.top_left.y + rect.dimensions.h) * ctx.win.w) as usize;

    let rectangle_slice = &mut ctx.frame_buf[first_line_start..last_line_end];
    let line_length = ctx.win.w_usize;
    let color_start = rect.top_left.x as usize;
    let color_end = (rect.top_left.x + rect.dimensions.w) as usize;

    // let mut total_lines = rect.dimensions.h as usize;


    let chunk_size = usize::div_ceil(rectangle_slice.len(), ctx.num_threads);
    let mut chunks: Vec<&mut [u32]> = rectangle_slice.chunks_mut(chunk_size).collect();

    // FIXME:
    // FIXME: There's a bug: when the number of threads cannot be divided by 2 or 4 or so on,
    // FIXME: the rectangle breaks
    // FIXME:
    // FIXME:

    thread::scope(|s| {
        // Iterate over the chunks and process each in its own thread
        for  chunk in chunks.iter_mut() {
            // `chunk_index + 1` since the main thread is already processing the 0-th chunk

            s.spawn(move ||
                        draw_lines_of_rectangle_thread(
                            chunk,
                            line_length,
                            chunk.len() / line_length,
                            color_start,
                            color_end,
                            color,
                            alpha_method,
                        )

            );

        }
    }); // The scope for the scoped threads ends here. All the threads are expected to be joined automagically at this point.







    // ==[ MULTIPLE THREADS ]=======================================================================

    //     let index = start_index + line *
    // }

    /*
        // let chunk_size = usize::div_ceil(buffer.len(), num_threads);
        let start_index = rect.top_left.y * ctx.win.w ;
        let end_index = (rect.top_left.y + rect.dimensions.h) * ctx.win.w;

        let x_start = rect.top_left.x;
        let x_end = rect.top_left.x + rect.dimensions.w;

        let mut current_line:u32 = 0;

        for i in start_index..= end_index {

             if i >= x_start && i <= x_end {
                ctx.frame_buf[i as usize] = color;
            }

            // let offset = current_line * ctx.win.w;
            // // Get rid of the `<` and `>`, this may help with the performance
            // if i-offset >= x_start   && i -offset <= x_end*current_line {
            //     ctx.frame_buf[i as usize] = color;
            // }
            //
            // if i % ctx.win.w == 0 {
            //     current_line += 1;
            // }

        }
    */

}
