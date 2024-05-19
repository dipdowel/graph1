use std::collections::HashMap;

pub fn get_c_c_red_alert_inet0<'a>() -> HashMap<char, u8> {

    // TODO: 1. Make a more efficient representation of the charmap by grouping symbols with the same properties into strings
    // TODO: 2. Maybe extract `CHAR_x_y_z` constants to a dedicated file? May be reused between different fonts
    // TODO: 3. Rename this file. Maybe make a dir with different font char maps and find a better name than "char map"?

    let mut char_map: HashMap<char, u8> = HashMap::new();

    char_map.insert(' ', 7);

    for character in 'A'..='D' {
        char_map.insert(character, 5);
    };

    char_map.insert('E', 4);
    char_map.insert('F', 4);
    char_map.insert('G', 5);
    char_map.insert('H', 5);
    char_map.insert('I', 1);
    char_map.insert('J', 4);
    char_map.insert('K', 5);
    char_map.insert('L', 4);
    char_map.insert('M', 7);

    for character in 'N'..='V' {
        char_map.insert(character, 5);
    };

    char_map.insert('W', 7);

    for character in 'X'..='Z' {
        char_map.insert(character, 5);
    };

    char_map.insert('a', 5);
    char_map.insert('b', 5);
    char_map.insert('c', 5);
    char_map.insert('d', 5);
    char_map.insert('e', 5);
    char_map.insert('f', 4);
    char_map.insert('g', 5);
    char_map.insert('h', 5);
    char_map.insert('i', 1);
    char_map.insert('j', 3);
    char_map.insert('k', 4);
    char_map.insert('l', 1);
    char_map.insert('m', 7);
    char_map.insert('n', 5);
    char_map.insert('o', 5);
    char_map.insert('p', 5);
    char_map.insert('q', 5);
    char_map.insert('r', 5);
    char_map.insert('s', 5);
    char_map.insert('t', 3);
    char_map.insert('u', 5);
    char_map.insert('v', 5);
    char_map.insert('w', 7);
    char_map.insert('x', 5);
    char_map.insert('y', 5);
    char_map.insert('z', 5);
    char_map.insert('0', 5);
    char_map.insert('1', 3);

    for character in '2'..='9' {
        char_map.insert(character, 5);
    };

    char_map.insert('!', 1);
    char_map.insert('"', 3);
    char_map.insert('#', 7);
    char_map.insert('$', 5);
    char_map.insert('%', 7);
    char_map.insert('&', 4);
    char_map.insert('\'', 2);
    char_map.insert('(', 3);
    char_map.insert(')', 3);
    char_map.insert('*', 6);
    char_map.insert('+', 5);
    char_map.insert(',', 2);
    char_map.insert('-', 3);
    char_map.insert('.', 2);
    char_map.insert('/', 6);
    char_map.insert(':', 2);
    char_map.insert(';', 2);
    char_map.insert('<', 4);
    char_map.insert('=', 4);
    char_map.insert('>', 4);
    char_map.insert('?', 4);
    char_map.insert('@', 7);
    char_map.insert('[', 3);
    char_map.insert('\\', 6);
    char_map.insert(']', 3);
    char_map.insert('^', 5);
    char_map.insert('_', 7);
    char_map.insert('`', 2);
    char_map.insert('{', 3);
    char_map.insert('|', 1);
    char_map.insert('}', 3);
    char_map.insert('~', 7);

    return char_map;
}
