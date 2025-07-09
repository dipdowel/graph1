use crate::buffer_op::buffer_fx::blur::box_blur::box_blur;
use crate::core::context::gpu::GpuContext;
use crate::primitives::plane::Dimensions2d;
use std::sync::Arc;
use std::thread;

/// Clamps a value between a minimum and a maximum
fn clamp(v: i32, min: i32, max: i32) -> i32 {
    v.max(min).min(max)
}

/// Multithreaded box blur effect for a 2D image buffer.
/// Each thread processes a chunk of rows from the buffer.
///
/// # Parameters
/// * `buf` - The mutable buffer representing pixels (RGBA packed in u32)
/// * `buf_dimensions` - The dimensions (width, height) of the buffer
/// * `kernel_radius` - The radius of the blur kernel
/// * `num_threads` - The number of threads to use (>= 1)
///
/// This function performs a box blur by averaging pixel values in a square
/// region around each pixel. Edge pixels are handled with clamping.
///
/// Each thread processes a non-overlapping range of rows to avoid race conditions.

pub fn box_blur_threaded(
    buf: &mut [u32],
    buf_dimensions: &Dimensions2d<u32>,
    kernel_radius: u32,
    num_threads: usize,
    gpu_context: &mut GpuContext,
) {
    if num_threads == 0 || buf.is_empty() || kernel_radius == 0 {
        return; // Nothing to do
    }

    if num_threads == 1 || gpu_context.is_enabled() {
        // Fallback to single-threaded blur if only one thread is requested or GPU is enabled
        box_blur(buf, buf_dimensions, kernel_radius, gpu_context);
        return;
    }

    let width = buf_dimensions.w as usize;
    let height = buf_dimensions.h as usize;
    let radius = kernel_radius as i32;
    let threads = num_threads.max(1);
    let chunk_size = (height + threads - 1) / threads;

    let src = Arc::new(buf.to_vec()); // Original read-only buffer
    let dst = buf as *mut [u32]; // Mutable buffer pointer

    unsafe {
        thread::scope(|s| {
            for thread_idx in 0..threads {
                let src = Arc::clone(&src);
                let dst_slice = &mut *dst;
                s.spawn(move || {
                    let start_row = thread_idx * chunk_size;
                    let end_row = (start_row + chunk_size).min(height);

                    // TODO: think about extracting the common logic between this and `box_blur()`, maybe make an always inlined function?
                    for y in start_row..end_row {
                        for x in 0..width {
                            let mut sum_r = 0_u32;
                            let mut sum_g = 0_u32;
                            let mut sum_b = 0_u32;
                            let mut sum_a = 0_u32;
                            let mut count = 0_u32;

                            for ky in -radius..=radius {
                                let ny = clamp(y as i32 + ky, 0, (height - 1) as i32) as usize;
                                for kx in -radius..=radius {
                                    let nx = clamp(x as i32 + kx, 0, (width - 1) as i32) as usize;
                                    let idx = ny * width + nx;
                                    let pixel = src[idx];

                                    let r = (pixel >> 24) & 0xFF;
                                    let g = (pixel >> 16) & 0xFF;
                                    let b = (pixel >> 8) & 0xFF;
                                    let a = pixel & 0xFF;

                                    sum_r += r;
                                    sum_g += g;
                                    sum_b += b;
                                    sum_a += a;
                                    count += 1;
                                }
                            }

                            let avg_r = (sum_r / count) & 0xFF;
                            let avg_g = (sum_g / count) & 0xFF;
                            let avg_b = (sum_b / count) & 0xFF;
                            let avg_a = (sum_a / count) & 0xFF;

                            dst_slice[y * width + x] =
                                (avg_r << 24) | (avg_g << 16) | (avg_b << 8) | avg_a;
                        }
                    }
                });
            }
        });
    }
}
