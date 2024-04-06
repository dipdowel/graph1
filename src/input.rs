use minifb::{CursorStyle, Key, KeyRepeat};
pub fn handle_keyboard( keys_pressed: &Vec<Key>, keys_released: &Vec<Key>) {
    println!("handle_keyboard");

    keys_pressed
        .iter()
        .for_each(|key| match key {
            Key::Up => {
                println!("P [ ↑ ]");
                // key_pressed = "[ ↑ ]";
                // test_count += 1;
                // pressed_key = Some(*key);
            }
            Key::Down => {
                println!("P [ ↓ ]");
                // key_pressed = "[ ↓ ]";
                // test_count -= 1;
                // pressed_key = Some(*key);
            }
            _ => {
                // key_pressed = "[  ]";
                // pressed_key = None;
            }
        });

    keys_released.iter().for_each(|key| match key {
        _ => {
            // key_pressed = "[  ]";
            // pressed_key = None;
        }
    });


}