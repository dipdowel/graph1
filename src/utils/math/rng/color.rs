use crate::primitives::math::{MinMax, MIN_MAX_U32, MIN_MAX_U64};
use crate::utils::math::rng::helpers::normalize_min_max::normalize_min_max;
use crate::utils::math::rng::XorShiftRng;

pub struct ColorRng {
    rng: XorShiftRng,
}

impl ColorRng {
    pub fn new(seed_32: u32, seed_64:u64) -> Self {
        Self {
            rng: XorShiftRng::new(seed_32, seed_64),
        }
    }

    
    // TODO: 1. Write proper tests! Use large and odd values as well! 
    // TODO:    - test the min>max case 
    
    // TODO: 2. Write some good documentation for all the methods!
    
    pub fn get_random_colors_32(
        &mut self,
        size: usize,
        color1: u32,
        color2: u32,
        seed: Option<u32>,
    ) -> Vec<u32> {
        if seed.is_some() {
            self.rng.set_seed_32(seed.unwrap());
        }

        // Color 1 as channels
        let r1 = (color1 >> 24) & 0xFF;
        let g1 = (color1 >> 16) & 0xFF;
        let b1 = (color1 >> 8) & 0xFF;
        let a1 = color1 & 0xFF;

        // Color 2 as channels
        let r2 = (color2 >> 24) & 0xFF;
        let g2 = (color2 >> 16) & 0xFF;
        let b2 = (color2 >> 8) & 0xFF;
        let a2 = color2 & 0xFF;

        // Desired output ranges per channel
        let r_range = normalize_min_max(&MinMax::new(r1, r2));
        let g_range = normalize_min_max(&MinMax::new(g1, g2));
        let b_range = normalize_min_max(&MinMax::new(b1, b2));
        let a_range = normalize_min_max(&MinMax::new(a1, a2));

        // Output ranges as deltas per channel
        let r_delta = r_range.max - r_range.min;
        let g_delta = g_range.max - g_range.min;
        let b_delta = b_range.max - b_range.min;
        let a_delta = a_range.max - a_range.min;

        // If a channel has no range, we can skip the computation of the random values for it
        let r_compute = r_delta != 0;
        let g_compute = g_delta != 0;
        let b_compute = b_delta != 0;
        let a_compute = a_delta != 0;

        // Check if any of the channels can be skipped due to no delta in the range
        // `num_channels` contains the factual number of channels we need to randomly populate
        let num_channels =
            r_compute as usize + g_compute as usize + b_compute as usize + a_compute as usize;

        // Adding 4 to guarantee we have enough random bytes to use
        let optimized_size: usize = 4 + size / 4 * num_channels;

        let random_data_u32 = self.rng.get_vec_u32( 4 + optimized_size, &MIN_MAX_U32);

        // The resulting vector of RGBA values
        let mut result: Vec<u32> = Vec::with_capacity(size);


        for i in (0..optimized_size).step_by(num_channels) {
            let mut r_tuple = (r1, r1, r1, r1);
            if r_compute {
                r_tuple = (
                    r_range.min + ((random_data_u32[i] >> 24) & 0xFF) % r_delta,
                    r_range.min + ((random_data_u32[i] >> 16) & 0xFF) % r_delta,
                    r_range.min + ((random_data_u32[i] >> 8) & 0xFF) % r_delta,
                    r_range.min + (random_data_u32[i] & 0xFF) % r_delta,
                );
            }

            let mut g_tuple = (g1, g1, g1, g1);
            if g_compute {
                g_tuple = (
                    g_range.min + ((random_data_u32[i + 1] >> 24) & 0xFF) % g_delta,
                    g_range.min + ((random_data_u32[i + 1] >> 16) & 0xFF) % g_delta,
                    g_range.min + ((random_data_u32[i + 1] >> 8) & 0xFF) % g_delta,
                    g_range.min + (random_data_u32[i + 1] & 0xFF) % g_delta,
                );
            }

            let mut b_tuple = (b1, b1, b1, b1);
            if b_compute {
                b_tuple = (
                    b_range.min + ((random_data_u32[i + 2] >> 24) & 0xFF) % b_delta,
                    b_range.min + ((random_data_u32[i + 2] >> 16) & 0xFF) % b_delta,
                    b_range.min + ((random_data_u32[i + 2] >> 8) & 0xFF) % b_delta,
                    b_range.min + (random_data_u32[i + 2] & 0xFF) % b_delta,
                );
            }

            let mut a_tuple = (a1, a1, a1, a1);
            if a_compute {
                a_tuple = (
                    a_range.min + ((random_data_u32[i + 3] >> 24) & 0xFF) % a_delta,
                    a_range.min + ((random_data_u32[i + 3] >> 16) & 0xFF) % a_delta,
                    a_range.min + ((random_data_u32[i + 3] >> 8) & 0xFF) % a_delta,
                    a_range.min + (random_data_u32[i + 3] & 0xFF) % a_delta,
                );
            }

            // Combine the channels back into an RGBA and add to the result
            result.push((r_tuple.0 << 24) | (g_tuple.0 << 16) | (b_tuple.0 << 8) | a_tuple.0);
            result.push((r_tuple.1 << 24) | (g_tuple.1 << 16) | (b_tuple.1 << 8) | a_tuple.1);
            result.push((r_tuple.2 << 24) | (g_tuple.2 << 16) | (b_tuple.2 << 8) | a_tuple.2);
            result.push((r_tuple.3 << 24) | (g_tuple.3 << 16) | (b_tuple.3 << 8) | a_tuple.3);
        }

        // make sure the result has the expected size
        result.resize(size, 0x00_00_00_00);
        result
    }



