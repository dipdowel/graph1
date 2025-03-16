use crate::primitives::math::{ColorPair, MinMax};
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

    pub fn get_random_monochromes(
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
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    // TODO: write proper tests!!!

    fn test_vec_range_0_10_applied() {
        let mut color_rng = ColorRng::new(42);
        let colors = color_rng.get_random_colors(10, 0x11_22_33_ff, 0x22_33_44_FF, None);

        // println!(">>> color: {:#010X?}",colors);

        let bw = color_rng.get_random_monochromes(8, 0x11, 0x22, 0xff, 0xff, None);

        println!(">>> bw: {:#010X?}", bw);

        assert_eq!(1, 1);
    }
}
