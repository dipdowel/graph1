use crate::primitives::math::{MinMax, MIN_MAX_U32, MIN_MAX_U64};
use crate::utils::math::rng::XorShiftRng;

pub struct GrayRng {
    rng: XorShiftRng,
}

impl GrayRng {
    pub fn new(seed: u32) -> Self {
        Self {
            rng: XorShiftRng::new(seed, seed as u64),
        }
    }

    // TODO: 1. Leave only the `fast` methods for 32 and 64 bit
    // TODO: 2. See what can be improved here to be more like `ColorRng`
    // TODO: 3. Write some good documentation for all the methods!
    // TODO: 4. Write proper tests! Use large and odd values as well!
    // TODO:    - test the min>max case

    // FIXME: Normalize the input range for the `XorShiftRng` in all the 3 functions below
    // FIXME: Normalize the input range for the `XorShiftRng` in all the 3 functions below
    // FIXME: Normalize the input range for the `XorShiftRng` in all the 3 functions below
    // FIXME: Normalize the input range for the `XorShiftRng` in all the 3 functions below
    // FIXME: Normalize the input range for the `XorShiftRng` in all the 3 functions below
    // FIXME: Normalize the input range for the `XorShiftRng` in all the 3 functions below
    // FIXME: Normalize the input range for the `XorShiftRng` in all the 3 functions below
    // FIXME: Normalize the input range for the `XorShiftRng` in all the 3 functions below
    // FIXME: Normalize the input range for the `XorShiftRng` in all the 3 functions below
    // FIXME: Normalize the input range for the `XorShiftRng` in all the 3 functions below

    pub fn get_random_gray(
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

        let color_range = MinMax::new(color1 as u32, color2 as u32);
        let alpha_range = MinMax::new(alpha1 as u32, alpha2 as u32);

        let colors = self.rng.get_vec_u32(size, &color_range);
        let alphas = self.rng.get_vec_u32(size, &alpha_range);

        let mut result: Vec<u32> = Vec::with_capacity(size);

        for i in 0..size {
            result.push((colors[i] << 24) | (colors[i] << 16) | (colors[i] << 8) | alphas[i]);
        }

        result
    }

    pub fn get_random_gray_fast(
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

        let color1 = color1 as u32;
        let color2 = color2 as u32;
        let alpha1 = alpha1 as u32;
        let alpha2 = alpha2 as u32;
        
        
        let adjusted_size:usize = 4 + size / 4;
        
        let colors = self.rng.get_vec_u32(adjusted_size, &MIN_MAX_U32);
        let alphas = self.rng.get_vec_u32(adjusted_size, &MIN_MAX_U32);

        let mut result: Vec<u32> = Vec::with_capacity(size);

        let color_delta = color2 - color1;
        let alpha_delta = alpha2 - alpha1;

        for i in 0..adjusted_size {
            let mut c = (color1, color1, color1, color1);
            if color_delta != 0 {
                c = (
                    color1 + ((colors[i] >> 24) & 0xFF) % color_delta,
                    color1 + ((colors[i] >> 16) & 0xFF) % color_delta,
                    color1 + ((colors[i] >> 8) & 0xFF) % color_delta,
                    color1 + (colors[i] & 0xFF) % color_delta,
                );
            }

            let mut a = (alpha1, alpha1, alpha1, alpha1);
            if alpha_delta != 0 {
                a = (
                    alpha1 + ((alphas[i] >> 24) & 0xFF) % alpha_delta,
                    alpha1 + ((alphas[i] >> 16) & 0xFF) % alpha_delta,
                    alpha1 + ((alphas[i] >> 8) & 0xFF) % alpha_delta,
                    alpha1 + (alphas[i] & 0xFF) % alpha_delta,
                );
            }
            result.push((c.0 << 24) | (c.0 << 16) | (c.0 << 8) | a.0);
            result.push((c.1 << 24) | (c.1 << 16) | (c.1 << 8) | a.1);
            result.push((c.2 << 24) | (c.2 << 16) | (c.2 << 8) | a.2);
            result.push((c.3 << 24) | (c.3 << 16) | (c.3 << 8) | a.3);
        }

        result.resize(size, 0x00_ff_00_ff);
        result
    }

