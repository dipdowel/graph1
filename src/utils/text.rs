/// Converts a UTF-8 character to a Vec<u16> with padding if necessary.
pub fn utf8_char_to_u16_vec(ch: char) -> Vec<u16> {
    // Convert the character to a UTF-8 byte array
    let mut utf8_bytes = ch.to_string().into_bytes();

    // Pad the byte array to ensure it has exactly 4 bytes
    utf8_bytes.resize(4, 0); // Pad with 0s to make it 4 bytes

    // Split the 4-byte array into two u16 values
    let part1 = u16::from_le_bytes([utf8_bytes[0], utf8_bytes[1]]);
    let part2 = u16::from_le_bytes([utf8_bytes[2], utf8_bytes[3]]);

    vec![part1, part2]
}

/// Converts a Vec<u16> back to the original UTF-8 character.
pub fn u16_vec_to_utf8_char(vec: Vec<u16>) -> char {
    assert_eq!(vec.len(), 2); // Ensure we have exactly two u16 values

    // Reassemble the u16 values into the original 4-byte UTF-8 array
    let reassembled_bytes = [
        (vec[0] & 0xFF) as u8,
        (vec[0] >> 8) as u8,
        (vec[1] & 0xFF) as u8,
        (vec[1] >> 8) as u8,
    ];

    // Remove padding (0s) from the byte array
    let reassembled_bytes: Vec<u8> = reassembled_bytes.into_iter().filter(|&x| x != 0).collect();

    // Convert the byte array back into a String (UTF-8 character)
    let reassembled_char = String::from_utf8(reassembled_bytes).unwrap();
    reassembled_char.chars().next().unwrap()
}

//TODO: ===========================================
//TODO: add unit tests for the two functions above!
//TODO: ===========================================
