use crate::core::context::GraphContext;
use crate::primitives::math::MinMax;
use crate::primitives::plane::RectArea;
use crate::utils::math::rng::XorShiftRng;
use std::thread;

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

fn horizontal_glitch_thread(
    rows_slice: &mut [u32],
    row_width: usize,
    row_count: usize,
    max_shift: MinMax<u32>,
    chance_threshold: f64,
    right_threshold: f64,
    mut rng: XorShiftRng,
) {
    let shifts = rng.get_vec_u32(row_count, &max_shift);
    let chances = rng.get_vec_f64(row_count);

    for (i, row) in rows_slice.chunks_mut(row_width).enumerate() {
        if chances[i] < chance_threshold {
            let shift = shifts[i] as usize;
            if rng.get_f64() < right_threshold {
                row.rotate_right(shift);
            } else {
                row.rotate_left(shift);
            }
        }
    }
}

/// Applies a simple horizontal glitch effect to the whole window.

pub fn horizontal_glitch<UserData>(
    ctx: &mut GraphContext<UserData>,
    props: &HorizontalGlitchProps,
) {
    let HorizontalGlitchProps {
        strength,
        chance,
        left_right_balance: horizontal_balance,
    } = *props;

    // These props as zeros make the rest of the effect logic useless
    if chance == 0 || strength == 0 || ctx.num_threads == 0 {
        return;
    }

    let row_width = ctx.win.w_usize;
    let height = ctx.frame_buf.len() / row_width;
    let max_shift = MinMax::new(1, strength.min(ctx.win.w));
    let chance_threshold = chance as f64 / u8::MAX as f64;
    let right_threshold: f64 = horizontal_balance as f64 / u8::MAX as f64;

    if ctx.num_threads == 1 {
        let rng = XorShiftRng::new(ctx.frame_count as u32, ctx.frame_count as u64);
        horizontal_glitch_thread(
            &mut ctx.frame_buf,
            row_width,
            height,
            max_shift,
            chance_threshold,
            right_threshold,
            rng,
        );
        return;
    }

    // ==[ MULTI THREADS ]=========================================================================
    let mut chunk_size = usize::div_ceil(ctx.frame_buf.len(), ctx.num_threads);
    chunk_size = chunk_size / row_width * row_width; // align to full rows

    let mut chunks: Vec<&mut [u32]> = ctx.frame_buf.chunks_mut(chunk_size).collect();

    let base_seed_u32 = ctx.frame_count as u32;
    let base_seed_u64 = ctx.frame_count as u64;

    thread::scope(|s| {
        for (chunk_index, chunk) in chunks.iter_mut().enumerate() {
            let row_count = chunk.len() / row_width;

            s.spawn(move || {
                let rng = XorShiftRng::new(
                    base_seed_u32 + chunk_index as u32,
                    base_seed_u64 + chunk_index as u64,
                );
                horizontal_glitch_thread(
                    chunk,
                    row_width,
                    row_count,
                    max_shift,
                    chance_threshold,
                    right_threshold,
                    rng,
                );
            });
        }
    });
}

/// Applies a simple horizontal glitch effect to a region of the window.
/// TODO: Implement a multi-threaded version of this function!
pub fn horizontal_glitch_region<UserData>(
    ctx: &mut GraphContext<UserData>,
    props: &HorizontalGlitchProps,
    rect: &RectArea,
) {
    let HorizontalGlitchProps {
        strength,
        chance,
        left_right_balance: horizontal_balance,
    } = *props;

    if chance == 0 || strength == 0 {
        return;
    }

    let row_width = ctx.win.w_usize;
    let chance_threshold = chance as f64 / u8::MAX as f64;
    let right_threshold = horizontal_balance as f64 / u8::MAX as f64;
    let max_shift = MinMax::new(1, strength.min(rect.dimensions.w));

    let y_start = rect.top_left.y.min(ctx.win.h);
    let y_end = (rect.top_left.y + rect.dimensions.h).min(ctx.win.h);
    let x_start = rect.top_left.x.min(ctx.win.w);
    let x_end = (rect.top_left.x + rect.dimensions.w).min(ctx.win.w);

    if y_start >= y_end || x_start >= x_end {
        return;
    }

    let mut rng = XorShiftRng::new(ctx.frame_count as u32, ctx.frame_count as u64);
    let row_count = (y_end - y_start) as usize;
    let shifts = rng.get_vec_u32(row_count, &max_shift);
    let chances = rng.get_vec_f64(row_count);

    for i in 0..row_count {
        if chances[i] >= chance_threshold {
            continue;
        }

        let y = y_start + i as u32;
        let row_offset = y as usize * row_width;
        let row = &mut ctx.frame_buf[row_offset..row_offset + row_width];

        let sub_row = &mut row[x_start as usize..x_end as usize];
        let shift = shifts[i] as usize;

        if rng.get_f64() < right_threshold {
            sub_row.rotate_right(shift);
        } else {
            sub_row.rotate_left(shift);
        }
    }
}

/*
// It's a working test, but it requires some human interaction for now.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::context::WindowContext;
    use std::time::{Duration, Instant};

    #[test]
    fn test_glitch_performance() {
        fn measure(
            print: bool,
            num_threads: usize,
            iterations: usize,
            ctx: &mut GraphContext,
            props: &HorizontalGlitchProps,
        ) -> Duration {
            ctx.num_threads = num_threads;
            let mut duration_sum: Duration = Duration::new(0, 0);

            for _ in 0..iterations {
                let start = Instant::now();
                horizontal_glitch(ctx, &props);
                duration_sum += start.elapsed();
                // println!("duration_sum:  {:?}", duration_sum);
            }

            let average = duration_sum.div_f32(iterations as f32);

            if print {
                println!("> THR: {}, DUR: {:?}", num_threads, average);
            }
            average
        }

        let mut ctx: GraphContext = GraphContext::new(
            WindowContext::new(1024, 768, Some(0x11_22_33_ff), Some(0x44_66_11_ff)),
            false,
            false,
            None,
            1,
        );

        let props = HorizontalGlitchProps {
            strength: 200,
            chance: 220,
            left_right_balance: 128,
        };

        let iterations: usize = 8_000;

        for num_threads in 0..13 {
            measure(true, num_threads, iterations, &mut ctx, &props);
        }
    }
}
*/
