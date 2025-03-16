use crate::primitives::math::{MinMax, MIN_MAX_U32, MIN_MAX_U64};
use crate::text::printer::print;
use crate::utils::math::rng::helpers::normalize_min_max::normalize_min_max;
use crate::utils::math::rng::XorShiftRng;

pub struct ColorRng {
    rng: XorShiftRng,
}

impl ColorRng {
    pub fn new(seed: u32) -> Self {
        Self {
            rng: XorShiftRng::new(seed, seed as u64),
        }
    }

    pub fn get_random_colors(
        &mut self,
        size: usize,
        color1: u32,
        color2: u32,
        seed: Option<u32>,
    ) -> Vec<u32> {
        let r1 = (color1 >> 24) & 0xFF;
        let g1 = (color1 >> 16) & 0xFF;
        let b1 = (color1 >> 8) & 0xFF;
        let a1 = color1 & 0xFF;

        let r2 = (color2 >> 24) & 0xFF;
        let g2 = (color2 >> 16) & 0xFF;
        let b2 = (color2 >> 8) & 0xFF;
        let a2 = color2 & 0xFF;

        if seed.is_some() {
            self.rng.set_seed_32(seed.unwrap());
        }

        let r_range = MinMax::new(r1, r2);
        let b_range = MinMax::new(b1, b2);
        let g_range = MinMax::new(g1, g2);
        let a_range = MinMax::new(a1, a2);

        let reds = self.rng.get_vec_u32(size, &r_range);
        let greens = self.rng.get_vec_u32(size, &g_range);
        let blues = self.rng.get_vec_u32(size, &b_range);
        let alphas = self.rng.get_vec_u32(size, &a_range);

        let mut colors: Vec<u32> = Vec::with_capacity(size);
        for i in 0..size {
            colors.push((reds[i] << 24) | (greens[i] << 16) | (blues[i] << 8) | alphas[i]);
        }

        colors
    }





    pub fn get_random_colors_fast(
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
        let r_range = normalize_min_max( &MinMax::new(r1, r2));
        let g_range = normalize_min_max( &MinMax::new(g1, g2));
        let b_range = normalize_min_max( &MinMax::new(b1, b2));
        let a_range = normalize_min_max( &MinMax::new(a1, a2));

        // let random_data_u32 = self.rng.get_vec_u32(4+ size, &MIN_MAX_U32);
        let random_data_u32 = self.rng.get_vec_u32(4+size, &MIN_MAX_U32);

        // Output ranges as deltas per channel
        let r_delta = r_range.max - r_range.min;
        let g_delta = g_range.max - g_range.min;
        let b_delta = b_range.max - b_range.min;
        let a_delta = a_range.max - a_range.min;

        // The resulting vector of RGBA values
        let mut result: Vec<u32> = Vec::with_capacity(size);

        for i in (0..size).step_by(4) {

            let r_tuple: (u32, u32, u32, u32);
            if r_delta != 0 {
                r_tuple = (
                    r_range.min + ((random_data_u32[i] >> 24) & 0xFF) % r_delta,
                    r_range.min + ((random_data_u32[i] >> 16) & 0xFF) % r_delta,
                    r_range.min + ((random_data_u32[i] >> 8) & 0xFF) % r_delta,
                    r_range.min + (random_data_u32[i] & 0xFF) % r_delta,
                );
            } else {
                r_tuple = (r1, r1, r1, r1); // FIXME: Optimize this!
            }

            let g_tuple: (u32, u32, u32, u32);
            if g_delta != 0 {
                g_tuple = (
                    g_range.min + ((random_data_u32[i+1] >> 24) & 0xFF) % g_delta,
                    g_range.min + ((random_data_u32[i+1] >> 16) & 0xFF) % g_delta,
                    g_range.min + ((random_data_u32[i+1] >> 8) & 0xFF) % g_delta,
                    g_range.min + (random_data_u32[i+1] & 0xFF) % g_delta,
                );
            } else {
                g_tuple = (g1, g1, g1, g1); // FIXME: Optimize this!
            }

            let b_tuple: (u32, u32, u32, u32);
            if b_delta != 0 {
                b_tuple = (
                    b_range.min + ((random_data_u32[i+2] >> 24) & 0xFF) % b_delta,
                    b_range.min + ((random_data_u32[i+2] >> 16) & 0xFF) % b_delta,
                    b_range.min + ((random_data_u32[i+2] >> 8) & 0xFF) % b_delta,
                    b_range.min + (random_data_u32[i+2] & 0xFF) % b_delta,
                );
            } else {
                b_tuple = (b1, b1, b1, b1); // FIXME: Optimize this!
            }
            let a_tuple: (u32, u32, u32, u32);
            if a_delta != 0 {
                a_tuple = (
                    a_range.min + ((random_data_u32[i+3] >> 24) & 0xFF) % a_delta,
                    a_range.min + ((random_data_u32[i+3] >> 16) & 0xFF) % a_delta,
                    a_range.min + ((random_data_u32[i+3] >> 8) & 0xFF) % a_delta,
                    a_range.min + (random_data_u32[i+3] & 0xFF) % a_delta,
                );
            } else {
                a_tuple = (a1, a1, a1, a1); // FIXME: Optimize this!
            }

            // Combine the components back into a single color and add to the gradient
            result.push((r_tuple.0 << 24) | (g_tuple.0 << 16) | (b_tuple.0 << 8) | a_tuple.0);
            result.push((r_tuple.1 << 24) | (g_tuple.1 << 16) | (b_tuple.1 << 8) | a_tuple.1);
            result.push((r_tuple.2 << 24) | (g_tuple.2 << 16) | (b_tuple.2 << 8) | a_tuple.2);
            result.push((r_tuple.3 << 24) | (g_tuple.3 << 16) | (b_tuple.3 << 8) | a_tuple.3);


            /*
            // Combine the components back into a single color and add to the gradient
            // + some extra mixing of channel order  for better randomness
            result.push((r_tuple.0 << 24) | (g_tuple.1 << 16) | (b_tuple.2 << 8) | a_tuple.3);
            result.push((r_tuple.1 << 24) | (g_tuple.2 << 16) | (b_tuple.3 << 8) | a_tuple.0);
            result.push((r_tuple.2 << 24) | (g_tuple.3 << 16) | (b_tuple.0 << 8) | a_tuple.1);
            result.push((r_tuple.3 << 24) | (g_tuple.0 << 16) | (b_tuple.1 << 8) | a_tuple.2);
            */


        }

        // check if this is still needed
        result.resize(size, 0x00_00_00_ff); // make sure the result has the expected size
        result
    }





