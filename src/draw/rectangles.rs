use crate::core::context::GraphContext;
use crate::primitives::numeric::Numeric;
use crate::primitives::plane::RectArea;

use crate::buffer_op::gpu::fill_rects::filled_multiple_gpu;
use crate::buffer_op::gpu::fill_rects_bucketed::fill_rects_bucketed;
use crate::buffer_op::gpu::fill_rects_spatial_tiles::fill_rects_spatial_tiles;
use std::thread;

pub enum SpatialPartitioning {
    /// No spatial partitioning, draw rectangles

    /// Spatial partitioning using 1 row of vertical buckets
    Bucket(Option<u32>),
    /// Spatial partitioning using a grid of tiles.
    /// Values should be in `[1..=32]`.
    Grid(Option<u32>, Option<u32>),
}

/// Renders rectangles from `rects` in a more efficient manner compared to rendering the rectangles one by one.
///
/// # Parameters
/// - `ctx`: The rendering context with a writable frame buffer.
/// - `rects`: A vector of rectangle references.
/// - `partitioning`: Give the GPU a hint regarding your preferred spatial partitioning strategy. `None` means no partitioning.
pub fn filled_multiple<
    DemoUserData,
    T: Numeric + Copy + std::ops::Add<Output = T> + Send + Sync + 'static,
>(
    ctx: &mut GraphContext<DemoUserData>,
    rects: &Vec<&RectArea<T>>,
    partitioning: Option<SpatialPartitioning>,
) {
    if ctx.gpu_context.is_enabled() {
        match partitioning {
            Some(SpatialPartitioning::Bucket(num_buckets)) => fill_rects_bucketed(
                &mut ctx.frame_buf,
                &ctx.win.dimensions,
                rects,
                ctx.win.foreground_color,
                num_buckets,
                &mut ctx.gpu_context,
            )
            .expect("GPU bucketed rect fill failed"),
            Some(SpatialPartitioning::Grid(tiles_x, tiles_y)) => fill_rects_spatial_tiles(
                &mut ctx.frame_buf,
                &ctx.win.dimensions,
                rects,
                ctx.win.foreground_color,
                tiles_x,
                tiles_y,
                &mut ctx.gpu_context,
            )
            .expect("GPU tiled rect fill failed"),
            _ => {
                // No spatial partitioning, just draw rectangles
                filled_multiple_gpu(
                    &mut ctx.frame_buf,
                    &ctx.win.dimensions,
                    rects,
                    ctx.win.foreground_color,
                    &mut ctx.gpu_context,
                )
                .expect("GPU rect fill failed")
            }
        }
    }

    // Nothing to do here...
    if ctx.num_threads < 1 {
        return;
    }

    let mut sorted_rects = rects.clone();
    sorted_rects.sort_by_key(|rect| (T::to_u32(rect.top_left.y), T::to_u32(rect.top_left.x)));

    let num_threads = ctx.num_threads;
    let mut sorted_rects = rects.clone();
    sorted_rects.sort_by_key(|rect| (T::to_u32(rect.top_left.y), T::to_u32(rect.top_left.x)));

    let height = ctx.win.h;
    let chunk_size = (ctx.win.h_usize + num_threads - 1) / num_threads;
    let width = ctx.win.w;
    let default_color = ctx.win.foreground_color;

    if ctx.num_threads == 1 {
        draw_rectangles_in_range_thread(
            &mut ctx.frame_buf,
            ctx.win.w,
            ctx.win.h,
            0,
            ctx.win.h,
            &sorted_rects,
            default_color,
        );
        return;
    }

    let frame_buf_ptr = ctx.frame_buf.as_mut_ptr();
    let frame_buf_len = ctx.frame_buf.len();

    thread::scope(|s| {
        for thread_id in 0..num_threads {
            let start_y = (thread_id * chunk_size) as u32;
            let end_y = ((thread_id + 1) * chunk_size).min(height as usize) as u32;
            let rects = sorted_rects.clone();
            let frame_buf = unsafe { std::slice::from_raw_parts_mut(frame_buf_ptr, frame_buf_len) };
            s.spawn(move || {
                draw_rectangles_in_range_thread(
                    frame_buf,
                    width,
                    height,
                    start_y,
                    end_y,
                    &rects,
                    default_color,
                );
            });
        }
    });
}



/// Draws all rectangles that intersect a vertical scanline range directly into the frame buffer.
///
/// # Parameters
/// - `frame_buf`: The mutable frame buffer to draw into.
/// - `width`: The width of the frame buffer (in pixels).
/// - `height`: The height of the frame buffer (in pixels).
/// - `start_y`: The first scanline (inclusive) the thread is responsible for.
/// - `end_y`: The last scanline (exclusive) the thread is responsible for.
/// - `rects`: A list of rectangle references to draw.
/// - `default_color`: The fallback color to use if a rectangle doesn't specify one.
fn draw_rectangles_in_range_thread<T: Numeric + Copy + std::ops::Add<Output = T>>(
    frame_buf: &mut [u32],
    width: u32,
    height: u32,
    start_y: u32,
    end_y: u32,
    rects: &[&RectArea<T>],
    default_color: u32,
) {
    for rect in rects {
        // let y_start = rect.top_left.y.to_u32().max(start_y);
        let y_start = T::to_u32(rect.top_left.y).max(start_y);
        let y_end = T::to_u32(rect.top_left.y + rect.dimensions.h).min(end_y);
        let x_start = T::to_u32(rect.top_left.x);
        let x_end = T::to_u32(rect.top_left.x + rect.dimensions.w);
        let color = rect.color.unwrap_or(default_color);

        for y in y_start..y_end {
            if y >= height {
                continue;
            }
            let base_index = (y * width) as usize;

            for x in x_start..x_end {
                if x < width {
                    frame_buf[base_index + x as usize] = color;
                }
            }
        }
    }
}
