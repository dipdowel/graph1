use std::f32::consts::PI;
// use std::collections::HashMap;
use std::fmt::Debug;
use std::time::Duration;

use minifb::{CursorStyle, Key, KeyRepeat};
use rand::seq::SliceRandom;
use rand::Rng; // Trait that provides the shuffle method.

use crate::draw::line;
use constants::*;

use crate::init::init_window::*;

use crate::input::handle_keyboard;
use crate::tools::fill::fill;
use crate::tools::init_screen_buffer::init_screen_buffer;
use crate::tools::primitives::{Pixel, Point, Point3D, Point3DF32};

// use crate::tools::draw::RectOptions;
// use tools::draw;
mod tools;

mod init;
// Make the global constants accessible here
mod constants;
mod cube;
mod draw;
mod input;

fn main() {
    #![allow(unused)]

    let (mut window, mut dev_window) = init_window(false);

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
                .update_with_buffer(buf_view, WIN_WIDTH as usize, WIN_HEIGHT as usize)
                .expect("Failed to update dev window memory")
        }
        _ => (),
    }

    let color = 0x00_00_bb_00;

    let mut points: Vec<Point> = vec![];
    let mut i = 0;
    let points_len = 115;
    let mut rng = rand::thread_rng(); // Creates a random number generator.

    // Generate a bunch of points to draw lines with later on
    loop {
        points.push(Point {
            x: rng.gen_range(WIN_WIDTH / 2 - 75..WIN_WIDTH / 2 + 75),
            y: rng.gen_range(WIN_HEIGHT / 2 - 75..WIN_HEIGHT / 2 + 75),
        });
        i += 1;
        if i == points_len {
            break;
        }
    }

    //----------------------------------------------------------------------------------------------
    // THE CUBE static
    //----------------------------------------------------------------------------------------------

    const SPEED_X: f32 = 3.0; // rps
    const SPEED_Y: f32 = 12.0; // rps
    const SPEED_Z: f32 = 3.0; // rps

    let cx: f32 = WIN_WIDTH as f32 / 2_f32 + 290.0 ;
    let cy = WIN_HEIGHT as f32 / 2_f32 - 180_f32;
    // let cy = WIN_HEIGHT as f32 / 2_f32 ;
    let cz = 0_f32;
    let size = WIN_HEIGHT as f32 / 24_f32;

    #[rustfmt::skip]
    let mut vertices: Vec<Point3DF32> = vec![
        Point3DF32 { x: cx - size, y: cy - size, z: cz - size },
        Point3DF32 { x: cx + size, y: cy - size, z: cz - size },
        Point3DF32 { x: cx + size, y: cy + size, z: cz - size },
        Point3DF32 { x: cx - size, y: cy + size, z: cz - size },
        Point3DF32 { x: cx - size, y: cy - size, z: cz + size },
        Point3DF32 { x: cx + size, y: cy - size, z: cz + size },
        Point3DF32 { x: cx + size, y: cy + size, z: cz + size },
        Point3DF32 { x: cx - size, y: cy + size, z: cz + size },

    ];
    #[rustfmt::skip]
    let edges = [
        [0, 1], [1, 2], [2, 3], [3, 0], // back face
        [4, 5], [5, 6], [6, 7], [7, 4], // front face
        [0, 4], [1, 5], [2, 6], [3, 7] // connecting sides
    ];

    // set up the animation loop
    let mut time_delta: f32;
    let mut time_last = 0.0_f32;
    let mut time_now: f32;

    ////////////////////////
    ////////////////////////
    // draw each edge
    let mut cube_pixel = Pixel {
        x: 0,
        y: 0,
        color: 0x00_ff_77_ff,
    };

    let mut cube_point = Point { x: 0, y: 0 };

    ////////////////////////
    ////////////////////////

    //----------------------------------------------------------------------------------------------

    // Main animation loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        ////////////////////////////////////////////////////////////////////////////////////////////
        // SOME BASIC CRUDE OSCILLATION
        let frequency_adjustment_factor = 8.0; // Frequency of the change
        // let frame_count_mod = frame_count.wrapping_add(1); // Safely handle overflow
        let sine_input = (frame_count as f64 / frequency_adjustment_factor).sin();
        // Transform sine output (-1 to 1) to 0 to points_len
        let normalized_value = (sine_input + 1.0) / 2.0; // Now between 0 and 1
        let oscillator = (normalized_value * points_len as f64) as usize;
        let oscillator= if oscillator == 0 {1} else { oscillator };

        ////////////////////////////////////////////////////////////////////////////////////////////
        // Clear screen
        fill(buf_view, 0x00_04_04_0F);

        // cube(buf_view, frame_count);

        //----------------------------------------------------------------------------------------------
        // THE CUBE dynamic
        //----------------------------------------------------------------------------------------------

        time_now = frame_count as f32;

        // calculate the time difference
        time_delta = time_now - time_last;
        time_last = time_now;

        // rotate the cube along the Z axis
        let angle = time_delta * 0.001 * SPEED_Z * PI * 2_f32;
        let cy = cy + oscillator  as f32;
        let cz = cz + oscillator as f32 ;

        for mut v in &mut vertices {
            let dx: f32 = v.x - cx;
            let dy = v.y - cy;
            let x = dx * f32::cos(angle) - dy * f32::sin(angle);
            let y = dx * f32::sin(angle) + dy * f32::cos(angle);
            v.x = x + cx;
            v.y = y + cy;
        }

        // rotate the cube along the X axis
        let angle = time_delta * 0.001 * SPEED_X * PI * 2_f32;
        for mut v in &mut vertices {
            let dy = v.y - cy;
            let dz = v.z - cz;
            let y = dy * f32::cos(angle) - dz * f32::sin(angle);
            let z = dy * f32::sin(angle) + dz * f32::cos(angle);
            v.y = y + cy;
            v.z = z + cz;
        }

        // rotate the cube along the Y axis
        let angle = time_delta * 0.001 * SPEED_Y * PI * 2_f32;
        for mut v in &mut vertices {
            let dx = v.x - cx;
            let dz = v.z - cz;
            let x = dz * f32::sin(angle) + dx * f32::cos(angle);
            let z = dz * f32::cos(angle) - dx * f32::sin(angle);
            v.x = x + cx;
            v.z = z + cz;
        }

        // draw each edge
        for edge in edges {
            cube_pixel.x = vertices[edge[0]].x as u32;
            cube_pixel.y = vertices[edge[0]].y as u32;
            cube_point.x = vertices[edge[1]].x as u32;
            cube_point.y = vertices[edge[1]].y as u32;
            // println!("cube_pixel: {:?}", cube_pixel);
            // println!("cube_point: {:?}", cube_point);

            line::between_two_points(buf_view, &cube_pixel, &cube_point);
        }

        //----------------------------------------------------------------------------------------------






                // println!("measure: {}", measure);

                points.shuffle(&mut rng);

                for i in 0..oscillator {
                    line::between_two_points(
                        &mut buf_view,
                        &Pixel{
                            x:points[i].x,
                            y:points[i].y,
                            color:0x00_44_44_ff
                        },
                        &points[i+1],
                    );
                }

                ////////////////////////////////////////////////////////////////////////////////////////////
                // DRAW FRAME
                let mut frame_pixel:Pixel = Pixel { x: WIN_WIDTH / 2 - 75, y: WIN_HEIGHT / 2 - 75, color:0x00_55_55_66 };
                line::horizontal(buf_view, &frame_pixel, 150);
                line::vertical(buf_view, &frame_pixel, 150);
                frame_pixel.y = WIN_HEIGHT / 2 + 75;
                line::horizontal(buf_view, &frame_pixel, 150);
                frame_pixel.x = WIN_WIDTH / 2 + 75;
                frame_pixel.y = WIN_HEIGHT / 2 - 75;
                line::vertical(buf_view, &frame_pixel, 150);

                //TODO: Make the FRAME pulsate using `oscillator` in inverted manner compared to the 'electricity'


                // "Quantum Universe" or something along those lines



        /*
                // line::horizontal(&mut buf_view, &Pixel { x: 10, y: 10, color }, frame_count%640);
                        line::vertical(&mut buf_view, &Pixel { x: 10, y: 0, color }, frame_count%480);
                        line::vertical(&mut buf_view, &Pixel { x: 13, y: 0, color }, 2+frame_count%480);


                        line::vertical(&mut buf_view, &Pixel { x: 630, y: 0, color }, frame_count%480);
                        line::vertical(&mut buf_view, &Pixel { x: 627, y: 0, color }, 3+frame_count%480);

                        draw::circle(&mut buf_view, &Pixel { x: 320, y: 320, color }, frame_count%42, 4);
                        draw::circle(&mut buf_view, &Pixel { x: 380, y: 301, color }, frame_count%65, 8);
                        draw::circle(&mut buf_view, &Pixel { x: 200, y: 201, color }, frame_count%173, 2);

                        draw::circle(&mut buf_view, &Pixel { x: WIN_WIDTH/2, y: WIN_HEIGHT/2, color }, frame_count%173, 4);

                        draw::circle(&mut buf_view, &Pixel { x: 0, y: 0, color }, frame_count%(WIN_WIDTH*2), 8);
                        draw::circle(&mut buf_view, &Pixel { x: WIN_WIDTH, y: 4, color }, frame_count%(WIN_WIDTH*2), 16);

        */

        // line::draw_line(&mut buf_view, &Pixel { x: frame_count%WIN_WIDTH/2, y: frame_count%WIN_HEIGHT,  color:0xff_00_ff_ff }, &Point{ x:200, y:frame_count%WIN_WIDTH/3 });

        // line::draw_line(&mut buf_view, &Pixel { x: 20, y: 20, color }, &Pixel{ x:120, y:120, color });
        // line::draw_line(&mut buf_view, &Pixel { x: 40, y: 40, color }, &Pixel{ x:400, y:400, color });

        // line::horizontal(buf_view, &Pixel{x:298, y:0 , color:0x00ffffff}, 44);

        //---------------------------
        // Read and handle keyboard
        //---------------------------
        let keys_pressed = window.get_keys_pressed(KeyRepeat::No);

        let keys_released = window.get_keys_released();

        if (keys_pressed.len() > 0 || keys_released.len() > 0) {
            handle_keyboard(&keys_pressed, &keys_released);
        }

        frame_count += 1;

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(buf_view, WIN_WIDTH as usize, WIN_HEIGHT as usize)
            .unwrap();

        match dev_window {
            Some(ref mut dev_window) => {
                dev_window
                    .update_with_buffer(buf_view, WIN_WIDTH as usize, WIN_HEIGHT as usize)
                    .unwrap();
            }
            _ => (),
        }
    }
}

//
//
//
//
// ------------------------------------------------------------------------------------------------------------------------
//
// PIXEL_PER_PAGE
// let screen_start = (screen_offset * WIN_WIDTH) as usize;
// let buf_view = &buffer[screen_start..screen_start + PIXEL_PER_PAGE];

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
