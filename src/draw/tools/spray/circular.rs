use crate::core::context::GraphContext;
use crate::primitives::math::MinMax;
use std::thread;

/// Applies a circular spray effect using the current brush.
/// Random points within the brush circle are colored. Uses multithreading when beneficial.
///
/// # Parameters
/// - `ctx`: The drawing context (framebuffer and brush info)
/// - `x`, `y`: Center of the spray in framebuffer coordinates
/// - `density`: Number of points to spray
/// - `colors`: Cycle of colors to use for each point
/// - `radius`: Radius of the circular spray
///
pub fn circular_spray<UserData>(
    ctx: &mut GraphContext<UserData>,
    x: u32,
    y: u32,
    density: u32,
    colors: &Vec<u32>,
    radius: f64,
) {
    // TODO: Consider adding support for Alpha!

    if ctx.num_threads == 0 || radius <= 0.0 {
        return;
    }

    // TODO: Attentively review the implementation of this function!!!!

    let colors_len = colors.len();
    let mut points = Vec::with_capacity(density as usize);
    let mut tries = 0;
    let max_tries = density * 6; // Allow more tries to fill density for high edge rejection
    let rad = radius as i32;
    let min = 0u32;
    let max = (2 * rad) as u32;

    // --- Random point generation (rejection sampling within circle) ---
    while points.len() < density as usize && tries < max_tries as usize {
        // FIXME: It's better to use `ctx.rng.get_vec_u32()` than `ctx.rng.get_u32` (should be more efficient)

        // Generate dx, dy in [-rad, rad] by shifting unsigned to signed domain
        let dx = ctx.rng.get_u32(&MinMax { min, max }) as i32 - rad;
        let dy = ctx.rng.get_u32(&MinMax { min, max }) as i32 - rad;
        if (dx * dx + dy * dy) as f64 <= radius * radius {
            let px = x as i32 + dx;
            let py = y as i32 + dy;
            let color_idx = points.len() % colors_len;
            points.push((px, py, color_idx));
        }
        tries += 1;
    }
    if points.is_empty() {
        return;
    }

    // Find min and max y among points
    let min_y = points.iter().map(|&(_, py, _)| py).min().unwrap();
    let max_y = points.iter().map(|&(_, py, _)| py).max().unwrap();
    let num_points = points.len();

    // --- Dynamic Thread Count Logic (same as rectangle) ---
    let min_points_per_thread = 128;
    let mut num_threads = (num_points + min_points_per_thread - 1) / min_points_per_thread;

    let min_threads = usize::min(2, ctx.num_threads);
    let max_threads = usize::max(2, ctx.num_threads);

    // Clamp to at least 2 threads, but not more than ctx.num_threads
    num_threads = num_threads.clamp(min_threads, max_threads);

    if ctx.num_threads == 1 || num_points < num_threads * 2 {
        // Single-threaded
        let w = ctx.win.w as i32;
        let h = ctx.win.h as i32;
        for &(px, py, color_idx) in &points {
            if px >= 0 && px < w && py >= 0 && py < h {
                let idx = (px + py * w) as usize;
                ctx.frame_buf[idx] = colors[color_idx];
            }
        }
        return;
    }

    // Band partitioning (by y coordinate)
    let band_height = ((max_y - min_y + 1) as usize + num_threads - 1) / num_threads;
    let w = ctx.win.w_i32;
    let h = ctx.win.h_i32;

    let mut band_points = vec![Vec::new(); num_threads];
    for (i, &(_, py, _)) in points.iter().enumerate() {
        let band = ((py - min_y) as usize / band_height).min(num_threads - 1);
        band_points[band].push(i);
    }
    let mut_band_ranges: Vec<(usize, usize)> = (0..num_threads)
        .map(|i| {
            let start_y = min_y + (i as i32 * band_height as i32);
            let end_y = ((start_y + band_height as i32).min(max_y + 1)).min(h);
            let start_idx = (start_y.max(0) * w).max(0) as usize;
            let end_idx = (end_y.max(0) * w).max(start_idx as i32) as usize;
            (start_idx, end_idx)
        })
        .collect();
    let frame_buf_ptr = ctx.frame_buf.as_mut_ptr();
    let frame_buf_len = ctx.frame_buf.len();
    let points = &points;

    // SAFETY: we guarantee unique non-overlapping slices
    thread::scope(|s| {
        for (band, indices) in band_points.into_iter().enumerate() {
            let (band_start, band_end) = mut_band_ranges[band];
            if band_start >= band_end || band_end > frame_buf_len {
                continue;
            }
            let band_slice = unsafe {
                std::slice::from_raw_parts_mut(frame_buf_ptr.add(band_start), band_end - band_start)
            };
            s.spawn(move || {
                for &i in &indices {
                    let (px, py, color_idx) = points[i];
                    if px >= 0 && px < w && py >= 0 && py < h {
                        // let rel_y = py - min_y;
                        let local_idx = ((px)
                            + (py - (min_y + (band as i32 * band_height as i32))) * w)
                            as usize;
                        if local_idx < band_slice.len() {
                            band_slice[local_idx] = colors[color_idx];
                        }
                    }
                }
            });
        }
    });
    // All threads joined here.
}
