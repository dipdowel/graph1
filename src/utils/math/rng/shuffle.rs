use std::collections::HashSet;
use crate::primitives::math::MinMax;
use crate::utils::math::rng::XorShiftRng;


#[derive(Debug)]
pub enum ShuffleSliceError {
    EmptySlice,
    SliceTooBig,
    RngFault,
}

const SAFEGUARD_LIMIT: usize = 64;

/// Shuffle the items in the slice in place using the provided RNG.
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


    let mut seen = HashSet::with_capacity(items.len());
    let mut unique_randoms:Vec<usize> = Vec::with_capacity(items.len());

    let mut safeguard_counter = 0;

    // Generate unique random indices until we have enough
    while unique_randoms.len() < target_len {
        let random_values: Vec<u32> = rng.get_vec_u32(
            target_len*2,
            &MinMax {
                min: 0,
                max: target_len as u32,
            },
        );
        for rand_val in random_values {
            if seen.insert(rand_val) {
                unique_randoms.push(rand_val as usize);
            }
        }
        println!(">> safeguard_counter: {}", safeguard_counter);
        if safeguard_counter == SAFEGUARD_LIMIT {
            return Err(ShuffleSliceError::RngFault);
        }

        safeguard_counter += 1;
    }



    for (i, random_value) in unique_randoms.iter().enumerate() {
        items.swap(i, *random_value as usize);
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

        // The original test data generated as `(1..=3000_000).collect()` increments `safeguard_counter` till 9,
        // so `const SAFEGUARD_LIMIT: usize = 64` tries should be sufficient.
        let original_test_data: Vec<u32> = (1..=1024).collect();
        let mut data = Vec::from(original_test_data.clone());

        // Ensure the data is initially in the original order
        assert_eq!(data, original_test_data);
        // Shuffle the data
        slice(&mut data, &mut rng);
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
