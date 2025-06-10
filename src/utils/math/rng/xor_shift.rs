use crate::primitives::math::{MinMax, MIN_MAX_U32, MIN_MAX_U64};
use crate::primitives::numeric::Numeric;
use crate::utils::math::rng::helpers::normalize_xor_shift_input::{
    normalize_input, NormalizedInput,
};

/// A Simple Random Number Generator (RNG) based on the XOR-Shift algorithm.
///
/// **NB:** Do not use this for cryptographic purposes!
#[derive(Debug)]
pub struct XorShiftRng {
    state_32: u32,
    state_64: u64,
}

impl XorShiftRng {
    pub fn new(seed_32: u32, seed_64: u64) -> Self {
        let mut rng = Self {
            // Avoid zero state (XOR-Shift fails if state is 0)
            state_32: if seed_32 == 0 { u32::MAX - 1 } else { seed_32 },
            state_64: if seed_64 == 0 { u64::MAX - 1 } else { seed_64 },
        };

        // initialize the state and discard the first (potentially low quality) values
        rng.get_f64();
        rng.get_f64();
        rng.get_u32(&MIN_MAX_U32);
        rng.get_u32(&MIN_MAX_U32);
        rng
    }

    /// Generates a vector of random `u64` values.
    /// * `size` - The size of the vector to generate.
    /// * `range` - The range of values to generate, `range.min` inclusive, `range.max` exclusive.
    pub fn get_vec_u64(&mut self, size: usize, range: &MinMax<u64>) -> Vec<u64> {
        let NormalizedInput {
            range,
            mut vec,
            is_constant,
        } = normalize_input(size, range);

        if is_constant {
            return vec;
        }

        let MinMax { min, max } = range;

        for _ in 0..size {
            self.state_64 ^= self.state_64 << 13;
            self.state_64 ^= self.state_64 >> 7;
            self.state_64 ^= self.state_64 << 17;

            if range == MIN_MAX_U64 {
                vec.push(self.state_64);
            } else {
                vec.push(min + (self.state_64 % (max - min)));
            }
        }
        vec
    }

    /// Generates a vector of random `u32` values.
    /// * `size` - The size of the vector to generate.
    /// * `range` - The range of values to generate, `range.min` inclusive, `range.max` exclusive.
    pub fn get_vec_u32(&mut self, size: usize, range: &MinMax<u32>) -> Vec<u32> {
        let NormalizedInput {
            range,
            mut vec,
            is_constant,
        } = normalize_input(size, range);

        if is_constant {
            return vec;
        }

        let MinMax { min, max } = range;

        for _ in 0..size {
            self.state_32 ^= self.state_32 << 13;
            self.state_32 ^= self.state_32 >> 17;
            self.state_32 ^= self.state_32 << 4;

            if range == MIN_MAX_U32 {
                vec.push(self.state_32);
            } else {
                vec.push(min + (self.state_32 % (max - min)));
            }
        }
        vec
    }

    /// Generates a random `u32` value.
    /// * `range` - The range of values to generate, `range.min` inclusive, `range.max` exclusive.
    /// * Returns a random `u32` value.
    /// * **NB:** If you need more than one value, use `get_vec_u32` instead, it's more efficient.
    pub fn get_u32(&mut self, range: &MinMax<u32>) -> u32 {
        self.get_vec_u32(1, range)[0]
    }

    /// Generates a random `u32` value.
    /// * `range` - The range of values to generate, `range.min` inclusive, `range.max` exclusive.
    /// * Returns a random `u32` value.
    /// * **NB:** If you need more than one value, use `get_vec_u32` instead, it's more efficient.
    pub fn get_u64(&mut self, range: &MinMax<u64>) -> u64 {
        self.get_vec_u64(1, range)[0]
    }

    /// Generates a vector of random `f64` values.
    /// * `size` - The size of the vector to generate.
    /// * Returns a vector of random `f64` values.
    ///
    pub fn get_vec_f64(&mut self, size: usize) -> Vec<f64> {
        let mut vec: Vec<f64> = Vec::with_capacity(size);
        for _ in 0..size {
            self.state_64 ^= self.state_64 << 13;
            self.state_64 ^= self.state_64 >> 7;
            self.state_64 ^= self.state_64 << 17;
            vec.push((self.state_64 >> 11) as f64 / (1u64 << 53) as f64);
        }
        vec
    }

    /// Generates a random `f64` value.
    /// **NB:** If you need more than one value, use `get_vec_f64` instead, it's more efficient.
    pub fn get_f64(&mut self) -> f64 {
        self.state_64 ^= self.state_64 << 13;
        self.state_64 ^= self.state_64 >> 7;
        self.state_64 ^= self.state_64 << 17;
        (self.state_64 >> 11) as f64 / (1u64 << 53) as f64
    }

    pub fn set_seed_32(&mut self, seed: u32) {
        self.state_32 = seed;
    }

