use crate::core::context::gpu::GpuContext;


use crate::buffer_op::gpu::kernel_bundle::KernelBundle;
#[cfg(feature = "gpu")]
use ocl::Kernel;

/// Apply scanline FX using the GPU, downloads result to host buffer after.
pub fn scanline_fx(
    cpu_frame_buf: &mut [u32],
    width: u32,
    size: u8,
    intensity: u8,
    gpu_context: &mut GpuContext,
) -> Result<(), String> {
    #[cfg(feature = "gpu")]
    {
        let bundle = scanline_get_kernel(cpu_frame_buf, width, size, intensity, gpu_context)?;
        let kernel = bundle.kernel;
        let gpu_frame_buf = bundle
            .buffers
            .get(0)
            .ok_or("No frame buffer in kernel bundle")?;

        // Upload current ctx.frame_buf to the GPU
        gpu_frame_buf
            .write(cpu_frame_buf.as_ref())
            .enq()
            .map_err(|e| format!("Failed to upload frame buffer: {e}"))?;

        unsafe {
            kernel
                .enq()
                .map_err(|e| format!("Failed to enqueue kernel: {e}"))?;
        }
        // Download result
        gpu_frame_buf
            .read(cpu_frame_buf)
            .enq()
            .map_err(|e| format!("Failed to read GPU buffer: {e}"))?;
        Ok(())
    }
    #[cfg(not(feature = "gpu"))]
    {
        let _ = cpu_frame_buf;
        let _ = width;
        let _ = size;
        let _ = intensity;
        let _ = gpu_context;
        Err("GPU support is not enabled at compile time.".to_string())
    }
}

/// Build and return the ready-to-enqueue OpenCL kernel for scanline FX.
/// Use with the kernel executor for pipelined/multi-effect GPU workflows.
/// Returns the kernel and frame buffer (so it lives long enough).

pub fn scanline_get_kernel(
    cpu_frame_buf: &mut [u32],
    width: u32,
    size: u8,
    intensity: u8,
    gpu_context: &mut GpuContext,
) -> Result<KernelBundle, String> {
    #[cfg(feature = "gpu")]
    {
        let kernel_src = include_str!("scanline_fx.c");
        let kernel_name = "scanline_fx";
        let program_name = "scanline_fx_program";

        let buf_len = cpu_frame_buf.len();

        // Load or cache the program
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
        let gpu_frame_buf = gpu_context.get_or_create_buffer(buf_len, queue_ref, cpu_frame_buf)?;

        // Build (do not enqueue)
        let kernel = Kernel::builder()
            .program(&program)
            .name(kernel_name)
            .queue(queue_value)
            .global_work_size(buf_len)
            .arg(&gpu_frame_buf)
            .arg(
                (intensity as u32) << 24
                    | (intensity as u32) << 16
                    | (intensity as u32) << 8
                    | 0xff,
            )
            .arg(width * size as u32)
            .arg(buf_len as u32)
            .arg(width)
            .build()
            .map_err(|e| format!("Failed to build kernel: {e}"))?;

        Ok(KernelBundle::new(kernel, vec![gpu_frame_buf]))
    }
    #[cfg(not(feature = "gpu"))]
    {
        let _ = cpu_frame_buf;
        let _ = width;
        let _ = size;
        let _ = intensity;
        let _ = gpu_context;
        Err("GPU support is not enabled at compile time.".to_string())
    }
}
