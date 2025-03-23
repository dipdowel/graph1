use crate::core::context::GraphContext;
use crate::primitives::math::MinMax;
use crate::primitives::plane::RectArea;
use crate::utils::math::rng::XorShiftRng;
use std::thread;

/// Properties for horizontal glitch effect
pub struct HorizontalGlitchProps {
    /// Maximum number of pixels a row can be shifted left or right
    pub strength: u32,

    /// Probability of applying the glitch to a row (0 = never, 255 = always)
    pub chance: u8,

    /// Shift direction bias (0 = left, 255 = right, 128 = balanced)
    pub left_right_balance: u8,
}

/// Applies the glitch effect to a given slice of rows
/// Each row is shifted left or right randomly, based on props
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

/// Applies horizontal glitching across the entire frame buffer
/// Automatically uses multithreading if ctx.num_threads > 1
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

    // If only 1 thread is allowed, run glitch logic directly
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

    // Spawn one thread per chunk
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

/// Applies glitching only within a rectangular region (single-threaded)
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

/// Threaded logic for applying horizontal glitching to a region
fn horizontal_glitch_region_thread(
    slice: &mut [u32],
    row_width: usize,
    row_count: usize,
    x_range: (usize, usize),
    max_shift: MinMax<u32>,
    chance_threshold: f64,
    right_threshold: f64,
    mut rng: XorShiftRng,
) {
    let shifts = rng.get_vec_u32(row_count, &max_shift);
    let chances = rng.get_vec_f64(row_count);

    for (i, row) in slice.chunks_mut(row_width).enumerate() {
        if chances[i] < chance_threshold {
            let shift = shifts[i] as usize;
            let sub_row = &mut row[x_range.0..x_range.1];

            if rng.get_f64() < right_threshold {
                sub_row.rotate_right(shift);
            } else {
                sub_row.rotate_left(shift);
            }
        }
    }
}

/// Multithreaded version of region glitching
pub fn horizontal_glitch_region_multi<UserData>(
    ctx: &mut GraphContext<UserData>,
    props: &HorizontalGlitchProps,
    rect: &RectArea,
) {
    let HorizontalGlitchProps {
        strength,
        chance,
        left_right_balance: horizontal_balance,
    } = *props;

    if chance == 0 || strength == 0 || ctx.num_threads == 0 {
        return;
    }

    let row_width = ctx.win.w_usize;
    let chance_threshold = chance as f64 / u8::MAX as f64;
    let right_threshold = horizontal_balance as f64 / u8::MAX as f64;

    let y_start = rect.top_left.y.min(ctx.win.h);
    let y_end = (rect.top_left.y + rect.dimensions.h).min(ctx.win.h);
    let x_start = rect.top_left.x.min(ctx.win.w) as usize;
    let x_end = (rect.top_left.x + rect.dimensions.w).min(ctx.win.w) as usize;

    if y_start >= y_end || x_start >= x_end {
        return;
    }

    let row_count = (y_end - y_start) as usize;
    let max_shift = MinMax::new(1, strength.min((x_end - x_start) as u32));

    let first_pixel = y_start as usize * row_width;
    let last_pixel = y_end as usize * row_width;
    let region_slice = &mut ctx.frame_buf[first_pixel..last_pixel];

    // ==[ SINGLE THREAD ]=======================================================================
    if ctx.num_threads == 1 {
        let rng = XorShiftRng::new(ctx.frame_count as u32, ctx.frame_count as u64);
        horizontal_glitch_region_thread(
            region_slice,
            row_width,
            row_count,
            (x_start, x_end),
            max_shift,
            chance_threshold,
            right_threshold,
            rng,
        );
        return;
    }

    // ==[ MULTI THREAD ]=======================================================================
    let mut chunk_size = usize::div_ceil(region_slice.len(), ctx.num_threads);
    chunk_size = chunk_size / row_width * row_width; // ensure row alignment

    let mut chunks: Vec<&mut [u32]> = region_slice.chunks_mut(chunk_size).collect();

    let base_seed_u32 = ctx.frame_count as u32;
    let base_seed_u64 = ctx.frame_count as u64;

    thread::scope(|s| {
        for (chunk_index, chunk) in chunks.iter_mut().enumerate() {
            let rows_in_chunk = chunk.len() / row_width;
            let seed_u32 = base_seed_u32 + chunk_index as u32;
            let seed_u64 = base_seed_u64 + chunk_index as u64;
            let x_range = (x_start, x_end);

            s.spawn(move || {
                let rng = XorShiftRng::new(seed_u32, seed_u64);
                horizontal_glitch_region_thread(
                    chunk,
                    row_width,
                    rows_in_chunk,
                    x_range,
                    max_shift,
                    chance_threshold,
                    right_threshold,
                    rng,
                );
            });
        }
    });
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
                // horizontal_glitch(ctx, &props);
                horizontal_glitch_region_multi(ctx, &props, &RectArea::new(0, 0, 800, 600, None));
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
            strength: 500,
            chance: 250,
            left_right_balance: 128,
        };

        let iterations: usize = 8_000;

        for num_threads in 0..8 {
            measure(true, num_threads, iterations, &mut ctx, &props);
        }
    }
}
*/