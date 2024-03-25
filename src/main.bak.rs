use minifb::{CursorStyle, Key, Window, WindowOptions};

mod tools;
use crate::tools::debug;
use crate::tools::draw::{GridOptions, RectOptions};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tools::draw;

const MONITOR_WIDTH: u32 = 3440;
const MONITOR_HEIGHT: u32 = 1440;

const WIN_WIDTH: u32 = 640;
const WIN_HEIGHT: u32 = 480;

const WIN_X: u32 = MONITOR_WIDTH / 2 - WIN_WIDTH / 2;
const WIN_Y: u32 = MONITOR_HEIGHT / 2 - WIN_HEIGHT / 2;

const WIN_NAME_PREFIX: &str = "ESC to exit";

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

    // Let's reserve enough memory for 4 screens!
    let mut buffer: Vec<u32> = vec![0x00_00_00_00; (WIN_WIDTH * WIN_HEIGHT * 4) as usize];

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    // window.limit_update_rate(Some(std::time::Duration::from_micros(138800)));

    window.set_position((WIN_X - 400) as isize, WIN_Y as isize);
    window.set_cursor_style(CursorStyle::Crosshair);

    let mut frame_count: u32 = 0;
    let mut window_title: String;

    let mut stats_map: HashMap<String, Vec<Duration>> = HashMap::new();
    stats_map.insert("Rect".to_string(), Vec::new());
    stats_map.insert("Grid".to_string(), Vec::new());
    stats_map.insert("Background".to_string(), Vec::new());

    let mut x: i32 = 0;
    let mut velocity_x: i32 = 1;

    let rect_width = 160;
    let rect_height = 160;

    let max_rect_x = WIN_WIDTH - rect_width;
    let max_rect_y = WIN_HEIGHT - rect_height;
    let shift = 7;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window_title = format!("{} :: frame: {}", WIN_NAME_PREFIX, frame_count);
        window.set_title(&window_title.as_str());

        // Background
        let benchmark_start = Instant::now();
        // ##########################################################################################
        // ##########################################################################################
        // ##########################################################################################
        /*
                for (i, pixel) in buffer.iter_mut().enumerate() {
                    let x = i % WIN_WIDTH as usize;
                    let y = i / WIN_WIDTH as usize;
                    *pixel = ((x as u32) << shift) | ((y as u32) << shift); // Create a gradient
                }
        */
        /*
        // ! EXPLORE FAST COPYING OF LARGE MEMORY CHUNKS !
        // ---------------------------------------------------
        unsafe{
            let src = buffer.as_ptr();
            // Calculate the destination pointer offset from the end of the buffer.
            let dst = buffer.as_mut_ptr().add(buffer.len() - 6400);

            std::ptr::copy(src, dst, 6400);
            std::ptr::copy_nonoverlapping(src, dst, 6400);
           }
        */

        // unsafe {
        //
        //
        //     // Obtain a raw pointer to the vector's buffer.
        //     let buffer_ptr = buffer.as_mut_ptr();
        //     // Calculate the end pointer for our loop. This is safe because we are not dereferencing the pointer yet.
        //     let end_ptr = buffer_ptr.add(buffer.len());
        //
        //     // Initialize a mutable pointer to iterate through the buffer.
        //     let mut current_ptr = buffer_ptr;
        //
        //     while current_ptr < end_ptr {
        //         // Calculate the current index based on the pointer offset from the start.
        //         let i = current_ptr.offset_from(buffer_ptr) as usize;
        //         let x = i % WIN_WIDTH as usize;
        //         let y = i / WIN_WIDTH as usize;
        //
        //         // Directly write to the memory location pointed to by current_ptr.
        //         *current_ptr = ((x as u32) << shift) | ((y as u32) << shift);
        //
        //         // Advance the pointer.
        //         current_ptr = current_ptr.add(1);
        //     }
        // }


        // SINGLE COLOR, unsafe high level, relatively fast.
        unsafe {
            // Obtain a raw pointer to the vector's buffer.
            let buffer_ptr = buffer.as_mut_ptr();

            // Calculate the end pointer for our loop.
            // This is safe because we are not dereferencing the pointer yet.
            let end_ptr = buffer_ptr.add(buffer.len());
            let mut current_ptr = buffer_ptr; // Initialize a mutable pointer to iterate through the buffer.
            while current_ptr < end_ptr {
                // Directly write to the memory location pointed to by current_ptr.
                *current_ptr = 0x00_00_33_00;
                // Advance the pointer.
                current_ptr = current_ptr.add(1);
            }
        }

        // ##########################################################################################
        // ##########################################################################################
        // ##########################################################################################
        // ##########################################################################################

        // This high level stuff is too slow
        //buffer.fill(0x00_33_11_22);

        let benchmark_duration = benchmark_start.elapsed();
        stats_map
            .get_mut("Background")
            .unwrap()
            .push(benchmark_duration);

        // for benchmarking function execution time.
        let benchmark_start = Instant::now();
        draw::square_grid(
            &GridOptions {
                s: 20,
                x: 0,
                y: 160,
                w: 32,
                h: 10,
                color: 0x00_77_77_99,
                // color: 0x00_00_00_00,
                win_h: WIN_HEIGHT as usize,
                win_w: WIN_WIDTH as usize,
            },
            &mut buffer,
        );

        let benchmark_duration = benchmark_start.elapsed();
        stats_map.get_mut("Grid").unwrap().push(benchmark_duration);

        let benchmark_start = Instant::now();
        draw::rect(
            &RectOptions {
                x: x as u32,
                y: 300,
                w: rect_width,
                h: rect_height,
                color: 0x00_ff_ff_ff,
                win_w: WIN_WIDTH,
                win_h: WIN_HEIGHT,
            },
            &mut buffer,
        );

        let benchmark_duration = benchmark_start.elapsed();
        stats_map.get_mut("Rect").unwrap().push(benchmark_duration);

        x += velocity_x;

        if (x as u32) > max_rect_x || (x as u32) < 1 {
            velocity_x *= -1;
        }

        frame_count += 1;
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIN_WIDTH as usize, WIN_HEIGHT as usize)
            .unwrap();
    }

    debug::make_stats(&"Grid".to_string(), stats_map.get("Grid").unwrap(), false);
    debug::make_stats(&"Rect".to_string(), stats_map.get("Rect").unwrap(), false);
    debug::make_stats(
        &"Background".to_string(),
        stats_map.get("Background").unwrap(),
        false,
    );
}
