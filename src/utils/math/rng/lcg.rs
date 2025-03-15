use crate::primitives::math::MinMax;
use crate::primitives::numeric::Numeric;

/// Configuration for a Linear Congruential Generator (LCG).
#[derive(Debug, Clone, Copy)]
pub struct LcgConfig {
    /// The modulus (`m`) defines the range of generated numbers.
    ///  Popular values:
    ///   - Use `2^32` (`4_294_967_296`) for 32-bit platforms.
    ///   - Use `2^64` (`18_446_744_073_709_551_616`) for 64-bit platforms.
    ///   - A large **prime number** can improve randomness for some applications.
    pub m_modulus: u64,
    /// The multiplier (`a`) determines how new numbers are generated from previous ones.
    /// Popular values:
    ///   - `1664525` (Numerical Recipes, good for 32-bit LCGs)
    ///   - `1103515245` (glibc `rand()`)
    ///   - `22695477` (Old Microsoft RNG)
    ///   - `6364136223846793005` (64-bit, PCG-inspired)
    pub a_multiplier: u64,
    /// The increment (`c`) helps control the periodicity and distribution of numbers.
    /// Popular values:
    ///   - `1013904223` (glibc `rand()`, ensures long cycle)
    ///   - `12345` (Old Microsoft, decent but shorter period)
    ///   - `1442695040888963407` (64-bit, used in some PCG variants)
    pub c_increment: u64,
}

/// Predefined LCG configurations
#[cfg(target_pointer_width = "64")]
pub const LCG_DEFAULT_64: LcgConfig = LcgConfig {
    // m_modulus: 1u64 << 64,
    m_modulus: u64::MAX,
    a_multiplier: 6364136223846793005,
    c_increment: 1442695040888963407,
};

#[cfg(target_pointer_width = "32")]
/// Common for 32-bit systems, based on the book "Numerical Recipes: The Art of Scientific Computing"
pub const DEFAULT_32: LcgConfig = LcgConfig {
    // m_modulus: 1u64 << 32,
    m_modulus: u32::MAX as u64,
    a_multiplier: 1664525,
    c_increment: 1013904223,
};

pub const LCG_NUMERICAL_RECIPES_32: LcgConfig = LcgConfig {
    // m_modulus: 1u64 << 32,
    m_modulus: u32::MAX as u64,
    a_multiplier: 1664525,
    c_increment: 1013904223,
};

/// Old Microsoft RNG (Used in `rand()` from older Windows versions)
pub const LCG_MICROSOFT: LcgConfig = LcgConfig {
    m_modulus: 1u64 << 32,
    a_multiplier: 22695477,
    c_increment: 1,
};

/// glibc `rand()` RNG (Used in many C standard libraries)
pub const LCG_GLIBC: LcgConfig = LcgConfig {
    m_modulus: 1u64 << 32,
    a_multiplier: 1103515245,
    c_increment: 12345,
};

/// A simple Linear Congruential Generator (LCG) for pseudo-random number generation.
///
/// This RNG is **not cryptographically secure**, but it is **fast** and suitable for simulations.
/// It supports different LCG configurations.
///
/// ### Example Usage
/// let mut rng = LcgRng::new(42, None); // Use default config
/// let random_value = rng.next_u32();
///
pub struct LcgRng<'a> {
    state: u64,
    config: &'a LcgConfig,
}

impl<'a> LcgRng<'a> {
    /// Creates a new LCG RNG with a seed.
    ///
    /// - If `config` is `None`, it defaults to the best LCG settings for the current architecture.
    /// - Otherwise, you can provide a custom `LcgRngConfig` reference.
    ///
    /// ### Example
    /// let mut rng = LcgRng::new(42, None);
    /// let value = rng.next_u32();
    ///
    pub fn new(seed: u64, config: Option<&'a LcgConfig>) -> Self {
        Self {
            state: seed,
            config: config.unwrap_or_else(|| {
                #[cfg(target_pointer_width = "64")]
                {
                    &LCG_DEFAULT_64
                }
                #[cfg(target_pointer_width = "32")]
                {
                    &DEFAULT_32
                }
            }),
        }
    }

