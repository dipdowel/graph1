use crate::graph1_core::context::GraphContext;
use crate::graph1::primitives::primitives::{Pixel, Point};

/// Attempts to draw a horizontal line (from left to right) with a specified color and length
/// in a specified screen buffer.
pub fn horizontal(ctx: &mut GraphContext, start: &Pixel, length: u32) {

    // if length+start.x > WIN_WIDTH {
    //     println!(">>> HORIZONTAL! {}, {:?}", length, start);
    // }
    // Don't draw off-screen or draw a zero-length line
    if start.x >= ctx.win.w || start.y >= ctx.win.h || length == 0 {
        return;
    }

    // Don't let the line overflow to the next line in the screen buffer
    let mut line_len = length;
    if start.x + length >= ctx.win.w {
        line_len = ctx.win.w - start.x-1;
    }

    let mut buf_index = (start.y * ctx.win.w + start.x) as usize;
    let buf_end_index = buf_index + line_len as usize;

    while buf_index < buf_end_index {
        ctx.buf_view[buf_index] = start.color;
        buf_index += 1;
    }
}

/// Attempts to draw a vertical line (from top to bottom) with a specified color and length
/// in a specified screen buffer.
pub fn vertical(ctx: &mut GraphContext, start: &Pixel, length: u32) {


    // Don't draw off-screen or draw a zero-length line
    if start.x >= ctx.win.w || start.y >= ctx.win.h || length == 0 {
        return;
    }

    // Don't let the line overflow the screen height
    let mut line_len = length;
    if start.y + length >= ctx.win.h {
        line_len = ctx.win.h - start.y-1;
    }

    // if length+start.y > WIN_HEIGHT {
    //     println!(">>> VERTICAL! length: {}, {:?}, line_len:{} ", length, start,line_len);
    // }

    let mut buf_index = (start.y * ctx.win.w + start.x) as usize;
    let buf_end_index = buf_index + (line_len * ctx.win.w) as usize;

    while buf_index < buf_end_index {
        ctx.buf_view[buf_index] = start.color;
        buf_index += ctx.win.w_usize;
    }
}

pub fn between_two_points(ctx: &mut GraphContext, start: &Pixel, end: &Point) {

    let mut start: Pixel = Pixel {
        x: u32::min(start.x, ctx.win.w-1),
        y: u32::min(start.y, ctx.win.h-1),
        color: start.color,
    };

    let end: Pixel = Pixel {
        x: u32::min(end.x, ctx.win.w-1),
        y: u32::min(end.y, ctx.win.h-1),
        color: start.color,
    };
/**/
    // Detect vertical lines and draw them using a more optimised approach
    if start.x == end.x && start.y != end.y {
        let start_pixel: &Pixel = if start.y < end.y { &start } else { &end };
        vertical(
            ctx,
            start_pixel,
            i32::abs(end.y as i32 - start.y as i32) as u32,
        );
        return;
    }

    // Detect horizontal lines and draw them using a more optimised approach
    if start.y == end.y && start.x != end.x {
        let start_pixel: &Pixel = if start.x < end.x { &start } else { &end };
        horizontal(
            ctx,
            &start_pixel,
            i32::abs(end.x as i32 - start.x as i32) as u32,
        );
        return;
    }

    // println!(">>> ARBITRARY!");

    /*
        This implementation of Bresenham's algorithm works by iteratively determining
        which pixel to "light up" next based on the steepness and direction of the line.
        The error factor `err` helps decide when to move along the y-axis instead of (or in addition to)
        the x-axis to ensure the line remains continuous and closely approximates a straight path
         between the start and end points.
    */



    // Calculate the difference in x and y between the start and end points.
    // `dx` and `dy` are used to determine the direction and steepness of the line.
    let dx = i32::abs(end.x as i32 - start.x as i32);
    let dy = -i32::abs(end.y as i32 - start.y as i32);

    // Determine the step direction for x and y.
    // `sx` and `sy` are either 1 or -1, indicating the direction to move on each axis.
    let sx = if start.x < end.x { 1 } else { -1 };
    let sy = if start.y < end.y { 1 } else { -1 };

    // The error factor, initially set to the sum of dx and dy.
    // It determines when to increment the y-coordinate (for steep lines).
    let mut err = dx + dy;

    loop {
        // Set the current pixel. The color can be set to a specific value or passed through the Pixel struct.
        let buf_index = (start.y * ctx.win.w + start.x) as usize;
        ctx.buf_view[buf_index] = start.color;

        // If the current position is the end point, exit the loop.
        if start.x == end.x && start.y == end.y {
            break;
        }

        // e2 is a doubled error factor, used to decide if the error is too high and needs correction.
        let e2 = 2 * err;

        // Adjust the x-coordinate and the error factor if needed.
        // This happens when the line is more horizontal than vertical.
        if e2 >= dy {
            err += dy; // Increase error factor to move y soon.
            start.x = ((start.x as i32) + sx) as u32; // Move x in the appropriate direction.
        }

        // Similarly, adjust the y-coordinate and the error factor if needed.
        // This is for when the line is more vertical than horizontal.
        if e2 <= dx {
            err += dx; // Compensate the error factor since we moved along y.
            start.y = ((start.y as i32) + sy) as u32; // Move y in the appropriate direction.
        }
    }
}
