use std::collections::HashMap;
use crate::graph1::text::font::PixelChar;
use crate::graph1::text::types::HashMapCharDescriptions;


const CHAR_7_7_0:PixelChar = PixelChar {
    w: 7,
    h: 7,
    margin_top: 0,
};

const CHAR_7_5_2:PixelChar = PixelChar {
    w: 7,
    h: 5,
    margin_top: 2,
};

const CHAR_7_3_0:PixelChar = PixelChar {
    w: 7,
    h: 3,
    margin_top: 0,
};

const CHAR_7_1_6:PixelChar = PixelChar {
    w: 7,
    h: 1,
    margin_top: 6,
};

const CHAR_6_6_1:PixelChar = PixelChar {
    w: 6,
    h: 6,
    margin_top: 1,
};


const CHAR_6_5_0:PixelChar = PixelChar {
    w: 6,
    h: 5,
    margin_top: 0,
};

const CHAR_5_7_0:PixelChar = PixelChar {
    w: 5,
    h: 7,
    margin_top: 0,
};

const CHAR_5_7_2:PixelChar = PixelChar {
    w: 5,
    h: 7,
    margin_top: 2,
};

const CHAR_5_5_2:PixelChar = PixelChar {
    w: 5,
    h: 5,
    margin_top: 2,
};

const CHAR_5_5_0:PixelChar = PixelChar {
    w: 5,
    h: 5,
    margin_top: 0,
};

const CHAR_5_3_0:PixelChar = PixelChar {
    w: 5,
    h: 3,
    margin_top: 0,
};

const CHAR_4_7_0:PixelChar = PixelChar {
    w: 4,
    h: 7,
    margin_top: 0,
};


const CHAR_4_3_2:PixelChar = PixelChar {
    w: 4,
    h: 3,
    margin_top: 2,
};
const CHAR_3_8_0:PixelChar = PixelChar {
    w: 3,
    h: 8,
    margin_top: 0,
};

const CHAR_3_7_0:PixelChar = PixelChar {
    w: 3,
    h: 7,
    margin_top: 0,
};



const CHAR_3_2_0:PixelChar = PixelChar {
    w: 3,
    h: 2,
    margin_top: 0,
};


const CHAR_3_1_3:PixelChar = PixelChar {
    w: 3,
    h: 1,
    margin_top: 3,
};

const CHAR_2_7_1:PixelChar = PixelChar {
    w: 2,
    h: 9,
    margin_top: 1,
};
const CHAR_2_6_1:PixelChar = PixelChar {
    w: 2,
    h: 9,
    margin_top: 1,
};

const CHAR_2_3_5:PixelChar = PixelChar {
    w: 2,
    h: 9,
    margin_top: 5,
};

const CHAR_2_3_0:PixelChar = PixelChar {
    w: 2,
    h: 9,
    margin_top: 0,
};


const CHAR_2_2_5:PixelChar = PixelChar {
    w: 2,
    h: 9,
    margin_top: 5,
};

const CHAR_1_7_0:PixelChar = PixelChar {
    w: 1,
    h: 9,
    margin_top: 0,
};

