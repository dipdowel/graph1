use crate::core::context::GraphContext;
// use crate::utils::color::math::{rgba_operation, ColorOperation};

use crate::primitives::math::MinMax;
use crate::utils::math::rng::XorShiftRng;


pub struct GlitchProps {
    /// horizontal shift strength
    pub strength: u32,
    /// The chance to apply the glitch effect 0 -- no chance, 255 -- always
    pub chance: u8,

}

/// Applies a glitch effect to the whole framebuffer (single-threaded).
/// - Random horizontal shifts of pixel rows.
/// - Occasional color channel artifacts.
pub fn window<UserData>(ctx: &mut GraphContext<UserData>, props: &GlitchProps) {
    let GlitchProps{
        strength,
        chance,

    } = *props;

    // These props as zeros render the rest of the effect logic useless
    if chance == 0 || strength == 0 {
        return;
    }

    let height = ctx.frame_buf.len()  / ctx.win.w_usize;
    let mut rng = XorShiftRng::new(ctx.frame_count as u32, ctx.frame_count as u64);
    let max_shift = MinMax::new(1, strength.min(ctx.win.w));


    let shifts = rng.get_vec_u32(height, &max_shift);

    let chances = rng.get_vec_f64(height);
    let chance_threshold: f64 = chance as f64 / u8::MAX as f64;

    for y in 0..height {
        // ~20% chance to glitch this row
        if chances[y] < chance_threshold {
            let y_start = (y * ctx.win.w_usize) as usize;
            let row = &mut ctx.frame_buf[y_start..y_start + ctx.win.w_usize];

            // let shift = rng.get_u32(&max_shift) as usize;
            let shift = shifts[y] as usize;
            let right = rng.get_f64() < 0.5;

            if right {
                row.rotate_right(shift);
            } else {
                row.rotate_left(shift);
            }

            // // ~30% chance to apply color noise
            // if rng.get_f64() < 0.3 {
            //     for px in row.iter_mut() {
            //         *px = rgba_operation(*px, 0x11223300, ColorOperation::Add, false);
            //     }
            // }
        }
    }
}
