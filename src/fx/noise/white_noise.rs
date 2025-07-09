#[cfg(feature = "gpu")]
use crate::buffer_op::gpu::white_noise as gpu_white_noise;
use crate::core::context::gpu::GpuContext;
use crate::utils::color::math::{rgba_operation, ColorOperation};
use crate::utils::math::rng::gray::GrayRng;

pub struct WhiteNoiseProps {
    pub min_color: u8,
    pub max_color: u8,
    pub min_alpha: u8,
    pub max_alpha: u8,
    pub operation: Option<ColorOperation>,
    pub step: Option<usize>,

    /// Size of the noise circular buffer.
    /// I.e. how much of unique noise values to generate.
    /// Setting this to small values will result in repeating noise patterns.
    /// If `None`, the noise size will equal the entire target buffer size.
    pub noise_size: Option<usize>,
}

impl WhiteNoiseProps {
    pub fn new(
        min_color: u8,
        max_color: u8,
        min_alpha: u8,
        max_alpha: u8,
        operation: Option<ColorOperation>,
        step: Option<usize>,
        noise_size: Option<usize>,
    ) -> Self {
        Self {
            min_color,
            max_color,
            min_alpha,
            max_alpha,
            operation,
            step,
            noise_size,
        }
    }
}

pub struct WhiteNoise<'a> {
    gray_rng: GrayRng,
    props: &'a WhiteNoiseProps,
}

impl<'a> WhiteNoise<'a> {
    pub fn new(seed: u32, props: &'a WhiteNoiseProps) -> Self {
        Self {
            gray_rng: GrayRng::new(seed),
            props,
        }
    }

    pub fn generate_32(
        &mut self,
        target_buf: &mut [u32],
        props: Option<&'a WhiteNoiseProps>,
        seed: Option<u32>,
        gpu_context: &mut GpuContext,
    ) {
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
            noise_size,
        } = self.props;

        let use_alpha = *min_alpha != *max_alpha && *min_alpha != 0xff;

        let noise = self.gray_rng.get_random_grays_32(
            noise_size.unwrap_or(target_buf.len()),
            *min_color,
            *max_color,
            *min_alpha,
            *max_alpha,
            seed,
        );

        #[cfg(feature = "gpu")]
        if gpu_context.is_enabled() {
            let seed = seed.unwrap_or(0);
            gpu_white_noise::white_noise(
                target_buf,
                &noise,
                operation,
                use_alpha,
                *step,
                gpu_context,
            )
            .expect("GPU white noise generation failed");
            return;
        }

        // let use_alpha = *min_alpha != *max_alpha && *min_alpha != 0xff;
        let mut step = step.unwrap_or(1);
        if step == 0 {
            step = 1;
        }

        let noise_len = noise.len();

        if let Some(operation) = operation {
            for i in (0..target_buf.len()).step_by(step) {
                target_buf[i] =
                    // The noise buffer is used in a circular manner
                    rgba_operation(target_buf[i], noise[(i / step) % noise_len], operation.clone(), use_alpha);
            }
        } else {
            for i in (0..target_buf.len()).step_by(step) {
                // The noise buffer is expected to be of the same size as the target buffer
                target_buf[i] = noise[i % noise_len];
            }
        }
    }
}