    pub fn set_seed_64(&mut self, seed: u64) {
        self.state_64 = seed;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_vec_range_0_10_applied() {

        let size: usize = 10_000;
        let range_64:MinMax<u64> = MinMax::new(0, 10);
        let range_32:MinMax<u32> = MinMax::new(0, 10);

        // Seed 42, vec_64, range 0..10
        let seed:usize = 42;
        let mut rng = XorShiftRng::new(seed as u32, seed as u64);
        let rand_vec_64 = rng.get_vec_u64(size, &range_64);
        for i in 0..10 {
            assert!( rand_vec_64.contains(&i), "vec_64, seed {seed}, {i} is expected")  ;
        }
        assert_eq!( rand_vec_64.contains(&10), false, "vec_64, seed {seed}, 10 is not expected")  ;

        // Seed 42, vec_32, range 0..10
        let rand_vec_32 = rng.get_vec_u32(size, &range_32);
        for i in 0..10 {
            assert!( rand_vec_32.contains(&i), "vec_32, seed {seed}, {i} is expected")  ;
        }
        assert_eq!( rand_vec_32.contains(&10), false, "vec_32, seed {seed}, 10 is not expected")  ;



        // Seed 0, vec_64, range 0..10
        let seed:usize = 0;
        let mut rng = XorShiftRng::new(seed as u32, seed as u64);
        let rand_vec_64 = rng.get_vec_u64(size, &range_64);
        
        for i in 0..10 {
            assert!( rand_vec_64.contains(&i), "vec_64, seed {seed}, {i} is expected")  ;
        }
        assert_eq!( rand_vec_64.contains(&10), false, "vec_64, seed {seed}, 10 is not expected")  ;

        // Seed 0, vec_32, range 0..10
        let rand_vec_32 = rng.get_vec_u32(size, &range_32);
        for i in 0..10 {
            assert!( rand_vec_32.contains(&i), "vec_32, seed {seed}, {i} is expected")  ;
        }
        assert_eq!( rand_vec_32.contains(&10), false, "vec_32, seed {seed}, 10 is not expected")  ;


        // Seed 1, vec_64, range 0..10
        let seed:usize = 1;
        let mut rng = XorShiftRng::new(seed as u32, seed as u64);
        let rand_vec_64 = rng.get_vec_u64(size, &range_64);
        for i in 0..10 {
            assert!( rand_vec_64.contains(&i), "vec_64, seed {seed}, {i} is expected")  ;
        }
        assert_eq!( rand_vec_64.contains(&10), false, "vec_64, seed {seed}, 10 is not expected")  ;

        // Seed 1, vec_32, range 0..10
        let rand_vec_32 = rng.get_vec_u32(size, &range_32);
        for i in 0..10 {
            assert!( rand_vec_32.contains(&i), "vec_32, seed {seed}, {i} is expected")  ;
        }
        assert_eq!( rand_vec_32.contains(&10), false, "vec_32, seed {seed}, 10 is not expected")  ;


        // Seed MAX, vec_64, range 0..10
        let seed_u64:u64 = u64::MAX ;
        let seed_u32:u32 = u32::MAX ;
        let mut rng = XorShiftRng::new(seed_u32, seed_u64);
        let rand_vec_64 = rng.get_vec_u64(size, &range_64);
        for i in 0..10 {
            assert!( rand_vec_64.contains(&i), "vec_64, seed {seed_u64}, {i} is expected")  ;
        }
        assert_eq!( rand_vec_64.contains(&10), false, "vec_64, seed {seed_u64}, 10 is not expected")  ;

        // Seed MAX, vec_32, range 0..10
        let rand_vec_32 = rng.get_vec_u32(size, &range_32);
        for i in 0..10 {
            assert!( rand_vec_32.contains(&i), "vec_32, seed {seed_u32}, {i} is expected")  ;
        }
        assert_eq!( rand_vec_32.contains(&10), false, "vec_32, seed {seed_u32}, 10 is not expected")  ;
    }

    #[test]
    fn test_single_values() {
        let mut rng = XorShiftRng::new(101, 101);

        for _ in 0..100 {
            let rand_u32 = rng.get_u32(&MinMax::new(0, 10));
            // println!("rand_u32: {:?}", rand_u32);
            assert!(rand_u32 >= 0 && rand_u32 < 10);

            let rand_u64 = rng.get_u32(&MinMax::new(0, 10));
            assert!(rand_u64 >= 0 && rand_u64 < 10);
        }
    }

    #[test]
    fn test_vec_f64() {
        let mut rng = XorShiftRng::new(3, 3);

        let random_f64 = rng.get_vec_f64(100_000);
        // let random_f64 = rng.get_vec_f64(100);
        // println!("vect random_f64: {:?}", random_f64);
        random_f64.iter().for_each(|&x| {
            assert!(x >= 0.0 && x < 1.0);
        });
    }

    #[test]
    fn test_f64() {
        let mut rng = XorShiftRng::new(3123, 3123);

        for _ in 0..100 {
            let random_f64 = rng.get_f64();
            // println!("{}", random_f64);
            assert!(random_f64 >= 0.0 && random_f64 < 1.0);
        }
    }

    // TODO: Add statistical analysis tests
    // TODO: Add statistical analysis tests
    // TODO: Add statistical analysis tests
    // TODO: Add statistical analysis tests
    // TODO: Add statistical analysis tests
}

// println!("rand_vec_64: {:?}", rand_vec_64);

// let mut buf: Vec<u64> = Vec::new();
// let mut buf: Vec<u32> = Vec::new();

// for _ in 0..5_000_000 {
//     buf.push(rng.next());
// }
//
// let mut seen = HashSet::new();
// let mut duplicates = HashSet::new();
//
// for num in buf.clone() {
//     if !seen.insert(num) {
//         duplicates.insert(num); // If already seen, it's a duplicate
//     }
// }
// let duplicates_found: Vec<u32> = duplicates.into_iter().collect();
//
// println!("duplicates_found: {:?}", duplicates_found);
// // println!("buf: {:?}", buf);
// assert_eq!(duplicates_found.len(), 0);
