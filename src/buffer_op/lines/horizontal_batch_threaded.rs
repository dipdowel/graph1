use crate::primitives::plane::Dimensions2d;
use std::thread;

/// Struct describing a single horizontal line to be drawn.
#[derive(Debug, Clone, Copy)]
struct LineMeta {
    x_start: u32,
    x_end: u32,
    y: u32,
    color: u32,
}

/// Struct representing a run of lines with the same y value.
#[derive(Debug, Clone, Copy)]
struct YGroup {
    y: u32,
    start: usize,
    len: usize,
}


/// Parses the input for the horizontal_lines_x3 format:
/// Each entry in `lines` is a vector where the first element is the color,
/// followed by triplets (x_start, x_end, y).
/// Returns Vec<LineMeta>, only keeping valid and clamped lines.
#[inline(always)]
fn parse_lines_x3(lines: &Vec<Vec<u32>>, width: u32, height: u32) -> Vec<LineMeta> {
    let mut result = Vec::new();
    if width == 0 || height == 0 {
        return result;
    }
    let max_x = width - 1;
    for batch in lines {
        if batch.len() < 4 || (batch.len() - 1) % 3 != 0 {
            continue;
        }
        let color = batch[0];  // Pick the color for this batch ONLY here
        for triplet in batch[1..].chunks(3) {
            let (raw_x_start, raw_x_end, y) = (triplet[0], triplet[1], triplet[2]);
            if y >= height {
                continue;
            }
            let x_start = raw_x_start.clamp(0, max_x);
            let x_end = raw_x_end.clamp(0, max_x);
            if x_start > x_end {
                continue;
            }
            result.push(LineMeta {
                x_start,
                x_end,
                y,
                color, // always use the batch's color
            });
        }
    }
    result
}




/// Parses a flat vector of u32 line data into structured LineMeta.
/// Returns a Vec<LineMeta>.
#[inline(always)]
fn parse_lines_x4(lines: &Vec<u32>, width: u32, height: u32) -> Vec<LineMeta> {
    let mut result = Vec::with_capacity(lines.len() / 4);
    if width == 0 || height == 0 {
        return result;
    }
    let max_x = width - 1;
    for chunk in lines.chunks(4) {
        let (raw_x_start, raw_x_end, y, color) = (chunk[0], chunk[1], chunk[2], chunk[3]);
        // Filter out vertical out-of-bounds
        if y >= height {
            continue;
        }
        // Clamp horizontal
        let x_start = raw_x_start.clamp(0, max_x);
        let x_end = raw_x_end.clamp(0, max_x);
        // Filter out lines that don't draw anything
        if x_start > x_end {
            continue;
        }
        result.push(LineMeta {
            x_start,
            x_end,
            y,
            color,
        });
    }
    result
}



/// Groups sorted lines into runs of identical y, returning Vec<YGroup>.
#[inline(always)]
fn group_by_y(parsed_lines: &[LineMeta]) -> Vec<YGroup> {
    let mut y_groups = Vec::new();
    let mut i = 0;
    while i < parsed_lines.len() {
        let y = parsed_lines[i].y;
        let start = i;
        let mut len = 1;
        while i + len < parsed_lines.len() && parsed_lines[i + len].y == y {
            len += 1;
        }
        y_groups.push(YGroup { y, start, len });
        i += len;
    }
    y_groups
}

/// Partitions y_groups into bands for balanced threading.
/// Returns Vec<Vec<usize>> where each Vec<usize> is indices into y_groups for a band.
#[inline(always)]
fn partition_bands(y_groups: &[YGroup], num_threads: usize, total_lines: usize) -> Vec<Vec<usize>> {
    let min_lines_per_band = total_lines / num_threads;
    let mut bands = Vec::with_capacity(num_threads);
    let mut current_band = Vec::new();
    let mut group_idx = 0;
    for t in 0..num_threads {
        let mut band_count = 0;
        while group_idx < y_groups.len()
            && (band_count < min_lines_per_band || t == num_threads - 1 && group_idx < y_groups.len() - (num_threads - t - 1))
        {
            band_count += y_groups[group_idx].len;
            current_band.push(group_idx);
            group_idx += 1;
        }
        bands.push(std::mem::take(&mut current_band));
    }
    bands
}