    /// Generates a vector of random `u64` integers.
    ///
    /// # Arguments
    /// * `size` - The number of random numbers to generate.
    /// * `range` - An optional range for the generated numbers.
    ///     - **NB:** If `None`, the generated numbers are between 0 and `u64::MAX`.
    ///     - **NB:** If `min == max`, a vector of `size` filled with `min` is returned.
    ///
    /// # Returns
    /// * `Vec<u64>` - A vector of random `u64` integers.
    pub fn get_vec_u64(&mut self, size: usize, range: Option<&MinMax<u64>>) -> Vec<u64> {
        let mut min: u64 = 0;
        let mut max: u64 = 0;

        if let Some(range) = range {
            // Nothing to do but return a vector filled with the same value
            if range.min == range.max {
                return vec![range.min; size];
            }

            min = range.min;
            max = range.max;

            // Ensure min <= max
            if min > max {
                min = range.max;
                max = range.min;
            }
        }

        let range = max - min;

        let mut random_numbers: Vec<u64> = Vec::with_capacity(size);
        for _ in 0..size {
            self.state = self
                .state
                .wrapping_mul(self.config.a_multiplier)
                .wrapping_add(self.config.c_increment)
                % self.config.m_modulus;

            if range > 0 {
                // Shift the result to the specified range [min, max)
                let val = min + (self.state % range);
                random_numbers.push(val);
            } else {
                random_numbers.push(self.state);
            }
        }
        random_numbers
    }

    /// Normalizes a range of `T` to a range of `u64`.
    fn normalize_range<T: Numeric>(&self, range: &MinMax<T>) -> MinMax<u64> {
        MinMax {
            min: range.min.to_u64(),
            max: range.max.to_u64(),
        }
    }

    /// Generates a vector of random `u32` integers.
    /// - If `range` is `None`, the generated numbers are between 0 and `u32::MAX`.
    /// - If `range` is provided, the generated numbers are in the specified range.
    ///   - **NB:** If `min == max`, a vector of `size` filled with `min` is returned.
    pub fn get_vec_u32(&mut self, size: usize, range: Option<&MinMax<u32>>) -> Vec<u32> {
        let result_64: Vec<u64> = match range {
            Some(range) => self.get_vec_u64(size, Some(&self.normalize_range(range))),
            None => self.get_vec_u64(size, None),
        };

        // Convert the result to a vector of `u32`
        result_64
            .iter()
            .map(|&number_u64| number_u64 as u32)
            .collect()
    }

    /// Generates a random `u64` integer.
    /// - If `range` is `None`, the generated number is between 0 and `u64::MAX`.
    /// - If `range` is provided, the generated number is in the specified range.
    ///    - **NB:** If `min == max`, `min` is returned.
    /// - **NB:** If you need more than 1 random number, use `get_vec_u64()` instead, it's faster than calling this method multiple times.
    pub fn get_u64(&mut self, range: Option<&MinMax<u64>>) -> u64 {
        self.get_vec_u64(1, range)[0]
    }

    pub fn get_u32(&mut self, range: Option<&MinMax<u32>>) -> u32 {
        self.get_vec_u32(1, range)[0]
    }

    /// Generates a vector of random `f64` numbers.
    /// - The generated numbers are in the range [0.0, 1.0).
    pub fn get_vec_f64(&mut self, size: usize) -> Vec<f64> {
        let result_64 = self.get_vec_u64(size, None);
        let is_modulus_64 = (self.config.m_modulus-1) > ((1u64 << 32)-1);
        let divider:f64 =  if is_modulus_64 {
                u64::MAX as f64
        } else {
            u32::MAX as f64
        };

        // Convert the result to a vector of `f64`
        result_64
            .iter()
            .map(|&rand_num| rand_num as f64 / divider)
            .collect()
    }

    /// Generates a random `f64` number.
    /// - The generated number is in the range [0.0, 1.0).
    /// - **NB:** If you need more than 1 random number, use `get_vec_f64()` instead, it's faster than calling this method multiple times.
    pub fn get_f64(&mut self) -> f64 {
        self.get_vec_f64(1)[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_vec_u64_no_range() {
        let mut rng = LcgRng::new(10, None);
        let result = rng.get_vec_u64(2, None);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_get_vec_u64_zero_range() {
        // let range = ;
        let range = Some(&MinMax { min: 10, max: 10 });
        let seed = 10;
        let size = 2;

        let mut rng = LcgRng::new(10, None);
        let result = rng.get_vec_u64(size, range);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], 10);
        assert_eq!(result[1], 10);

        let mut rng = LcgRng::new(seed, Some(&LCG_NUMERICAL_RECIPES_32));
        let result = rng.get_vec_u64(size, range);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], 10);
        assert_eq!(result[1], 10);

