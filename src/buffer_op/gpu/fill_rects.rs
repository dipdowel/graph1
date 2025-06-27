use crate::core::context::gpu::GpuContext;
use crate::primitives::numeric::Numeric;
use crate::primitives::plane::RectArea;

pub fn filled_multiple_gpu<T: Numeric +  Copy + 'static >(
    buffer: &mut [u32],
    width: u32,
    height: u32,
    rects: &Vec<&RectArea<T>>,
    default_color: u32,
    gpu_context: &mut GpuContext,
) -> Result<(), String> {
    #[cfg(feature = "gpu")]
    {
        use ocl::{Buffer, Kernel};
        let kernel_src = include_str!("fill_rects.c");
        let kernel_name = "fill_rects";
        let program_name = "fill_rects_program";
        let buf_len = buffer.len();

        // Prepare rectangles as [x, y, w, h, color, ...]
        let mut flat_rects: Vec<u32> = Vec::with_capacity(rects.len() * 5);
        for rect in rects.iter() {

            flat_rects.push(rect.top_left.x.to_u32());
            flat_rects.push(rect.top_left.y.to_u32());
            flat_rects.push(rect.dimensions.w.to_u32());
            flat_rects.push(rect.dimensions.h.to_u32());
            flat_rects.push(rect.color.unwrap_or(default_color));
        }

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

        // --- Upload data ---
        let queue_value = queue.as_ref().clone();
        // Upload rectangles
        let rects_buf = Buffer::<u32>::builder()
            .queue(queue_value.clone())
            .len(flat_rects.len())
            .copy_host_slice(&flat_rects)
            .build()
            .map_err(|e| format!("Failed to create OpenCL rects buffer: {e}"))?;

        // Upload frame buffer (pooled)
        let frame_buf = gpu_context.get_or_create_buffer(buf_len, queue_ref, buffer)?;

        frame_buf.write(buffer.as_ref()).enq().map_err(|e| format!("Failed to upload frame buffer: {e}"))?;


        // --- Launch kernel ---
        let kernel = Kernel::builder()
            .program(&program)
            .name(kernel_name)
            .queue(queue_value)
            .global_work_size(buf_len)
            .arg(&frame_buf)
            .arg(width)
            .arg(height)
            .arg(&rects_buf)
            .arg(rects.len() as u32)
            .build()
            .map_err(|e| format!("Failed to build kernel: {e}"))?;

        unsafe {
            kernel.enq().map_err(|e| format!("Failed to enqueue kernel: {e}"))?;
        }

        // Download result to CPU buffer
        frame_buf.read(buffer).enq()
            .map_err(|e| format!("Failed to read GPU buffer back to host: {e}"))?;

        Ok(())
    }
    #[cfg(not(feature = "gpu"))]
    {
        Err("GPU support is not enabled at compile time.".to_string())
    }
}