/// Returns Some((y_start, y_end)) for a band, or None if empty.
#[inline(always)]
fn band_y_range(band: &[usize], y_groups: &[YGroup]) -> Option<(u32, u32)> {
    if band.is_empty() {
        None
    } else {
        let first = &y_groups[*band.first().unwrap()];
        let last = &y_groups[*band.last().unwrap()];
        Some((first.y, last.y))
    }
}

/// Flattens a band's y_group indices into a Vec<&LineMeta> covering all lines for that band.
#[inline(always)]
fn collect_band_lines<'a>(band: &[usize], y_groups: &'a [YGroup], parsed_lines: &'a [LineMeta]) -> Vec<&'a LineMeta> {
    band.iter()
        .flat_map(|&gidx| {
            let g = &y_groups[gidx];
            parsed_lines[g.start .. g.start + g.len].iter()
        })
        .collect()
}

/// Draws all lines in a band into the given mutable buffer slice.
/// Each line.y must be within the range [y_start, y_end], and the slice must cover all those rows.
#[inline(always)]
fn draw_band_lines(band_lines: &[&LineMeta], band_slice: &mut [u32], y_start: u32, width: u32, _height: u32) {
    // band_lines are guaranteed to be valid and in-bounds from parse_lines()
    for line in band_lines {
        let row_offset = ((line.y - y_start) * width) as usize;
        for x in line.x_start..=line.x_end {
            band_slice[row_offset + x as usize] = line.color;
        }
    }
}


/// Draws horizontal lines on a buffer using multiple threads, with load balancing by line count.
/// Each line is represented by four consecutive elements: `x_start`, `x_end`, `y`, and `color`.
///  **NB:** This function is CPU-only!
///  **NB:**  If you need GPU-powered line drawing, either check low-level functions under `src/buffer_op/gpu`,
///  **NB:**  Or use the high-level functions from `draw::lines_batches`
/// # Parameters
/// - `buf`: The buffer of pixels to draw the lines on (will be split among threads)
/// - `buf_dimensions`: Dimensions of the buffer (width, height)
/// - `lines`: A vector containing the line information (as described above)
/// - `num_threads`: Number of threads to use for rendering
pub fn horizontal_lines_x4_threaded(buf: &mut [u32], buf_dimensions: &Dimensions2d, lines: &Vec<u32>, num_threads: usize) {
    if lines.len() % 4 != 0 || num_threads == 0 {
        return;
    }

    // 1. Parse and sort
    let mut parsed_lines = parse_lines_x4(lines, buf_dimensions.w, buf_dimensions.h);
    parsed_lines.sort_by_key(|line| line.y);

    // 2. Group and partition
    let y_groups = group_by_y(&parsed_lines);
    let total_lines = parsed_lines.len();
    let bands = partition_bands(&y_groups, num_threads, total_lines);

    let width = buf_dimensions.w;
    let height = buf_dimensions.h;
    let buf_ptr = buf.as_mut_ptr();

    // 3. Launch threads
    thread::scope(|scope| {
        for band in &bands {
            let y_range_opt = band_y_range(band, &y_groups);
            if band.is_empty() || y_range_opt.is_none() { continue; }
            let (y_start, y_end) = y_range_opt.unwrap();
            let band_lines = collect_band_lines(band, &y_groups, &parsed_lines);
            let n_rows = y_end - y_start + 1;
            let band_slice_len = (n_rows as usize) * (width as usize);
            let band_slice = unsafe {
                std::slice::from_raw_parts_mut(
                    buf_ptr.add((y_start * width) as usize),
                    band_slice_len
                )
            };
            scope.spawn(move || {
                draw_band_lines(&band_lines, band_slice, y_start, width, height);
            });
        }
    });
}


