use crate::core::context::alpha::AlphaMethod;
use crate::core::context::GraphContext;
use crate::primitives::plane::RectArea;
use crate::utils::color::alpha::{blend_pixel_f32, blend_pixel_int};
use std::cmp::PartialEq;

impl PartialEq for AlphaMethod {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (AlphaMethod::Int, AlphaMethod::Int) => true,
            (AlphaMethod::Float, AlphaMethod::Float) => true,
            _ => false,
        }
    }
}

/// Draws a rectangle with dimensions and filled with a color specified in the `RectArea` struct.
pub fn filled<UserData>(ctx: &mut GraphContext<UserData>, rect: &RectArea) {
    let color = rect.color.unwrap_or(ctx.win.foreground_color);

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
    let first_line_start:usize = (rect.top_left.y * ctx.win.w) as usize;
    // index of the last pixel in the last horizontal line of the rectangle
    let last_line_end:usize = ((rect.top_left.y + rect.dimensions.h) * ctx.win.w) as usize;

    let rectangle_slice = &mut ctx.frame_buf[first_line_start..last_line_end];
    let line_length = ctx.win.w_usize;
    let total_lines = rect.dimensions.h as usize;
    let color_start = rect.top_left.x as usize;
    let color_end = (rect.top_left.x + rect.dimensions.w) as usize;

    for line in 0..total_lines {
        for pixel in 0..line_length {
            let index = line * line_length + pixel;
            if pixel >= color_start && pixel < color_end {
                rectangle_slice[index] = color;
            }
        }
    }




    /*
    // This is a working solution, but it requires the context, so let's leave it aside for now
    for line in 0..rect.dimensions.h {
        let start_index = (rect.top_left.y + line) * ctx.win.w + rect.top_left.x;
        for column in 0..rect.dimensions.w {
            let index = (start_index + column) as usize;

            // No alpha enabled, just assign the color
            if !ctx.alpha.enabled {
                ctx.frame_buf[index ] = color;
                continue;
            }
            // Alpha Int
            if ctx.alpha.method == AlphaMethod::Int {
                ctx.frame_buf[index ] = blend_pixel_int(ctx.frame_buf[index ], color);
            }
            // Alpha Float
            if ctx.alpha.method == AlphaMethod::Float {
                ctx.frame_buf[index ] = blend_pixel_f32(ctx.frame_buf[index ], color);

            }
        }
    }
    */


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

    //take a mutable sub-slice of the

    /*


    // Dereference the options
    let start_x = rect.top_left.x;
    let start_y = rect.top_left.y;
    let width = rect.dimensions.w;
    let height = rect.dimensions.h;

    // Nothing to draw here
    if width == 0 || height == 0 {
        return;
    }

    let end_x = start_x + width;
    let end_y = start_y + height;

    let mut x = start_x;
    let mut y = start_y;

    // Which pixel in the vector should be filled in next.
    let mut pixel_index: usize;

    let mut resulting_color: u32 = color;

    loop {
        pixel_index = (y * ctx.win.w + x) as usize;

        // If alpha blending is enabled, blend the new color with the existing pixel color
        // according to the alpha channel.
        if ctx.alpha.enabled {
            if ctx.alpha.method == AlphaMethod::Int {
                resulting_color = blend_pixel_int(ctx.frame_buf[pixel_index], color);
            }
            if ctx.alpha.method == AlphaMethod::Float {
                resulting_color = blend_pixel_f32(ctx.frame_buf[pixel_index], color);
            }
        }
        ctx.frame_buf[pixel_index] = resulting_color;

        x += 1;

        if x == end_x || x == ctx.win.w {
            y += 1;
            x = start_x;
        };

        if y == end_y || y == ctx.win.h {
            break;
        }
    }
    */
}
