// use std::collections::HashMap;
use std::fmt::Debug;

use minifb::{Key, KeyRepeat};
use rand::seq::SliceRandom;
use rand::Rng;
use crate::tools::color_math::argb_math::argb_math;
use constants::*;

use crate::cube::Cube;
use crate::draw::line;
use crate::draw::star;

use crate::init::init_window::*;
use crate::input::handle_keyboard;
use crate::state::{init_app_state, APP_STATE};
use crate::tools::fill::fill;
use crate::tools::operations::ColorOperation;
use crate::tools::pixel_copy::trans_copy::trans_copy;
use crate::tools::pixel_copy::trans_copy_math::trans_copy_math;
use crate::tools::pixel_copy::trans_copy_math_multi_dest::trans_copy_math_multi_dest;
use crate::tools::pixel_copy::trans_copy_multi_dest::trans_copy_multi_dest;
use crate::tools::primitives::{
    Dimensions2d, Pixel, Point, Point3DF32, StarProperties, RectArea,
};

// Trait that provides the shuffle method.

// use crate::tools::draw::RectOptions;
// use tools::draw;
mod tools;

mod init;
// Make the global constants accessible here
mod constants;

mod cube;
mod draw;
mod input;
mod state;

const DEV_MODE: bool = true;

fn slice_buffer_in_4(buffer: &mut Vec<u32>) -> (&mut [u32], &mut [u32], &mut [u32], &mut [u32]) {
    // Calculate indices for splitting the vector into four equal parts
    let first_split = PIXEL_PER_PAGE;
    let second_split = PIXEL_PER_PAGE * 2;
    let third_split = PIXEL_PER_PAGE * 3;

    // Split the buffer to avoid borrowing conflicts
    let (first_half, second_half) = buffer.split_at_mut(second_split);
    let (buf_view_1, buf_view_2) = first_half.split_at_mut(first_split);
    let (buf_view_3, buf_view_4) = second_half.split_at_mut(first_split);

    (buf_view_1, buf_view_2, buf_view_3, buf_view_4)
}

