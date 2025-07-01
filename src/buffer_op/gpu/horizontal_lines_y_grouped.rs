use crate::core::context::gpu::GpuContext;
use crate::primitives::plane::Dimensions2d;
use crate::buffer_op::gpu::kernel_bundle::KernelBundle;

#[cfg(feature = "gpu")]
use ocl::{Kernel, Buffer};

// TODO: DELETE THIS FILE AND ITS OCL KERNEL!
// TODO: DELETE THIS FILE AND ITS OCL KERNEL!
// TODO: DELETE THIS FILE AND ITS OCL KERNEL!
// TODO: DELETE THIS FILE AND ITS OCL KERNEL!
// TODO: DELETE THIS FILE AND ITS OCL KERNEL!
// TODO: DELETE THIS FILE AND ITS OCL KERNEL!
// TODO: DELETE THIS FILE AND ITS OCL KERNEL!
// TODO: DELETE THIS FILE AND ITS OCL KERNEL!
// TODO: DELETE THIS FILE AND ITS OCL KERNEL!
// TODO: DELETE THIS FILE AND ITS OCL KERNEL!
// TODO: DELETE THIS FILE AND ITS OCL KERNEL!
// TODO: DELETE THIS FILE AND ITS OCL KERNEL!
// TODO: DELETE THIS FILE AND ITS OCL KERNEL!
// TODO: DELETE THIS FILE AND ITS OCL KERNEL!


/// Draw grouped horizontal lines by y (GPU accelerated).
/// This version uploads the frame buffer, runs the kernel, and downloads the result.
pub fn horizontal_lines_y_grouped(
    cpu_frame_buf: &mut [u32],
    buf_dimensions: &Dimensions2d,
    scanline_dict: &Vec<Vec<usize>>,
    scanline_data: &Vec<u32>,
    gpu_context: &mut GpuContext,
) -> Result<(), String> {
    #[cfg(feature = "gpu")]
    {
        let bundle = horizontal_lines_y_grouped_get_kernel(cpu_frame_buf, buf_dimensions, scanline_dict, scanline_data, gpu_context)?;
        let kernel = &bundle.kernel;
        let frame_buf = bundle.buffers.get(0).ok_or("No frame buffer in kernel bundle")?;

        // Upload current frame buffer before kernel (if necessary)
        frame_buf.write(cpu_frame_buf.as_ref())
            .enq().map_err(|e| format!("Failed to upload frame buffer: {e}"))?;

        unsafe { kernel.enq().map_err(|e| format!("Failed to enqueue kernel: {e}"))?; }
        // Download updated buffer
        frame_buf.read(cpu_frame_buf)
            .enq().map_err(|e| format!("Failed to read GPU buffer back to host: {e}"))?;
        Ok(())
    }
    #[cfg(not(feature = "gpu"))]
    {
        let _ = cpu_frame_buf;
        let _ = buf_dimensions;
        let _ = scanline_dict;
        let _ = scanline_data;
        let _ = gpu_context;
        Err("GPU support is not enabled at compile time.".to_string())
    }
}

