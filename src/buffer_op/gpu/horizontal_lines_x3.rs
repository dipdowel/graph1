use crate::core::context::gpu::GpuContext;
use crate::primitives::plane::Dimensions2d;

#[cfg(feature = "gpu")]
use ocl::{Kernel, Buffer};
use crate::buffer_op::gpu::kernel_bundle::KernelBundle;

/// Draw batches of colored horizontal lines on the GPU, then download result to host buffer.
pub fn horizontal_lines_x3_gpu(
    buf: &mut [u32],
    buf_dimensions: &Dimensions2d,
    lines: &Vec<Vec<u32>>,
    gpu_context: &mut GpuContext,
) -> Result<(), String> {
    #[cfg(feature = "gpu")]
    {

        let bundle = horizontal_lines_x3_get_kernel(
            buf, buf_dimensions, lines, gpu_context,
        )?;
        let kernel = bundle.kernel;
        let frame_buf = bundle.buffers.get(0)
            .ok_or("No frame buffer in kernel bundle")?;
        let lines_buf = bundle.buffers.get(1)
            .ok_or("No lines buffer in kernel bundle")?;


        unsafe { kernel.enq().map_err(|e| format!("Failed to enqueue kernel: {e}"))?; }
        frame_buf.read(buf).enq().map_err(|e| format!("Failed to read GPU buffer back to host: {e}"))?;
        Ok(())
    }
    #[cfg(not(feature = "gpu"))]
    {
        let _ = buf;
        let _ = buf_dimensions;
        let _ = lines;
        let _ = gpu_context;
        Err("GPU support is not enabled at compile time.".to_string())
    }
}

/// Build and return the ready-to-enqueue OpenCL kernel for batch horizontal lines.
/// Use with the kernel executor for pipelined/multi-kernel workflows.
/// Returns the kernel and the relevant buffer objects (to keep them alive).
pub fn horizontal_lines_x3_get_kernel(
    buf: &mut [u32],
    buf_dimensions: &Dimensions2d,
    lines: &Vec<Vec<u32>>,
    gpu_context: &mut GpuContext,
) -> Result<KernelBundle, String> {
    #[cfg(feature = "gpu")]
    {

        // println!("horizontal_lines_x3_get_kernel.....................!!!!");


        let kernel_src = include_str!("horizontal_lines_x3.c");
        let kernel_name = "horizontal_lines_x3";
        let program_name = "horizontal_lines_x3_program";

        // Flatten lines: [color, x_start, x_end, y, ...]
        let mut flat_lines: Vec<u32> = Vec::new();
        for color_batch in lines.iter() {
            if (color_batch.len() - 1) % 3 != 0 { continue; }
            let color = color_batch[0];
            for seg in color_batch[1..].chunks(3) {
                let x_start = seg[0];
                let x_end   = seg[1];
                let y       = seg[2];
                flat_lines.push(color);
                flat_lines.push(x_start);
                flat_lines.push(x_end);
                flat_lines.push(y);
            }
        }
        let num_lines = flat_lines.len() / 4;
        if num_lines == 0 { return Err("No lines to process.".into()); }
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
            .len(flat_lines.len())
            .copy_host_slice(&flat_lines)
            .build()
            .map_err(|e| format!("Failed to create OpenCL lines buffer: {e}"))?;

        // Upload frame buffer (pooled)
        let frame_buf = gpu_context.get_or_create_buffer(buf_len, queue_ref, buf)?;

        // (Optional) upload current frame buffer to device
        // frame_buf.write(buf.as_ref()).enq().map_err(|e| format!("Failed to upload frame buffer: {e}"))?;

        // Build (but do not enqueue) the kernel
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

        Ok(KernelBundle {
            kernel,
            buffers: vec![frame_buf, lines_buf],
        })
    }
    #[cfg(not(feature = "gpu"))]
    {
        let _ = buf;
        let _ = buf_dimensions;
        let _lines = lines;
        let _ = gpu_context;
        Err("GPU support is not enabled at compile time.".to_string())
    }
}
