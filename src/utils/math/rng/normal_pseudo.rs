/// Generates a vector of pseudo-random `usize` numbers with a somewhat normal distribution.
/// This function uses an LCG-based pseudo-random number generator without external dependencies.
/// **NB**: This is not a cryptographically secure random number generator!
/// **NB**: The distribution is only an approximation of normality.
///
/// # Arguments
/// * `length` - The number of random values to generate.
/// * `min` - The minimum possible value.
/// * `max` - The maximum possible value.
/// * `num_samples` - More samples = better approximation of normal distribution. E.g.: `6`
/// * `seed` - The initial seed value for the pseudo-random number generator.
///
/// # Returns
/// * `Vec<usize>` - A vector containing normally distributed pseudo-random numbers.
pub fn normal_pseudo(length: usize, min: usize, max: usize, num_samples:usize, seed: usize) -> Vec<usize> {

    let mut min = min;
    let mut max = max;

    // Ensure min <= max
    if min > max {
        let temp = max;
        max = min;
        min = temp;
    }

    let mut result = Vec::with_capacity(length);
    let mut rng_state = seed;

    for _ in 0..length {
        let mut sum:usize = 0;
        // Generate multiple uniform random values and sum them
        for _ in 0..num_samples {
            rng_state = lcg_next(rng_state);
            sum += min + (rng_state % (max - min + 1));
        }

        // Normalize the sum to fit within the given range
        let avg = sum / num_samples;
        result.push(avg);
    }

    result
}
//
// // const A: usize = 6364136223846793005;
// const A: usize = 6364136223846793;
// const C: usize = 1;
// @See https://doc.rust-lang.org/reference/conditional-compilation.html
#[cfg(target_pointer_width = "64")]
const A: usize = 6364136223846793005;

#[cfg(target_pointer_width = "32")]
const A: usize = 1664525;

// Commonly used LCG increment (Is it?)
const C: usize = 1013904223; 

/// A simple Linear Congruential Generator (LCG) for pseudo-random number generation.
/// `A` and `C` chosen to provide reasonable randomness.
///
/// # Arguments
/// * `state` - The current state of the generator.
///
/// # Returns
/// * `usize` - The next pseudo-random number in the sequence.
fn lcg_next(state: usize) -> usize {
    state.wrapping_mul(A).wrapping_add(C)
}
