use crate::core::context::gpu::GpuContext;
use crate::primitives::plane::Dimensions2d;

pub fn horizontal_lines_x4_gpu(
    buf: &mut [u32],
    buf_dimensions: &Dimensions2d,
    lines: &Vec<u32>,
    gpu_context: &mut GpuContext,
) -> Result<(), String> {
    #[cfg(feature = "gpu")]
    {

        println!("HorizontalLines X4 on the GPU!!!");

        use ocl::{Buffer, Kernel};
        let kernel_src = include_str!("horizontal_lines_x4.c");
        let kernel_name = "horizontal_lines_x4";
        let program_name = "horizontal_lines_x4_program";

        let num_lines = lines.len() / 4;
        if num_lines == 0 { return Ok(()); }
        let buf_len = buf.len();

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
            .copy_host_slice(&lines)
            .build()
            .map_err(|e| format!("Failed to create OpenCL lines buffer: {e}"))?;

        // Upload frame buffer (pooled)
        let frame_buf = gpu_context.get_or_create_buffer(buf_len, queue_ref, buf)?;

        // Launch kernel
        let kernel = Kernel::builder()
            .program(&program)
            .name(kernel_name)
            .queue(queue_value)
            .global_work_size(num_lines)
            .arg(&frame_buf)
            .arg(buf_dimensions.w)
            .arg(buf_dimensions.h)
            .arg(&lines_buf)
            .arg(num_lines as u32)
            .build()
            .map_err(|e| format!("Failed to build kernel: {e}"))?;

        unsafe {
            kernel.enq().map_err(|e| format!("Failed to enqueue kernel: {e}"))?;
        }

        // Download results to host
        frame_buf.read(buf).enq()
            .map_err(|e| format!("Failed to read GPU buffer back to host: {e}"))?;

        Ok(())
    }
    #[cfg(not(feature = "gpu"))]
    {
        Err("GPU support is not enabled at compile time.".to_string())
    }
}
