use crate::core::context::gpu::GpuContext;
use crate::primitives::plane::Dimensions2d;

#[cfg(feature = "gpu")]
use ocl::{Kernel, Buffer};
use crate::buffer_op::gpu::kernel_bundle::KernelBundle;

///  Draw a batch of horizontal lines with GPU, download result into host buffer.
pub fn horizontal_lines_x4_gpu(
    cpu_frame_buf: &mut [u32],
    buf_dimensions: &Dimensions2d,
    lines: &Vec<u32>,
    gpu_context: &mut GpuContext,
) -> Result<(), String> {
    #[cfg(feature = "gpu")]
    {

        let bundle = horizontal_lines_x4_get_kernel(
            cpu_frame_buf, buf_dimensions, lines, gpu_context,
        )?;

        let kernel = bundle.kernel;
        let gpu_frame_buf = bundle.buffers.get(0)
            .ok_or("No frame buffer in kernel bundle")?;
        let lines_buf = bundle.buffers.get(1)
            .ok_or("No lines buffer in kernel bundle")?;

        // Upload current ctx.frame_buf to the GPU
        gpu_frame_buf.write(cpu_frame_buf.as_ref()).enq().map_err(|e| format!("Failed to upload frame buffer: {e}"))?;

        unsafe { kernel.enq().map_err(|e| format!("Failed to enqueue kernel: {e}"))?; }
        gpu_frame_buf.read(cpu_frame_buf).enq().map_err(|e| format!("Failed to read GPU buffer back to host: {e}"))?;
        Ok(())
    }
    #[cfg(not(feature = "gpu"))]
    {
        let _ = cpu_frame_buf;
        let _ = buf_dimensions;
        let _ = lines;
        let _ = gpu_context;
        Err("GPU support is not enabled at compile time.".to_string())
    }
}

/// Build and return the ready-to-enqueue kernel for batch horizontal lines.
/// Use with the kernel executor for GPU pipelines. Returns kernel and all relevant buffers.
pub fn horizontal_lines_x4_get_kernel(
    cpu_frame_buf: &mut [u32],
    buf_dimensions: &Dimensions2d,
    lines: &Vec<u32>,
    gpu_context: &mut GpuContext,
) -> Result<KernelBundle, String> {
    #[cfg(feature = "gpu")]
    {
        let kernel_src = include_str!("horizontal_lines_x4.c");
        let kernel_name = "horizontal_lines_x4";
        let program_name = "horizontal_lines_x4_program";

        let num_lines = lines.len() / 4;
        if num_lines == 0 { return Err("No lines to process.".into()); }
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

        // Upload line data
        let lines_buf = Buffer::<u32>::builder()
            .queue(queue_value.clone())
            .len(lines.len())
            .copy_host_slice(lines)
            .build()
            .map_err(|e| format!("Failed to create OpenCL lines buffer: {e}"))?;

        // Upload frame buffer (pooled)
        let gpu_frame_buf = gpu_context.get_or_create_buffer(buf_len, queue_ref, cpu_frame_buf)?;



        // Build (but do not enqueue) the kernel
        let kernel = Kernel::builder()
            .program(&program)
            .name(kernel_name)
            .queue(queue_value)
            .global_work_size(num_lines)
            .arg(&gpu_frame_buf)
            .arg(buf_dimensions.w)
            .arg(buf_dimensions.h)
            .arg(&lines_buf)
            .arg(num_lines as u32)
            .build()
            .map_err(|e| format!("Failed to build kernel: {e}"))?;


        Ok(KernelBundle {
            kernel,
            buffers: vec![gpu_frame_buf, lines_buf],
        })
    }
    #[cfg(not(feature = "gpu"))]
    {
        let _ = cpu_frame_buf;
        let _ = buf_dimensions;
        let _ = lines;
        let _ = gpu_context;
        Err("GPU support is not enabled at compile time.".to_string())
    }
}
