use std::fs;

use crate::core::context::gpu::GpuContext;
use crate::primitives::plane::{Dimensions2d, RectArea};
use ocl::{Buffer, Kernel};

/// Draws a filled rectangle in a linear RGBA buffer using OpenCL.
///
/// # Parameters:
/// - `buffer`: Mutable reference to a flat u32 RGBA buffer.
/// - `buffer_size`: Dimensions of the buffer (in pixels).
/// - `rect`: Rectangle area (position, dimensions, color).
/// - `gpu_context`: OpenCL GPU context.
pub fn draw_rectangle_gpu(buffer: &mut [u32], buffer_size: &Dimensions2d<u32>, rect: &RectArea<u32>, gpu_context: &mut GpuContext) {


    // println!("!!!! Drawing rectangle on GPU using OpenCL !!!!");

    let kernel_src = include_str!("rectangle.cl");
    let kernel_name = "draw_rectangle";
    let program_name = "rectangle_program";

    if gpu_context.get_program(program_name).is_none() {
        gpu_context
            .load_program(&kernel_src, program_name)
            .expect("Failed to build OpenCL program");
    }

    let program = gpu_context.get_program(program_name).unwrap();
    let queue = gpu_context.queue.as_ref().unwrap().as_ref();

    let buf_len = buffer_size.w * buffer_size.h;
    let frame_buf = unsafe {
        Buffer::<u32>::builder()
            .queue(queue.clone())
            .len(buf_len)
            .use_host_slice(buffer)
            .build()
            .unwrap()
    };

    let color = rect.color.unwrap_or(0);

    let kernel = Kernel::builder()
        .program(&program)
        .name(kernel_name)
        .queue(queue.clone())
        .global_work_size([rect.dimensions.w, rect.dimensions.h])
        .arg(&frame_buf)
        .arg(&buffer_size.w)
        .arg(&buffer_size.h)
        .arg(&rect.top_left.x)
        .arg(&rect.top_left.y)
        .arg(&rect.dimensions.w)
        .arg(&rect.dimensions.h)
        .arg(&color)
        .build()
        .unwrap();

    unsafe {
        kernel.enq().unwrap();
    }

}
