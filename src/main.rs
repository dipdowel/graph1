// use std::collections::HashMap;

// use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use image::EncodableLayout;
use chrono::prelude::*;
// use crate::graph1::utils::color_math::argb_math::argb_math;
// use crate::graph1::draw::curves::draw_bezier_curve;
// use star::StarProperties;
// use crate::graph1::draw::star;
// graph1::utils::
// use crate::graph1::utils::color_math::operations::ColorOperation;

use constants::*;
use minifb::{Key, KeyRepeat};
// use rand::seq::SliceRandom;
use rand::Rng;

use crate::cube::Cube;

use crate::graph1::draw::line;

use crate::graph1_core::context::{ContextWindow, GraphContext};

use crate::graph1::tools::fill;

use crate::init::init_window::*;
use crate::input::handle_keyboard;
use crate::state::{APP_STATE, init_app_state};

// use image::io::Reader as ImageReader;
// use image::GenericImageView;

use self::graph1::primitives::primitives::{
    Pixel, Point, Point3DF32,
};
// use self::graph1::utils::pixel_copy::trans_copy::trans_copy;
// use self::graph1::utils::pixel_copy::trans_copy_math::trans_copy_math;
// use self::graph1::utils::pixel_copy::trans_copy_math_multi_dest::trans_copy_math_multi_dest;
// use self::graph1::utils::pixel_copy::trans_copy_multi_dest::trans_copy_multi_dest;

// Trait that provides the shuffle method.
// use crate::tools::draw::RectOptions;
// use tools::draw;

mod tools;

mod init;
// Make the global constants accessible here
mod constants;

mod cube;
mod graph1;
use crate::animation_context::{AnimationContext, Oscillators};
use crate::graph1::text::font::Spacing;
use crate::graph1::text::font_embedder::{EmbeddedFonts, instantiate_embedded_font};
use crate::graph1::text::{char_width_map, font, font_constants, printer};
use crate::graph1::utils::mem::slice_buffer_in_4;
use graph1::graph1_core;
use crate::graph1::utils::bit_operations;
use crate::graph1::utils::text::utf8_char_to_u16_vec;

mod about;
mod animation_context;
mod input;
mod scenes;
mod state;