    pub fn get_random_colors_64(
        &mut self,
        size: usize,
        color1: u32,
        color2: u32,
        seed: Option<u64>,
    ) -> Vec<u32> {
        if seed.is_some() {
            self.rng.set_seed_64(seed.unwrap());
        }

        // Color 1 as channels
        let r1 = (color1 >> 24) & 0xFF;
        let g1 = (color1 >> 16) & 0xFF;
        let b1 = (color1 >> 8) & 0xFF;
        let a1 = color1 & 0xFF;

        // Color 2 as channels
        let r2 = (color2 >> 24) & 0xFF;
        let g2 = (color2 >> 16) & 0xFF;
        let b2 = (color2 >> 8) & 0xFF;
        let a2 = color2 & 0xFF;

        // Desired output ranges per channel
        let r_range = normalize_min_max(&MinMax::new(r1, r2));
        let g_range = normalize_min_max(&MinMax::new(g1, g2));
        let b_range = normalize_min_max(&MinMax::new(b1, b2));
        let a_range = normalize_min_max(&MinMax::new(a1, a2));

        // Output ranges as deltas per channel
        let r_delta = r_range.max - r_range.min;
        let g_delta = g_range.max - g_range.min;
        let b_delta = b_range.max - b_range.min;
        let a_delta = a_range.max - a_range.min;

        // If a channel has no range, we can skip the computation of the random values for it
        let r_compute = r_delta != 0;
        let g_compute = g_delta != 0;
        let b_compute = b_delta != 0;
        let a_compute = a_delta != 0;

        // Check if any of the channels can be skipped due to no delta in the range
        // `num_channels` contains the factual number of channels we need to randomly populate
        let num_channels =
            r_compute as usize + g_compute as usize + b_compute as usize + a_compute as usize;

        // Adding 16 to guarantee we have enough random bytes to use
        // `/ 2` since we are using u64 and need two times fewer values
        let optimized_size: usize = 16 + (size / 4 * num_channels) / 2;

        // Adding 16 to guarantee we have enough random bytes to use
        let random_data_u64 = self.rng.get_vec_u64(16+optimized_size, &MIN_MAX_U64);

        // The resulting vector of RGBA values
        let mut result: Vec<u32> = Vec::with_capacity(size);


        for i in (0..optimized_size).step_by(num_channels) {
            let mut r_tuple = (r1, r1, r1, r1, r1, r1, r1, r1);
            if r_compute {
                r_tuple = (
                    r_range.min + ((random_data_u64[i] >> 56) & 0xFF) as u32 % r_delta,
                    r_range.min + ((random_data_u64[i] >> 48) & 0xFF) as u32 % r_delta,
                    r_range.min + ((random_data_u64[i] >> 40) & 0xFF) as u32 % r_delta,
                    r_range.min + ((random_data_u64[i] >> 32) & 0xFF) as u32 % r_delta,
                    r_range.min + ((random_data_u64[i] >> 24) & 0xFF) as u32 % r_delta,
                    r_range.min + ((random_data_u64[i] >> 16) & 0xFF) as u32 % r_delta,
                    r_range.min + ((random_data_u64[i] >> 8) & 0xFF) as u32 % r_delta,
                    r_range.min + (random_data_u64[i] & 0xFF) as u32 % r_delta,
                );
            }

            let mut g_tuple = (g1, g1, g1, g1, g1, g1, g1, g1);
            if g_compute {
                g_tuple = (
                    g_range.min + ((random_data_u64[i] >> 56) & 0xFF) as u32 % g_delta,
                    g_range.min + ((random_data_u64[i] >> 48) & 0xFF) as u32 % g_delta,
                    g_range.min + ((random_data_u64[i] >> 40) & 0xFF) as u32 % g_delta,
                    g_range.min + ((random_data_u64[i] >> 32) & 0xFF) as u32 % g_delta,
                    g_range.min + ((random_data_u64[i] >> 24) & 0xFF) as u32 % g_delta,
                    g_range.min + ((random_data_u64[i] >> 16) & 0xFF) as u32 % g_delta,
                    g_range.min + ((random_data_u64[i] >> 8) & 0xFF) as u32 % g_delta,
                    g_range.min + (random_data_u64[i] & 0xFF) as u32 % g_delta,
                );
            }

            let mut b_tuple  = (b1, b1, b1, b1, b1, b1, b1, b1);
            if b_compute {
                b_tuple = (
                    b_range.min + ((random_data_u64[i] >> 56) & 0xFF) as u32 % b_delta,
                    b_range.min + ((random_data_u64[i] >> 48) & 0xFF) as u32 % b_delta,
                    b_range.min + ((random_data_u64[i] >> 40) & 0xFF) as u32 % b_delta,
                    b_range.min + ((random_data_u64[i] >> 32) & 0xFF) as u32 % b_delta,
                    b_range.min + ((random_data_u64[i] >> 24) & 0xFF) as u32 % b_delta,
                    b_range.min + ((random_data_u64[i] >> 16) & 0xFF) as u32 % b_delta,
                    b_range.min + ((random_data_u64[i] >> 8) & 0xFF) as u32 % b_delta,
                    b_range.min + (random_data_u64[i] & 0xFF) as u32 % b_delta,
                );
            }

            let mut a_tuple = (a1, a1, a1, a1, a1, a1, a1, a1);
            if a_compute {
                a_tuple = (
                    a_range.min + ((random_data_u64[i] >> 56) & 0xFF) as u32 % a_delta,
                    a_range.min + ((random_data_u64[i] >> 48) & 0xFF) as u32 % a_delta,
                    a_range.min + ((random_data_u64[i] >> 40) & 0xFF) as u32 % a_delta,
                    a_range.min + ((random_data_u64[i] >> 32) & 0xFF) as u32 % a_delta,
                    a_range.min + ((random_data_u64[i] >> 24) & 0xFF) as u32 % a_delta,
                    a_range.min + ((random_data_u64[i] >> 16) & 0xFF) as u32 % a_delta,
                    a_range.min + ((random_data_u64[i] >> 8) & 0xFF) as u32 % a_delta,
                    a_range.min + (random_data_u64[i] & 0xFF) as u32 % a_delta,
                );
            }

            // Combine the channels back into an RGBA and add to the result
            // NB: The tuple indices are shuffled for extra randomness (look at the indices horizontally)
            result.push((r_tuple.0 << 24) | (g_tuple.5 << 16) | (b_tuple.2 << 8) | a_tuple.3);
            result.push((r_tuple.7 << 24) | (g_tuple.6 << 16) | (b_tuple.3 << 8) | a_tuple.1);
            result.push((r_tuple.2 << 24) | (g_tuple.7 << 16) | (b_tuple.4 << 8) | a_tuple.5);
            result.push((r_tuple.3 << 24) | (g_tuple.4 << 16) | (b_tuple.0 << 8) | a_tuple.6);
            result.push((r_tuple.4 << 24) | (g_tuple.3 << 16) | (b_tuple.1 << 8) | a_tuple.7);
            result.push((r_tuple.5 << 24) | (g_tuple.2 << 16) | (b_tuple.7 << 8) | a_tuple.0);
            result.push((r_tuple.6 << 24) | (g_tuple.1 << 16) | (b_tuple.5 << 8) | a_tuple.4);
            result.push((r_tuple.1 << 24) | (g_tuple.0 << 16) | (b_tuple.6 << 8) | a_tuple.2);


        }
        // make sure the result has the expected size
        result.resize(size, 0x00_00_00_00);
        result
    }



}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    // #[test]
    // fn test_random_colors_speed() {
    //     let mut color_rng = ColorRng::new(321,321);
    //     let start = Instant::now();
    //     let random_colors =
    //         color_rng.get_random_colors_32(10_000_000, 0x10_20_30_40, 0x80_90_a0_40, None);
    //     let duration = start.elapsed(); // Measure elapsed time
    //     println!("[32] Execution time: {:?}, random_colors.len: {}", duration, random_colors.len());
    //     assert_eq!(1, 1);
    //
    //     let mut color_rng = ColorRng::new(321,321);
    //     let start = Instant::now();
    //     let random_colors =
    //         color_rng.get_random_colors_64(10_000_000, 0x10_20_30_40, 0x80_90_a0_40, None);
    //     let duration = start.elapsed(); // Measure elapsed time
    //     println!("[64] Execution time: {:?}, random_colors.len: {}", duration, random_colors.len());
    //     assert_eq!(1, 1);
    // }

    #[test]
    fn test_random_colors_32() {
        let mut color_rng = ColorRng::new(321, 321);
        let start = Instant::now();
        // let random_colors = color_rng.get_random_colors_32(115200, 0x00_11_00_ff, 0x00_ff_00_ff, None);
        let random_colors = color_rng.get_random_colors_32(1, 0x00_11_00_ff, 0x00_ff_00_ff, None);
        println!("[32] random_colors: {:#010X?}", random_colors);
        assert_eq!(1, 1);
    }

    #[test]
    fn test_random_colors_64() {
        let mut color_rng = ColorRng::new(321, 321);
        let start = Instant::now();
        // let random_colors = color_rng.get_random_colors_64(115200, 0x00_11_00_ff, 0x00_ff_00_ff, None);
        let random_colors = color_rng.get_random_colors_64(1, 0x00_11_00_ff, 0x00_ff_00_ff, None);
        println!("[64] random_colors: {:#010X?}", random_colors);
        assert_eq!(1, 1);
    }


    #[test]
    fn test_random_colors_64_large() {
        let mut color_rng = ColorRng::new(321, 321);
        let start = Instant::now();
        let random_colors = color_rng.get_random_colors_64(115200, 0x00_11_00_ff, 0x00_ff_00_ff, None);
        println!("[64] large random_colors len: {:?}", random_colors.len());
        assert_eq!(1, 1);
    }

    #[test]
    fn test_random_colors_32_large() {
        let mut color_rng = ColorRng::new(321, 321);
        let start = Instant::now();
        // let random_colors = color_rng.get_random_colors_32(115200, 0x00_11_00_ff, 0x00_ff_00_ff, None);
        let random_colors = color_rng.get_random_colors_32(115200, 0x00_11_00_ff, 0x00_ff_00_ff, None);
        println!("[32] large random_colors len: {:?}", random_colors.len());
        assert_eq!(1, 1);
    }



    // #[test]
    fn test_vec_range_0_10_applied() {
        /*
                let mut color_rng = ColorRng::new(10);

                let start = Instant::now(); // Start timing

                let random_colors_fast = color_rng.get_random_colors_fast(10, 0x10_30_50_70, 0x20_40_60_80, None);
                let duration = start.elapsed(); // Measure elapsed time
                println!("[FAST] Execution time: {:?}", duration);
                println!(">>> random_colors_fast: {:?}", random_colors_fast.len());
                // println!(">>> random_colors_fast: {:#010X?}", random_colors_fast);


                let random_colors_slow = color_rng.get_random_colors(10, 0x10_30_50_70, 0x20_40_60_80, None);
                println!(">>> random_colors_slow: {:?}", random_colors_slow.len());

                // println!(">>> random_colors_slow: {:#010X?}", random_colors_slow);
        */

        // TODO: write proper tests!!!
        // TODO: write proper tests!!!
        // TODO: write proper tests!!!
        // TODO: write proper tests!!!
        // TODO: write proper tests!!!
        /*

        let colors = color_rng.get_random_colors(10, 0x11_22_33_ff, 0x22_33_44_FF, None);
        // println!(">>> color: {:#010X?}",colors);
        let bw = color_rng.get_random_monochromes(8, 0x11, 0x22, 0xff, 0xff, None);
        println!(">>> bw: {:#010X?}", bw);
         */
        assert_eq!(1, 1);
    }
}
