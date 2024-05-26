use minifb::{ Key, /*CursorStyle, KeyRepeat*/};
use crate::state::APP_STATE;

pub fn handle_keyboard( keys_pressed: &Vec<Key>, keys_released: &Vec<Key>) {
    // println!("handle_keyboard");
    let mut state = APP_STATE.lock().unwrap();

    keys_pressed
        .iter()
        .for_each(|key| match key {
            Key::Up => {
                state.hero_velocity.y = -4; // println!("P [ ↑ ]");
            }
            Key::Down => {
                state.hero_velocity.y = 4; // println!("P [ ↓ ]");
            }

            Key::Left => {
                state.hero_velocity.x = -4; // println!("P [ ↑ ]");
            }
            Key::Right => {
                state.hero_velocity.x = 4; // println!("P [ ↓ ]");
            }


            _ => {
                // key_pressed = "[  ]";
                // pressed_key = None;
            }
        });

    keys_released.iter().for_each(|key| match key {
        Key::Up => {
            state.hero_velocity.y = 0;
        }
        Key::Down => {
            state.hero_velocity.y = 0;
        }

        Key::Left => {
            state.hero_velocity.x = 0; // println!("P [ ↑ ]");
        }
        Key::Right => {
            state.hero_velocity.x = 0; // println!("P [ ↓ ]");
        }

        Key::Key1 => {
            state.dev_buffer_number = 1;
        }

        Key::Key2 => {
            state.dev_buffer_number = 2;
        }
        Key::Key3 => {
            state.dev_buffer_number = 3;
        }

        Key::Key4 => {
            state.dev_buffer_number = 4;
        }
        _ => {
            // key_pressed = "[  ]";
            // pressed_key = None;
        }
    });


}