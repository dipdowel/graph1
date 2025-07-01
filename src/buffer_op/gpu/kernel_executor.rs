// src/buffer_op/gpu/kernel_executor.rs

#[cfg(feature = "gpu")]
use ocl::{Kernel, Buffer};

#[cfg(not(feature = "gpu"))]
use crate::buffer_op::gpu::kernel_bundle::{Kernel, Buffer};

/// Execute a sequence of kernels on the GPU in order, then read the result to the CPU buffer.
/// `kernels`: all kernels to be enqueued (should operate on the same frame buffer).
/// `frame_buf`: the OpenCL buffer representing the device-side framebuffer.
/// `cpu_buffer`: the mutable slice to receive the result (must match the device buffer size).
pub fn execute_kernels_and_read(
    kernels: &[Kernel],
    frame_buf: &Buffer<u32>,
    cpu_buffer: &mut [u32],
) -> Result<(), String> {
    #[cfg(feature = "gpu")]
    {
        // Enqueue all kernels in order (no intermediate download)
        for kernel in kernels {
            unsafe { kernel.enq().map_err(|e| format!("Failed to enqueue kernel: {e}"))?; }
        }
        // Download result back to CPU only ONCE at the end
        frame_buf.read(cpu_buffer).enq().map_err(|e| format!("Failed to read GPU buffer: {e}"))?;
        Ok(())
    }
    #[cfg(not(feature = "gpu"))]
    {
        let _ = kernels;
        let _ = frame_buf;
        let _ = cpu_buffer;
        Err("GPU support is not enabled at compile time.".to_string())
    }
}
