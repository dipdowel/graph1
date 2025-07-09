use crate::buffer_op;
use crate::core::context::gpu::GpuContext;
use crate::primitives::plane::Dimensions2d;

/// Clamps a value between a minimum and a maximum
fn clamp(v: i32, min: i32, max: i32) -> i32 {
    v.max(min).min(max)
}

/// Box blur effect, **single-threader**
pub fn box_blur(
    buf: &mut [u32],
    buf_dimensions: &Dimensions2d<u32>,
    kernel_radius: u32,
    gpu_context: &mut GpuContext,
) {
    // ==[ GPU OpenCL ]=======================================================================
    if gpu_context.is_enabled() {
        buffer_op::gpu::box_blur::box_blur(buf, buf_dimensions, kernel_radius, gpu_context)
            .expect("gpu::fill::fill() failed :(");
        return;
    }

    // ==[ CPU ]================================================================================
    let width = buf_dimensions.w as usize;
    let height = buf_dimensions.h as usize;
    let radius = kernel_radius as i32;

    let src = buf.to_vec(); // read-only original
    let dst = buf; // write directly to input buffer

    for y in 0..height {
        for x in 0..width {
            let mut sum_r = 0_u32;
            let mut sum_g = 0_u32;
            let mut sum_b = 0_u32;
            let mut sum_a = 0_u32;
            let mut count = 0_u32;

            // Iterate over the kernel window centered around (x, y)
            for ky in -radius..=radius {
                // Neighbor y
                let ny = clamp(y as i32 + ky, 0, (height - 1) as i32) as usize;
                // Kernel x offset (see b
                for kx in -radius..=radius {
                    let nx = clamp(x as i32 + kx, 0, (width - 1) as i32) as usize;
                    let idx = ny * width + nx;
                    let pixel = src[idx];

                    // Extract RGBA components
                    let r = (pixel >> 24) & 0xFF;
                    let g = (pixel >> 16) & 0xFF;
                    let b = (pixel >> 8) & 0xFF;
                    let a = pixel & 0xFF;

                    // Accumulate color channels
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

            let out_pixel = (avg_r << 24) | (avg_g << 16) | (avg_b << 8) | avg_a;
            dst[y * width + x] = out_pixel;
        }
    }
}

//  How Kernel Window works
//---------------------------------------------
// If the kernel_radius = 1, the kernel is 3×3:
// When the loop is at the center pixel (x, y), kx iterates from -1 to +1,
// representing how far left or right the current neighbor is from the center pixel x.
// `kx` is added to `x` to get the neighbor’s X coordinate (nx).
// That neighbor is then read and included in the blur average.
//---------------------------------------------
// kx:     -1     0     +1
//      ┌────────────────────┐
// ky -1│(-1,-1)(0,-1)(+1,-1)│
// ky  0│(-1, 0)(0, 0)(+1, 0)│
// ky +1│(-1,+1)(0,+1)(+1,+1)│
//      └────────────────────┘
