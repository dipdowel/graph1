use minifb::{CursorStyle, Key, KeyRepeat, Window, WindowOptions};

mod tools;

use crate::tools::debug;
use crate::tools::draw::{GridOptions, RectOptions};
use std::collections::HashMap;
use std::fmt::Debug;
use std::time::{Duration, Instant};
use tools::draw;

// Make the global constants accessible here
mod constants;
use constants::*;


fn main() {
    #![allow(unused)]
    let mut window = Window::new(
        WIN_NAME_PREFIX,
        WIN_WIDTH as usize,
        WIN_HEIGHT as usize,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut dev_window = Window::new(
        DEV_WIN_NAME_PREFIX,
        WIN_WIDTH as usize,
        WIN_HEIGHT as usize,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Let's reserve enough memory for 4 screens!
    let mut buffer: Vec<u32> = vec![DEFAULT_BG_COLOR; TOTAL_BUFFER_SIZE];

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    dev_window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    // window.limit_update_rate(Some(std::time::Duration::from_micros(138800)));

    window.set_position((WIN_X - 400) as isize, WIN_Y as isize);
    dev_window.set_position((WIN_X - 400 - WIN_WIDTH) as isize, WIN_Y as isize);

    window.set_cursor_style(CursorStyle::Crosshair);

    let mut frame_count: u32 = 0;
    let mut window_title: String;
    let mut dev_window_title: String;

    let mut stats_map: HashMap<String, Vec<Duration>> = HashMap::new();
    stats_map.insert("Rect".to_string(), Vec::new());
    stats_map.insert("Grid".to_string(), Vec::new());
    stats_map.insert("Background".to_string(), Vec::new());

    let mut x: i32 = 0;
    let mut velocity_x: i32 = 1;

    let rect_width = 100;
    let rect_height = 100;

    let max_rect_x = WIN_WIDTH - rect_width;
    let max_rect_y = WIN_HEIGHT - rect_height;
    let shift = 7;

    let active_page: usize = 0;

    let mut is_key_down: bool = false;
    let mut is_key_up: bool = false;

    let mut key_pressed: &str = " ";
    let mut screen_offset: u32 = 2 * WIN_HEIGHT as u32;
    let mut pressed_key: Option<Key> = None;
    let max_y = WIN_HEIGHT * NUM_OF_PAGES as u32;

    /***[ DEV WINDOW ]************************************************************************/

    draw::rect(
        &RectOptions {
            // x: x as u32,
            x: 100,
            y: 100,
            w: rect_width,
            h: rect_height,
            color: 0x00_dd_00_00,
            win_w: WIN_WIDTH,
            win_h: WIN_HEIGHT,
        },
        &mut buffer,
    );

    // PIXEL_PER_PAGE
    let screen_start = (0) as usize;
    let buf_view = &buffer[screen_start..screen_start + PIXEL_PER_PAGE];

    // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
    dev_window
        .update_with_buffer(buf_view, WIN_WIDTH as usize, WIN_HEIGHT as usize)
        .unwrap();
    /*****************************************************************************************/

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .get_keys_pressed(KeyRepeat::No)
            .iter()
            .for_each(|key| match key {
                Key::Up => {
                    key_pressed = "[ ↑ ]";
                    // test_count += 1;
                    pressed_key = Some(*key);
                }
                Key::Down => {
                    key_pressed = "[ ↓ ]";
                    // test_count -= 1;
                    pressed_key = Some(*key);
                }
                _ => {
                    key_pressed = "[  ]";
                    pressed_key = None;
                }
            });
        window.get_keys_released().iter().for_each(|key| match key {
            _ => {
                key_pressed = "[  ]";
                pressed_key = None;
            }
        });

        window_title = format!(
            "{} :: {} :: {} :: pressed_key: {:?} FR: {}",
            WIN_NAME_PREFIX, key_pressed, screen_offset, pressed_key, frame_count
        );

        window.set_title(&window_title.as_str());

        buffer.fill(DEFAULT_BG_COLOR);

        if let Some(Key::Up) = pressed_key {
            if frame_count % 1 == 0 && screen_offset < max_y {
                screen_offset -= 1;
            }
        }

        if let Some(Key::Down) = pressed_key {
            if frame_count % 1 == 0 && screen_offset > 1 {
                screen_offset += 1;
            }
        }

        let mut rect_index = 0;
        let loop_till = WIN_HEIGHT * (NUM_OF_PAGES as u32) - rect_height * 2;
        let mut rect_color = 0x00_ff_ff_ff;
        loop {
            draw::rect(
                &RectOptions {
                    // x: x as u32,
                    x: 100,
                    y: rect_index,
                    w: rect_width,
                    h: rect_height,
                    color: rect_color,
                    win_w: WIN_WIDTH,
                    win_h: WIN_HEIGHT,
                },
                &mut buffer,
            );

            if rect_index >= loop_till {
                break;
            }
            rect_index += rect_height;
            rect_color += 0x00_11_11_11;
        }

        x += velocity_x;

        if (x as u32) > max_rect_x || (x as u32) < 1 {
            velocity_x *= -1;
        }

        frame_count += 1;

        // PIXEL_PER_PAGE
        let screen_start = (screen_offset * WIN_WIDTH) as usize;
        let buf_view = &buffer[screen_start..screen_start + PIXEL_PER_PAGE];

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(buf_view, WIN_WIDTH as usize, WIN_HEIGHT as usize)
            .unwrap();
    }
}