    pub fn get_random_colors_fast_opti(
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
        let r_range = normalize_min_max( &MinMax::new(r1, r2));
        let g_range = normalize_min_max( &MinMax::new(g1, g2));
        let b_range = normalize_min_max( &MinMax::new(b1, b2));
        let a_range = normalize_min_max( &MinMax::new(a1, a2));



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
        let num_channels = r_compute as usize + g_compute as usize + b_compute as usize + a_compute as usize;
        let optimized_size:usize = size / 4 * num_channels;


        // Adding 4 to guarantee we have enough random bytes to use
        let random_data_u32 = self.rng.get_vec_u32(4 + optimized_size, &MIN_MAX_U32);

        // The resulting vector of RGBA values
        let mut result: Vec<u32> = Vec::with_capacity(size);

        for i in (0..optimized_size).step_by(num_channels) {

            let mut r_tuple: (u32, u32, u32, u32) = (r1, r1, r1, r1);
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
                    g_range.min + ((random_data_u32[i+1] >> 24) & 0xFF) % g_delta,
                    g_range.min + ((random_data_u32[i+1] >> 16) & 0xFF) % g_delta,
                    g_range.min + ((random_data_u32[i+1] >> 8) & 0xFF) % g_delta,
                    g_range.min + (random_data_u32[i+1] & 0xFF) % g_delta,
                );
            }

            let mut b_tuple: (u32, u32, u32, u32) = (b1, b1, b1, b1);
            if b_compute {
                b_tuple = (
                    b_range.min + ((random_data_u32[i+2] >> 24) & 0xFF) % b_delta,
                    b_range.min + ((random_data_u32[i+2] >> 16) & 0xFF) % b_delta,
                    b_range.min + ((random_data_u32[i+2] >> 8) & 0xFF) % b_delta,
                    b_range.min + (random_data_u32[i+2] & 0xFF) % b_delta,
                );
            }

            let mut a_tuple: (u32, u32, u32, u32) = (a1, a1, a1, a1);
            if a_compute {
                a_tuple = (
                    a_range.min + ((random_data_u32[i+3] >> 24) & 0xFF) % a_delta,
                    a_range.min + ((random_data_u32[i+3] >> 16) & 0xFF) % a_delta,
                    a_range.min + ((random_data_u32[i+3] >> 8) & 0xFF) % a_delta,
                    a_range.min + (random_data_u32[i+3] & 0xFF) % a_delta,
                );
            }


