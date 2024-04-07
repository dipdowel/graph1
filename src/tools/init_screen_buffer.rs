use crate::draw;
use crate::tools::fill::fill;
use crate::tools::primitives::Pixel;

// pub fn init_screen_buffer(buffer: &mut Vec<u32>) {
pub fn init_screen_buffer(buf_view: &mut [u32]) {
    // let buf_len = buffer.len();
    // let buf_view: &mut [u32] = &mut buffer[0..buf_len];
    let buf_len = buf_view.len();


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




     // draw::circle(buf_view, &Pixel { x: 320, y: 95, color: 0x00_00_ff_00 }, 20, 0);
    draw::circle(buf_view, &Pixel { x: 320, y: 95, color: 0x00_00_ff_00 }, 20, 0);
    draw::circle(buf_view, &Pixel { x: 320, y: 95*2, color: 0x00_00_ff_00 }, 30, 1);
    draw::circle(buf_view, &Pixel { x: 320, y: 95*3, color: 0x00_00_ff_00 }, 40, 2);
    draw::circle(buf_view, &Pixel { x: 320, y: 95*4, color: 0x00_00_ff_00 }, 50, 3);




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
