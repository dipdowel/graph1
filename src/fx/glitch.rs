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

/// Glitch logic that processes a rectangular band of rows (either full-frame or a region)
fn horizontal_glitch_thread(
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

/// Applies a horizontal glitch to the full frame buffer or a just a region if `rect` is provided
/// # Arguments
/// * `ctx` - The graph context
/// * `props` - The properties for the horizontal glitch effect
/// * `region` - The rectangular region to apply the glitch to. If `None`, the full frame buffer gets glitched.
/// **NB:** If `region` is relatively  small (e.g. 128*128), the best performance is achieved with `ctx.num_threads = 1`!
/// **NB:** If `region` is larger, multiple threads noticeably improve performance.
/// **NB:** See some test results below (CPU with 6 cores):
///
/// #### 1 Thread:
/// - { w: 128, h: 128 } -- 23 microseconds
/// - { w: 800, h: 600 } -- 1124 microseconds
/// #### 4 Threads:
/// - { w: 128, h: 128 } -- 124 microseconds
/// - { w: 800, h: 600 } -- 720 microseconds
/// #### 6 Threads:
/// - { w: 128, h: 128 } -- 189 microseconds
/// - { w: 800, h: 600 } -- 503 microseconds
pub fn horizontal_glitch<UserData>(
    ctx: &mut GraphContext<UserData>,
    props: &HorizontalGlitchProps,
    region: Option<&RectArea>,
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

    // Define glitch bounds based on rect or full frame
    let (y_start, y_end, x_start, x_end) = match region {
        Some(r) => {
            let y_start = r.top_left.y.min(ctx.win.h);
            let y_end = (r.top_left.y + r.dimensions.h).min(ctx.win.h);
            let x_start = r.top_left.x.min(ctx.win.w) as usize;
            let x_end = (r.top_left.x + r.dimensions.w).min(ctx.win.w) as usize;
            (y_start, y_end, x_start, x_end)
        }
        None => (0, ctx.win.h, 0, ctx.win.w_usize),
    };

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
        horizontal_glitch_thread(
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
    chunk_size = chunk_size / row_width * row_width;
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
                horizontal_glitch_thread(
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
// It's a working test suite, but it requires some human interaction for now.
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
            rect: Option<&RectArea>,
        ) -> Duration {
            ctx.num_threads = num_threads;
            let mut duration_sum: Duration = Duration::new(0, 0);

            for _ in 0..iterations {
                let start = Instant::now();
                // horizontal_glitch(ctx, &props);

                horizontal_glitch(ctx, &props, rect);

                duration_sum += start.elapsed();
                // println!("duration_sum:  {:?}", duration_sum);
            }

            let average = duration_sum.div_f32(iterations as f32);

            let mut rect_status: String = String::from("None");
            if rect.is_some() {
                rect_status = format!("{:?}", rect.unwrap().dimensions);
            }
            if print {
                println!(
                    "> THR: {:?}, rect: {:?}, DUR: {:?} microsec",
                    num_threads,
                    rect_status,
                    average.as_micros()
                );
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

        let small_region = RectArea::new(128, 128, 128, 128, None);
        let large_region = RectArea::new(128, 128, 800, 600, None);
        let full_window_region = RectArea::new(0, 0, 1024, 768, None);

        let iterations: usize = 5_000;

        for num_threads in [1, 4, 6] {
            // measure(true, num_threads, iterations, &mut ctx, &props, None);
            measure(
                true,
                num_threads,
                iterations,
                &mut ctx,
                &props,
                Some(&small_region),
            );
            measure(
                true,
                num_threads,
                iterations,
                &mut ctx,
                &props,
                Some(&large_region),
            );
            // measure(true, num_threads, iterations, &mut ctx, &props, Some(&full_window_region));
        }
    }
}
*/