use std::thread;
use crate::buffer_op::gpu;
use crate::core::context::GraphContext;
use crate::utils::color::math::{rgba_operation, ColorOperation};


fn buffer_scanline_fx_thread(buffer: &mut [u32], intensity: u32, line_flipper: u32, pixel_count_offset: u32) {
    // println!("[threads: {}] thread: {}, chunk {:?}/{:?}", total_chunks,chunk_index, chunk_index+1, total_chunks);
    // let thread_id = thread::current().id();
    // println!("[thread: {:?}] ", thread_id);

    let mut pixel_count = pixel_count_offset;

    unsafe {
        // Obtain a raw pointer
        let buffer_ptr: *mut u32 = buffer.as_mut_ptr();

        // Calculate the end pointer for our loop. This is safe because we are not dereferencing the pointer yet.
        let end_ptr: *mut u32 = buffer_ptr.add(buffer.len());

        // Initialize a mutable pointer to iterate through the buffer.
        let mut current_ptr: *mut u32 = buffer_ptr;

        while current_ptr < end_ptr {
            // Advance the pointer.
            current_ptr = current_ptr.add(1);

            pixel_count += 1;

            if (pixel_count / line_flipper) % 2 == 0 {
                // Directly write to the memory location pointed to by current_ptr.
                *current_ptr =
                    rgba_operation(*current_ptr, intensity, ColorOperation::Subtract, false);
            }
        }
    }
}

/// Applies a scanline effect to the window (`ctx.frame_buf` & `ctx.win`).
/// # Arguments
/// * `ctx` - The graph context
/// * `size` - The size of the scanline effect. Must be greater than 0.
/// * `intensity` - The intensity of the scanline effect. Must be between 0 and 255.
pub fn window<UserData>(ctx: &mut GraphContext<UserData>, size: u8, intensity: u8) {

    if ctx.gpu_context.is_enabled() {
        gpu::scanline::scanline_fx(
            &mut ctx.frame_buf,
            ctx.win.w,
            size,
            intensity,
            &mut ctx.gpu_context,
        ).expect("GPU scanline failed");
        return;
    }



    if ctx.num_threads < 1 {
        return;
    }

    // don't let the size be zero
    let size: u32 = if size == 0 { 1 } else {size as u32};
    let intensity: u32 =  intensity as u32;
    let intensity: u32 = intensity << 24 | intensity << 16 | intensity << 8 | 0xff;
    let line_flipper = ctx.win.w * size;

    if ctx.num_threads == 1 {
        // No extra threads to spawn
        buffer_scanline_fx_thread(&mut ctx.frame_buf, intensity, line_flipper, 0);
        return;
    }

    // ==[ MULTIPLE THREADS ]=======================================================================
    // let total_num_threads: usize = num_threads ;
    let chunk_size = usize::div_ceil(ctx.frame_buf.len(), ctx.num_threads);

    // Split the buffer into mutable chunks to process in parallel threads
    let mut chunks: Vec<&mut [u32]> = ctx.frame_buf.chunks_mut(chunk_size).collect();

    thread::scope(|s| {
        // Iterate over the chunks and process each in its own thread
        // for chunk in &mut chunks.iter_mut() {
        for (chunk_index, chunk) in &mut chunks.iter_mut().enumerate() {

            // How many pixels were processed in the previous threads. Helps the currently created thread to know where to start.
            let pixel_count_offset = (chunk_index * chunk.len()) as u32;
            s.spawn(move || buffer_scanline_fx_thread(chunk, intensity, line_flipper, pixel_count_offset));
        }
    }); // The scope for the scoped threads ends here. All the threads are expected to be joined automagically at this point.


}
