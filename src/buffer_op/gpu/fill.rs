use std::fs;

use crate::core::context::gpu::GpuContext;

pub fn fill(buffer: &mut [u32], buf_len:usize,  color: u32, gpu_context: &mut GpuContext) {
    #[cfg(feature = "gpu")]{
    use ocl::{Buffer, Kernel};

    println!("!!!! Filling a buffer with color on GPU using OpenCL !!!!");
    let mut buffer:Vec<u32>  = Vec::from(buffer);

    let kernel_src = include_str!("fill_buffer.cl");


    let kernel_name = "fill_buffer";
    let program_name = "fill_buffer_program";

    // Build/load the program if not already cached --
        if gpu_context.get_program(program_name).is_none() {
            gpu_context
                .load_program(&kernel_src, program_name)
                .expect("Failed to build OpenCL program");
        }


    let program = gpu_context.get_program(program_name).unwrap();

    // Use the OpenCL queue as a reference from Arc<Queue>
    let queue = gpu_context.queue.as_ref().unwrap().as_ref();



    let  frame_buf = unsafe {Buffer::<u32>::builder().queue(queue.clone())
        .flags(ocl::flags::MEM_READ_WRITE)
        .len(buf_len)
        .use_host_slice(&mut buffer) // <- this enables zero-copy
        .build()
        .expect("Failed to create OpenCL buffer over framebuffer")
    };

    // Launch the kernel
    let kernel = Kernel::builder()
        .program(&program)
        .name(kernel_name)
        .queue(queue.clone())
        .global_work_size(buf_len)
        .arg(&frame_buf)
        .arg(color)
        .arg(buf_len as u32)
        .build()
        .expect("Failed to build kernel");

    unsafe { kernel.enq().expect("Failed to enqueue kernel"); }

    // Ensure the host buffer is synchronized (may be a no-op on iGPU, but good for safety)
    frame_buf.read(&mut buffer).enq().ok();
    }
}