/// Draws horizontal lines on a buffer. **Multi-threaded!**
/// Each line is represented by a vector of `u32` where the first element is the color,
/// followed by triplets of `x_start`, `x_end`, and `y` coordinates.
///  **NB:** This function is CPU-only!
///  **NB:**  If you need GPU-powered line drawing, either check low-level functions under `src/buffer_op/gpu`,
///  **NB:**  Or use the high-level functions from `draw::lines_batches`
/// # Parameters
/// - `buf`: The buffer of pixels to draw the lines on
/// - `buf_dimensions`: Dimensions of the buffer (width, height)
/// - `lines`: A vector of vectors, where each inner vector contains a color in RGBA, followed by triplets of `x_start`, `x_end`, and `y`
pub fn horizontal_lines_x3_threaded(
    buf: &mut [u32],
    buf_dimensions: &Dimensions2d,
    lines: &Vec<Vec<u32>>,
    num_threads: usize
) {
    if lines.is_empty() || num_threads == 0 {
        return;
    }

    let mut parsed_lines = parse_lines_x3(lines, buf_dimensions.w, buf_dimensions.h);
    parsed_lines.sort_by_key(|line| line.y);

    let y_groups = group_by_y(&parsed_lines);
    let total_lines = parsed_lines.len();
    let bands = partition_bands(&y_groups, num_threads, total_lines);

    let width = buf_dimensions.w;
    let height = buf_dimensions.h;
    let buf_ptr = buf.as_mut_ptr();

    thread::scope(|scope| {
        for band in &bands {
            let y_range_opt = band_y_range(band, &y_groups);
            if band.is_empty() || y_range_opt.is_none() { continue; }
            let (y_start, y_end) = y_range_opt.unwrap();
            let band_lines = collect_band_lines(band, &y_groups, &parsed_lines);
            let n_rows = y_end - y_start + 1;
            let band_slice_len = (n_rows as usize) * (width as usize);
            let band_slice = unsafe {
                std::slice::from_raw_parts_mut(
                    buf_ptr.add((y_start * width) as usize),
                    band_slice_len
                )
            };
            scope.spawn(move || {
                draw_band_lines(&band_lines, band_slice, y_start, width, height);
            });
        }
    });
}




#[cfg(test)]
mod tests {
    use super::*;

    fn blank_buffer(w: u32, h: u32) -> Vec<u32> {
        vec![0xFF000000; (w * h) as usize]
    }

    #[test]
    fn test_parse_lines_x4_filters_and_clamps() {
        // Buffer is 10x5, lines: in bounds, partially out of bounds, reversed, negative, etc
        let lines = vec![
            0, 9, 2, 0xFFAA0000,     // valid, full width, y in bounds
            5, 12, 1, 0xFF00BB00,    // x_end clamps to 9
            8, 4, 0, 0xFF00BBFF,     // x_start > x_end, should be filtered out
            1, 5, 10, 0xFFFFFF00,    // y out of bounds, filtered
            0, 2, 3, 0xFFFF00FF,     // valid
        ];
        let parsed = parse_lines_x4(&lines, 10, 5);
        assert_eq!(parsed.len(), 3);
        assert_eq!(parsed[0].x_end, 9);
        assert_eq!(parsed[1].x_end, 9);
        assert_eq!(parsed[2].y, 3);
    }

    #[test]
    fn test_parse_lines_x3_batch_and_colors() {
        // Buffer is 6x4, each batch is a different color
        let lines = vec![
            vec![0xFF0000FF, 1, 3, 1, 4, 5, 1], // two lines at y=1, blue
            vec![0xFF00FF00, 2, 4, 2],          // one line at y=2, green
        ];
        let parsed = parse_lines_x3(&lines, 6, 4);
        assert_eq!(parsed.len(), 3);
        assert_eq!(parsed[0].color, 0xFF0000FF);
        assert_eq!(parsed[2].color, 0xFF00FF00);
        assert_eq!(parsed[2].x_start, 2);
        assert_eq!(parsed[2].x_end, 4);
    }

