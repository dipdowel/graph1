use crate::primitives::plane::Dimensions2d;
use crate::primitives::Pixel;
use std::collections::VecDeque;
use std::thread;
use std::time::Instant;

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
    let start = Instant::now();

    // We were instructed not to do anything
    if num_threads < 1 {
        #[cfg(debug_assertions)]
        {
            let duration = start.elapsed();
            println!(
                "[ threads: {} | fill::buffer() ] Execution time: {} ms, buffer len:{} ",
                num_threads,
                duration.as_millis(),
                buffer.len()
            );
        }
        return;
    }

    // If there are no threads to spawn, just run the logic in the main thread and return
    if num_threads == 1 {
        // No extra threads to spawn
        buffer_fill_thread(buffer, color);

        #[cfg(debug_assertions)]
        {
            let duration = start.elapsed();
            println!(
                "[ threads: {} | fill::buffer() ] Execution time: {} ms, buffer len:{} ",
                num_threads,
                duration.as_millis(),
                buffer.len()
            );
        }
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

    #[cfg(debug_assertions)]
    {
        let duration = start.elapsed();
        println!(
            "[ threads: {} | fill::buffer() ] Execution time: {} ms, buffer len:{} ",
            num_threads,
            duration.as_millis(),
            buffer.len()
        );
    }
}

/// Flood fills a shape with a color and starting at a position specified by `start`
pub fn flood(buf: &mut [u32], buf_dimensions: &Dimensions2d, start_pixel: &Pixel) {
    // Buffer dimensions
    let width = buf_dimensions.w;
    let height = buf_dimensions.h;

    // Get the color of the pixel where we start the flood fill
    let initial_color = buf[(start_pixel.y * width + start_pixel.x) as usize];

    // New color to be applied to all the pixels with `initial_color` inside the shape
    let fill_color = start_pixel.color;

    // Nothing to do if the initial and the fill colors match.
    if initial_color == fill_color {
        return;
    }

    // Queue to manage pixels to be filled
    let mut queue = VecDeque::new();
    // Add the starting pixel to the queue
    queue.push_back((start_pixel.x, start_pixel.y));

    // Process the queue until it's empty
    while let Some((x, y)) = queue.pop_front() {
        let index = (y * width + x) as usize;

        // skip attempts to fill outside the screen
        if x > width || y > height {
            continue;
        }

        // Check if the current pixel has the initial color
        if buf[index] == initial_color {
            // Change the color of the current pixel to the fill color
            buf[index] = fill_color;

            // Add neighboring pixels to the queue
            // Left neighbor
            if x > 0 {
                queue.push_back((x - 1, y));
            }
            // Right neighbor
            if x < width - 1 {
                queue.push_back((x + 1, y));
            }
            // Top neighbor
            if y > 0 {
                queue.push_back((x, y - 1));
            }
            // Bottom neighbor
            if y < height - 1 {
                queue.push_back((x, y + 1));
            }
        }
    }
}
