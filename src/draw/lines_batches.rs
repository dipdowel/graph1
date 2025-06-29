use crate::buffer_op;
use crate::core::context::GraphContext;

#[inline(always)]
fn convert_scanlines_to_flat_and_pointers(scanlines: &Vec<Vec<u32>>) -> (Vec<u32>, Vec<usize>) {
    // Convert `scanlines` into a flat Vec<u32> (`scanline_data`)
    // and a Vec<usize> (`scanline_pointers`) with the start index of each scanline in `scanline_data`.
    let mut scanline_data = Vec::new();
    let mut scanline_pointers = Vec::with_capacity(scanlines.len() + 1);

    for scanline in scanlines {
        scanline_pointers.push(scanline_data.len());
        scanline_data.extend(scanline.iter());
    }
    // Final pointer marks the end of the last scanline
    scanline_pointers.push(scanline_data.len());

    (scanline_data, scanline_pointers)
}

/// Draws horizontal lines on a buffer.  **Single-threaded!**
/// Each scanline is represented by a vector of `u32` where the 0th element is the `y` coordinate,
/// the 1st element is the color in RGBA, and the rest are pairs of `x_start` and  `x_end` values
/// of line segments on that scanline.
///
/// # Parameters
/// - `ctx` : A mutable reference to the Graph1 context.
/// - `scanlines`: A vector where each element describes line segments on one `y` scanline as follows
///   - `[0]` - the `y` coordinate
///   - `[1]` - the color in RGBA,
///   - `[2]` - `x_start` of the 1st line segment
///   - `[3]` - `x_end` of the 1st line segment
///   - `[4]` - `x_start` of the 2nd line segment
///   - `[5]` - `x_end` of the 2nd line segment
///   - Etc.
pub fn horizontal_lines_y_grouped<UserData>(
    ctx: &mut GraphContext<UserData>,
    scanlines: &Vec<Vec<u32>>,
) {
    let (scanline_data, scanline_pointers) = convert_scanlines_to_flat_and_pointers(scanlines);
    buffer_op::lines::horizontal_batch::horizontal_lines_y_grouped(
        &mut ctx.frame_buf,
        &ctx.win.dimensions,
        &scanline_data,
        &scanline_pointers,
    );
}

/// Draws horizontal lines on a buffer. **Multi-threaded!**
/// Each scanline is represented by a vector of `u32` where the 0th element is the `y` coordinate,
/// the 1st element is the color in RGBA, and the rest are pairs of `x_start` and  `x_end` values
/// of line segments on that scanline.
///
/// # Parameters
/// - `ctx` : A mutable reference to the Graph1 context.
/// - `scanlines`: A vector where each element describes line segments on one `y` scanline as follows
///   - `[0]` - the `y` coordinate
///   - `[1]` - the color in RGBA,
///   - `[2]` - `x_start` of the 1st line segment
///   - `[3]` - `x_end` of the 1st line segment
///   - `[4]` - `x_start` of the 2nd line segment
///   - `[5]` - `x_end` of the 2nd line segment
///   - Etc.
pub fn horizontal_lines_y_grouped_threaded<UserData>(
    ctx: &mut GraphContext<UserData>,
    scanlines: &Vec<Vec<u32>>,
) {
    let (scanline_data, scanline_pointers) = convert_scanlines_to_flat_and_pointers(scanlines);

    buffer_op::lines::horizontal_batch::horizontal_lines_y_grouped(
        &mut ctx.frame_buf,
        &ctx.win.dimensions,
        &scanline_data,
        &scanline_pointers,
    );
}
