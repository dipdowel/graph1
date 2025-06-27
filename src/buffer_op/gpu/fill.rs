use crate::core::context::gpu::GpuContext;

pub fn fill(
    buffer: &mut [u32],
    buf_len: usize,
    color: u32,
    gpu_context: &mut GpuContext,
) -> Result<(), String> {
    #[cfg(feature = "gpu")]
    {
        use ocl::Kernel;

        if !gpu_context.is_enabled() {
            return Err("GPU context is not enabled".to_string());
        }

        // OpenCL kernel
        let kernel_src = include_str!("fill_buffer.cl");
        let kernel_name = "fill_buffer";
        let program_name = "fill_buffer_program";

        // Load/cached program
        if gpu_context.get_program(program_name).is_none() {
            gpu_context
                .load_program(&kernel_src, program_name)
                .map_err(|e| format!("Failed to build OpenCL program: {e}"))?;
        }
        // Get everything by immutable borrow up front
        let program = gpu_context
            .get_program(program_name)
            .ok_or("Program not loaded (unknown error)")?;
        let queue = gpu_context
            .queue
            .as_ref()
            .ok_or("No OpenCL queue in context")?
            .clone(); // Arc<Queue>
        let queue_ref = queue.as_ref();


        let frame_buf = gpu_context.get_or_create_buffer(buf_len, queue_ref, buffer)?;

        println!("GPU filling! Using OpenCL buffer of size: {} bytes", buf_len * std::mem::size_of::<u32>());
        let queue_value = queue.as_ref().clone();

        // Prepare kernel
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

        // Launch kernel
        unsafe {
            kernel
                .enq()
                .map_err(|e| format!("Failed to enqueue kernel: {e}"))?;
        }

        // Synchronize to make sure host buffer is up to date (blocking read, no copy if mapped)
        frame_buf
            .read(buffer)
            .enq()
            .map_err(|e| format!("Failed to read GPU buffer back to host: {e}"))?;

        Ok(())
    }
    #[cfg(not(feature = "gpu"))]
    {
        Err("GPU support is not enabled at compile time.".to_string())
    }
}