/// Build and return the ready-to-enqueue kernel bundle for horizontal_lines_y_grouped.
/// This is used for pipelined/multi-effect GPU workflows.
pub fn horizontal_lines_y_grouped_get_kernel(
    cpu_frame_buf: &mut [u32],
    buf_dimensions: &Dimensions2d,
    scanline_dict: &Vec<Vec<usize>>,
    scanline_data: &Vec<u32>,
    gpu_context: &mut GpuContext,
) -> Result<KernelBundle, String> {
    #[cfg(feature = "gpu")]
    {
        let kernel_src = include_str!("horizontal_lines_y_grouped.c");
        let kernel_name = "horizontal_lines_y_grouped";
        let program_name = "horizontal_lines_y_grouped_program";

        let height = buf_dimensions.h as usize;
        let width = buf_dimensions.w as usize;
        let buf_len = cpu_frame_buf.len();

        // ---- Flatten scanline_dict to lookup + pointers arrays ----
        let mut lookup_table: Vec<u32> = Vec::with_capacity(height + 1);
        let mut scanline_pointers: Vec<u32> = Vec::new();

        lookup_table.push(0); // First y always starts at offset 0

        for y in 0..height {
            let temp = vec![0];

            // Defensive: If scanline_dict has fewer entries, treat as no scanlines for that y
            let y_record = scanline_dict.get(y).unwrap_or(&temp);
            let num = *y_record.get(0).unwrap_or(&0) as usize;

            // Defensive: Don't go out of bounds on pointers
            let end_idx = 1 + num;
            let pointers = if end_idx <= y_record.len() {
                &y_record[1..end_idx]
            } else {
                &[]
            };

            // Extend flat pointers, always as u32
            scanline_pointers.extend(pointers.iter().map(|&x| x as u32));
            lookup_table.push(scanline_pointers.len() as u32); // Where next y starts
        }

        // ---- Defensive: If no scanlines, provide minimal buffers ----
        let scanline_data_slice = if scanline_data.is_empty() { &[0u32] } else { scanline_data.as_slice() };
        let scanline_pointers_slice = if scanline_pointers.is_empty() { &[0u32] } else { scanline_pointers.as_slice() };
        let lookup_table_slice = if lookup_table.is_empty() { &[0u32, 0u32] } else { lookup_table.as_slice() };

        // ---- Build OpenCL program and queue ----
        if gpu_context.get_program(program_name).is_none() {
            gpu_context
                .load_program(&kernel_src, program_name)
                .map_err(|e| format!("Failed to build OpenCL program: {e}"))?;
        }
        let program = gpu_context.get_program(program_name).ok_or("Program not loaded (unknown error)")?;
        let queue = gpu_context.queue.as_ref().ok_or("No OpenCL queue in context")?.clone();
        let queue_value = queue.as_ref().clone();
        let queue_ref = queue.as_ref();

        // ---- Create device buffers ----
        let scanline_data_buf = Buffer::<u32>::builder()
            .queue(queue_value.clone())
            .len(scanline_data_slice.len())
            .copy_host_slice(scanline_data_slice)
            .build()
            .map_err(|e| format!("Failed to create OpenCL scanline_data buffer: {e}"))?;

        let scanline_pointers_buf = Buffer::<u32>::builder()
            .queue(queue_value.clone())
            .len(scanline_pointers_slice.len())
            .copy_host_slice(scanline_pointers_slice)
            .build()
            .map_err(|e| format!("Failed to create OpenCL scanline_pointers buffer: {e}"))?;

        let lookup_table_buf = Buffer::<u32>::builder()
            .queue(queue_value.clone())
            .len(lookup_table_slice.len())
            .copy_host_slice(lookup_table_slice)
            .build()
            .map_err(|e| format!("Failed to create OpenCL lookup_table buffer: {e}"))?;

        let frame_buf = gpu_context.get_or_create_buffer(buf_len, queue_ref, cpu_frame_buf)?;
        let global_work_size = [buf_dimensions.w as usize, buf_dimensions.h as usize]; // 2D grid

        // ---- Build the kernel ----
        let kernel = Kernel::builder()
            .program(&program)
            .name(kernel_name)
            .queue(queue_value)
            .global_work_size(global_work_size)
            .arg(&frame_buf)
            .arg(width as u32)
            .arg(height as u32)
            .arg(&scanline_data_buf)
            .arg(&scanline_pointers_buf)
            .arg(&lookup_table_buf)
            .build()
            .map_err(|e| format!("Failed to build kernel: {e}"))?;

        Ok(KernelBundle::new(
            kernel,
            vec![
                frame_buf,
                scanline_data_buf,
                scanline_pointers_buf,
                lookup_table_buf,
            ],
        ))
    }
    #[cfg(not(feature = "gpu"))]
    {
        let _ = cpu_frame_buf;
        let _ = buf_dimensions;
        let _ = scanline_dict;
        let _ = scanline_data;
        let _ = gpu_context;
        Err("GPU support is not enabled at compile time.".to_string())
    }
}
