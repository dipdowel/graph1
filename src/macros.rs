/// Generates a deterministic pseudorandom `u32` based on a hash of the input value.
/// Useful for fast generation of random values based on `ctx.frame_count`
/// **NB:** Do not use for cryptographic purposes! The generated values are deterministic!
/// - With 1 argument: returns a raw hash value.
/// - With 3 arguments: maps the hash into the range [`min`, `max`)
///   - If `min == max`, returns `min`
///   - If `min > max`, they are swapped, i.e. the smaller value is always used as the lower bound.
///
/// # Examples
/// ```text
/// let raw = hash_random_u32!(ctx.frame_count);
/// let raw = hash_random_u32!(45);
/// let ranged = hash_random_u32!(45, 10, 100);
/// let same = hash_random_u32!(45, 5, 5); // returns 5
/// let swapped = hash_random_u32!(45, 100, 10); // treated as (10, 100)
/// ```
#[macro_export]
macro_rules! hash_random_u32 {
    // No min/max range provided
    ($value:expr) => {{
        use $crate::utils::math::constants::golden_ratio::GOLDEN_RATIO_U32;
        use $crate::utils::math::constants::mersenne::MERSENNE_LIKE_PRIME_32;

        (($value as u64 * GOLDEN_RATIO_U32 as u64) % MERSENNE_LIKE_PRIME_32 as u64) as u32
    }};

    // min/max range provided
($value:expr, $min:expr, $max:expr) => {{
    let mut min = $min;
    let mut max = $max;

    if min == max {
        min
    } else {
        if min > max {
            core::mem::swap(&mut min, &mut max);
        }

        let hash_random = $crate::hash_random_u32!($value);
        let range = max - min;
        min + (hash_random % range)
    }
}};
}


#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    // Import the macro from the crate root
    // use super::*;
    use crate::hash_random_u32;

    #[test]
    fn returns_raw_hash() {
        let x = hash_random_u32!(12345);
        let y = hash_random_u32!(12345);
        assert_eq!(x, y); // should be deterministic
    }

    #[test]
    fn works_with_range() {
        let val = hash_random_u32!(42, 10, 20);
        assert!(val >= 10 && val < 20);
    }

    #[test]
    fn swaps_if_min_greater() {
        let a = hash_random_u32!(100, 30, 10);
        let b = hash_random_u32!(100, 10, 30);
        assert_eq!(a, b);
    }

    #[test]
    fn returns_min_if_equal() {
        let val = hash_random_u32!(999, 5, 5);
        assert_eq!(val, 5);
    }

    #[test]
    fn returns_relatively_random() {
        let mut random_values: Vec<u32> = Vec::with_capacity(4096);
        // Produce 4096 random values
        for i in 0..4096 {
            random_values.push(hash_random_u32!(i));
        }

        // Filter out duplicates from the created random values
        let mut seen = HashSet::new();
        let filtered_values:Vec<u32> = random_values.iter()
            .cloned()                     // convert &u32 to u32
            .filter(|x| seen.insert(*x))  // insert returns false if already present
            .collect();

        // println!("random_values: {:?}",random_values);
        // println!("filtered_values: {:?}",filtered_values);

        // were all the initially generated values unique?
        assert_eq!(random_values.len(), filtered_values.len());
    }

    // #[test]
    // fn distribution_is_uniform_enough() {
    //     use crate::hash_random_u32;
    //     use std::collections::HashMap;
    //
    //     let mut counts = HashMap::new();
    //     let min = 100;
    //     let max = 500;
    //
    //     for i in 0..10_000 {
    //         let val = hash_random_u32!(i as u32, min, max);
    //         *counts.entry(val).or_insert(0) += 1;
    //     }
    //
    //     // Make sure all values in the range [0, 10) occurred at least once
    //     for expected in min..max {
    //         assert!(counts.contains_key(&expected), "Missing value {}", expected);
    //     }
    //
    //     // Optionally: check that none are wildly off
    //     let avg = 10_000 / (max - min);
    //     for (val, count) in &counts {
    //         let delta = (*count - avg as i32).abs() as u32;
    //
    //         // Allow some wiggle room (50% deviation from average)
    //         assert!(
    //             delta < avg / 2,
    //             "Value {} appears too often or too rarely: {} times",
    //             val,
    //             count
    //         );
    //     }
    // }


}
