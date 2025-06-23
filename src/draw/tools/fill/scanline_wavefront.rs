use crate::primitives::plane::Dimensions2d;
use crate::primitives::Pixel;

/// A span of consecutive pixels on the same scanline (y coordinate)
struct Span {
    y: u32,
    x_left: u32,
    x_right: u32, // inclusive
}

/// Scanline/Wavefront fill.
/// In most cases it should be much faster than `fill::flood()`
/// Single-threaded. Operates on spans of pixels on the same scanline.
/// > A span is a continuous run of horizontally adjacent pixels on the same scanline (row)
/// > that all share the same target color to be filled.
/// > Instead of processing individual pixels, the scanline fill algorithm operates on these spans
/// > to improve efficiency and reduce redundant checks.
///
/// # Parameters
/// - `buf`: The buffer of pixels to fill
/// - `buf_dimensions`: Dimensions of the buffer (width, height)
/// - `start_pixel`: The starting pixel (with fill color)
pub fn scanline_wavefront(buf: &mut [u32], buf_dimensions: &Dimensions2d, start_pixel: &Pixel) {
    let width = buf_dimensions.w;
    let height = buf_dimensions.h;
    let fill_color = start_pixel.color;
    let initial_color = buf[(start_pixel.y * width + start_pixel.x) as usize];

    // Nothing to do if the initial and the fill colors match.
    if initial_color == fill_color {
        return;
    }

    let mut stack = Vec::new();
    // Start with the initial pixel as a span
    stack.push(Span {
        y: start_pixel.y,
        x_left: start_pixel.x,
        x_right: start_pixel.x,
    });

    let mut xl:u32;
    let mut xr:u32;

    while let Some(span) = stack.pop() {
        let y = span.y;
        xl = span.x_left;
        xr = span.x_right;
        // Extend left
        while xl > 0 && buf[(y * width + (xl - 1)) as usize] == initial_color {
            xl -= 1;
        }
        // Extend right
        while xr < width - 1 && buf[(y * width + (xr + 1)) as usize] == initial_color {
            xr += 1;
        }
        // Fill the span
        for x in xl..=xr {
            buf[(y * width + x) as usize] = fill_color;
        }

        // Check neighbors above and below
        // The edge case of `0.wrapping_sub(1)` here gives 0xFFFF_FFFF (u32::MAX),
        // which is then skipped with `continue` in the `if ny >= height` check.
        // That allows us to avoid an extra check hence no extra branch.
        for &ny in [y.wrapping_sub(1), y + 1].iter() {
            if ny >= height {
                continue;
            }
            let mut x = xl;
            while x <= xr {
                let idx = (ny * width + x) as usize;
                if buf[idx] == initial_color {
                    let run_start = x;
                    while x < width && x <= xr && buf[(ny * width + x) as usize] == initial_color {
                        x += 1;
                    }
                    stack.push(Span {
                        y: ny,
                        x_left: run_start,
                        x_right: x - 1,
                    });
                }
                x += 1;
            }
        }
    }
}
