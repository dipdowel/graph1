/// Converts a Vec<u32> with 0RGBA colors to a  bit array representing black and white colors
/// as `0` for black and `1` for white.
/// Every 0x0u32 value gets converted to `0`
/// and any other value is treated as white and is converted to `1`
pub fn rgb_to_one_bit_image(data: &Vec<u32>) -> Vec<u8> {
     let bit_array_size = (data.len() + 7) / 8; // <--- this does the same as `.div_ceil()` !
    // let bit_array_size = data.len().div_ceil(8);

    // bit array to accumulate the results
    let mut bit_array = Vec::with_capacity(bit_array_size);

    let mut current_byte = 0u8;
    let mut bit_index = 0;

    for value in data {
        // `0x00` translates to bit 0, and any other value translates to bit 1
        if *value != 0 {
            // shift current bit value in the current byte to their respective position
            current_byte |= 1 << (7 - bit_index);
        }
        bit_index += 1;

        // If we've filled a byte, push it to the array and reset
        if bit_index == 8 {
            bit_array.push(current_byte);
            current_byte = 0;
            bit_index = 0;
        }
    }

    // Push the remaining bits if any
    if bit_index != 0 {
        bit_array.push(current_byte);
    }

    bit_array
}


/// Converts a bit array (Vec<u8>) representation of black and white colours
/// to a Vec<u32> 0RGBA representation, where:
/// - bit `0` translates to `0x00_00_00_00`
/// - bit `1` translates to `0x00_ff_ff_ff`
pub fn one_bit_image_to_rgb(bit_array: &Vec<u8>) -> Vec<u32> {
    let mut data = Vec::with_capacity(bit_array.len() * 8);

    for byte in bit_array {
        for bit_index in 0..8 {
            // Extract the bit at the current position
            let bit = (*byte >> (7 - bit_index)) & 1;
            // Translate bit 0 to 0x00000000 and bit 1 to 0x00ffffff
            if bit == 0 {
                data.push(0x00_00_00_00);
            } else {
                data.push(0x00_ff_ff_ff);
            }
        }
    }
    data
}


#[cfg(test)]
mod tests {
    use super::*;

    // TESTS FOR `convert_to_bit_array()`
    //==============================================================================================
    #[test]
    fn test_convert_to_bit_array_basic() {
        let data = vec![0, 1, 0, 1];
        let bit_array = rgb_to_one_bit_image(&data);
        assert_eq!(bit_array, vec![0b_0101_0000]); // Expecting 0101 in the byte
    }

    #[test]
    fn test_convert_to_bit_array_full_byte() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let bit_array = rgb_to_one_bit_image(&data);
        assert_eq!(bit_array, vec![0b_1111_1111]); // Expecting all bits set in the byte
    }

    #[test]
    fn test_convert_to_bit_array_partial_byte() {
        let data = vec![1, 0, 1, 0, 1, 0];
        let bit_array = rgb_to_one_bit_image(&data);
        assert_eq!(bit_array, vec![0b_1010_1000]); // Expecting 1010 1000 in the byte
    }

    #[test]
    fn test_convert_to_bit_array_empty() {
        let data: Vec<u32> = vec![];
        let bit_array = rgb_to_one_bit_image(&data);
        assert_eq!(bit_array, vec![]); // Expecting empty bit array
    }

    #[test]
    fn test_convert_to_bit_array_mixed() {
        let data = vec![0, 1, 2, 0, 3, 0, 0, 4, 0];
        let bit_array = rgb_to_one_bit_image(&data);
        assert_eq!(bit_array, vec![0b_0110_1001,0b_0000_0000]); // Expecting 0110 1000 in first byte and 1000 0000 in the second byte
    }

    // TESTS FOR `...()`
    //==============================================================================================
    #[test]
    fn test_convert_from_bit_array_basic() {
        let bit_array = vec![0b0101_0000];
        let data = one_bit_image_to_rgb(&bit_array);
        assert_eq!(data, vec![0x00000000, 0x00ffffff, 0x00000000, 0x00ffffff, 0x00000000, 0x00000000, 0x00000000, 0x00000000]);
    }

    #[test]
    fn test_convert_from_bit_array_full_byte() {
        let bit_array = vec![0b1111_1111];
        let data = one_bit_image_to_rgb(&bit_array);
        assert_eq!(data, vec![0x00ffffff, 0x00ffffff, 0x00ffffff, 0x00ffffff, 0x00ffffff, 0x00ffffff, 0x00ffffff, 0x00ffffff]);
    }

    #[test]
    fn test_convert_from_bit_array_partial_byte() {
        let bit_array = vec![0b1010_1000];
        let data = one_bit_image_to_rgb(&bit_array);
        assert_eq!(data, vec![0x00ffffff, 0x00000000, 0x00ffffff, 0x00000000, 0x00ffffff, 0x00000000, 0x00000000, 0x00000000]);
    }

    #[test]
    fn test_convert_from_bit_array_empty() {
        let bit_array: Vec<u8> = vec![];
        let data = one_bit_image_to_rgb(&bit_array);
        assert_eq!(data, vec![]);
    }

    #[test]
    fn test_convert_from_bit_array_mixed() {
        let bit_array = vec![0b0110_1000, 0b1000_0000];
        let data = one_bit_image_to_rgb(&bit_array);
        assert_eq!(data, vec![
            0x00000000, 0x00ffffff, 0x00ffffff, 0x00000000,
            0x00ffffff, 0x00000000, 0x00000000, 0x00000000,
            0x00ffffff, 0x00000000, 0x00000000, 0x00000000,
            0x00000000, 0x00000000, 0x00000000, 0x00000000
        ]);



    }
}
