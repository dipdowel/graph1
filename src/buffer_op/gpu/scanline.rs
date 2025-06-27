use crate::core::context::gpu::GpuContext;

pub fn scanline_fx(
    buffer: &mut [u32],
    width: u32,
    size: u8,
    intensity: u8,
    gpu_context: &mut GpuContext,
) -> Result<(), String> {
    #[cfg(feature = "gpu")]
    {
        use ocl::Kernel;
        let kernel_src = include_str!("scanline_fx.c");
        let kernel_name = "scanline_fx";
        let program_name = "scanline_fx_program";

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

        let buf_len = buffer.len();

        let frame_buf = gpu_context.get_or_create_buffer(buf_len, queue_ref, buffer)?;

        //TODO: remove the debug print in production code!
        // println!("GPU scanline!");

        let queue_value = queue.as_ref().clone();

        // Prepare kernel
        let kernel = Kernel::builder()
            .program(&program)
            .name(kernel_name)
            .queue(queue_value)
            .global_work_size(buf_len)
            .arg(&frame_buf)
            .arg((intensity as u32) << 24 | (intensity as u32) << 16 | (intensity as u32) << 8 | 0xff) // match CPU packing
            .arg(width * size as u32)
            .arg(buf_len as u32)
            .arg(width)
            .build()
            .map_err(|e| format!("Failed to build kernel: {e}"))?;

        frame_buf.write(buffer.as_ref()).enq().map_err(|e| format!("Failed to upload frame buffer: {e}"))?;


        // Launch kernel
        unsafe { kernel.enq().map_err(|e| format!("Failed to enqueue kernel: {e}"))?; }

        // Ensure host buffer is up-to-date
        frame_buf.read(buffer).enq()
            .map_err(|e| format!("Failed to read GPU buffer back to host: {e}"))?;

        Ok(())
    }
    #[cfg(not(feature = "gpu"))]
    {
        Err("GPU support is not enabled at compile time.".to_string())
    }
}
