/// Unsafely fills in a buffer with a given color
pub fn fill(buf_view: &mut [u32], color:u32){

    unsafe {
        // Obtain a raw pointer
        let buffer_ptr = buf_view.as_mut_ptr();

        // Calculate the end pointer for our loop. This is safe because we are not dereferencing the pointer yet.
        let end_ptr = buffer_ptr.add(buf_view.len());

        // Initialize a mutable pointer to iterate through the buffer.
        let mut current_ptr = buffer_ptr;

        while current_ptr < end_ptr {
            // Directly write to the memory location pointed to by current_ptr.
            *current_ptr = color;

            // Advance the pointer.
            current_ptr = current_ptr.add(1);
        }
    }

}