        let mut rng = LcgRng::new(seed, Some(&LCG_MICROSOFT));
        let result = rng.get_vec_u64(size, range);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], 10);
        assert_eq!(result[1], 10);

        let mut rng = LcgRng::new(seed, Some(&LCG_GLIBC));
        let result = rng.get_vec_u64(size, range);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], 10);
        assert_eq!(result[1], 10);
    }

    #[test]
    fn test_get_vec_range_handled_correctly() {
        let seed = 12;
        let mut rng_0 = LcgRng::new(seed, None);
        let mut rng_1 = LcgRng::new(seed, Some(&LCG_NUMERICAL_RECIPES_32));
        let mut rng_2 = LcgRng::new(seed, Some(&LCG_MICROSOFT));
        let mut rng_3 = LcgRng::new(seed, Some(&LCG_GLIBC));

        let size: usize = 256;

        let range_u64: Option<&MinMax<u64>> = Some(&MinMax { min: 10, max: 15 });
        let result_u64_0 = rng_0.get_vec_u64(size, range_u64);
        let result_u64_1 = rng_1.get_vec_u64(size, range_u64);
        let result_u64_2 = rng_2.get_vec_u64(size, range_u64);
        let result_u64_3 = rng_3.get_vec_u64(size, range_u64);

        let range_u32: Option<&MinMax<u32>> = Some(&MinMax {
            min: 1000,
            max: 1010,
        });
        let result_u32_0 = rng_0.get_vec_u32(size, range_u32);
        let result_u32_1 = rng_1.get_vec_u32(size, range_u32);
        let result_u32_2 = rng_2.get_vec_u32(size, range_u32);
        let result_u32_3 = rng_3.get_vec_u32(size, range_u32);

        // let range_f64: Option<&MinMax<f64>> = Some(&MinMax { min: 10.0, max: 15.0 });
        let result_f64_0 = rng_0.get_vec_f64(size);
        let result_f64_1 = rng_1.get_vec_f64(size);
        let result_f64_2 = rng_2.get_vec_f64(size);
        let result_f64_3 = rng_3.get_vec_f64(size);

        for i in 0..size {
            assert_eq!(result_u64_0[i] < 15 && result_u64_0[i] >= 10, true);
            assert_eq!(result_u64_1[i] < 15 && result_u64_1[i] >= 10, true);
            assert_eq!(result_u64_2[i] < 15 && result_u64_2[i] >= 10, true);
            assert_eq!(result_u64_3[i] < 15 && result_u64_3[i] >= 10, true);

            assert_eq!(result_u32_0[i] < 1010 && result_u32_0[i] >= 1000, true);
            assert_eq!(result_u32_1[i] < 1010 && result_u32_1[i] >= 1000, true);
            assert_eq!(result_u32_2[i] < 1010 && result_u32_2[i] >= 1000, true);
            assert_eq!(result_u32_3[i] < 1010 && result_u32_3[i] >= 1000, true);


            assert_eq!(result_f64_0[i] < 1.0 && result_f64_0[i] >= 0.0, true);
            assert_eq!(result_f64_1[i] < 1.0 && result_f64_1[i] >= 0.0, true);
            assert_eq!(result_f64_2[i] < 1.0 && result_f64_2[i] >= 0.0, true);
            assert_eq!(result_f64_3[i] < 1.0 && result_f64_3[i] >= 0.0, true);
        }
    }

    #[test]
    fn test_single_value() {
        let seed = 12;
        let mut rng_0 = LcgRng::new(seed, None);

        let range_64: Option<&MinMax<u64>> = Some(&MinMax { min: 0, max: 5 });
        let range_32: Option<&MinMax<u32>> = Some(&MinMax {
            min: 1000,
            max: 1005,
        });

        for i in 0..100 {
            let result_64 = rng_0.get_u64(range_64);
            let result_32 = rng_0.get_u32(range_32);
            assert_eq!(result_64 < 5 && result_64 >= 0, true);
            assert_eq!(result_32 < 1005 && result_32 >= 1000, true);
        }
    }

    #[test]
    fn test_swapped_min_max() {
        let seed = 12;
        let mut rng_0 = LcgRng::new(seed, None);

        // `min` is greater than `max`, they are expected to be swapped internally
        let range_64: Option<&MinMax<u64>> = Some(&MinMax { min: 10, max: 1 });
        let range_32: Option<&MinMax<u32>> = Some(&MinMax { min: 10, max: 1 });

        for i in 0..100 {
            let result_64 = rng_0.get_u64(range_64);
            let result_32 = rng_0.get_u32(range_32);
            assert_eq!(result_64 < 10 && result_64 >= 1, true);
            assert_eq!(result_32 < 10 && result_32 >= 1, true);
        }
    }

}
