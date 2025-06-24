use crate::core::context::alpha::AlphaMethod;
use crate::core::context::GraphContext;
use crate::primitives::numeric::Numeric;
use crate::primitives::plane::RectArea;
use crate::primitives::point::Point;
use crate::utils::color::alpha::{blend_pixel_f32, blend_pixel_int};

use crate::draw;
use std::cmp::PartialEq;

use std::thread;

#[cfg(feature = "gpu")]
use crate::buffer_op;


impl PartialEq for AlphaMethod {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (AlphaMethod::Int, AlphaMethod::Int) => true,
            (AlphaMethod::Float, AlphaMethod::Float) => true,
            _ => false,
        }
    }
}
/// A lower-level function that draws a part of a rectangle in a slice of the frame buffer.
/// Meant to be used as a thread, while parallelizing the drawing of a rectangle.
///
/// # Arguments
/// * `rectangle_slice` - A mutable slice of the frame buffer where a part of the rectangle will be drawn.
/// * `line_length` - The length of a horizontal line in the frame buffer (kind of like a scanline on a TV screen).
/// * `total_lines` - The total number of horizontal lines in the slice.
/// * `color_start` - The index of the first pixel to be colored in the 'scanline'
/// * `color_end` - The index of the last pixel to be colored in the 'scanline'
/// * `color` - The color to fill the rectangle with
/// * `alpha_method` - The method of alpha blending to use, or `None` for direct overwrite
fn draw_lines_of_rectangle_thread(
    rectangle_slice: &mut [u32],
    line_length: usize,
    total_lines: usize,
    color_start: usize,
    color_end: usize,
    color: u32,
    alpha_method: Option<AlphaMethod>,
) {
    match alpha_method {
        // No alpha blending — just overwrite pixels
        None => {
            for line in 0..total_lines {
                let base = line * line_length;
                // Ensure we don't go beyond the edge of the screen on the right and on the bottom
                // to avoid unwanted glitches and buffer overflow.
                let line_end = (base + color_end)
                    .min(base + line_length)
                    .min(rectangle_slice.len());
                let line_start = (base + color_start).min(line_end); // Ensure we don’t go backwards
                for idx in line_start..line_end {
                    rectangle_slice[idx] = color;
                }
            }
        }
        // Integer-based alpha blending
        Some(AlphaMethod::Int) => {
            for line in 0..total_lines {
                let base = line * line_length;
                // Ensure we don't go beyond the edge of the screen on the right and on the bottom
                // to avoid unwanted glitches and buffer overflow.
                let line_end = (base + color_end)
                    .min(base + line_length)
                    .min(rectangle_slice.len());
                let line_start = (base + color_start).min(line_end); // Ensure we don’t go backwards
                for idx in line_start..line_end {
                    rectangle_slice[idx] = blend_pixel_int(rectangle_slice[idx], color);
                }
            }
        }
        // Float-based alpha blending
        Some(AlphaMethod::Float) => {
            for line in 0..total_lines {
                let base = line * line_length;
                // Ensure we don't go beyond the edge of the screen on the right and on the bottom
                // to avoid unwanted glitches and buffer overflow.
                let line_end = (base + color_end)
                    .min(base + line_length)
                    .min(rectangle_slice.len());
                let line_start = (base + color_start).min(line_end); // Ensure we don’t go backwards
                for idx in line_start..line_end {
                    rectangle_slice[idx] = blend_pixel_f32(rectangle_slice[idx], color);
                }
            }
        }
    }
}

