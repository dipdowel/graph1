use crate::primitives::math::MinMax;
use crate::utils::math::rng::XorShiftRng;


#[derive(Debug)]
pub enum ShuffleSliceError {
    EmptySlice,
    SliceTooBig,
}

/// Shuffle the items in the slice in place using the provided RNG.
/// **NB!:** This function modifies the original slice.
/// # Arguments
/// * `items` - The slice of items to be shuffled
/// * `rng` - The random number generator to use for shuffling
/// # Returns
/// * `Ok(())` if the shuffle was successful
/// * `Err(ShuffleSliceError)` if the slice is empty or too large
pub fn slice<T>(items: &mut [T], rng: &mut XorShiftRng) ->Result<(), ShuffleSliceError> {

    if items.len() == 0 {
        return Err(ShuffleSliceError::EmptySlice);
    }
    if items.len() >= (u32::MAX as usize) {
        return Err(ShuffleSliceError::SliceTooBig);
    }

    // No need to shuffle if there's only one item
    if items.len() == 1 {
        return Ok(());
    }


    let random_u32: Vec<u32> = rng.get_vec_u32(
        items.len(),
        &MinMax {
            min: 0,
            max: items.len() as u32,
        },
    );

    for (i, random_value) in random_u32.iter().enumerate() {
        items.swap(i, *random_value as usize);
    }

     Ok(())
}

#[cfg(test)]
mod tests {

    const TEST_DATA: &[u32] = &[
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
        49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72,
        73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100,
        101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120,
        121, 122, 123, 124, 125, 126, 127, 128
    ];

    use super::*;

    #[test]
    fn test_shuffle() {
        let mut rng = XorShiftRng::new(12345, 67890);
        let mut data = Vec::from(TEST_DATA);

        // Ensure the data is initially in the original order
        assert_eq!(data, TEST_DATA);
        // Shuffle the data
        slice(&mut data, &mut rng);
        // Ensure the data has been shuffled (not equal to original)
        assert_ne!(data, TEST_DATA);

        // Sort the shuffled data and ensure it matches the original
        let mut sorted_data = data.clone();
        sorted_data.sort();
        assert_eq!(sorted_data, TEST_DATA);
    }
    #[test]
    fn test_empty_slice() {
        let mut rng = XorShiftRng::new(12345, 67890);
        let mut data: Vec<u32> = Vec::new();

        let result = slice(&mut data, &mut rng);
        assert!(matches!(result, Err(ShuffleSliceError::EmptySlice)));
    }

    #[test]
    fn test_single_element_slice() {
        let mut rng = XorShiftRng::new(12345, 67890);
        let mut data = vec![42];

        let result = slice(&mut data, &mut rng);
        assert!(result.is_ok());
        assert_eq!(data, vec![42]); // Single element should remain unchanged
    }

}
