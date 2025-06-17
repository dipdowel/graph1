use crate::core::context::GraphContext;
use crate::primitives::math::MinMax;
use crate::primitives::plane::Dimensions2d;
use std::thread;

/// Applies a simple spray effect using the current brush.
/// Random points within the brush rectangle are colored. Uses multithreading when beneficial.
///
/// # Parameters
/// - `ctx`: The drawing context (framebuffer and brush info)
/// - `x`, `y`: Center of the spray in framebuffer coordinates
/// - `density`: Number of points to spray
/// - `colors`: Cycle of colors to use for each point
///
/// # Multithreading
/// If the spray area is large enough and the context requests >1 thread, this function will:
/// - Generate all spray points and their colors.
/// - Find the minimum and maximum y of the spray points.
/// - Divide the range [min_y, max_y] into horizontal bands (one per thread).
/// - Assign points to threads by which band their y value falls into.
/// - Each thread writes directly into its band’s slice of the framebuffer (safe, no data races).
/// Falls back to single-threaded if too few points or only one thread requested.
pub fn rectangular_spray<UserData>(
    ctx: &mut GraphContext<UserData>,
    x: u32,
    y: u32,
    density: u32,
    colors: &Vec<u32>,
    size:&Dimensions2d
) {
    // TODO: Consider adding support for Alpha!

    if ctx.num_threads == 0 {
        return;
    }

            let random_xs = ctx.rng.get_vec_u32(density as usize, &MinMax { min: 0, max: size.w - 1 });
            let random_ys = ctx.rng.get_vec_u32(density as usize, &MinMax { min: 0, max: size.h - 1 });
            let colors_len = colors.len();

            // Collect all spray points: (frame_x, frame_y, color_index)
            let mut points = Vec::with_capacity(density as usize);
            for ((&rnd_x, &rnd_y), i) in random_xs.iter().zip(random_ys.iter()).zip(0..) {
                let px = x as i32 + rnd_x as i32 - size.w as i32 / 2;
                let py = y as i32 + rnd_y as i32 - size.h as i32 / 2;
                points.push((px, py, i % colors_len));
            }

            if points.is_empty() {
                return;
            }


            // Find min and max y among points
            let min_y = points.iter().map(|&(_, py, _)| py).min().unwrap();
            let max_y = points.iter().map(|&(_, py, _)| py).max().unwrap();
            let num_points = points.len();


            // --- Dynamic Thread Count Logic ---
            // Use at least 2 threads, but never more than ctx.num_threads or n_points/min_points_per_thread.
            // For small sprays, use fewer threads to avoid overhead.
            let min_points_per_thread = 128; // Tweakable: minimum points per thread for multithreading to make sense

            let mut num_threads = (num_points + min_points_per_thread - 1) / min_points_per_thread;

            let min_threads = usize::min(2, ctx.num_threads);
            let max_threads = usize::max(2, ctx.num_threads);

            // Clamp to at least 2 threads, but not more than ctx.num_threads
            num_threads = num_threads.clamp(min_threads, max_threads);




            // If we still don't have enough points, or user requests 1 thread, fallback to single-threaded
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
            // ---------------------------------

            // Band partitioning
            let band_height = ((max_y - min_y + 1) as usize + num_threads - 1) / num_threads;

            let w = ctx.win.w_i32;
            let h = ctx.win.h_i32;
            // let frame_w = ctx.win.w as usize;

            // Allocate bands: each band will have a vector of point indices into `points`
            let mut band_points = vec![Vec::new(); num_threads];
            for (i, &(_, py, _)) in points.iter().enumerate() {
                let band = ((py - min_y) as usize / band_height).min(num_threads - 1);
                band_points[band].push(i);
            }

            // println!("num_threads: {}, band_height: {}, min_y: {}, max_y: {}", num_threads, band_height, min_y, max_y);

            // Mutable borrow splitting: slices for each band
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
            let points = &points; // Only borrow points (not move)

            // SAFETY: we guarantee unique non-overlapping slices
            thread::scope(|s| {
                for (band, indices) in band_points.into_iter().enumerate() {
                    let (band_start, band_end) = mut_band_ranges[band];
                    // If band is out of buffer, skip
                    if band_start >= band_end || band_end > frame_buf_len {
                        continue;
                    }
                    let band_slice = unsafe { std::slice::from_raw_parts_mut(frame_buf_ptr.add(band_start), band_end - band_start) };
                    s.spawn(move || {
                        // println!("Spray: Band {} processing {} points", band, indices.len());
                        for &i in &indices {
                            let (px, py, color_idx) = points[i];
                            if px >= 0 && px < w && py >= 0 && py < h {
                                let rel_y = py - min_y;
                                let local_idx = ((px) + (py - (min_y + (band as i32 * band_height as i32))) * w) as usize;
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
