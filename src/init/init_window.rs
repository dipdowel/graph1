use minifb::{CursorStyle, /*ScaleMode,*/ Window, WindowOptions};
// use std::fmt::Debug;
use std::time::Duration;

/*
    This file contains all the constants and functions needed to initialize the app window(s).
*/

pub const MONITOR_WIDTH: u32 = 3440;
pub const MONITOR_HEIGHT: u32 = 1440;

// pub const WIN_WIDTH: u32 = (1024.0*1.4) as u32;
// pub const WIN_HEIGHT: u32 = (768.0*1.4) as u32;

// pub const WIN_WIDTH: u32 = 1024;
// pub const WIN_HEIGHT: u32 = 768;
pub const WIN_WIDTH: u32 = 800;
pub const WIN_HEIGHT: u32 = 600;

pub const WIN_WIDTH_US: usize = WIN_WIDTH as usize;
pub const WIN_HEIGHT_US: usize = WIN_HEIGHT as usize;




pub const WIN_X: u32 = MONITOR_WIDTH / 2 - WIN_WIDTH / 2;
pub const WIN_Y: u32 = MONITOR_HEIGHT / 2 - WIN_HEIGHT / 2;

pub const WIN_NAME_PREFIX: &str = "GF ";
pub const DEV_WIN_NAME_PREFIX: &str = "[dev] ";

pub fn init_window(include_debug: bool) -> (Window, Option<Window>) {
    // first element in the returned tuple is the main window
    let mut window = Window::new(
        WIN_NAME_PREFIX,
        WIN_WIDTH as usize,
        WIN_HEIGHT as usize,
        WindowOptions::default(),
        /*
        WindowOptions {
            borderless: true, // Set to true for a borderless window
            title: true,
            resize: true,
            scale: minifb::Scale::X1,
            // aspect_ratio: minifb::AspectRatio::Free,
            topmost: true,
            transparency: true,
            none: false,
            // fullscreen: true, // Enable full-screen mode
            // scale_mode: ScaleMode::AspectRatioStretch,
            scale_mode: ScaleMode::Center,
        },
        */
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut dev_window: Option<Window> = None;

    if include_debug {
        // second element in the returned tuple is the dev window
        dev_window = Some(
            Window::new(
                DEV_WIN_NAME_PREFIX,
                WIN_WIDTH as usize,
                WIN_HEIGHT as usize,
                WindowOptions::default(),
            )
            .unwrap_or_else(|e| {
                panic!("{}", e);
            }),
        );
    }

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(Duration::from_micros(16600)));
    // Position windows on the screen
    window.set_position((WIN_X - 400) as isize, WIN_Y as isize);
    window.set_cursor_style(CursorStyle::Crosshair);

    if include_debug {
        if let Some(ref mut dw) = dev_window {
            dw.limit_update_rate(Some(Duration::from_micros(16600)));

            dw.set_position(0_isize, 0_isize);
        }
    }

    return (window, dev_window);
}
