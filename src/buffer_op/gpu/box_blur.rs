use crate::core::context::gpu::GpuContext;

use crate::primitives::plane::Dimensions2d;

#[cfg(feature = "gpu")]
use ocl::{Kernel};
#[cfg(feature = "gpu")]
use crate::buffer_op::gpu::kernel_bundle::KernelBundle;

/// GPU-accelerated box blur effect using OpenCL.
/// This uploads the image buffer to the GPU, processes it with the box blur kernel,
/// and downloads the result back to the CPU buffer.
///
/// # Parameters
/// * `cpu_buf` - The mutable CPU-side buffer (RGBA pixels packed in u32)
/// * `buf_dimensions` - The 2D dimensions of the buffer (width x height)
/// * `kernel_radius` - Radius of the blur kernel (square kernel)
/// * `gpu_context` - The GPU context for managing OpenCL resources
///
/// # Returns
/// * `Result<(), String>` - Ok on success, Err on any GPU failure
pub fn box_blur(
    cpu_buf: &mut [u32],
    buf_dimensions: &Dimensions2d<u32>,
    kernel_radius: u32,
    gpu_context: &mut GpuContext,
) -> Result<(), String> {
    #[cfg(feature = "gpu")]
    {
        let width = buf_dimensions.w as usize;
        let height = buf_dimensions.h as usize;        
        let bundle = box_blur_get_kernel(cpu_buf, width, height, kernel_radius, gpu_context)?;
        let kernel = bundle.kernel;
        let gpu_buf = bundle.buffers.get(0)
            .ok_or("No frame buffer in kernel bundle")?;

        gpu_buf.write(cpu_buf.as_ref()).enq().map_err(|e| format!("Upload error: {e}"))?;
        unsafe { kernel.enq().map_err(|e| format!("Kernel enqueue error: {e}"))?; }
        gpu_buf.read(cpu_buf).enq().map_err(|e| format!("Download error: {e}"))?;
        Ok(())
    }
    #[cfg(not(feature = "gpu"))]
    {
        let _ = cpu_buf;
        let _ = buf_dimensions;
        let _ = kernel_radius;
        let _ = gpu_context;
        Err("GPU support is not enabled.".to_string())
    }
}

/// Builds the OpenCL kernel and frame buffer for GPU-based box blur.
/// Returns a ready-to-enqueue kernel and its associated GPU buffer.
#[cfg(feature = "gpu")]
fn box_blur_get_kernel(
    cpu_buf: &mut [u32],
    width: usize,
    height: usize,
    kernel_radius: u32,
    gpu_context: &mut GpuContext,
) -> Result<KernelBundle, String> {
    #[cfg(feature = "gpu")]
    {
        let kernel_src = include_str!("box_blur.c");
        let kernel_name = "box_blur_kernel";
        let program_name = "box_blur_program";
        let buf_len = width * height;

        if gpu_context.get_program(program_name).is_none() {
            gpu_context.load_program(&kernel_src, program_name)
                .map_err(|e| format!("Program load error: {e}"))?;
        }
        let program = gpu_context.get_program(program_name)
            .ok_or("Program missing unexpectedly")?;
        let queue = gpu_context.queue.as_ref().ok_or("Missing GPU queue")?.clone();
        let gpu_buf = gpu_context.get_or_create_buffer(buf_len, queue.as_ref(), cpu_buf)?;

        let queue_value = queue.as_ref().clone();

        let kernel = Kernel::builder()
            .program(&program)
            .name(kernel_name)
            .queue(queue_value.clone())
            .global_work_size(buf_len)
            .arg(&gpu_buf)
            .arg(width as u32)
            .arg(height as u32)
            .arg(kernel_radius)
            .build()
            .map_err(|e| format!("Kernel build error: {e}"))?;

        Ok(KernelBundle::new(kernel, vec![gpu_buf]))
    }
    #[cfg(not(feature = "gpu"))]
    {
        let _ = cpu_buf;
        let _ = width;
        let _ = height;
        let _ = kernel_radius;
        let _ = gpu_context;
        Err("GPU support is not enabled.".to_string())
    }
}