    #[test]
    fn test_group_by_y_correct_runs() {
        let mut lines = vec![
            LineMeta { x_start: 0, x_end: 1, y: 2, color: 0 },
            LineMeta { x_start: 2, x_end: 3, y: 2, color: 1 },
            LineMeta { x_start: 0, x_end: 2, y: 3, color: 2 },
        ];
        lines.sort_by_key(|l| l.y);
        let groups = group_by_y(&lines);
        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0].y, 2);
        assert_eq!(groups[0].len, 2);
        assert_eq!(groups[1].y, 3);
        assert_eq!(groups[1].len, 1);
    }

    #[test]
    fn test_partition_bands_even_split() {
        // 4 groups, 2 lines each, 2 threads
        let y_groups = vec![
            YGroup { y: 0, start: 0, len: 2 },
            YGroup { y: 1, start: 2, len: 2 },
            YGroup { y: 2, start: 4, len: 2 },
            YGroup { y: 3, start: 6, len: 2 },
        ];
        let bands = partition_bands(&y_groups, 2, 8);
        assert_eq!(bands.len(), 2);
        assert_eq!(bands[0].len() + bands[1].len(), 4);
        // Each band should have 4 lines in total
        let band0_count: usize = bands[0].iter().map(|&i| y_groups[i].len).sum();
        let band1_count: usize = bands[1].iter().map(|&i| y_groups[i].len).sum();
        assert_eq!(band0_count + band1_count, 8);
    }

    #[test]
    fn test_draw_band_lines_draws_correct_pixels() {
        let mut buf = blank_buffer(5, 2);
        let lines = vec![
            LineMeta { x_start: 1, x_end: 3, y: 0, color: 0xFF00FF00 },
            LineMeta { x_start: 2, x_end: 4, y: 1, color: 0xFFFF0000 },
        ];
        let band_lines: Vec<&LineMeta> = lines.iter().collect();
        draw_band_lines(&band_lines, &mut buf, 0, 5, 2);
        // Row 0: buf[1], buf[2], buf[3] should be green
        assert_eq!(buf[1], 0xFF00FF00);
        assert_eq!(buf[2], 0xFF00FF00);
        assert_eq!(buf[3], 0xFF00FF00);
        // Row 1: buf[5+2], buf[5+3], buf[5+4] should be red
        assert_eq!(buf[7], 0xFFFF0000);
        assert_eq!(buf[8], 0xFFFF0000);
        assert_eq!(buf[9], 0xFFFF0000);
    }

    #[test]
    fn test_horizontal_lines_x4_threaded_simple() {
        let mut buf = blank_buffer(8, 3);
        let dims = Dimensions2d { w: 8, h: 3 };
        let lines = vec![
            1, 4, 0, 0xFF00FF00, // y=0, green
            3, 7, 1, 0xFFFF0000, // y=1, red
        ];
        horizontal_lines_x4_threaded(&mut buf, &dims, &lines, 2);
        // Check that row 0, buf[1..=4] is green
        assert_eq!(&buf[1..=4], &[0xFF00FF00; 4]);
        // Check that row 1, buf[3..=7] is red
        assert_eq!(&buf[8+3..=8+7], &[0xFFFF0000; 5]);
    }

    #[test]
    fn test_horizontal_lines_x3_threaded_simple() {
        let mut buf = blank_buffer(6, 2);
        let dims = Dimensions2d { w: 6, h: 2 };
        let lines = vec![
            vec![0xFFFF00FF, 1, 4, 0],     // y=0, magenta
            vec![0xFF00FFFF, 2, 5, 1],     // y=1, cyan
        ];
        horizontal_lines_x3_threaded(&mut buf, &dims, &lines, 2);
        assert_eq!(&buf[1..=4], &[0xFFFF00FF; 4]); // row 0
        assert_eq!(&buf[6+2..=6+5], &[0xFF00FFFF; 4]); // row 1
    }
}