                // Combine the channels back into an RGBA and add to the result
                // + some extra mixing of channel order  for better randomness
                result.push((r_tuple.0 << 24) | (g_tuple.1 << 16) | (b_tuple.2 << 8) | a_tuple.3);
                result.push((r_tuple.1 << 24) | (g_tuple.2 << 16) | (b_tuple.3 << 8) | a_tuple.0);
                result.push((r_tuple.2 << 24) | (g_tuple.3 << 16) | (b_tuple.0 << 8) | a_tuple.1);
                result.push((r_tuple.3 << 24) | (g_tuple.0 << 16) | (b_tuple.1 << 8) | a_tuple.2);


        }

        // check if this is still needed
        result.resize(size, 0x00_00_00_ff); // make sure the result has the expected size
        result
    }










    // TODO: implement `_fast` and `_fast_64` versions 
    // TODO: implement `_fast` and `_fast_64` versions 
    // TODO: implement `_fast` and `_fast_64` versions 
    // TODO: implement `_fast` and `_fast_64` versions 
    // TODO: implement `_fast` and `_fast_64` versions 
    



}

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use super::*;


    #[test]
    fn test_random_colors_compete() {

        let mut color_rng = ColorRng::new(10);
        let start = Instant::now();
        let random_colors = color_rng.get_random_colors(10_000_000, 0x10_20_30_40, 0x10_60_70_80, None);
        let duration = start.elapsed(); // Measure elapsed time
        println!("[SLOW] Execution time: {:?}", duration);
        // println!(">>> random_colors: {:?}", random_colors.len());
        // println!(">>> random_colors: {:#010X?}", random_colors_fast);
        assert_eq!(1, 1);


        let mut color_rng = ColorRng::new(10);
        let start = Instant::now();
        let random_colors = color_rng.get_random_colors_fast_opti(10_000_000, 0x10_20_30_40, 0x10_60_70_80, None);
        let duration = start.elapsed(); // Measure elapsed time
        println!("[OPTI] Execution time: {:?}", duration);
        // println!(">>> random_colors: {:?}", random_colors.len());
        // println!(">>> random_colors: {:#010X?}", random_colors_fast);
        assert_eq!(1, 1);




        let mut color_rng = ColorRng::new(10);
        let start = Instant::now();
        let random_colors = color_rng.get_random_colors_fast(10_000_000, 0x10_20_30_40, 0x10_60_70_80, None);
        let duration = start.elapsed(); // Measure elapsed time
        println!("[FAST] Execution time: {:?}", duration);
        // println!(">>> random_colors: {:?}", random_colors.len());
        // println!(">>> random_colors: {:#010X?}", random_colors_fast);
        assert_eq!(1, 1);




    }

    // #[test]
    // fn test_random_colors_slow() {
    //     let mut color_rng = ColorRng::new(10);
    //     let start = Instant::now();
    //     let random_colors = color_rng.get_random_colors(1_000_000, 0x10_30_50_70, 0x20_40_60_80, None);
    //     let duration = start.elapsed(); // Measure elapsed time
    //     println!("[SLOW] Execution time: {:?}", duration);
    //     // println!(">>> random_colors: {:?}", random_colors.len());
    //     // println!(">>> random_colors: {:#010X?}", random_colors_fast);
    //     assert_eq!(1, 1);
    // }

    // #[test]
    // fn test_random_colors_fast() {
    //     let mut color_rng = ColorRng::new(10);
    //     let start = Instant::now();
    //     let random_colors = color_rng.get_random_colors_fast(1_000_000, 0x10_30_50_70, 0x20_40_60_80, None);
    //     let duration = start.elapsed(); // Measure elapsed time
    //     println!("[FAST] Execution time: {:?}", duration);
    //     // println!(">>> random_colors: {:?}", random_colors.len());
    //     // println!(">>> random_colors: {:#010X?}", random_colors_fast);
    //     assert_eq!(1, 1);
    // }

    // #[test]
    // fn test_random_colors_fast_opti() {
    //     let mut color_rng = ColorRng::new(10);
    //     let start = Instant::now();
    //     let random_colors = color_rng.get_random_colors_fast_opti(1_000_000, 0x10_30_50_70, 0x20_40_60_80, None);
    //     let duration = start.elapsed(); // Measure elapsed time
    //     println!("[OPTI] Execution time: {:?}", duration);
    //     // println!(">>> random_colors: {:?}", random_colors.len());
    //     // println!(">>> random_colors: {:#010X?}", random_colors_fast);
    //     assert_eq!(1, 1);
    // }

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
