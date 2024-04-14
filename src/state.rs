use std::sync::Mutex;
use crate::init::init_window::{WIN_HEIGHT, WIN_WIDTH};
use crate::tools::primitives::Point;

#[derive(Debug)]
pub struct HeroMoves {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct GlobalState {
    pub hero_velocity: HeroMoves,
    pub hero_position: Point,
    pub dev_buffer_number: u32,
}

pub static APP_STATE: Mutex<GlobalState> =
    Mutex::new(GlobalState {
        hero_velocity: HeroMoves {
            x: 0,
            y: 0,
        },
        hero_position: Point {
            x: WIN_WIDTH / 2,
            y: WIN_HEIGHT / 2,
        },
        dev_buffer_number:1
    });


pub fn init_app_state() {
    println!("\t\t IT DOES NOTHING!!!!!!!!!!!!!!");

    /*
    let mut state = APP_STATE.lock().unwrap();
    // Definitive list of keys known to the application
    state.keyboard_pressed .insert(Key::Up, false);
    state.keyboard_pressed.insert(Key::Down, false);
    state.keyboard_pressed.insert(Key::Left, false);
    state.keyboard_pressed.insert(Key::Right, false);

    state.keyboard_pressed.insert(Key::Key1, false);
    state.keyboard_pressed.insert(Key::Key2, false);
    state.keyboard_pressed.insert(Key::Key3, false);
    state.keyboard_pressed.insert(Key::Key4, false);

    state.keyboard_pressed.insert(Key::Space, false);
    state.keyboard_pressed.insert(Key::Enter, false);
*/

}
