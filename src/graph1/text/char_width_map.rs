use std::collections::HashMap;

pub fn get_c_c_red_alert_inet0<'a>() -> HashMap<char, u8> {

    // The resulting map: each character in the charset mapped to the character width (in pixels)
    let mut char_map: HashMap<char, u8> = HashMap::new();

    // Map character width to the characters with that width
    let mut width_to_chars: HashMap<u8, &str> = HashMap::new();
    width_to_chars.insert(7, "MWmw#%@_~");
    width_to_chars.insert(6, "*/\\");
    width_to_chars.insert(5, "ABCDGHKNOPQRSTUVXYZabcdeghnopqrsuvxyz023456789$+^");
    width_to_chars.insert(4, " EFJLfk&<=>?");
    width_to_chars.insert(3, "jt1\"()-[]{}");
    width_to_chars.insert(2, "',.:;`");
    width_to_chars.insert(1, "Iil!|");

    for (width, chars_same_w) in width_to_chars {
        for ch in chars_same_w.chars() {
            char_map.insert(ch, width);
        }
    }
    return char_map;
}

//
// let chars_same_w = " MWmw#%@_~";
// for ch in chars_same_w.chars() {
//     char_map.insert(ch, 7);
// }
//
// let chars_same_w = "*/\\";
// for ch in chars_same_w.chars() {
//     char_map.insert(ch, 6);
// }
//
//
// let chars_same_w = "ABCDGHKNOPQRSTUVXYZabcdeghnopqrsuvxyz023456789$+^";
// for ch in chars_same_w.chars() {
//     char_map.insert(ch, 5);
// }
//
// let chars_same_w = "EFJLfk&<=>?";
// for ch in chars_same_w.chars() {
//     char_map.insert(ch, 4);
// }
//
// let chars_same_w = "jt1\"()-[]{}";
// for ch in chars_same_w.chars() {
//     char_map.insert(ch, 3);
// }
//
// let chars_same_w = "',.:;`";
// for ch in chars_same_w.chars() {
//     char_map.insert(ch, 2);
// }
//
// let chars_same_w = "Iil!|";
// for ch in chars_same_w.chars() {
//     char_map.insert(ch, 1);
// }