/// Draws a rectangle with dimensions and filled with a color specified in the `RectArea` struct.
/// Multithreaded. Falls back to single-threaded rendering if threading is disabled or not feasible.
///
/// # Arguments
/// * `ctx` - The graph context
/// * `rect` - The rectangle to draw
pub fn filled<UserData, T: Numeric>(ctx: &mut GraphContext<UserData>, rect: &RectArea<T>) {
    // ==[ GPU / OpenCL ]=======================================================================
    #[cfg(feature = "gpu")]
    {
        let mut gpu_context: Option<&mut crate::core::context::gpu::GpuContext> = None;
        if ctx.gpu_context.enabled {
            gpu_context = Some(&mut ctx.gpu_context);
            // Use GPU/OpenCL to fill the buffer
            buffer_op::gpu::draw::rectangle::draw_rectangle_gpu(
                &mut ctx.frame_buf,
                &ctx.win.dimensions,
                &rect.convert::<u32>(),
                gpu_context
                    .expect("gpu::draw_rectangle() failed, something went wrong with GPU context"),
            );
        }
    }

    // Do nothing if threading is not enabled
    if ctx.num_threads == 0 {
        return;
    }

    // Determine color: fallback to foreground if not set
    let color = rect.color.unwrap_or(ctx.win.foreground_color);

    // Determine if alpha blending is enabled
    let alpha_method = ctx.alpha.enabled.then_some(ctx.alpha.method);

    // Framebuffer and rectangle layout variables
    let line_length = ctx.win.w_usize;
    let total_lines = T::to_u32(rect.dimensions.h) as usize;
    let color_start = T::to_u32(rect.top_left.x) as usize;
    let color_end = T::to_u32(rect.top_left.x + rect.dimensions.w) as usize;

    // Slice start/end in framebuffer
    let first_line_start = (T::to_u32(rect.top_left.y) * ctx.win.w) as usize;
    let last_line_end = (T::to_u32(rect.top_left.y + rect.dimensions.h) * ctx.win.w) as usize;

    let last_line_end = last_line_end.min(ctx.frame_buf.len()); // Ensure we don’t go beyond the framebuffer length
    let rectangle_slice = &mut ctx.frame_buf[first_line_start..last_line_end];

    // Calculate how much work each thread should do
    let chunk_len = rectangle_slice.len();
    let mut chunk_size = usize::div_ceil(chunk_len, ctx.num_threads);

    // If chunks are smaller than a full line, it’s not worth threading
    let too_small_for_threads = chunk_size < line_length;
    if ctx.num_threads == 1 || too_small_for_threads {
        draw_lines_of_rectangle_thread(
            rectangle_slice,
            line_length,
            total_lines,
            color_start,
            color_end,
            color,
            alpha_method,
        );
        return;
    }

    // Ensure chunk size is line-aligned so each thread gets whole scanlines
    chunk_size = (chunk_size / line_length) * line_length;

    let mut chunks: Vec<&mut [u32]> = rectangle_slice.chunks_mut(chunk_size).collect();

    // Use thread::scope to safely spawn threads
    thread::scope(|s| {
        for chunk in chunks.iter_mut() {
            let lines = chunk.len() / line_length;

            s.spawn(move || {
                draw_lines_of_rectangle_thread(
                    chunk,
                    line_length,
                    lines,
                    color_start,
                    color_end,
                    color,
                    alpha_method,
                );
            });
        }
    }); // The scope for the scoped threads ends here. All the threads are expected to be joined automagically at this point.
}

/// Draws outline of the given rectangle using settings from `LineContext` (`ctx.line`).
///
/// # Arguments
/// * `ctx` - Mutable reference to the rendering context
/// * `rect_area` - The rectangle area to draw the outline for
///
/// This function draws four sides (top, bottom, left, right) of the rectangle.
pub fn outline<UserData, T: Numeric>(ctx: &mut GraphContext<UserData>, rect_area: &RectArea<T>) {
    let RectArea {
        top_left,
        dimensions,
        color,
    } = rect_area;
    let color = *color;

    let x = T::to_i32(top_left.x);
    let y = T::to_i32(top_left.y);
    let w = T::to_i32(dimensions.w);
    let h = T::to_i32(dimensions.h);

    let top_left_i32 = Point::new(x, y);
    let top_right_i32 = Point::new(x + w - 1, y);
    let bottom_left_i32 = Point::new(x, y + h - 1);

    draw::line::horizontal(ctx, &top_left_i32, w as u32, color); // Top edge
    draw::line::horizontal(ctx, &bottom_left_i32, w as u32, color); // Bottom edge
    draw::line::vertical(ctx, &top_left_i32, h as u32, color); // Left edge
    draw::line::vertical(ctx, &top_right_i32, h as u32, color); // Right edge
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




/// Renders rectangles from `rects` in a more efficient manner compared to rendering the rectangles one by one.
///
/// # Parameters
/// - `ctx`: The rendering context with a writable frame buffer.
/// - `rects`: A vector of rectangle references.
pub fn filled_multiple<DemoUserData, T: Numeric + Copy + std::ops::Add<Output = T> + Send + Sync + 'static>(
    ctx: &mut GraphContext<DemoUserData>,
    rects: &Vec<&RectArea<T>>,
) {
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
            let frame_buf = unsafe {
                std::slice::from_raw_parts_mut(frame_buf_ptr, frame_buf_len)
            };
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












// /// Renders rectangles from `rects` in a more efficient manner compared to rendering the rectangles one by one.
// ///
// /// # Parameters
// /// - `ctx`: The rendering context with a writable frame buffer.
// /// - `rects`: A vector of rectangle references.
// pub fn filled_multiple_old<UserData, T: Numeric + Copy + std::ops::Add<Output = T>>(
//     ctx: &mut GraphContext<UserData>,
//     rects: &Vec<&RectArea<T>>,
// ) {
//     // Sort rectangles by their top-left corner to ensure consistent rendering order
//     let mut sorted_rects = rects.clone();
//     sorted_rects.sort_by_key(|rect| (T::to_u32(rect.top_left.y), T::to_u32(rect.top_left.x)));
//
//     let width = ctx.win.w;
//     let height = ctx.win.h;
//
//     for rect in sorted_rects {
//         let y_start = T::to_u32(rect.top_left.y);
//         let y_end = T::to_u32(rect.top_left.y + rect.dimensions.h);
//         let x_start = T::to_u32(rect.top_left.x);
//         let x_end = T::to_u32(rect.top_left.x + rect.dimensions.w);
//         let color = rect.color.unwrap_or(ctx.win.foreground_color);
//
//         for y in y_start..y_end {
//             if y >= height {
//                 continue;
//             }
//             let base_index = (y * width) as usize;
//
//             for x in x_start..x_end {
//                 if x < width {
//                     ctx.frame_buf[base_index + x as usize] = color;
//                 }
//             }
//         }
//     }
// }