fn main() {
    #![allow(unused)]

    init_app_state();

    let (mut window, mut dev_window) = init_window(DEV_MODE);

    // Let's reserve enough memory for 4 screens!
    let mut buffer: Vec<u32> = vec![DEFAULT_BG_COLOR; TOTAL_BUFFER_SIZE];

    let mut frame_count: u32 = 0;
    let mut window_title: String;
    let mut dev_window_title: String;

    // PIXEL_PER_PAGE
    let screen_start: usize = 0;

    let (buf_view_1, buf_view, buf_view_3, buf_view_4) = slice_buffer_in_4(&mut buffer);

    // let mut buf_view = buf_view_2;

    fill(buf_view_1, 0x00_44_00_00);
    fill(buf_view_3, 0x00_00_44_00);
    fill(buf_view_4, 0x00_00_00_44);

    match dev_window {
        Some(ref mut dev_window) => {
            // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
            dev_window
                .update_with_buffer(buf_view_1, WIN_WIDTH as usize, WIN_HEIGHT as usize)
                .expect("Failed to update dev window memory")
        }
        _ => (),
    }

    let color = 0x00_33_33_99;

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

    ////////////////////////
    ////////////////////////

    //////////////////////////////
    // CUBE
    //////////////////////////////
    let state = APP_STATE.lock().unwrap();

    let mut cube = Cube::new(
        Point3DF32 {
            x: 6_f32,
            y: -6_f32,
            z: 6_f32,
        },
        Point3DF32 {
            // x: state.hero_position.x as f32,
            // y: state.hero_position.y as f32,
            x: 64_f32,
            y: 64_f32,
            z: 0_f32,
        },
        (WIN_HEIGHT as f32 / 64_f32),
        0x00_ff_ff_ff,
    );

    let mut cube2 = Cube::new(
        Point3DF32 {
            x: 3_f32,
            y: -3_f32,
            z: 3_f32,
        },
        Point3DF32 {
            x: 64_f32,
            y: 64_f32,
            z: 0_f32,
        },
        (WIN_HEIGHT as f32 / 32_f32),
        0x00_00_00_ff,
    );

    let mut cube3 = Cube::new(
        Point3DF32 {
            x: 2_f32,
            y: -2_f32,
            z: 2_f32,
        },
        Point3DF32 {
            x: 64_f32,
            y: 64_f32,
            z: 0_f32,
        },
        (WIN_HEIGHT as f32 / 24_f32),
        0x00_ff_00_ff,
    );

    drop(state);

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
        let oscillator = if oscillator == 0 { 1 } else { oscillator };

        ////////////////////////////////////////////////////////////////////////////////////////////
        // Clear screen
        fill(buf_view_1, 0x00_44_00_00);

        fill(buf_view, 0x00_04_04_0F);

        // Draw kinda dotted grid
        let grid_factor: usize = 20;
        for i in 0..WIN_WIDTH_US {
            if i % grid_factor == 0 {
                buf_view[i] = 0x00_55_55_aa;
            }
        }
        for i in 1..WIN_HEIGHT_US / grid_factor {
            buf_view.copy_within(0..WIN_WIDTH_US, i * grid_factor * WIN_WIDTH_US);
        }

        ///////////// STAR START ////////////////////

        // Example usage
        let mut center_pixel = Pixel {
            x: 300 + oscillator as u32,
            y: 300 - oscillator as u32,
            color:0xff_00_ff_dd
        };
        center_pixel.color = argb_math(&0xff_ff_ff_ff, &(frame_count * oscillator as u32), &ColorOperation::Subtract);
        let star_props_1 = StarProperties {
            center: center_pixel,
            num_vertices: 18, // e.g., a 10-point star
            outer_radius: 50 + (oscillator / 2) as u32,
            inner_radius: 40,
            rotation_angle: frame_count as f64 * 1.5,
        };


        star(buf_view, &star_props_1);

        center_pixel.color = argb_math(&0xff_22_22_99, &(frame_count * oscillator as u32), &ColorOperation::Add);
        let star_props_2 = StarProperties {
            center: center_pixel,
            num_vertices: 18, // e.g., a 10-point star
            outer_radius: 40 + (oscillator / 4) as u32,
            inner_radius: 30,
            rotation_angle: frame_count as f64 * 1.5,
        };



        star(buf_view, &star_props_2);

        ///////////// STAR END ////////////////////

        //----------------------------------------------------------------------------------------------
        // THE CUBE dynamic
        //----------------------------------------------------------------------------------------------
        let mut state = APP_STATE.lock().unwrap();
        let dev_buffer_number = state.dev_buffer_number;
        // println!(":::: dev_buffer_number: {:?}", state);

        // draw::circle(buf_view, &Pixel { x: state.hero_position.x, y: state.hero_position.y, color: 0x00_ff_ff_ff }, 3,2);

        state.hero_position.x = (state.hero_position.x as i32 + state.hero_velocity.x) as u32;
        state.hero_position.y = (state.hero_position.y as i32 + state.hero_velocity.y) as u32;

        let mut translation: Option<Point3DF32> = None;

        if (state.hero_velocity.x != 0 || state.hero_velocity.y != 0) {
            translation = Some(Point3DF32 {
                // x: state.hero_position.x as f32,
                x: 0_f32,
                // x: state.hero_velocity.x as f32,
                // y: 6_f32*oscillator as f32,
                // y: oscillator as f32,
                // y: state.hero_velocity.y as f32,
                y: 0_f32,
                // z: oscillator as f32,
                z: 0_f32,
            });
        }

        // println!(">>> translation: {:?}", translation);

        // cube.render_frame(buf_view, 10.0 + frame_count as f32, &translation );

        cube.render_frame(buf_view_1, frame_count as f32, None);
        cube2.render_frame(buf_view_1, frame_count as f32, None);
        cube3.render_frame(buf_view_1, frame_count as f32, None);

        // buf_view.copy_from_slice(&buf_view_1[0..1*WIN_WIDTH as usize]);
        // buf_view.cop

        // buffer.copy_within(0..40 * WIN_WIDTH as usize, PIXEL_PER_PAGE * 2 + 20*WIN_WIDTH as usize);

        //TODO: This prevents the app from crashing but it's not accurate and needs an improvement!
        let hero_lim = 78;
        if state.hero_position.x < hero_lim {
            state.hero_position.x = hero_lim
        }
        if state.hero_position.x > WIN_WIDTH - hero_lim {
            state.hero_position.x = WIN_WIDTH - hero_lim
        }

        if state.hero_position.y < hero_lim {
            state.hero_position.y = hero_lim
        }
        if state.hero_position.y > WIN_HEIGHT - hero_lim {
            state.hero_position.y = WIN_HEIGHT - hero_lim
        }

        line::horizontal(
            buf_view,
            &Pixel {
                x: 0,
                y: WIN_HEIGHT / 8 * 5,
                color: 0x00_55_55_aa,
            },
            WIN_WIDTH,
        );

        /*
                // TESTED! WORKS!
                trans_copy(
                    buf_view_1,
                    buf_view,
                    &RectArea {
                        top_left: Point { x: 0, y: 0 },
                        dimensions: Dimensions2d { w: 154, h: 154 },
                    },
                    &Point { x: state.hero_position.x+140, y: state.hero_position.y-120},
                    &0x00_44_00_00
                );
        */

        /*
              // TESTED! WORKS!
                let dest_vec_pixel: Vec<Pixel> = vec![
                    Pixel { x: state.hero_position.x, y: state.hero_position.y+100, color: 0x00_cc_cc_cc },
                    Pixel { x: state.hero_position.x, y: state.hero_position.y, color: 0x00_88_88_88 },
                    Pixel { x: state.hero_position.x, y: state.hero_position.y-100, color: 0x00_44_44_44 },
                    Pixel { x: state.hero_position.x, y: state.hero_position.y-200, color: 0x00_00_00_00 },
                ];

                trans_copy_math_multi_dest(
                    buf_view_1,
                    buf_view,
                    &RectArea {
                        top_left: Point { x: 0, y: 0 },
                        dimensions: Dimensions2d { w: 154, h: 154 },
                    },
                    &dest_vec_pixel,
                    &0x00_44_00_00,
                    &ColorOperation::Add
                );
        */

        /*
                // TESTED! WORKS!
                let dest_vec_point: Vec<Point> = vec![
                    Point { x: state.hero_position.x, y: state.hero_position.y},
                    Point { x: state.hero_position.x+140, y: state.hero_position.y},
                    Point { x: state.hero_position.x+280, y: state.hero_position.y},
                ];

                trans_copy_multi_dest(
                    buf_view_1,
                    buf_view,
                    &RectArea {
                        top_left: Point { x: 0, y: 0 },
                        dimensions: Dimensions2d { w: 154, h: 154 },
                    },
                    &dest_vec_point,
                    &0x00_44_00_00,
                );
        */

        /*
            // TESTED! WORKS!
            trans_copy_math(
                buf_view_1,
                buf_view,
                &RectArea {
                    top_left: Point { x: 0, y: 0 },
                    dimensions: Dimensions2d { w: 154, h: 154 },
                },
                &Pixel { x: state.hero_position.x+140, y: state.hero_position.y, color: 0x00_22_22_55 },
                &0x00_44_00_00,
                &ColorOperation::Subtract
            );
        */

        drop(state);

        /*******************************************************************************************
        //  A WORKING SOLUTIONS:  Combine all the 4 buf_views into one and do copy within!
        //-------------------------------------------------------------------------------------------
        let mut complete_slice =  unsafe { std::slice::from_raw_parts_mut(buf_view_1.as_mut_ptr(), TOTAL_BUFFER_SIZE)};
        complete_slice.copy_within(0..100 * WIN_WIDTH as usize, PIXEL_PER_PAGE+ 300 * WIN_WIDTH as usize);
        *******************************************************************************************/

        // let mut v = complete_slice.to_vec();
        // v.copy_within(0..100*WIN_WIDTH as usize, PIXEL_PER_PAGE*2);

        // complete_slice

        //----------------------------------------------------------------------------------------------

        //---------------------------
        // Read and handle keyboard
        //---------------------------
        let keys_pressed = window.get_keys_pressed(KeyRepeat::No);

        // let mut state = APP_STATE.lock().unwrap();
        // state.keyboard_raw.keys_pressed = window.get_keys_pressed(KeyRepeat::No);
        // state.keyboard_raw.keys_released = window.get_keys_released();
        // drop(state); // release the mutex by moving `state` out of scope.

        handle_keyboard(
            &window.get_keys_pressed(KeyRepeat::No),
            &window.get_keys_released(),
        );

        // println!("State: {:?}", state);

        frame_count += 1;

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(buf_view, WIN_WIDTH as usize, WIN_HEIGHT as usize)
            .unwrap();

        match dev_window {
            Some(ref mut dev_window) => {
                let mut dev_buf_view: &mut [u32];
                match dev_buffer_number {
                    1 => dev_buf_view = buf_view_1,
                    2 => dev_buf_view = buf_view,
                    3 => dev_buf_view = buf_view_3,
                    _ => dev_buf_view = buf_view_4,
                }

                dev_window
                    .update_with_buffer(dev_buf_view, WIN_WIDTH as usize, WIN_HEIGHT as usize)
                    .unwrap();
            }
            _ => (),
        }
    }
}

