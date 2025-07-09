// src/buffer_op/gpu/kernel_executor.rs

#[cfg(feature = "gpu")]
use ocl::{Buffer, Kernel};

#[cfg(not(feature = "gpu"))]
use crate::buffer_op::gpu::kernel_bundle::{Buffer, Kernel};

/// Execute a sequence of kernels on the GPU in order, then read the result to the CPU buffer.
/// `kernels`: all kernels to be enqueued (should operate on the same frame buffer).
/// `frame_buf`: the OpenCL buffer representing the device-side framebuffer.
/// `cpu_buffer`: the mutable slice to receive the result (must match the device buffer size).
pub fn execute_kernels_and_read(
    kernels: &[Kernel],
    gpu_frame_buf: &Buffer<u32>,
    cpu_frame_buf: &mut [u32],
    upload_cpu_buf_to_gpu: bool,
) -> Result<(), String> {
    #[cfg(feature = "gpu")]
    {

        if upload_cpu_buf_to_gpu {
        // Upload current ctx.frame_buf to the GPU
        gpu_frame_buf
            .write(cpu_frame_buf.as_ref())
            .enq()
            .map_err(|e| format!("Failed to upload frame buffer: {e}"))?;

        }
        // Enqueue all kernels in order (no intermediate download)
        for kernel in kernels {
            unsafe {
                kernel
                    .enq()
                    .map_err(|e| format!("Failed to enqueue kernel: {e}"))?;
            }
        }
        // Download result back to CPU only ONCE at the end
        gpu_frame_buf
            .read(cpu_frame_buf)
            .enq()
            .map_err(|e| format!("Failed to read GPU buffer: {e}"))?;
        Ok(())
    }
    #[cfg(not(feature = "gpu"))]
    {
        let _ = kernels;
        let _ = gpu_frame_buf;
        let _ = cpu_frame_buf;
        let _ = upload_cpu_buf_to_gpu;
        Err("GPU support is not enabled at compile time.".to_string())
    }
}
