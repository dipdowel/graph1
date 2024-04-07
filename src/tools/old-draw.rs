/// Options to draw a rectangle on the screen
pub struct RectOptions {
    /// Rectangle width
    pub w: u32,
    /// Rectangle height
    pub h: u32,
    /// Rectangle starting x
    pub x: u32,
    /// Rectangle starting y
    pub y: u32,
    /// An RGB value with the highest byte ignored: 0x00_RR_GG_BB
    pub color: u32,
    /// Window width
    pub win_w: u32,
    /// Window height
    pub win_h: u32,
}

/// Draws a rectangle.
/// If rectangle bleeds beyond the window it gets truncated at window's width and height.
/// * `options` - Config of the rectangle
/// * `buffer` - A reference to the screen/window buffer
pub fn rect(options: &RectOptions, buffer: &mut Vec<u32>) {
    let RectOptions {
        x,
        y,
        w,
        h,
        color,
        win_w,
        win_h,
    } = options;

    // Dereference the options
    let start_x = *x;
    let start_y = *y;
    let width = *w;
    let height = *h;
    let win_width = *win_w;
    let win_height = *win_h;

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

    loop {
        pixel_index = (y * win_width + x) as usize;
        buffer[pixel_index] = *color;
        x += 1;

        if x == end_x || x == win_width {
            y += 1;
            x = start_x;
        };

        if y == end_y || y == win_height {
            break;
        }
    }
}

/// Options to draw a rectangle on the screen
pub struct GridOptions {
    /// Size of one grid unit
    pub s: usize,
    /// Grid width, in units
    pub w: usize,
    /// Grid height, in units
    pub h: usize,
    /// Where grid starts, x
    pub x: usize,
    /// Where grid starts, y
    pub y: usize,
    /// An RGB value with the highest byte ignored: 0x00_RR_GG_BB
    pub color: u32,
    /// Window width
    pub win_w: usize,
    /// Window height
    pub win_h: usize,
}

pub fn square_grid(options: &GridOptions, buffer: &mut Vec<u32>) {
    // #![allow(unused)]
    let GridOptions {
        s,
        w,
        h,
        x,
        y,
        color,
        win_w,
        win_h,
    } = options;

    // Dereference the options
    let size = *s;
    let start_x = *x;
    let start_y = *y;

    let width = *w;
    let height = *h;
    let win_width = *win_w;
    let win_height = *win_h;

    let end_x = start_x + size * width;
    let end_y = start_y + size * height;



    let hor_line_len = size * width;
    let ver_line_len = size * height;

    let color = *color;

    let binding = vec![color; hor_line_len as usize];
    let hor_line = binding.as_slice();


    let mut row: usize = start_x;

    // let mut slice:&mut[u32] = &mut buffer[6400..(6400+hor_line_len)as usize];
    let mut slice: &mut [u32];

    // slice.copy_from_slice(hor_line);

    // row += size as usize * win_width;

    let mut row_num = 0;
    loop {
        slice = &mut buffer[row..row + hor_line_len];
        slice.copy_from_slice(hor_line);
        row = start_y* win_width+ start_x + win_width * size * row_num;
        row_num += 1;
        if row_num >= height {
            break;
        }
    }

    let max_pixel_index = buffer.len();
    let mut pixel_index = start_y* win_width;

    let mut x: usize = 0;
    let mut y: usize = start_y;

    loop {
        if pixel_index % size == 0 &&
            x > start_x &&
            x < start_x + hor_line_len &&
            y > start_y &&
            y < start_y + ver_line_len
            {
            buffer[pixel_index] = color;
        }
        x += 1;
        if x == win_width {
            y+=1;
            x = 0;
        }

        pixel_index += 1;
        if pixel_index == max_pixel_index {
            break;
        }
    }

}