//TODO: Make the FRAME pulsate using `oscillator` in inverted manner compared to the 'electricity'
/*
    // "Quantum Universe" or something along those lines

    // line::horizontal(&mut buf_view, &Pixel { x: 10, y: 10, color }, frame_count%640);
    line::vertical(
        &mut buf_view,
        &Pixel { x: 10, y: 0, color },
        frame_count % 480,
    );
    line::vertical(
        &mut buf_view,
        &Pixel { x: 13, y: 0, color },
        2 + frame_count % 480,
    );

    line::vertical(
        &mut buf_view,
        &Pixel {
            x: 630,
            y: 0,
            color,
        },
        frame_count % 480,
    );
    line::vertical(
        &mut buf_view,
        &Pixel {
            x: 627,
            y: 0,
            color,
        },
        3 + frame_count % 480,
    );

    draw::circle(
        &mut buf_view,
        &Pixel {
            x: 320,
            y: 320,
            color,
        },
        frame_count % 42,
        4,
    );
    draw::circle(
        &mut buf_view,
        &Pixel {
            x: 380,
            y: 301,
            color,
        },
        frame_count % 65,
        8,
    );
    draw::circle(
        &mut buf_view,
        &Pixel {
            x: 200 + (oscillator) as u32,
            y: 201,
            color,
        },
        frame_count % 173,
        2,
    );

    draw::circle(
        &mut buf_view,
        &Pixel {
            x: WIN_WIDTH / 2,
            y: WIN_HEIGHT / 2,
            color,
        },
        frame_count % 173,
        4,
    );

    draw::circle(
        &mut buf_view,
        &Pixel { x: 0, y: 0, color },
        frame_count % (WIN_WIDTH * 2),
        8,
    );
    draw::circle(
        &mut buf_view,
        &Pixel {
            x: WIN_WIDTH,
            y: 4,
            color,
        },
        frame_count % (WIN_WIDTH * 2),
        16,
    );
*/
// line::draw_line(&mut buf_view, &Pixel { x: frame_count%WIN_WIDTH/2, y: frame_count%WIN_HEIGHT,  color:0xff_00_ff_ff }, &Point{ x:200, y:frame_count%WIN_WIDTH/3 });

