use crate::primitives::math::{MinMax, MIN_MAX_U32, MIN_MAX_U64};
use crate::utils::math::rng::helpers::normalize_min_max::normalize_min_max;
use crate::utils::math::rng::XorShiftRng;

/// A Random Number Generator (RNG) that generates random grayscale colors.
pub struct GrayRng {
    rng: XorShiftRng,
}

impl GrayRng {
    pub fn new(seed: u32) -> Self {
        Self {
            rng: XorShiftRng::new(seed, seed as u64),
        }
    }



    /// Generates a vector of random grayscale colors using a 32-bit RNG
    /// * `size` - The size of the vector to generate.
    /// * `color1` - The minimum grayscale color value.
    /// * `color2` - The maximum grayscale color value. If `color1 == color2`, the vector is filled with `color1`.
    /// * `alpha1` - The minimum alpha channel value.
    /// * `alpha2` - The maximum alpha channel value. If `alpha1 == alpha2`, the alpha channel is set to `alpha1`.
    /// * `seed` - An optional randomization seed.
    /// # Returns
    /// * `Vec<u32>` - A vector of random grayscale colors.
    
    pub fn get_random_grays_32(
        &mut self,
        size: usize,
        color1: u8,
        color2: u8,
        alpha1: u8,
        alpha2: u8,
        seed: Option<u32>,
    ) -> Vec<u32> {
        if seed.is_some() {
            self.rng.set_seed_32(seed.unwrap());
        }

        let adjusted_size: usize = 4 + size / 4;

        // generate random color channel and alpha channel values
        let colors = self.rng.get_vec_u32(adjusted_size, &MIN_MAX_U32);
        let alphas = self.rng.get_vec_u32(adjusted_size, &MIN_MAX_U32);

        let color_range = normalize_min_max(&MinMax::new(color1 as u32, color2 as u32));
        let alpha_range = normalize_min_max(&MinMax::new(alpha1 as u32, alpha2 as u32));
        let color_delta = color_range.max - color_range.min;
        let alpha_delta = alpha_range.max - alpha_range.min;

        let mut result: Vec<u32> = Vec::with_capacity(size);


        for i in 0..adjusted_size {
            // If color1 === color2, fill in `c` with `color1`
            let mut c = (
                color_range.min,
                color_range.min,
                color_range.min,
                color_range.min,
            );
            if color_delta != 0 {
                c = (
                    color_range.min + ((colors[i] >> 24) & 0xFF) % color_delta,
                    color_range.min + ((colors[i] >> 16) & 0xFF) % color_delta,
                    color_range.min + ((colors[i] >> 8) & 0xFF) % color_delta,
                    color_range.min + (colors[i] & 0xFF) % color_delta,
                );
            }

            // If alpha1 === alpha2, fill in `a` with `alpha1`
            let mut a = (
                alpha_range.min,
                alpha_range.min,
                alpha_range.min,
                alpha_range.min,
            );
            if alpha_delta != 0 {
                a = (
                    alpha_range.min + ((alphas[i] >> 24) & 0xFF) % alpha_delta,
                    alpha_range.min + ((alphas[i] >> 16) & 0xFF) % alpha_delta,
                    alpha_range.min + ((alphas[i] >> 8) & 0xFF) % alpha_delta,
                    alpha_range.min + (alphas[i] & 0xFF) % alpha_delta,
                );
            }

            result.push((c.0 << 24) | (c.0 << 16) | (c.0 << 8) | a.0);
            result.push((c.1 << 24) | (c.1 << 16) | (c.1 << 8) | a.1);
            result.push((c.2 << 24) | (c.2 << 16) | (c.2 << 8) | a.2);
            result.push((c.3 << 24) | (c.3 << 16) | (c.3 << 8) | a.3);
        }

        result.resize(size, 0x00_00_00_00);
        result
    }
    /// Generates a vector of random grayscale colors using a 64-bit RNG
    /// On a 64-bit machine works faster than `get_random_grays_32()`
    /// * `size` - The size of the vector to generate.
    /// * `color1` - The minimum grayscale color value.
    /// * `color2` - The maximum grayscale color value. If `color1 == color2`, the vector is filled with `color1`.
    /// * `alpha1` - The minimum alpha channel value.
    /// * `alpha2` - The maximum alpha channel value. If `alpha1 == alpha2`, the alpha channel is set to `alpha1`.
    /// * `seed` - An optional randomization seed.
    /// # Returns
    /// * `Vec<u32>` - A vector of random grayscale colors.
    pub fn get_random_grays_64(
        &mut self,
        size: usize,
        color1: u8,
        color2: u8,
        alpha1: u8,
        alpha2: u8,
        seed: Option<u64>,
    ) -> Vec<u32> {
        if seed.is_some() {
            self.rng.set_seed_64(seed.unwrap());
        }

        let adjusted_size: usize = 8 + size / 8;

        // generate random color channel and alpha channel values
        let colors = self.rng.get_vec_u64(adjusted_size, &MIN_MAX_U64);
        let alphas = self.rng.get_vec_u64(adjusted_size, &MIN_MAX_U64);

        // normalize the color and alpha ranges, prepare the deltas
        let color_range = normalize_min_max(&MinMax::new(color1 as u64, color2 as u64));
        let alpha_range = normalize_min_max(&MinMax::new(alpha1 as u64, alpha2 as u64));
        let color_delta = color_range.max - color_range.min;
        let alpha_delta = alpha_range.max - alpha_range.min;

        let c_min = color_range.min as u32;
        let a_min = alpha_range.min as u32;

        let mut result: Vec<u32> = Vec::with_capacity(size);

        for i in 1..adjusted_size {
            let mut c = (c_min, c_min, c_min, c_min, c_min, c_min, c_min, c_min);
            if color_delta != 0 {
                c = (
                    c_min + (((colors[i] >> 56) & 0xFF) % color_delta) as u32,
                    c_min + (((colors[i] >> 48) & 0xFF) % color_delta) as u32,
                    c_min + (((colors[i] >> 40) & 0xFF) % color_delta) as u32,
                    c_min + (((colors[i] >> 32) & 0xFF) % color_delta) as u32,
                    c_min + (((colors[i] >> 24) & 0xFF) % color_delta) as u32,
                    c_min + (((colors[i] >> 16) & 0xFF) % color_delta) as u32,
                    c_min + (((colors[i] >> 8) & 0xFF) % color_delta) as u32,
                    c_min + ((colors[i] & 0xFF) % color_delta) as u32,
                );
            }

            let mut a = (a_min, a_min, a_min, a_min, a_min, a_min, a_min, a_min);
            if alpha_delta != 0 {
                a = (
                    a_min + (((alphas[i] >> 56) & 0xFF) % alpha_delta) as u32,
                    a_min + (((alphas[i] >> 48) & 0xFF) % alpha_delta) as u32,
                    a_min + (((alphas[i] >> 40) & 0xFF) % alpha_delta) as u32,
                    a_min + (((alphas[i] >> 32) & 0xFF) % alpha_delta) as u32,
                    a_min + (((alphas[i] >> 24) & 0xFF) % alpha_delta) as u32,
                    a_min + (((alphas[i] >> 16) & 0xFF) % alpha_delta) as u32,
                    a_min + (((alphas[i] >> 8) & 0xFF) % alpha_delta) as u32,
                    a_min + ((alphas[i] & 0xFF) % alpha_delta) as u32,
                );
            }

            // Combine the single color channel into RGBA and add to the result
            // NB: The tuple indices are shuffled for extra randomness (look at the indices vertically)
            result.push((c.0 << 24) | (c.0 << 16) | (c.0 << 8) | a.0);
            result.push((c.6 << 24) | (c.6 << 16) | (c.6 << 8) | a.1);
            result.push((c.2 << 24) | (c.2 << 16) | (c.2 << 8) | a.2);
            result.push((c.7 << 24) | (c.7 << 16) | (c.7 << 8) | a.3);
            result.push((c.4 << 24) | (c.4 << 16) | (c.4 << 8) | a.4);
            result.push((c.5 << 24) | (c.5 << 16) | (c.5 << 8) | a.5);
            result.push((c.1 << 24) | (c.1 << 16) | (c.1 << 8) | a.6);
            result.push((c.3 << 24) | (c.3 << 16) | (c.3 << 8) | a.7);
        }

        result.resize(size, 0x00_00_00_00);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grays_32_constant_alpha() {
        let mut gray_rng = GrayRng::new(421);
        let grays_32 = gray_rng.get_random_grays_32(100, 0x00, 0x44, 0xff, 0xff, None);
        // println!(">>> grays_32: {:#010X?}", grays_32);
        for i in 0..grays_32.len() {
            let r = (grays_32[i] >> 24) & 0xFF;
            let g = (grays_32[i] >> 16) & 0xFF;
            let b = (grays_32[i] >> 8) & 0xFF;
            let a = grays_32[i] & 0xFF;

            assert!(r == g && g == b);
            assert!(r > 0x00 && r <= 0x44);
            assert_eq!(a, 0xff);
        }
    }

    #[test]
    fn test_grays_32_constant_gray() {
        let mut gray_rng = GrayRng::new(421);
        let grays_32 = gray_rng.get_random_grays_32(100, 0x44, 0x44, 0x10, 0x20, None);
        // println!(">>> grays_32: {:#010X?}", grays_32);
        for i in 0..grays_32.len() {
            let r = (grays_32[i] >> 24) & 0xFF;
            let g = (grays_32[i] >> 16) & 0xFF;
            let b = (grays_32[i] >> 8) & 0xFF;
            let a = grays_32[i] & 0xFF;

            assert!(r == g && g == b);
            assert!(r == 0x44);
            assert!(a >= 0x10 && a <= 0x20);
        }
    }

    #[test]
    fn test_grays_64_constant_alpha() {
        let mut gray_rng = GrayRng::new(421);
        let grays_64 = gray_rng.get_random_grays_64(100, 0x00, 0x44, 0xff, 0xff, None);
        // println!(">>> grays_32: {:#010X?}", grays_32);
        for i in 0..grays_64.len() {
            let r = (grays_64[i] >> 24) & 0xFF;
            let g = (grays_64[i] >> 16) & 0xFF;
            let b = (grays_64[i] >> 8) & 0xFF;
            let a = grays_64[i] & 0xFF;

            assert!(r == g && g == b, "r=g=b");
            assert!(r > 0x00 && r <= 0x44, "r in the expected range");
            assert_eq!(a, 0xff, "alpha is 0xff");
        }
    }

    #[test]
    fn test_grays_64_constant_gray() {
        let mut gray_rng = GrayRng::new(421);
        let grays_64 = gray_rng.get_random_grays_64(100, 0x44, 0x44, 0x10, 0x20, None);
        // println!(">>> grays_32: {:#010X?}", grays_32);
        for i in 0..grays_64.len() {
            let r = (grays_64[i] >> 24) & 0xFF;
            let g = (grays_64[i] >> 16) & 0xFF;
            let b = (grays_64[i] >> 8) & 0xFF;
            let a = grays_64[i] & 0xFF;

            assert!(r == g && g == b, "r=g=b");
            assert_eq!(r, 0x44, "color channel is 0x44");
            assert!(a >= 0x10 && a <= 0x20, "alpha in the expected range");
        }
    }

    /*
    #[test]
    fn test_performance() {
        let mut gray_rng = GrayRng::new(123);
        let start = Instant::now(); // Start timing
        let grays_32 = gray_rng.get_random_grays_32(8_000_000, 0x00, 0x55, 0x1f, 0xff, None);
        let duration_32 = start.elapsed(); // Measure elapsed time
        println!("[grays_32] duration:  {:?}", duration_32);

        let mut gray_rng = GrayRng::new(123);
        let start = Instant::now(); // Start timing
        let grays_64 = gray_rng.get_random_grays_64(8_000_000, 0x00, 0x55, 0x1f, 0xff, None);
        let duration_64 = start.elapsed(); // Measure elapsed time
        println!("[grays_64] duration:  {:?}", duration_64);
        assert!(duration_64 < duration_32);
    }
     */
}
