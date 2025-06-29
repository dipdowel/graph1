use crate::buffer_op;
use crate::primitives::plane::Dimensions2d;
use std::collections::BTreeMap;
use std::thread;


/// Draws horizontal lines on a buffer. **Multi-threaded!**
/// Ownership of scanline_data is passed in for efficient moves (not copies).
/// 
/// # Parameters
/// - `buf`: A mutable slice of `u32` representing the pixel buffer.
/// - `buf_dimensions`: Dimensions of the buffer.
/// - `scanline_data`: A vector of `u32` where the 0th element is the `y` coordinate,
///   the 1st element is the color in RGBA, and the rest are pairs of `x_start` and `x_end` values
///   of line segments on that scanline.
/// - `scanline_pointers`: A vector  pointers into `scanline_data` that mark the start and end
/// of each scanline.
pub fn horizontal_lines_y_grouped_threaded(
    buf: &mut [u32],
    buf_dimensions: &Dimensions2d,
    scanline_data: Vec<u32>,   // TAKE OWNERSHIP
    scanline_pointers: Vec<usize>, // TAKE OWNERSHIP
    num_threads: usize
) {
    if scanline_data.is_empty() || scanline_pointers.len() < 2 || num_threads == 0 {
        return;
    }

    // If only one thread is available, just use the single-threaded version
    if num_threads == 1 {

        buffer_op::lines::horizontal_batch::horizontal_lines_y_grouped(
            buf,
            buf_dimensions,
            &scanline_data,
            &scanline_pointers,
        );
        return;
    }


    let width = buf_dimensions.w;
    let height = buf_dimensions.h;
    let buf_ptr = buf.as_mut_ptr();

    // 1. Group scanlines by y, preserving order, moving data (NO COPY)
    // BTreeMap for sorted y order
    let mut y_groups: BTreeMap<u32, Vec<Vec<u32>>> = BTreeMap::new();
    for w in scanline_pointers.windows(2) {
        let scanline_start = w[0];
        let scanline_end = w[1];
        if scanline_end <= scanline_start { continue; }

        // Move out the slice (no copy)
        let scanline: Vec<u32> = scanline_data[scanline_start..scanline_end].to_vec();
        if scanline.len() < 4 { continue; }
        let y = scanline[0];
        if y >= height { continue; }
        y_groups.entry(y).or_default().push(scanline);
    }

    // 2. Distribute y-groups to threads in balanced bands
    let mut bands: Vec<Vec<(u32, Vec<Vec<u32>>)>> = vec![Vec::new(); num_threads];
    let mut band_load: Vec<usize> = vec![0; num_threads];
    for (y, group) in y_groups {
        // Greedily assign this y-group to the least loaded thread
        let mut min_band = 0;
        for i in 1..num_threads {
            if band_load[i] < band_load[min_band] {
                min_band = i;
            }
        }
        band_load[min_band] += group.len();
        bands[min_band].push((y, group));
    }

    // 3. Each thread: flatten its scanlines into a Vec<u32> + Vec<usize> pointers
    thread::scope(|scope| {
        for band in bands.into_iter() {
            if band.is_empty() { continue; }
            // Calculate band min/max y for efficient buffer slicing
            let min_y = band.first().unwrap().0;
            let max_y = band.last().unwrap().0;
            let row_offset = (min_y * width) as usize;
            let n_rows = (max_y - min_y + 1) as usize;
            let band_buf = unsafe {
                std::slice::from_raw_parts_mut(buf_ptr.add(row_offset), n_rows * width as usize)
            };

            // Flatten all scanlines for this band into contiguous vector
            let mut band_scanline_data = Vec::new();
            let mut band_pointers = Vec::new();
            for (_y, scanlines) in &band {
                for scanline in scanlines {
                    band_pointers.push(band_scanline_data.len());
                    band_scanline_data.extend_from_slice(scanline);
                }
            }
            band_pointers.push(band_scanline_data.len());

            let band_width = width;
            let band_min_y = min_y;
            let band_buf_dimensions = *buf_dimensions;
            scope.spawn(move || {
                for window in band_pointers.windows(2) {
                    let scanline_start = window[0];
                    let scanline_end = window[1];
                    let scanline = &band_scanline_data[scanline_start..scanline_end];
                    let y = scanline[0];
                    let color = scanline[1];
                    if y >= band_buf_dimensions.h || scanline.len() < 4 { continue; }
                    let row_in_band = (y - band_min_y) as usize;
                    let row_buf = &mut band_buf[row_in_band * band_width as usize .. (row_in_band + 1) * band_width as usize];
                    for x_pair in scanline[2..].chunks(2) {
                        if x_pair.len() != 2 { continue; }
                        let x_start = x_pair[0].clamp(0, band_width - 1);
                        let x_end = x_pair[1].clamp(0, band_width - 1);
                        if x_start > x_end { continue; }
                        for x in x_start..=x_end {
                            row_buf[x as usize] = color;
                        }
                    }
                }
            });
        }
    });
}
