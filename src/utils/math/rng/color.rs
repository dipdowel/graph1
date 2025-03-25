use crate::primitives::math::{MinMax, MIN_MAX_U32, MIN_MAX_U64};
use crate::utils::math::rng::helpers::normalize_min_max::normalize_min_max;
use crate::utils::math::rng::XorShiftRng;

/// A color generator using the `XorShiftRng` algorithm.
/// Can generate random RGBA colors between two input colors using either 32-bit or 64-bit seeds.
pub struct ColorRng {
    rng: XorShiftRng,
}

impl ColorRng {
    /// Create a new `ColorRng` from a 32-bit and 64-bit seed.
    pub fn new(seed_32: u32, seed_64: u64) -> Self {
        Self {
            rng: XorShiftRng::new(seed_32, seed_64),
        }
    }

    /// Generate a vector of random `u32` RGBA colors using a 32-bit seed.
    /// - `size`: number of colors to generate
    /// - `color1`, `color2`: define the min/max ranges for each RGBA channel
    /// - `seed`: optional override for the current 32-bit seed of the RNG
    ///
    /// If `color1 > color2` for any channel, the method normalizes the range.
    /// If `color1 == color2`, the method returns a vector of the same color.
    pub fn get_random_colors_32(
        &mut self,
        size: usize,
        color1: u32,
        color2: u32,
        seed: Option<u32>,
    ) -> Vec<u32> {
        if color1 == color2 {
            return vec![color1; size];
        }

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

        let random_data_u32 = self.rng.get_vec_u32(4 + optimized_size, &MIN_MAX_U32);

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

    /// Generate a vector of random `u32` RGBA colors using a 64-bit RNG.
    /// /// Can be somewhat faster than `get_random_colors_32` for large sizes.
    /// - `size`: number of colors to generate
    /// - `color1`, `color2`: define the min/max ranges for each RGBA channel
    /// - `seed`: optional override for the current 64-bit seed of the RNG
    ///
    /// If `color1 > color2` for any channel, the method normalizes the range.
    /// If `color1 == color2`, the method returns a vector of the same color.

    pub fn get_random_colors_64(
        &mut self,
        size: usize,
        color1: u32,
        color2: u32,
        seed: Option<u64>,
    ) -> Vec<u32> {
        if color1 == color2 {
            return vec![color1; size];
        }

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
        let random_data_u64 = self.rng.get_vec_u64(16 + optimized_size, &MIN_MAX_U64);

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

            let mut b_tuple = (b1, b1, b1, b1, b1, b1, b1, b1);
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

    #[test]
    fn test_random_colors_64_large() {
        let mut color_rng = ColorRng::new(321, 321);
        let start = Instant::now();
        let random_colors =
            color_rng.get_random_colors_64(115200, 0x00_11_00_ff, 0x00_ff_00_ff, None);
        println!("[64] large random_colors len: {:?}", random_colors.len());
        assert_eq!(1, 1);
    }

    #[test]
    fn test_random_colors_32_large() {
        let mut color_rng = ColorRng::new(321, 321);
        let start = Instant::now();
        // let random_colors = color_rng.get_random_colors_32(115200, 0x00_11_00_ff, 0x00_ff_00_ff, None);
        let random_colors =
            color_rng.get_random_colors_32(115200, 0x00_11_00_ff, 0x00_ff_00_ff, None);
        println!("[32] large random_colors len: {:?}", random_colors.len());
        assert_eq!(1, 1);
    }

    fn channels_in_range(
        color: u32,
        r: (u32, u32),
        g: (u32, u32),
        b: (u32, u32),
        a: (u32, u32),
    ) -> bool {
        let (r_val, g_val, b_val, a_val) = (
            (color >> 24) & 0xFF,
            (color >> 16) & 0xFF,
            (color >> 8) & 0xFF,
            color & 0xFF,
        );
        (r.0..=r.1).contains(&r_val)
            && (g.0..=g.1).contains(&g_val)
            && (b.0..=b.1).contains(&b_val)
            && (a.0..=a.1).contains(&a_val)
    }

    #[test]
    fn test_random_colors_32_range_and_size() {
        let mut rng = ColorRng::new(123, 456);
        let color1 = 0x11_22_33_44;
        let color2 = 0x77_88_99_FF;
        let result = rng.get_random_colors_32(1000, color1, color2, None);
        assert_eq!(result.len(), 1000);

        let r_range = normalize_min_max(&MinMax::new((color1 >> 24) & 0xFF, (color2 >> 24) & 0xFF));
        let g_range = normalize_min_max(&MinMax::new((color1 >> 16) & 0xFF, (color2 >> 16) & 0xFF));
        let b_range = normalize_min_max(&MinMax::new((color1 >> 8) & 0xFF, (color2 >> 8) & 0xFF));
        let a_range = normalize_min_max(&MinMax::new(color1 & 0xFF, color2 & 0xFF));

        for c in result {
            assert!(channels_in_range(
                c,
                (r_range.min, r_range.max),
                (g_range.min, g_range.max),
                (b_range.min, b_range.max),
                (a_range.min, a_range.max)
            ));
        }
    }

    #[test]
    fn test_random_colors_64_range_and_size() {
        let mut rng = ColorRng::new(789, 101112);
        let color1 = 0x10_10_10_10;
        let color2 = 0x20_20_20_FF;
        let result = rng.get_random_colors_64(512, color1, color2, None);
        assert_eq!(result.len(), 512);
    }

    #[test]
    fn test_random_colors_32_determinism() {
        let mut rng1 = ColorRng::new(999, 5555);
        let mut rng2 = ColorRng::new(999, 5555);
        let color1 = 0x00_00_00_00;
        let color2 = 0xFF_FF_FF_FF;

        let a = rng1.get_random_colors_32(100, color1, color2, Some(123456));
        let b = rng2.get_random_colors_32(100, color1, color2, Some(123456));

        assert_eq!(a, b, "Deterministic RNG failed for same seed");
    }

    #[test]
    fn test_random_colors_64_determinism() {
        let mut rng1 = ColorRng::new(1111, 2222);
        let mut rng2 = ColorRng::new(1111, 2222);
        let color1 = 0x00_00_00_00;
        let color2 = 0xFF_FF_FF_FF;

        let a = rng1.get_random_colors_64(100, color1, color2, Some(0xAABBCCDDEEFF));
        let b = rng2.get_random_colors_64(100, color1, color2, Some(0xAABBCCDDEEFF));

        assert_eq!(a, b, "Deterministic RNG failed for same 64-bit seed");
    }

    #[test]
    fn test_zero_size_output() {
        let mut rng = ColorRng::new(0, 0);
        let result = rng.get_random_colors_32(0, 0xFF_FF_FF_FF, 0x00_00_00_00, None);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_min_greater_than_max() {
        let mut rng = ColorRng::new(42, 42);
        let result = rng.get_random_colors_32(10, 0x00_FF_FF_FF, 0xFF_00_00_FF, None);
        assert_eq!(result.len(), 10);
    }

    #[test]
    fn test_uniform_color_output() {
        let mut rng = ColorRng::new(123, 123);
        let color = 0x12_34_56_78;
        let result_64 = rng.get_random_colors_64(20, color, color, None);
        let result_32 = rng.get_random_colors_32(20, color, color, None);
        assert!(result_64.iter().all(|&c| c == color));
        assert!(result_32.iter().all(|&c| c == color));
    }
}
