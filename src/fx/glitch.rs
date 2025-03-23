use crate::core::context::GraphContext;
use crate::primitives::math::MinMax;
use crate::utils::math::rng::XorShiftRng;

pub struct HorizontalGlitchProps {
    /// horizontal shift strength
    pub strength: u32,

    /// The chance to apply the glitch effect 0 -- no chance, 255 -- always
    pub chance: u8,

    /// The balance between left and right shifts.
    /// 0 -- 100% of glitches gravitate to the left
    /// 128 -- balanced
    /// 255 -- 100% of glitches gravitate to the right
    pub left_right_balance: u8,
}

/// Applies a simple glitch effect to the whole window (single-threaded).
/// -
pub fn horizontal_glitch<UserData>(ctx: &mut GraphContext<UserData>, props: &HorizontalGlitchProps) {
    let HorizontalGlitchProps {
        strength,
        chance,
        left_right_balance: horizontal_balance,
    } = *props;

    // These props as zeros render the rest of the effect logic useless
    if chance == 0 || strength == 0 {
        return;
    }

    let height = ctx.frame_buf.len() / ctx.win.w_usize;
    let mut rng = XorShiftRng::new(ctx.frame_count as u32, ctx.frame_count as u64);
    let max_shift = MinMax::new(1, strength.min(ctx.win.w));
    let shifts = rng.get_vec_u32(height, &max_shift);
    let chances = rng.get_vec_f64(height);
    let chance_threshold: f64 = chance as f64 / u8::MAX as f64;

    let right_threshold = horizontal_balance as f64 / u8::MAX as f64;

    for y in 0..height {
        //  chance to glitch this row
        if chances[y] < chance_threshold {
            let y_start = y * ctx.win.w_usize;
            let row = &mut ctx.frame_buf[y_start..y_start + ctx.win.w_usize];
            let shift = shifts[y] as usize;

            // Glitch to the left or to the right
            if rng.get_f64() < right_threshold {
                row.rotate_right(shift);
            } else {
                row.rotate_left(shift);
            }

        }
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::context::WindowContext;

    #[test]
    fn test_glitch() {
        let mut ctx: GraphContext =
            GraphContext::new(WindowContext::default(), false, false, None, 1);
        let props = GlitchProps {
            strength: 10,
            chance: 16,
            horizontal_balance: 128,

        };

        window(&mut ctx, &props);
    }
}
*/