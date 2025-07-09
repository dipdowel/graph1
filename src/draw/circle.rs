use crate::core::context::GraphContext;
use crate::primitives::Pixel;
use std::thread;

/// Helper thread function to draw a portion of the circle on a frame buffer slice.
fn draw_lines_of_circle_thread(
    circle_slice: &mut [u32],
    line_length: usize,
    chunk_top_y: u32,
    center_x: u32,
    center_y: u32,
    radius_sq: i32,
    color: u32,
    skip_every: u32,
) {
    for line in 0..(circle_slice.len() / line_length) {
        let global_y = chunk_top_y + line as u32;
        let delta_y = global_y as i32 - center_y as i32;

        // Skip stylized lines
        if skip_every > 1 && (line as u32) % skip_every != 0 {
            continue;
        }

        let delta_y_sq = delta_y * delta_y;
        if delta_y_sq > radius_sq {
            continue;
        }

        let width_half = ((radius_sq - delta_y_sq) as f64).sqrt().round() as i32;

        let start_x = center_x as i32 - width_half;
        let end_x = center_x as i32 + width_half;

        let start_x = start_x.max(0) as usize;
        let end_x = end_x.min(line_length as i32 - 1) as usize;

        let row_start = line * line_length;
        for x in start_x..=end_x {
            circle_slice[row_start + x] = color;
        }
    }
}

/// Draws a filled circle with multithreading support (like rectangle::filled).
/// **NB:** Circle drawing logic seems to be buggy at the moment! Don't use it!
/// FIXME: fix the circle drawing logic.
/// FIXME: fix the circle drawing logic.
/// FIXME: fix the circle drawing logic.
/// FIXME: fix the circle drawing logic.
/// FIXME: fix the circle drawing logic.

pub fn filled<UserData>(
    ctx: &mut GraphContext<UserData>,
    center: &Pixel,
    radius: u32,
    skip_every: u32,
) {
    if ctx.num_threads == 0 {
        return;
    }

    let radius_sq = (radius * radius) as i32;
    let skip_every = skip_every.max(1);

    // Bounding box
    let top_y = center.y.saturating_sub(radius);
    let bottom_y = (center.y + radius).min(ctx.win.h - 1);
    let line_length = ctx.win.w_usize;

    let first_line_start = (top_y * ctx.win.w) as usize;
    let last_line_end = ((bottom_y + 1) * ctx.win.w) as usize;

    let mut chunk_size = usize::div_ceil(last_line_end - first_line_start, ctx.num_threads);

    // Force single-threaded if chunk too small
    if ctx.num_threads == 1 || chunk_size < line_length {
        let circle_slice = &mut ctx.frame_buf[first_line_start..last_line_end];
        draw_lines_of_circle_thread(
            circle_slice,
            line_length,
            top_y,
            center.x,
            center.y,
            radius_sq,
            center.color,
            skip_every,
        );
        return;
    }

    // Align chunk size to full rows
    chunk_size = chunk_size / line_length * line_length;

    let circle_slice = &mut ctx.frame_buf[first_line_start..last_line_end];
    let mut chunks: Vec<&mut [u32]> = circle_slice.chunks_mut(chunk_size).collect();

    thread::scope(|s| {
        for (i, chunk) in chunks.iter_mut().enumerate() {
            let chunk_top_y = top_y + ((i * chunk_size) / line_length) as u32;

            s.spawn(move || {
                draw_lines_of_circle_thread(
                    chunk,
                    line_length,
                    chunk_top_y,
                    center.x,
                    center.y,
                    radius_sq,
                    center.color,
                    skip_every,
                )
            });
        }
    });
}
