use crate::init::init_window::{WIN_HEIGHT, WIN_WIDTH};
use crate::tools::fill::fill;
use crate::tools::primitives::Pixel;



// fn draw_circle(center_x: u32, center_y: u32, radius: u32) {
fn draw_circle(buf_view: &mut [u32], center: &Pixel, radius: u32, skip_every:u32) {
    let radius_sq = radius.pow(2) as i32;

    let mut skip_every = skip_every;
    if skip_every==0 {
        skip_every = 1;
    }



    let mut lines_count = 0;

    for delta_y in 0..=radius {
        let delta_y_sq = (delta_y.pow(2)) as i32;

        // Calculate the horizontal displacement for the current slice of the circle
        // from the center to one side.
        let width_half = ((radius_sq - delta_y_sq) as f64).sqrt().round() as i32;

        // Calculate start and end points for the current line segment.
        let start_x = center.x as i32 - width_half;
        let end_x = center.x as i32 + width_half;

        // Calculate the length of the line segment. Ensure that we handle cases where start_x could be negative.
        let length = if start_x < 0 { end_x } else { end_x - start_x } as u32;

        // Adjust start_x for when it's negative.
        let adjusted_start_x = if start_x < 0 { 0 } else { start_x } as u32;

        if lines_count % skip_every == 0 {
            // Draw the upper half of the circle.
            line_horizontal(buf_view, &Pixel{x:adjusted_start_x, y:center.y + delta_y, color:center.color}, length);
            // Draw the lower half of the circle, avoiding the central line being drawn twice.
            if delta_y > 0 {
                line_horizontal(buf_view, &Pixel{x:adjusted_start_x, y:center.y - delta_y, color:center.color}, length);
            }
        }
        lines_count += 1;
    }
}


/// Attempts to draw a horizontal line (from left to right) with a specified color and length
/// in a specified screen buffer.
fn line_horizontal (buf_view: &mut [u32], start: &Pixel, length: u32){

    // Don't draw off-screen or draw a zero-length line
    if  start.x >= WIN_WIDTH || start.y >= WIN_HEIGHT || length == 0 {
        return;
    }

    // Don't let the line overflow to the next line in the screen buffer
    let mut line_len = length;
    if start.x + length >= WIN_WIDTH {
        line_len = WIN_WIDTH - start.x;
    }

    let mut buf_index = (start.y * WIN_WIDTH + start.x) as usize;
    let buf_end_index = buf_index + line_len as usize;

    while buf_index < buf_end_index {
        buf_view[buf_index] = start.color;
        buf_index += 1;
    }
}

/// Attempts to draw a vertical line (from top to bottom) with a specified color and length
/// in a specified screen buffer.
fn line_vertical (buf_view: &mut [u32], start: &Pixel, length: u32){
    // Don't draw off-screen or draw a zero-length line
    if  start.x >= WIN_WIDTH || start.y >= WIN_HEIGHT || length == 0 {
        return;
    }

    // Don't let the line overflow the screen height
    let mut line_len = length;
    if start.y + length >= WIN_HEIGHT {
        line_len = WIN_HEIGHT - start.y;
    }

    let mut buf_index = (start.y * WIN_WIDTH + start.x) as usize;
    let buf_end_index = buf_index + (line_len * WIN_WIDTH) as usize;

    while buf_index < buf_end_index {
        buf_view[buf_index] = start.color;
        buf_index += WIN_WIDTH as usize;
    }

}


pub fn init_screen_buffer(buffer: &mut Vec<u32>) {
    let buf_len = buffer.len();
    let buf_view: &mut [u32] = &mut buffer[0..buf_len];

    fill(buf_view, 0x00_00_1E_00);

    /*
    let total_scanlines = buf_len as u32 / WIN_WIDTH;
    let mut cur_scanline: u32 = 0;

    while cur_scanline < total_scanlines {
        line_horizontal(buf_view, &Pixel { x: 0, y: cur_scanline, color: 0x00_00_ff_00  }, WIN_WIDTH);
        cur_scanline += 40;
    }

    let mut cur_x: u32 = 0;
    while cur_x < WIN_WIDTH {
        line_vertical(buf_view, &Pixel { x: cur_x, y: 0, color: 0x00_00_88_00 }, WIN_HEIGHT);
        cur_x += 40;
    }
*/

    draw_circle(buf_view, &Pixel { x: 320, y: 95, color: 0x00_00_ff_00 }, 40, 0);
    draw_circle(buf_view, &Pixel { x: 320, y: 95*2, color: 0x00_00_ff_00 }, 40, 1);
    draw_circle(buf_view, &Pixel { x: 320, y: 95*3, color: 0x00_00_ff_00 }, 40, 2);
    draw_circle(buf_view, &Pixel { x: 320, y: 95*4, color: 0x00_00_ff_00 }, 40, 3);




    // while line_num < total_lines {
    //     while line_dot < line_end {
    //         pixel_index = (line_num * WIN_WIDTH + line_dot) as usize;
    //         line_dot+=1;
    //         buf_view[pixel_index] = 0x00_00_ff_00;
    //     }

    // }
}

/*
   let mut buf_index: usize = 0;
   let mut pixel_index: usize = 0;
   let mut line_count: u32  = 0;

   while pixel_index < buf_len {
       if pixel_index % 20 == 0 {
           buf_view[pixel_index] = 0x00_00_ff_00;
       }
       pixel_index += 1;
   }
*/
