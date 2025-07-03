use crate::core::context::gpu::GpuContext;
use crate::buffer_op::gpu::kernel_bundle::KernelBundle;

#[cfg(feature = "gpu")]
use ocl::{Kernel, Buffer};

use crate::utils::color::math::ColorOperation;

pub fn white_noise(
    target_buf: &mut [u32],
    noise_buf: &[u32],
    operation: &Option<ColorOperation>,
    use_alpha: bool,
    gpu_context: &mut GpuContext,
) -> Result<(), String> {
    #[cfg(feature = "gpu")]
    {
        let bundle = get_white_noise_kernel(target_buf, noise_buf, operation, use_alpha, gpu_context)?;
        let kernel = bundle.kernel;
        let target_gpu_buf = bundle.buffers.get(0).ok_or("Missing target buffer")?;
        let noise_gpu_buf = bundle.buffers.get(1).ok_or("Missing noise buffer")?;

        target_gpu_buf.write(target_buf.as_ref()).enq().map_err(|e| format!("Upload target failed: {e}"))?;
        noise_gpu_buf.write(noise_buf).enq().map_err(|e| format!("Upload noise failed: {e}"))?;

        unsafe { kernel.enq().map_err(|e| format!("Kernel enqueue failed: {e}"))?; }

        target_gpu_buf.read(target_buf).enq().map_err(|e| format!("Download target failed: {e}"))?;
        Ok(())
    }
    #[cfg(not(feature = "gpu"))]
    {
        let _ = target_buf;
        let _ = noise_buf;
        let _ = operation;
        let _ = use_alpha;
        let _ = gpu_context;
        Err("GPU support not enabled.".to_string())
    }
}

fn get_white_noise_kernel(
    target_buf: &mut [u32],
    noise_buf: &[u32],
    operation: &Option<ColorOperation>,
    use_alpha: bool,
    gpu_context: &mut GpuContext,
) -> Result<KernelBundle, String> {
    #[cfg(feature = "gpu")]
    {
        let kernel_src = include_str!("white_noise.c");
        let kernel_name = "apply_noise_kernel";
        let program_name = "apply_noise_program";
        let len = target_buf.len();
        let noise_len = noise_buf.len();

        if gpu_context.get_program(program_name).is_none() {
            gpu_context.load_program(kernel_src, program_name)
                .map_err(|e| format!("Program load error: {e}"))?;
        }
        let program = gpu_context.get_program(program_name).unwrap();
        let queue = gpu_context.queue.as_ref().ok_or("Missing GPU queue")?.clone();

        let target_gpu_buf = gpu_context.get_or_create_buffer(len, queue.as_ref(), target_buf)?;
        let noise_gpu_buf = gpu_context.get_or_create_readonly_buffer(noise_len, queue.as_ref(), noise_buf)?;

        let op_code = match operation {
            Some(ColorOperation::Add) => 0u8,
            Some(ColorOperation::Subtract) => 1u8,
            None => return Err("No color operation specified".into()),
        };

        let kernel = Kernel::builder()
            .program(&program)
            .name(kernel_name)
            .queue(queue.as_ref().clone())
            .global_work_size(len)
            .arg(&target_gpu_buf)
            .arg(&noise_gpu_buf)
            .arg(len as u32)
            .arg(noise_len as u32)
            .arg(op_code)
            .arg(if use_alpha { 1u8 } else { 0u8 })
            .build()
            .map_err(|e| format!("Kernel build error: {e}"))?;

        Ok(KernelBundle::new(kernel, vec![target_gpu_buf, noise_gpu_buf]))
    }
    #[cfg(not(feature = "gpu"))]
    {
        let _ = target_buf;
        let _ = noise_buf;
        let _ = operation;
        let _ = use_alpha;
        let _ = gpu_context;
        Err("GPU support not enabled.".to_string())
    }
}