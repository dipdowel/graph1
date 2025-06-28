use crate::core::context::gpu::GpuContext;


#[cfg(feature = "gpu")]
use ocl::{Kernel, Buffer};
use crate::buffer_op::gpu::kernel_bundle::KernelBundle;

/// Fill the GPU buffer with a color (in-place, downloads to host after).
pub fn fill(
    buffer: &mut [u32],
    buf_len: usize,
    color: u32,
    gpu_context: &mut GpuContext,
) -> Result<(), String> {
    #[cfg(feature = "gpu")]
    {
        let bundle = fill_get_kernel(buffer, buf_len, color, gpu_context)?;
        let kernel = bundle.kernel;
        let frame_buf = bundle.buffers.get(0)
            .ok_or("No frame buffer in kernel bundle")?;
        
        unsafe { kernel.enq().map_err(|e| format!("Failed to enqueue kernel: {e}"))?; }
        // Download
        frame_buf.read(buffer).enq().map_err(|e| format!("Failed to read GPU buffer back to host: {e}"))?;
        Ok(())
    }
    #[cfg(not(feature = "gpu"))]
    {
        let _ = buffer;
        let _ = buf_len;
        let _ = color;
        let _ = gpu_context;
        Err("GPU support is not enabled at compile time.".to_string())
    }
}

/// Build and return the ready-to-enqueue OpenCL kernel for fill operation.
/// Use with the kernel executor for multi-effect GPU pipelines.
/// Returns the kernel and frame buffer (so it lives long enough).
pub fn fill_get_kernel(
    buffer: &mut [u32],
    buf_len: usize,
    color: u32,
    gpu_context: &mut GpuContext,
) -> Result<KernelBundle, String> {
    #[cfg(feature = "gpu")]
    {
        let kernel_src = include_str!("fill_buffer.c");
        let kernel_name = "fill_buffer";
        let program_name = "fill_buffer_program";

        // Load or cache program
        if gpu_context.get_program(program_name).is_none() {
            gpu_context
                .load_program(&kernel_src, program_name)
                .map_err(|e| format!("Failed to build OpenCL program: {e}"))?;
        }
        let program = gpu_context
            .get_program(program_name)
            .ok_or("Program not loaded (unknown error)")?;
        let queue = gpu_context
            .queue
            .as_ref()
            .ok_or("No OpenCL queue in context")?
            .clone();
        let queue_ref = queue.as_ref();
        let queue_value = queue.as_ref().clone();

        // Frame buffer (pooled)
        let frame_buf = gpu_context.get_or_create_buffer(buf_len, queue_ref, buffer)?;

        // (Optional) upload current buffer to device
        frame_buf.write(buffer.as_ref()).enq().map_err(|e| format!("Failed to upload frame buffer: {e}"))?;

        // Build (do not enqueue)
        let kernel = Kernel::builder()
            .program(&program)
            .name(kernel_name)
            .queue(queue_value)
            .global_work_size(buf_len)
            .arg(&frame_buf)
            .arg(color)
            .arg(buf_len as u32)
            .build()
            .map_err(|e| format!("Failed to build kernel: {e}"))?;

        Ok(KernelBundle::new(kernel, vec![frame_buf]))
    }
    #[cfg(not(feature = "gpu"))]
    {
        let _ = buffer;
        let _ = buf_len;
        let _ = color;
        let _ = gpu_context;
        Err("GPU support is not enabled at compile time.".to_string())
    }
}
