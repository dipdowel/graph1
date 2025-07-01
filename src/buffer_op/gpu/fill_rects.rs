use crate::core::context::gpu::GpuContext;
use crate::primitives::numeric::Numeric;
use crate::primitives::plane::{Dimensions2d, RectArea};

#[cfg(feature = "gpu")]
use ocl::{Kernel, Buffer};
use crate::buffer_op::gpu::kernel_bundle::KernelBundle;

/// Fills multiple rectangles using the GPU. Will download result into the buffer.
pub fn filled_multiple_gpu<T: Numeric + Copy + 'static>(
    cpu_frame_buf: &mut [u32],
    buf_dimensions: &Dimensions2d<u32>,
    rects: &Vec<&RectArea<T>>,
    default_color: u32,
    gpu_context: &mut GpuContext,
) -> Result<(), String> {
    #[cfg(feature = "gpu")]
    {
        let bundle = fill_rects_get_kernel(
            cpu_frame_buf, buf_dimensions, rects, default_color, gpu_context,
        )?;
        let kernel = bundle.kernel;
        let gpu_frame_buf = bundle.buffers.get(0)
            .ok_or("No frame buffer in kernel bundle")?;


        // Upload current ctx.frame_buf to the GPU
        gpu_frame_buf.write(cpu_frame_buf.as_ref()).enq().map_err(|e| format!("Failed to upload frame buffer: {e}"))?;

        unsafe { kernel.enq().map_err(|e| format!("Failed to enqueue kernel: {e}"))?; }
        gpu_frame_buf.read(cpu_frame_buf).enq().map_err(|e| format!("Failed to read GPU buffer: {e}"))?;
        Ok(())
    }
    #[cfg(not(feature = "gpu"))]
    {
        let _ = cpu_frame_buf;
        let _ = buf_dimensions;
        let _ = rects;
        let _ = default_color;
        let _ = gpu_context;
        Err("GPU support is not enabled at compile time.".to_string())
    }
}

/// Build and return a ready-to-enqueue OpenCL kernel for filling multiple rectangles.
/// Returns the kernel and all relevant buffer objects (to keep them alive for the kernel's duration).
/// Use this when composing pipelines with the kernel executor.
///
/// # Arguments
/// * `buffer` - The host buffer to fill with rectangle colors.
/// * `buf_dimensions` - Dimensions of the buffer.
/// * `rects` - Vector of rectangle areas to fill.
/// * `default_color` - Color to use for rectangles that do not have a specific color set.
/// * `gpu_context` - The GPU context containing OpenCL resources.
/// * `upload_fb2gpu` - Whether to upload the current Graph1  frame buffer to the GPU before running the kernel.
///
pub fn fill_rects_get_kernel<T: Numeric + Copy + 'static>(
    cpu_frame_buf: &mut [u32],
    buf_dimensions: &Dimensions2d<u32>,
    rects: &Vec<&RectArea<T>>,
    default_color: u32,
    gpu_context: &mut GpuContext,
    // upload_fb2gpu: bool,
) -> Result<KernelBundle, String> {
    #[cfg(feature = "gpu")]
    {
        // println!("fill_rects_get_kernel!!!!");

        let kernel_src = include_str!("fill_rects.c");
        let kernel_name = "fill_rects";
        let program_name = "fill_rects_program";
        let buf_len = cpu_frame_buf.len();

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
        let queue_value = queue.as_ref().clone();

        // Upload rectangles
        let rects_buf = Buffer::<u32>::builder()
            .queue(queue_value.clone())
            .len(flat_rects.len())
            .copy_host_slice(&flat_rects)
            .build()
            .map_err(|e| format!("Failed to create OpenCL rects buffer: {e}"))?;

        // Upload frame buffer (pooled)
        let gpu_frame_buf = gpu_context.get_or_create_buffer(buf_len, queue_ref, cpu_frame_buf)?;



        // Build (but do not enqueue) the kernel
        let kernel = Kernel::builder()
            .program(&program)
            .name(kernel_name)
            .queue(queue_value)
            .global_work_size(buf_len)
            .arg(&gpu_frame_buf)
            .arg(buf_dimensions.w)
            .arg(buf_dimensions.h)
            .arg(&rects_buf)
            .arg(rects.len() as u32)
            .build()
            .map_err(|e| format!("Failed to build kernel: {e}"))?;

        // Ok((kernel, frame_buf, rects_buf))
        Ok(KernelBundle {
            kernel,
            buffers: vec![gpu_frame_buf, rects_buf],
        })
    }
    #[cfg(not(feature = "gpu"))]
    {
        let _ = cpu_frame_buf;
        let _ = buf_dimensions;
        let _ = rects;
        let _ = default_color;
        let _ = gpu_context;
        Err("GPU support is not enabled at compile time.".to_string())
    }
}
