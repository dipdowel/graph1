// use std::collections::HashMap;
use std::fmt::Debug;
use std::time::Duration;

use minifb::{CursorStyle, Key, KeyRepeat};

use constants::*;
use crate::draw::line;



use crate::init::init_window::*;

use crate::input::handle_keyboard;
use crate::tools::fill::fill;
use crate::tools::init_screen_buffer::init_screen_buffer;
use crate::tools::primitives::Pixel;


// use crate::tools::draw::RectOptions;
// use tools::draw;
mod tools;

mod init;
// Make the global constants accessible here
mod constants;
mod input;
mod draw;

fn main() {
    #![allow(unused)]

    let (mut window, mut dev_window) = init_window(true);

    // Let's reserve enough memory for 4 screens!
    let mut buffer: Vec<u32> = vec![DEFAULT_BG_COLOR; TOTAL_BUFFER_SIZE];

    let mut frame_count: u32 = 0;
    let mut window_title: String;
    let mut dev_window_title: String;

    // PIXEL_PER_PAGE
    let screen_start: usize = 0;
    let mut buf_view = &mut buffer[0..=PIXEL_PER_PAGE];
    //


     match dev_window {
        Some(ref mut dev_window) => {
            // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
            dev_window
                .update_with_buffer(buf_view, WIN_WIDTH as usize, WIN_HEIGHT as usize).expect("Failed to update dev window memory")

        },
        _ => ()
    }


    // Main animation loop
    while window.is_open() && !window.is_key_down(Key::Escape) {

        // init_screen_buffer(&mut buf_view);
        fill(buf_view, 0x00_00_1E_00);

        line::horizontal(&mut buf_view, &Pixel { x: 10, y: 10, color: 0x00_00_dd_00 }, frame_count%640);
        line::vertical(&mut buf_view, &Pixel { x: 10, y: 10, color: 0x00_00_dd_00 }, frame_count%480);

        draw::circle(&mut buf_view, &Pixel { x: 320, y: 320, color: 0x00_00_dd_00 }, frame_count%42, 4);
        draw::circle(&mut buf_view, &Pixel { x: 380, y: 301, color: 0x00_00_dd_00 }, frame_count%65, 8);
        draw::circle(&mut buf_view, &Pixel { x: 200, y: 201, color: 0x00_00_dd_00 }, frame_count%173, 2);

        draw::circle(&mut buf_view, &Pixel { x: 320, y: 240, color: 0x00_00_dd_00 }, frame_count%(WIN_HEIGHT/2), 4);

        //---------------------------
        // Read and handle keyboard
        //---------------------------
        let keys_pressed = window
            .get_keys_pressed(KeyRepeat::No);

        let keys_released = window
            .get_keys_released();

        if(keys_pressed.len() > 0 || keys_released.len() > 0){
            handle_keyboard(&keys_pressed, &keys_released);
        }


        frame_count += 1;

        // PIXEL_PER_PAGE
        // let screen_start = (screen_offset * WIN_WIDTH) as usize;
        // let buf_view = &buffer[screen_start..screen_start + PIXEL_PER_PAGE];

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(buf_view, WIN_WIDTH as usize, WIN_HEIGHT as usize)
            .unwrap();



        match dev_window {
            Some(ref mut dev_window) => {

                dev_window
                    .update_with_buffer(buf_view, WIN_WIDTH as usize, WIN_HEIGHT as usize)
                    .unwrap();

            },
            _ => ()
        }

    }
}

//
//
//
//
// ------------------------------------------------------------------------------------------------------------------------
//

// let mut key_pressed: &str = " ";
// let mut screen_offset: u32 = 2 * WIN_HEIGHT as u32;
// let mut pressed_key: Option<Key> = None;
// let max_y = WIN_HEIGHT * NUM_OF_PAGES as u32;

// let mut is_key_down: bool = false;
// let mut is_key_up: bool = false;
//

// let mut stats_map: HashMap<String, Vec<Duration>> = HashMap::new();
// stats_map.insert("Rect".to_string(), Vec::new());
// stats_map.insert("Grid".to_string(), Vec::new());
// stats_map.insert("Background".to_string(), Vec::new());

// let mut x: i32 = 0;
// let mut velocity_x: i32 = 1;

// let rect_width = 100;
// let rect_height = 100;

// let max_rect_x = WIN_WIDTH - rect_width;
// let max_rect_y = WIN_HEIGHT - rect_height;
// let shift = 7;

// let active_page: usize = 0;


/***[ DEV WINDOW ]************************************************************************/

// draw::rect(
//     &RectOptions {
//         // x: x as u32,
//         x: 100,
//         y: 100,
//         w: rect_width,
//         h: rect_height,
//         color: 0x00_dd_00_00,
//         win_w: WIN_WIDTH,
//         win_h: WIN_HEIGHT,
//     },
//     &mut buffer,
// );


// window_title = format!(
//     "{} :: {} :: {} :: pressed_key: {:?} FR: {}",
//     WIN_NAME_PREFIX, key_pressed, screen_offset, pressed_key, frame_count
// );
//
// window.set_title(&window_title.as_str());

// buffer.fill(DEFAULT_BG_COLOR);

// if let Some(Key::Up) = pressed_key {
//     if frame_count % 1 == 0 && screen_offset < max_y {
//         screen_offset -= 1;
//     }
// }
//
// if let Some(Key::Down) = pressed_key {
//     if frame_count % 1 == 0 && screen_offset > 1 {
//         screen_offset += 1;
//     }
// }

// let mut rect_index = 0;
// let loop_till = WIN_HEIGHT * (NUM_OF_PAGES as u32) - rect_height * 2;
// let mut rect_color = 0x00_ff_ff_ff;
// loop {
//     draw::rect(
//         &RectOptions {
//             // x: x as u32,
//             x: 100,
//             y: rect_index,
//             w: rect_width,
//             h: rect_height,
//             color: rect_color,
//             win_w: WIN_WIDTH,
//             win_h: WIN_HEIGHT,
//         },
//         &mut buffer,
//     );
//
//     if rect_index >= loop_till {
//         break;
//     }
//     rect_index += rect_height;
//     rect_color += 0x00_11_11_11;


// x += velocity_x;

// if (x as u32) > max_rect_x || (x as u32) < 1 {
//     velocity_x *= -1;
// }