    pub fn get_random_gray_fast_64(
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

        let color1 = color1 as u32;
        let color2 = color2 as u32;
        let alpha1 = alpha1 as u32;
        let alpha2 = alpha2 as u32;

        let adjusted_size:usize = 8 + size / 8;

        let colors = self.rng.get_vec_u64(adjusted_size, &MIN_MAX_U64);
        let alphas = self.rng.get_vec_u64(adjusted_size, &MIN_MAX_U64);

        let mut result: Vec<u32> = Vec::with_capacity(size);

        let color_delta = (color2 - color1) as u64;
        let alpha_delta = (alpha2 - alpha1) as u64;

        
        
        for i in 1..adjusted_size {
            let mut c = (color1, color1, color1, color1, color1, color1, color1, color1);
            if color_delta != 0 {
                c = (
                    color1 + (((colors[i] >> 56) & 0xFF) % color_delta) as u32,
                    color1 + (((colors[i] >> 48) & 0xFF) % color_delta) as u32,
                    color1 + (((colors[i] >> 40) & 0xFF) % color_delta) as u32,
                    color1 + (((colors[i] >> 32) & 0xFF) % color_delta) as u32,
                    color1 + (((colors[i] >> 24) & 0xFF) % color_delta) as u32,
                    color1 + (((colors[i] >> 16) & 0xFF) % color_delta) as u32,
                    color1 + (((colors[i] >> 8) & 0xFF) % color_delta) as u32,
                    color1 + ((colors[i] & 0xFF) % color_delta) as u32,
                );
            }

            let mut a = (alpha1, alpha1, alpha1, alpha1, alpha1, alpha1, alpha1, alpha1);
            if alpha_delta != 0 {
                a = (
                    alpha1 + (((alphas[i] >> 56) & 0xFF) % alpha_delta) as u32,
                    alpha1 + (((alphas[i] >> 48) & 0xFF) % alpha_delta) as u32,
                    alpha1 + (((alphas[i] >> 40) & 0xFF) % alpha_delta) as u32,
                    alpha1 + (((alphas[i] >> 32) & 0xFF) % alpha_delta) as u32,
                    alpha1 + (((alphas[i] >> 24) & 0xFF) % alpha_delta) as u32,
                    alpha1 + (((alphas[i] >> 16) & 0xFF) % alpha_delta) as u32,
                    alpha1 + (((alphas[i] >> 8) & 0xFF) % alpha_delta) as u32,
                    alpha1 + ((alphas[i] & 0xFF) % alpha_delta) as u32,
                );
            }

            result.push((c.0 << 24) | (c.0 << 16) | (c.0 << 8) | a.0);
            result.push((c.1 << 24) | (c.1 << 16) | (c.1 << 8) | a.1);
            result.push((c.2 << 24) | (c.2 << 16) | (c.2 << 8) | a.2);
            result.push((c.3 << 24) | (c.3 << 16) | (c.3 << 8) | a.3);

            result.push((c.4 << 24) | (c.4 << 16) | (c.4 << 8) | a.4);
            result.push((c.5 << 24) | (c.5 << 16) | (c.5 << 8) | a.5);
            result.push((c.6 << 24) | (c.6 << 16) | (c.6 << 8) | a.6);
            result.push((c.7 << 24) | (c.7 << 16) | (c.7 << 8) | a.7);
        }

        result.resize(size, 0x00_ff_00_ff);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;
    

    // TODO: write proper tests!!!
    // TODO: write proper tests!!!
    // TODO: write proper tests!!!
    // TODO: write proper tests!!!
    // TODO: write proper tests!!!

    #[test]
    fn test_vec_range_0_10_applied() {
        /*
        let mut color_rng = ColorRng::new(42);
        let colors = color_rng.get_random_colors(10, 0x11_22_33_ff, 0x22_33_44_FF, None);
        // println!(">>> color: {:#010X?}",colors);
        let bw = color_rng.get_random_monochromes(8, 0x11, 0x22, 0xff, 0xff, None);
        println!(">>> bw: {:#010X?}", bw);
         */
        assert_eq!(1, 1);
    }
    #[test]
    fn test_vec_bw() {
        let mut gray_rng = GrayRng::new(123);

        let start = Instant::now(); // Start timing
        let bw =
            // color_rng.get_random_monochromes(11520000, 0x11, 0x22, 0x22, 0xff, Some(123));
            gray_rng.get_random_gray(20, 0x00, 0x55, 0x22, 0xff, Some(222));

        let duration = start.elapsed(); // Measure elapsed time
                                        // println!("[NORM] Execution time: {:?}", duration);
        println!("[NORM] : {:#010X?}", bw);

        assert_eq!(1, 1);
    }
    #[test]
    fn test_vec_bw_fast() {
        let mut gray_rng = GrayRng::new(123);

        let start = Instant::now(); // Start timing
        let bw =
            // color_rng.get_random_monochromes_fast(11520000, 0x11, 0x22, 0x22, 0xff, Some(123));
            gray_rng.get_random_gray_fast(20, 0x00, 0x55, 0x22, 0xff, Some(222));

        let duration = start.elapsed(); // Measure elapsed time
                                        // println!("[FAST] Execution time: {:?}", duration);
        println!("[FAST] : {:#010X?}", bw);

        assert_eq!(1, 1);
    }

    #[test]
    fn test_vec_bw_fast_64() {
        let mut gray_rng = GrayRng::new(123);

        let start = Instant::now(); // Start timing
        let bw =
            // color_rng.get_random_monochromes_fast_64(11520000, 0x11, 0x22, 0x22, 0xff, Some(123));
            gray_rng.get_random_gray_fast_64(20, 0x00, 0x55, 0x22, 0xff, Some(222));

        let duration = start.elapsed(); // Measure elapsed time
                                        // println!("[FAST 64] Execution time: {:?}", duration);
        println!("[FAST 64] : {:#010X?}", bw);

        assert_eq!(1, 1);
    }
}
