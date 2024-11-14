use crate::core::context::GraphContext;
use crate::utils::color::math::{rgba_operation, ColorOperation};

/// Applies a scanline effect to the frame buffer (ctx.frame_buf)
/// # Arguments
/// * `ctx` - The graph context
/// * `size` - The size of the scanline effect. Must be greater than 0.
/// * `intensity` - The intensity of the scanline effect. Must be between 0 and 255.
pub fn buffer<UserData>(ctx: &mut GraphContext<UserData>, size: u8, intensity: u8) {

    // don't let the size be zero
    let size: u32 = if size == 0 { 1 } else {size as u32};

    let intensity: u32 =  intensity as u32;

    let intensity: u32 = intensity << 24 | intensity << 16 | intensity << 8 | 0xff;

    let line_flipper = ctx.win.w * size;

    let mut pixel_count = 0;

    unsafe {
        // Obtain a raw pointer
        let buffer_ptr: *mut u32 = ctx.frame_buf.as_mut_ptr();

        // Calculate the end pointer for our loop. This is safe because we are not dereferencing the pointer yet.
        let end_ptr: *mut u32 = buffer_ptr.add(ctx.frame_buf.len());

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