pub fn get_c_c_red_alert_inet0<'a>() -> HashMapCharDescriptions<'a> {

    // TODO: 1. Make a more efficient representation of the charmap by grouping symbols with the same properties into strings
    // TODO: 2. Maybe extract `CHAR_x_y_z` constants to a dedicated file? May be reused between different fonts
    // TODO: 3. Rename this file. Maybe make a dir with different font char maps and find a better name than "char map"?

    let mut char_map: HashMap<char, &PixelChar> = HashMap::new();

    char_map.insert(' ', &CHAR_7_7_0);

    for character in 'A'..='D' {
        char_map.insert(character, &CHAR_5_7_0);
    };

    char_map.insert('E', &CHAR_4_7_0);
    char_map.insert('F', &CHAR_4_7_0);
    char_map.insert('G', &CHAR_5_7_0);
    char_map.insert('H', &CHAR_5_7_0);
    char_map.insert('I', &CHAR_1_7_0);
    char_map.insert('J', &CHAR_4_7_0);
    char_map.insert('K', &CHAR_5_7_0);
    char_map.insert('L', &CHAR_4_7_0);
    char_map.insert('M', &CHAR_7_7_0);

    for character in 'N'..='V' {
        char_map.insert(character, &CHAR_5_7_0);
    };

    char_map.insert('W', &CHAR_7_7_0);

    for character in 'X'..='Z' {
        char_map.insert(character, &CHAR_5_7_0);
    };

    char_map.insert('a', &CHAR_5_5_2);
    char_map.insert('b', &CHAR_5_7_0);
    char_map.insert('c', &CHAR_5_5_2);
    char_map.insert('d', &CHAR_5_7_0);
    char_map.insert('e', &CHAR_5_5_2);
    char_map.insert('f', &CHAR_4_7_0);
    char_map.insert('g', &CHAR_5_7_2);
    char_map.insert('h', &CHAR_5_7_0);
    char_map.insert('i', &CHAR_1_7_0);
    char_map.insert('j', &CHAR_3_8_0);
    char_map.insert('k', &CHAR_4_7_0);
    char_map.insert('l', &CHAR_1_7_0);
    char_map.insert('m', &CHAR_7_5_2);
    char_map.insert('n', &CHAR_7_5_2);
    char_map.insert('o', &CHAR_5_5_2);
    char_map.insert('p', &CHAR_5_7_2);
    char_map.insert('q', &CHAR_5_7_2);
    char_map.insert('r', &CHAR_5_5_2);
    char_map.insert('s', &CHAR_5_5_2);
    char_map.insert('t', &CHAR_3_7_0);
    char_map.insert('u', &CHAR_5_5_2);
    char_map.insert('v', &CHAR_5_5_2);
    char_map.insert('w', &CHAR_7_5_2);
    char_map.insert('x', &CHAR_5_5_2);
    char_map.insert('y', &CHAR_5_7_2);
    char_map.insert('z', &CHAR_5_5_2);
    char_map.insert('0', &CHAR_5_7_0);
    char_map.insert('1', &CHAR_3_7_0);

    for character in '2'..='9' {
        char_map.insert(character, &CHAR_5_7_0);
    };

    char_map.insert('!', &CHAR_1_7_0);
    char_map.insert('"', &CHAR_3_2_0);
    char_map.insert('#', &CHAR_7_7_0);
    char_map.insert('$', &CHAR_5_7_0);
    char_map.insert('%', &CHAR_7_7_0);
    char_map.insert('&', &CHAR_4_7_0);
    char_map.insert('\'', &CHAR_2_3_0);
    char_map.insert('(', &CHAR_3_7_0);
    char_map.insert(')', &CHAR_3_7_0);
    char_map.insert('*', &CHAR_6_5_0);
    char_map.insert('+', &CHAR_5_5_0);
    char_map.insert(',', &CHAR_2_3_5);
    char_map.insert('-', &CHAR_3_1_3);
    char_map.insert('.', &CHAR_2_2_5);
    char_map.insert('/', &CHAR_6_6_1);
    char_map.insert(':', &CHAR_2_6_1);
    char_map.insert(';', &CHAR_2_7_1);
    char_map.insert('<', &CHAR_4_7_0);
    char_map.insert('=', &CHAR_4_3_2);
    char_map.insert('>', &CHAR_4_7_0);
    char_map.insert('?', &CHAR_4_7_0);
    char_map.insert('@', &CHAR_7_7_0);
    char_map.insert('[', &CHAR_3_7_0);
    char_map.insert('\\', &CHAR_6_6_1);
    char_map.insert(']', &CHAR_3_7_0);
    char_map.insert('^', &CHAR_5_3_0);
    char_map.insert('_', &CHAR_7_1_6);
    char_map.insert('`', &CHAR_2_3_0);
    char_map.insert('{', &CHAR_3_7_0);
    char_map.insert('|', &CHAR_1_7_0);
    char_map.insert('}', &CHAR_3_7_0);
    char_map.insert('~', &CHAR_7_3_0);

    return char_map;
}
