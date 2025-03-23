use crate::core::context::GraphContext;
// use crate::utils::color::math::{rgba_operation, ColorOperation};

use crate::primitives::math::MinMax;
use crate::utils::math::rng::XorShiftRng;

/// Applies a glitch effect to the whole framebuffer (single-threaded).
/// - Random horizontal shifts of pixel rows.
/// - Occasional color channel artifacts.
pub fn window<UserData>(ctx: &mut GraphContext<UserData>, strength: u32) {
    
    let height = ctx.frame_buf.len()  / ctx.win.w_usize;

    let mut rng = XorShiftRng::new(ctx.frame_count as u32, ctx.frame_count as u64);
    let max_shift = MinMax::new(1, strength.min(ctx.win.w));

    for y in 0..height {
        // ~20% chance to glitch this row
        if rng.get_f64() < 0.2 {
            let y_start = (y * ctx.win.w_usize) as usize;
            let row = &mut ctx.frame_buf[y_start..y_start + ctx.win.w_usize];

            let shift = rng.get_u32(&max_shift) as usize;
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
