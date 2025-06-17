use crate::core::context::GraphContext;

use ocl::{Buffer, Kernel};

pub unsafe fn clear_screen<UserData>(ctx: &mut GraphContext<UserData>) {
    if ctx.gpu_context.enabled {
        let kernel_name = "fill_u32";
        let program_name = "fill_u32_program";
        let src = r#"
            __kernel void fill_u32(__global uint *buffer, uint color, uint len) {
                uint idx = get_global_id(0);
                if (idx < len) buffer[idx] = color;
            }
        "#;

        // Build/load the program if not already cached
        if ctx.gpu_context.get_program(program_name).is_none() {
            ctx.gpu_context
                .load_program(src, program_name)
                .expect("Failed to build OpenCL program");
        }
        let program = ctx.gpu_context.get_program(program_name).unwrap();

        // Use the OpenCL queue as a reference from Arc<Queue>
        let queue = ctx.gpu_context.queue.as_ref().unwrap().as_ref();

        // Create a zero-copy buffer over the framebuffer
        let len = ctx.frame_buf.len();
        let  frame_buf = Buffer::<u32>::builder()
            .queue(queue.clone())
            .flags(ocl::flags::MEM_WRITE_ONLY)
            .len(len)
            .use_host_slice(&mut ctx.frame_buf) // <- this enables zero-copy
            .build()
            .expect("Failed to create OpenCL buffer over framebuffer");

        // Launch the kernel
        let kernel = Kernel::builder()
            .program(&program)
            .name(kernel_name)
            .queue(queue.clone())
            .global_work_size(len)
            .arg(&frame_buf)
            .arg(ctx.win.background_color)
            .arg(len as u32)
            .build()
            .expect("Failed to build kernel");

        unsafe { kernel.enq().expect("Failed to enqueue kernel"); }

        // Ensure the host buffer is synchronized (may be a no-op on iGPU, but good for safety)
        frame_buf.read(&mut ctx.frame_buf).enq().ok();
    } else {
        crate::draw::tools::fill::buffer(&mut ctx.frame_buf, ctx.win.background_color, ctx.num_threads);
    }
}



// /// Fills the frame buffer with the background color of the window (`ctx.win.background_color`)
// pub fn clear_screen<UserData>(ctx: &mut GraphContext<UserData>) {
// 
//     if ctx.gpu_context.enabled {
//         // TODO: Implement the GPU/OpenCL version for setting the frame buffer `ctx.frame_buf` to the background color `ctx.win.background_color`
//     } else {
//         // Software rendering: fill the frame buffer with the background color
//         crate::draw::tools::fill::buffer(&mut ctx.frame_buf, ctx.win.background_color, ctx.num_threads);
//     }
// }
