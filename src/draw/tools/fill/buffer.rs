use std::thread;

/// Buffer fill logic to be executed by each thread in the multithreaded buffer operation
/// Unsafely fills a buffer with a given color
///
/// # Arguments
/// * `buffer` - A buffer (or a chunk of a buffer) to fill with a color.
/// * `color` - The color to fill the buffer with.
fn buffer_fill_thread(buffer: &mut [u32], color: u32) {
    // println!("[threads: {}] thread: {}, chunk {:?}/{:?}", total_chunks,chunk_index, chunk_index+1, total_chunks);
    // let thread_id = thread::current().id();
    // println!("[thread: {:?}] ", thread_id);

    unsafe {
        // Obtain a raw pointer
        let buffer_ptr = buffer.as_mut_ptr();

        // Calculate the end pointer for our loop. This is safe because we are not dereferencing the pointer yet.
        let end_ptr = buffer_ptr.add(buffer.len());

        // Initialize a mutable pointer to iterate through the buffer chunk.
        let mut current_ptr = buffer_ptr;

        while current_ptr < end_ptr {
            // Directly write to the memory location pointed to by current_ptr.
            *current_ptr = color;

            // Advance the pointer.
            current_ptr = current_ptr.add(1);
        }
    }
}

/// * A low-level buffer fill tool.
/// * Unsafely fills a buffer with a given color
/// * Multithreaded.
/// * @See `Multithreaded operations` in `README.md` for details on how `num_threads` is interpreted.
///
/// # Arguments
/// * `buffer` - A mutable buffer to fill
/// * `color` - The color to fill the buffer with
/// * `num_threads` - The number of threads to spawn.
pub fn buffer(buffer: &mut [u32], color: u32, num_threads: usize) {
    // let start = Instant::now();

    // We were instructed not to do anything
    if num_threads < 1 {
        // #[cfg(debug_assertions)]
        // {
        //     let duration = start.elapsed();
        //     println!(
        //         "[ threads: {} | fill::buffer() ] Execution time: {} ms, buffer len:{} ",
        //         num_threads,
        //         duration.as_millis(),
        //         buffer.len()
        //     );
        // }
        return;
    }

    // If there are no threads to spawn, just run the logic in the main thread and return
    if num_threads == 1 {
        // No extra threads to spawn
        buffer_fill_thread(buffer, color);

        // #[cfg(debug_assertions)]
        // {
        //     let duration = start.elapsed();
        //     println!(
        //         "[ threads: {} | fill::buffer() ] Execution time: {} ms, buffer len:{} ",
        //         num_threads,
        //         duration.as_millis(),
        //         buffer.len()
        //     );
        // }
        return;
    }

    // ==[ MULTIPLE THREADS ]=======================================================================

    // let total_num_threads: usize = num_threads ;
    let chunk_size = usize::div_ceil(buffer.len(), num_threads);

    // Split the buffer into mutable chunks to process in parallel threads
    let mut chunks: Vec<&mut [u32]> = buffer.chunks_mut(chunk_size).collect();

    thread::scope(|s| {
        // Iterate over the chunks and process each in its own thread
        for chunk in &mut chunks.iter_mut() {
            s.spawn(move || buffer_fill_thread(chunk, color));
        }
    }); // The scope for the scoped threads ends here. All the threads are expected to be joined automagically at this point.

    // #[cfg(debug_assertions)]
    // {
    //     let duration = start.elapsed();
    //     println!(
    //         "[ threads: {} | fill::buffer() ] Execution time: {} ms, buffer len:{} ",
    //         num_threads,
    //         duration.as_millis(),
    //         buffer.len()
    //     );
    // }
}
