use crate::utils::color::math::{rgba_operation, ColorOperation};
use crate::utils::math::rng::gray::GrayRng;

pub struct WhiteNoiseProps {
    pub min_color: u8,
    pub max_color: u8,
    pub min_alpha: u8,
    pub max_alpha: u8,
    pub operation: Option<ColorOperation>,
    pub step: Option<usize>,
}

impl WhiteNoiseProps {
    pub fn new(
        min_color: u8,
        max_color: u8,
        min_alpha: u8,
        max_alpha: u8,
        operation: Option<ColorOperation>,
        step: Option<usize>,
    ) -> Self {
        Self {
            min_color,
            max_color,
            min_alpha,
            max_alpha,
            operation,
            step,
        }
    }
}

pub struct WhiteNoise<'a> {
    gray_rng: GrayRng,
    props: &'a WhiteNoiseProps,
}

impl<'a> WhiteNoise<'a> {
    pub fn new(seed:u32, props: &'a WhiteNoiseProps) -> Self {
        Self {
            gray_rng: GrayRng::new(seed),
            props,
        }
    }

    pub fn generate_32(&mut self, target_buf: &mut [u32], props: Option<&'a WhiteNoiseProps>, seed:Option<u32>) {
        // Save new props, if provided
        if let Some(props) = props {
            self.props = props;
        }


        let WhiteNoiseProps {
            min_color,
            max_color,
            min_alpha,
            max_alpha,
            operation,
            step,


        } = self.props;

        let noise = self.gray_rng.get_random_grays_32(
            target_buf.len(),
            *min_color,
            *max_color,
            *min_alpha,
            *max_alpha,
            seed,
        );



        let use_alpha = *min_alpha != *max_alpha && *min_alpha != 0xff;
        let mut step = step.unwrap_or(1);
        if step == 0 {
            step = 1;
        }




        if let Some(operation) = operation {
            for i in (0..target_buf.len()).step_by(step) {
                target_buf[i] = rgba_operation(target_buf[i], noise[i / step],
                                               operation.clone(), use_alpha);
            }
        } else {
            for i in (0..target_buf.len()).step_by(step) {
                target_buf[i] = noise[i];
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_white_noise_basic_fill() {
        let props = WhiteNoiseProps::new(0x10, 0x20, 0xff, 0xff, None, None);
        let mut noise = WhiteNoise::new(123, &props);
        let mut buffer = vec![0u32; 16];
        noise.generate_32(&mut buffer, None, None);

        for &px in &buffer {
            let gray = (px >> 24) & 0xff;
            let alpha = px & 0xff;
            assert!(gray >= 0x10 && gray <= 0x20);
            assert_eq!(px >> 16 & 0xff, gray);
            assert_eq!(px >> 8 & 0xff, gray);
            assert_eq!(alpha, 0xff);
        }
    }

    #[test]
    fn test_white_noise_with_add_operation() {
        let props = WhiteNoiseProps::new(0x10, 0x10, 0xff, 0xff, Some(ColorOperation::Add), None);
        let mut buffer = vec![0x10_10_10_ff; 4];
        let mut noise = WhiteNoise::new(42, &props);
        noise.generate_32(&mut buffer, None, None);

        for &px in &buffer {
            let r = (px >> 24) & 0xff;
            let g = (px >> 16) & 0xff;
            let b = (px >> 8) & 0xff;
            assert_eq!(r, g);
            assert_eq!(g, b);
            assert!(r >= 0x10);
        }
    }

    #[test]
    fn test_white_noise_step_interval() {
        let props = WhiteNoiseProps::new(0x00, 0xff, 0xff, 0xff, None, Some(2));
        let mut buffer = vec![0x12345678; 8];
        let mut noise = WhiteNoise::new(999, &props);
        noise.generate_32(&mut buffer, None, None);

        for i in 0..8 {
            if i % 2 == 0 {
                assert_ne!(buffer[i], 0x12345678);
            } else {
                assert_eq!(buffer[i], 0x12345678);
            }
        }
    }

    #[test]
    fn test_white_noise_alpha_range() {
        let props = WhiteNoiseProps::new(0x40, 0x40, 0x10, 0x20, None, None);
        let mut buffer = vec![0u32; 8];
        let mut noise = WhiteNoise::new(888, &props);
        noise.generate_32(&mut buffer, None, None);

        for &px in &buffer {
            let gray = (px >> 24) & 0xff;
            let alpha = px & 0xff;
            assert_eq!(gray, 0x40);
            assert!(alpha >= 0x10 && alpha <= 0x20);
        }
    }
}