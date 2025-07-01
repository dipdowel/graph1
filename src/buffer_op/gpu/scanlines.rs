use crate::core::context::gpu::GpuContext;
use crate::primitives::plane::Dimensions2d;

#[cfg(feature = "gpu")]
use ocl::{Kernel, Buffer};
use crate::buffer_op::gpu::kernel_bundle::KernelBundle;

/// GPU-accelerated scanlines drawing (using scanline_sizes as y lookup).
pub fn scanlines_gpu(
    cpu_frame_buf: &mut [u32],
    buf_dimensions: &Dimensions2d,
    flat_scanline_data: &Vec<u32>,
    flat_data_ptrs: &Vec<u32>,
    scanline_sizes: &Vec<u32>,
    gpu_context: &mut GpuContext,
) -> Result<(), String> {
    #[cfg(feature = "gpu")]
    {
        let bundle = scanlines_get_kernel(
            cpu_frame_buf,
            buf_dimensions,
            flat_scanline_data,
            flat_data_ptrs,
            scanline_sizes,
            gpu_context,
        )?;
        let kernel = bundle.kernel;
        let gpu_frame_buf = bundle.buffers.get(0).ok_or("No frame buffer in kernel bundle")?;

        // Upload buffer to GPU
        gpu_frame_buf.write(cpu_frame_buf.as_ref()).enq().map_err(|e| format!("Failed to upload frame buffer: {e}"))?;

        unsafe { kernel.enq().map_err(|e| format!("Failed to enqueue kernel: {e}"))?; }
        gpu_frame_buf.read(cpu_frame_buf).enq().map_err(|e| format!("Failed to read GPU buffer back to host: {e}"))?;
        Ok(())
    }
    #[cfg(not(feature = "gpu"))]
    {
        let _ = cpu_frame_buf;
        let _ = buf_dimensions;
        let _ = flat_scanline_data;
        let _ = flat_data_ptrs;
        let _ = scanline_sizes;
        let _ = gpu_context;
        Err("GPU support is not enabled at compile time.".to_string())
    }
}

/// Returns ready-to-enqueue OpenCL kernel for scanlines operation.
/// See scanlines.c for details.
pub fn scanlines_get_kernel(
    cpu_frame_buf: &mut [u32],
    buf_dimensions: &Dimensions2d,
    flat_scanline_data: &Vec<u32>,
    flat_data_ptrs: &Vec<u32>,
    scanline_sizes: &Vec<u32>,
    gpu_context: &mut GpuContext,
) -> Result<KernelBundle, String> {
    #[cfg(feature = "gpu")]
    {
        let kernel_src = include_str!("scanlines.c");
        let kernel_name = "scanlines";
        let program_name = "scanlines_program";
        let buf_len = cpu_frame_buf.len();

        // Load/cached program
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

        // Upload all data to device
        let flat_scanline_data_buf = Buffer::<u32>::builder()
            .queue(queue_value.clone())
            .len(flat_scanline_data.len())
            .copy_host_slice(&flat_scanline_data)
            .build()
            .map_err(|e| format!("Failed to create OpenCL flat_scanline_data buffer: {e}"))?;
        let flat_data_ptrs_buf = Buffer::<u32>::builder()
            .queue(queue_value.clone())
            .len(flat_data_ptrs.len())
            .copy_host_slice(&flat_data_ptrs)
            .build()
            .map_err(|e| format!("Failed to create OpenCL flat_data_ptrs buffer: {e}"))?;
        let scanline_sizes_buf = Buffer::<u32>::builder()
            .queue(queue_value.clone())
            .len(scanline_sizes.len())
            .copy_host_slice(&scanline_sizes)
            .build()
            .map_err(|e| format!("Failed to create OpenCL scanline_sizes buffer: {e}"))?;
        let gpu_frame_buf = gpu_context.get_or_create_buffer(buf_len, queue_ref, cpu_frame_buf)?;

        let kernel = Kernel::builder()
            .program(&program)
            .name(kernel_name)
            .queue(queue_value)
            .global_work_size(buf_len)
            .arg(&gpu_frame_buf)
            .arg(buf_dimensions.w)
            .arg(buf_dimensions.h)
            .arg(&flat_scanline_data_buf)
            .arg(&flat_data_ptrs_buf)
            .arg(&scanline_sizes_buf)
            .build()
            .map_err(|e| format!("Failed to build kernel: {e}"))?;

        Ok(KernelBundle {
            kernel,
            buffers: vec![
                gpu_frame_buf,
                flat_scanline_data_buf,
                flat_data_ptrs_buf,
                scanline_sizes_buf,
            ],
        })
    }
    #[cfg(not(feature = "gpu"))]
    {
        Err("GPU support is not enabled at compile time.".to_string())
    }
}