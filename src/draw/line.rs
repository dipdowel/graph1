use crate::init::init_window::{WIN_HEIGHT, WIN_WIDTH};
use crate::tools::primitives::Pixel;


/// Attempts to draw a horizontal line (from left to right) with a specified color and length
/// in a specified screen buffer.
pub fn horizontal(buf_view: &mut [u32], start: &Pixel, length: u32){

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
pub fn vertical (buf_view: &mut [u32], start: &Pixel, length: u32){
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