// line::draw_line(&mut buf_view, &Pixel { x: 20, y: 20, color }, &Pixel{ x:120, y:120, color });
// line::draw_line(&mut buf_view, &Pixel { x: 40, y: 40, color }, &Pixel{ x:400, y:400, color });

// line::horizontal(buf_view, &Pixel{x:298, y:0 , color:0x00ffffff}, 44);
/*
    //==================================================================================================
    //=== ELECTRO-BOX!
    //==================================================================================================
    points.shuffle(&mut rng);

    for i in 0..oscillator {
        line::between_two_points(
            &mut buf_view,
            &Pixel {
                x: points[i].x,
                y: points[i].y,
                color: 0x00_44_44_ff,
            },
            &points[i + 1],
        );
    }



    // DRAW FRAME
    let mut frame_pixel: Pixel = Pixel {
        x: WIN_WIDTH / 2 - 75,
        y: WIN_HEIGHT / 2 - 75,
        color: 0x00_55_55_66,
    };
    line::horizontal(buf_view, &frame_pixel, 150);
    line::vertical(buf_view, &frame_pixel, 150);
    frame_pixel.y = WIN_HEIGHT / 2 + 75;
    line::horizontal(buf_view, &frame_pixel, 150);
    frame_pixel.x = WIN_WIDTH / 2 + 75;
    frame_pixel.y = WIN_HEIGHT / 2 - 75;
    line::vertical(buf_view, &frame_pixel, 150);
    //==================================================================================================
*/
