use crate::primitives::math::MinMax;
use crate::utils::math::rng::XorShiftRng;


#[derive(Debug)]
pub enum ShuffleSliceError {
    /// The provided slice is empty
    EmptySlice,
    /// The provided slice is too large (>= u32::MAX)
    SliceTooBig,
}


/// Shuffle the items in the slice in place using the provided RNG
/// using `Fisher-Yates shuffle` algorithm.
/// **NB!:** This function modifies the original slice.
/// # Arguments
/// * `items` - The slice of items to be shuffled
/// * `rng` - The random number generator to use for shuffling
/// # Returns
/// * `Ok(())` if the shuffle was successful
/// * `Err(ShuffleSliceError)` if the slice is empty or too large
pub fn slice<T>(items: &mut [T], rng: &mut XorShiftRng) ->Result<(), ShuffleSliceError> {
    let target_len = items.len();

    if items.is_empty() {
        return Err(ShuffleSliceError::EmptySlice);
    }
    if target_len >= (u32::MAX as usize) {
        return Err(ShuffleSliceError::SliceTooBig);
    }

    // No need to shuffle if there's only one item
    if target_len == 1 {
        return Ok(());
    }

    for i in (1..target_len).rev() {
        let j = rng.get_u32(&MinMax::new(0, (i + 1) as u32)) as usize;
        items.swap(i, j);
    }

    Ok(())
}
#[cfg(test)]
mod tests {

    const RNG_SEED_32: u32 = 619;
    const RNG_SEED_64: u64 = 421;

    use super::*;

    #[test]
    fn test_shuffle() {
        let mut rng = XorShiftRng::new(RNG_SEED_32, RNG_SEED_64);

        let part1: Vec<u32> = (0..=4095).collect();
        let part2: Vec<u32> = (0..=1023).collect();
        let mut test_data: Vec<u32> = part1.iter().chain(&part2).cloned().collect();
        test_data.sort();
        let original_test_data: Vec<u32> = test_data;

        let mut data = Vec::from(original_test_data.clone());

        // Ensure the data is initially in the original order
        assert_eq!(data, original_test_data);
        // Shuffle the data
        slice(&mut data, &mut rng).ok();
        // Ensure the data has been shuffled (not equal to original)
        assert_ne!(data, original_test_data);

        // Sort the shuffled data and ensure it matches the original
        let mut sorted_data = data.clone();
        sorted_data.sort();
        assert_eq!(sorted_data, original_test_data);
    }
    #[test]
    fn test_empty_slice() {
        let mut rng = XorShiftRng::new(RNG_SEED_32, RNG_SEED_64);
        let mut data: Vec<u32> = Vec::new();

        let result = slice(&mut data, &mut rng);
        assert!(matches!(result, Err(ShuffleSliceError::EmptySlice)));
    }

    #[test]
    fn test_single_element_slice() {
        let mut rng = XorShiftRng::new(RNG_SEED_32, RNG_SEED_64);
        let mut data = vec![42];

        let result = slice(&mut data, &mut rng);
        assert!(result.is_ok());
        assert_eq!(data, vec![42]); // Single element should remain unchanged
    }



}
