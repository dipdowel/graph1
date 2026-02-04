use crate::core::context::gpu::GpuContext;
use crate::utils::color::math::ColorOperation;


use crate::buffer_op::gpu::kernel_bundle::KernelBundle;
#[cfg(feature = "gpu")]
use ocl::Kernel;

/// GPU-accelerated fade effect using OpenCL.
/// Applies a color operation to each pixel in the buffer.
///
/// # Parameters
/// * `cpu_buf` - The mutable CPU-side buffer (RGBA pixels packed in u32)
/// * `width` - Width of the image
/// * `height` - Height of the image
/// * `color_operand` - The color operand (as RGBA packed in u32)
/// * `op` - Color operation (Add or Subtract)
/// * `use_alpha` - Whether alpha is affected
/// * `gpu_context` - The GPU context for managing OpenCL resources
#[allow(dead_code)]
pub fn fade_gpu(
    cpu_buf: &mut [u32],
    width: u32,
    height: u32,
    color_operand: u32,
    op: ColorOperation,
    use_alpha: bool,
    gpu_context: &mut GpuContext,
) -> Result<(), String> {
    #[cfg(feature = "gpu")]
    {
        let buf_len = (width * height) as usize;
        let bundle = fade_get_kernel(
            cpu_buf,
            width,
            height,
            color_operand,
            op,
            use_alpha,
            gpu_context,
        )?;
        let kernel = bundle.kernel;
        let gpu_buf = bundle.buffers.get(0).ok_or("No GPU buffer")?;

        gpu_buf
            .write(cpu_buf.as_ref())
            .enq()
            .map_err(|e| format!("Upload error: {e}"))?;
        unsafe {
            kernel
                .enq()
                .map_err(|e| format!("Kernel enqueue error: {e}"))?;
        }
        gpu_buf
            .read(cpu_buf)
            .enq()
            .map_err(|e| format!("Download error: {e}"))?;
        Ok(())
    }
    #[cfg(not(feature = "gpu"))]
    {
        let _ = (
            cpu_buf,
            width,
            height,
            color_operand,
            op,
            use_alpha,
            gpu_context,
        );
        Err("GPU support is not enabled.".to_string())
    }
}


pub fn fade_get_kernel(
    cpu_buf: &mut [u32],
    width: u32,
    height: u32,
    color_operand: u32,
    op: ColorOperation,
    use_alpha: bool,
    gpu_context: &mut GpuContext,
) -> Result<KernelBundle, String> {
    #[cfg(feature = "gpu")]
    {
        let kernel_src = include_str!("fade.c");
        let kernel_name = "fade_kernel";
        let program_name = "fade_program";
        let buf_len = (width * height) as usize;

        if gpu_context.get_program(program_name).is_none() {
            gpu_context
                .load_program(kernel_src, program_name)
                .map_err(|e| format!("Program load error: {e}"))?;
        }

        let program = gpu_context
            .get_program(program_name)
            .ok_or("Program missing unexpectedly")?;
        let queue = gpu_context
            .queue
            .as_ref()
            .ok_or("Missing GPU queue")?
            .clone();
        let gpu_buf = gpu_context.get_or_create_buffer(buf_len, queue.as_ref(), cpu_buf)?;
        let queue_value = queue.as_ref().clone();

        let kernel = Kernel::builder()
            .program(&program)
            .name(kernel_name)
            .queue(queue_value)
            .global_work_size(buf_len)
            .arg(&gpu_buf)
            .arg(width)
            .arg(height)
            .arg(color_operand)
            .arg(op as i32)
            .arg(use_alpha as i32)
            .build()
            .map_err(|e| format!("Kernel build error: {e}"))?;

        Ok(KernelBundle::new(kernel, vec![gpu_buf]))
    }
    #[cfg(not(feature = "gpu"))]
    {
        let _ = (cpu_buf, width, height, color_operand, op, use_alpha, gpu_context);
        Err("GPU support is not enabled.".to_string())
    }
}