const DEV_MODE: bool = true;

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

    fill::buffer(buf_view_1, 0x00_44_00_00);
    fill::buffer(buf_view_3, 0x00_ee_ee_ee);
    fill::buffer(buf_view_4, 0x00_00_00_44);

    let context_window: ContextWindow = ContextWindow {
        w: WIN_WIDTH,
        h: WIN_HEIGHT,
        w_usize: WIN_WIDTH_US,
        h_usize: WIN_HEIGHT_US,
    };

    let mut ctx: GraphContext = GraphContext {
        buf_view,
        win: &context_window,
        default_color: 0x00_ff_00_00,
    };

    let mut ani_ctx: AnimationContext = AnimationContext {
        frame_count: 0,
        oscillators: Oscillators { o1: 231 },
    };

    let mut ctx_draft: GraphContext = GraphContext {
        buf_view: buf_view_1,
        win: &context_window,
        default_color: 0x00_00_ff_00,
    };

    match dev_window {
        Some(ref mut dev_window) => {
            // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
            dev_window
                .update_with_buffer(ctx_draft.buf_view, ctx.win.w_usize, ctx.win.h_usize)
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
            x: rng.gen_range(ctx.win.w / 2 - 75..ctx.win.w / 2 + 75),
            y: rng.gen_range(ctx.win.h / 2 - 75..ctx.win.h / 2 + 75),
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
     //----------------------------------------------------------------------------------------------
    // TODO: Move font reading / writing functionality into a separate project!

    let mut font_image_buf: Vec<u32> = Vec::new();

    // TODO: extract the path after the development will have been finished.
    let font_name = "assets/fonts/c_c_red_alert_inet0_ext.png";
    // let font_name = "assets/01_test_palette.png";
    // let font_name = "assets/fonts/empty.dat"; // for testing

    if (!graph1::utils::io::file::read_image(font_name, &mut font_image_buf)) {
        panic!("Font initialization failed!");
    };

    println!("{}", "-".repeat(20));
    println!(">>> image_buf.len: {:?}", font_image_buf.len());

    // ==============================================================================================

    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // =========[ BEGIN BITWISE writing font data ]=================================================

    //^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    //  Header consists of a bunch of `u16` values:
    // -----------------------------------------------------------------------------------
    // [00] - magic number `CBF0` for "Compact Bitmap F0nt"
    // [01] - version of CBF format
    // [02] - size of `font_name`, size of the name of the font
    // [03] - size of `author_signature`, size of the author's name
    // [04] - size of `char_order`
    // [05] - size of `char_width`
    // [06] - font image width
    // [07] - font image height
    // [08] - spacing props: lower byte -- kerning, higher byte -- leading.
    // [09] - UTF8 default char: lower 2 bytes.
    // [10] - UTF8 default char: higher 2 bytes.
    // [11] - version of the font
    // [12] - date: year
    // [13] - date: lower byte -- day, higher byte -- month.
    // -----------------------------------------------------------------------------------
    // Font body fields:
    //      * font_name -- A string with the name of the font.
    //      * author_signature -- A string with the name of the author of the font.
    //      * char_order -- order of characters in the font.
    //      * char_widths -- how many pixels wide a char is. In the order of `char_order`
    //      * font_pixel_data -- 1-bit image data of the font (0 for black, 1 for white)

    // PREPARE DATA FOR THE HEADER
    // ---------------------------
    let font_image_path = "src/graph1/text/cbf_data/c_c_red_alert_inet0.cbf";
    let font_image_width: u16 = 518;
    let font_image_height: u16 = 9;
    let spacing_props:u16 = 0x_04_02; // 0x_leading_kerning
    let font_version:u16 = 1002;


    let now = Utc::now();

    // Extract year, month, and day as integers
    let year = now.year() as u16;
    let month = now.month() as u8;
    let day = now.day() as u8;

    let year: u16 = 2008;
    let month: u8 = 02;
    let day: u8 = 06;
    let month_day: u16 = ((month as u16) << 8) | (day as u16);


    // let test_default_char = '😀';
    let test_default_char = '?';
    println!(">>> test_default_char len: {}", test_default_char.len_utf8());

    let default_char_parts = utf8_char_to_u16_vec(test_default_char);


    let font_name = "C&C Red Alert [internet]".as_bytes();
    let author_signature = "N3tRunn3r (N3tRunn3r@hotmail.de)".as_bytes();

    // Char order in the font as bytes
    let char_order = font::DEFAULT_CHAR_ORDER.as_bytes();
    let char_map = char_width_map::get_c_c_red_alert_inet0(1);
    let char_widths: Vec<u8> = font::DEFAULT_CHAR_ORDER.chars().map(|ch| { *char_map.get(&ch).unwrap() }).collect();

    // VALIDATE VARIABLE-LENGTH DATA SIZES
    // -----------------------------------
    let max_string_size = (u16::MAX- 1) as usize;
    if(font_name.len() > max_string_size){
        panic!("Font name is too big!"); // TODO: replace with returning a result with enum variant `NameTooBig(usize)`
    }
    if(char_order.len() > max_string_size){
        panic!("Char order is too big!"); // TODO: replace with returning a result with enum variant `CharOrderTooBig(usize)`
    }

    if(char_widths.len() > max_string_size){
        panic!("Char widths are is too big!"); // TODO: replace with returning a result with enum variant `CharWidthTooBig(usize)`
    }

    // FILL IN THE HEADER
    // ------------------
    let mut font_header: Vec<u16> = vec![0; 14];

    // File identification
    font_header[0] = font_constants::CBF_MAGIC_NUMBER; // The `CBF0` magic number
    font_header[1] = font_constants::CBF_VERSION;       // CBF format version

    // Sizes of the variable-length data fields
    font_header[2] = font_name.len() as u16;
    font_header[3] = author_signature.len() as u16;
    font_header[4] = char_order.len() as u16;
    font_header[5] = char_widths.len() as u16;

    // Font image and font properties
    font_header[6] = font_image_width;
    font_header[7] = font_image_height;
    font_header[8] = spacing_props;

    // The font's default char (utf8, hence can be up to 4 bytes, hence 2 u16 values needed.
    font_header[9] = default_char_parts[0];
    font_header[10] = default_char_parts[1];

    // Date of creation of the font
    font_header[11] = font_version;
    font_header[12] = year;
    font_header[13] = month_day;

    // FILL IN THE BODY
    // ----------------
    let mut font_body: Vec<u8> = Vec::from(font_name);
    font_body.extend(author_signature);
    font_body.extend(char_order);
    font_body.extend(char_widths);
    font_body.extend(bit_operations::rgb_to_one_bit_image(&font_image_buf).as_bytes());


    let mut file = File::create(font_image_path).unwrap();

    for num in font_header {
        file.write_all(&num.to_le_bytes()).unwrap(); // Using little endian encoding
    }
    for num in font_body {
        file.write_all(&num.to_le_bytes()).unwrap(); // Using little endian encoding
    }


    // =========[ END BITWISE writing font data ]===================================================
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // =========[ BEGIN reading font data ]=========================================================
    /*
        let mut file = File::open(font_image_path).unwrap();
        let mut buffer = vec![0u8; 8]; // Buffer to hold the first 4 bytes
        file.read_exact(&mut buffer).unwrap();

        // Convert the first 4 bytes into a Vec<u16>
        let font_header_raw: Vec<u16> = buffer
            .chunks(2)
            .map(|chunk| {
                let mut array = [0u8; 2];
                array.copy_from_slice(chunk);
                u16::from_le_bytes(array) // Decode from little endian and return
            })
            .collect();

        // Read the remaining bytes into a Vec<u8>
        let mut font_body_raw = vec![];
        file.read_to_end(&mut font_body_raw).unwrap();


        // let font_header: Vec<u32> = font_header_raw.into_iter().map(|x| x as u32).collect();
        let font_header: Vec<u32> = font_header_raw.into_iter().map(u32::from).collect();

        let font_body: Vec<u32> = font_body_raw
            .into_iter()
            .map(|x| if x == 0x0_u8 { 0x0_u32 } else { 0x00_ff_ff_ff })
            .collect();

        let font_1_width = font_header[0];
        let font_1_height = font_header[1];
    */
    // =========[ END writing font data ]=========================================================

    // =========[ BEGIN EmBEDDiNG font data ]=========================================================

    // let mut data: &[u8] = include_bytes!("graph1/text/cbf_data/c_c_red_alert_inet0.cbf");
    //
    // let mut buffer = vec![0u8; 8]; // Buffer to hold the first 4 bytes
    // data.read_exact(&mut buffer).unwrap();
    //
    // // Convert the first 4 bytes into a Vec<u16>
    // let font_header_raw: Vec<u16> = buffer
    //     .chunks(2)
    //     .map(|chunk| {
    //         let mut array = [0u8; 2];
    //         array.copy_from_slice(chunk);
    //         u16::from_le_bytes(array) // Decode from little endian and return
    //     })
    //     .collect();
    //
    // // Read the remaining bytes into a Vec<u8>
    // let mut font_body_raw = vec![];
    // data.read_to_end(&mut font_body_raw).unwrap();
    //
    // // let font_header: Vec<u32> = font_header_raw.into_iter().map(|x| x as u32).collect();
    // let font_header: Vec<u32> = font_header_raw.into_iter().map(u32::from).collect();
    //
    // let font_body: Vec<u32> = font_body_raw
    //     .into_iter()
    //     .map(|x| if x == 0x0_u8 { 0x0_u32 } else { 0x00_ff_ff_ff })
    //     .collect();
    //
    // let font_1_width = font_header[0];
    // let font_1_height = font_header[1];

    // =========[ END writing font data ]=========================================================

    // ==============================================================================================

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
        ani_ctx.oscillators.o1 = oscillator;

        ////////////////////////////////////////////////////////////////////////////////////////////
        // Clear screen

        fill::buffer(ctx_draft.buf_view, 0x00_44_00_00);
        // fill::buffer(ctx.buf_view, 0x00_04_04_0F);
        fill::buffer(ctx.buf_view, 0x00_bb_bb_bb);
        fill::buffer(ctx.buf_view, 0x00_66_33_66);
        // fill::buffer(ctx.buf_view, 0x00_ff_ff_ff);

        // drop(ctx.buf_view);

        ////////////////////////////////////////////////////////////////////////////////////////////
        // === SCENES START === ////////////////////////////////////////////////////////////////////

        // scenes::s00_dot_grid::render(&mut ctx);
        // scenes::s01_bezier::render(&mut ctx, &ani_ctx);
        // scenes::s02_star::render(&mut ctx, &ani_ctx);

        // === SCENES END === //////////////////////////////////////////////////////////////////////
        ////////////////////////////////////////////////////////////////////////////////////////////

        let text_data = vec![
            "Alright, let's see what we can see... Everybody's online, looking good!",
            "Next line of text here.",
            "We all are living in a yellow submarine, yellow submarine, yellow submarine...",
            "Boys and Girls come out to play,",
            "On the busy motorway",
            "Окей, что тут?",
        ];

        let color_props: printer::ColorProperties = printer::ColorProperties {
            color: Some(0x00_ff_44_ff),
            // color: None,
            // color_transformer: None,
            color_transformer: Some( |color:u32, x:u32, y:u32, w:u32, h:u32| -> u32  {
                if y % 2 == 0 {
                    return 0x00_ff_ee_ff;
                }
                return 0x00_99_33_99
            })
        };


        let font = instantiate_embedded_font(EmbeddedFonts::CCRedAlertInet0, 1,  None, None);


        let font_2x = instantiate_embedded_font(
            EmbeddedFonts::CCRedAlertInet0,
            2,
            Some(Spacing {
                kerning_px: 3,
                leading_px: 2,
            }),
            None,
        );
        // let font_4x = instantiate_embedded_font(EmbeddedFonts::CCRedAlertInet0, 4, None,None);

        // Debug output of the entire charset
        printer::print_line(
            &mut ctx,
            &Point { x: 100, y: 100 },
            &font,
            &color_props,
            font::DEFAULT_CHAR_ORDER,
        );

        printer::print_line(
            &mut ctx,
            &Point { x: 100, y: 130 },
            &font,
            &color_props,
            &font.to_string()
        );


        //
        // printer::print_line(
        //     &mut ctx,
        //     &Point { x: 100, y: 130 },
        //     &font,
        //     &color_props,
        //     ""
        // );

        // printer::print(
        //     &mut ctx,
        //     &Point { x: 100, y: 120 },
        //     // &font_4x,
        //     &font,
        //     &color_props,
        //     &text_data,
        // );

        printer::print(
            &mut ctx,
            &Point { x: 10, y: 200 },
            &font_2x,
            &color_props,
            &text_data,
        );

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

        cube.render_frame(&mut ctx_draft, frame_count as f32, None);
        cube2.render_frame(&mut ctx_draft, frame_count as f32, None);
        cube3.render_frame(&mut ctx_draft, frame_count as f32, None);

        // buf_view.copy_from_slice(&buf_view_1[0..1*WIN_WIDTH as usize]);
        // buf_view.cop

        // buffer.copy_within(0..40 * WIN_WIDTH as usize, PIXEL_PER_PAGE * 2 + 20*WIN_WIDTH as usize);

        //==================================================================================================
        //TODO: This prevents the app from crashing but it's not accurate and needs an improvement!
        let hero_lim = 78;
        if state.hero_position.x < hero_lim {
            state.hero_position.x = hero_lim
        }
        if state.hero_position.x > ctx.win.w - hero_lim {
            state.hero_position.x = ctx.win.w - hero_lim
        }

        if state.hero_position.y < hero_lim {
            state.hero_position.y = hero_lim
        }
        if state.hero_position.y > ctx.win.h - hero_lim {
            state.hero_position.y = ctx.win.h - hero_lim
        }

        line::horizontal(
            &mut ctx,
            &Pixel {
                x: 0,
                y: WIN_HEIGHT / 8 * 5,
                color: 0x00_55_55_aa,
            },
            WIN_WIDTH,
        );
        //==================================================================================================

        /*

                // !!!! THIS IS WHERE THE CUBE GETS COPIED!!!!!
                trans_copy(
                    ctx_draft.buf_view,
                    ctx.buf_view,
                    &RectArea {
                        top_left: Point { x: 0, y: 0 },
                        dimensions: Dimensions2d { w: 154, h: 154 },
                    },
                    &Point {
                        x: state.hero_position.x + 140,
                        y: state.hero_position.y - 120,
                    },
                    &0x00_44_00_00,
                    ctx.win,
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
                            ctx_draft.buf_view,
                            ctx.buf_view,
                            &RectArea {
                                top_left: Point { x: 0, y: 0 },
                                dimensions: Dimensions2d { w: 154, h: 154 },
                            },
                            &dest_vec_pixel,
                            &0x00_44_00_00,
                            &ColorOperation::Add,
                            ctx.win
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
                            ctx_draft.buf_view,
                            ctx.buf_view,
                            &RectArea {
                                top_left: Point { x: 0, y: 0 },
                                dimensions: Dimensions2d { w: 154, h: 154 },
                            },
                            &dest_vec_point,
                            &0x00_44_00_00,
                            ctx.win
                        );
        */
        /*

                    // TESTED! WORKS!
                    trans_copy_math(
                        ctx_draft.buf_view,
                        ctx.buf_view,
                        &RectArea {
                            top_left: Point { x: 0, y: 0 },
                            dimensions: Dimensions2d { w: 154, h: 154 },
                        },
                        &Pixel { x: state.hero_position.x+140, y: state.hero_position.y, color: 0x00_22_22_55 },
                        &0x00_44_00_00,
                        &ColorOperation::Subtract,
                        ctx.win
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
        ani_ctx.frame_count = frame_count;

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(ctx.buf_view, ctx.win.w_usize, ctx.win.h_usize)
            .unwrap();

        match dev_window {
            Some(ref mut dev_window) => {
                let mut dev_buf_view: &mut [u32];
                match dev_buffer_number {
                    1 => dev_buf_view = ctx_draft.buf_view,
                    2 => dev_buf_view = ctx.buf_view,
                    3 => dev_buf_view = buf_view_3,
                    _ => dev_buf_view = buf_view_4,
                }

                dev_window
                    .update_with_buffer(dev_buf_view, ctx.win.w_usize, ctx.win.h_usize)
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
