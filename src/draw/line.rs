use crate::core::context::GraphContext;
use crate::core::misc::line_clipping_style::LineClippingStyle;
use crate::draw::helpers::write_pixel::{write_pixel, write_pixel_f32, write_pixel_with_blending};
use crate::primitives::point::Point;
use crate::utils::clip;

/// Fast, integer-only line drawing without thickness or anti-aliasing.
#[inline(always)]
fn draw_line_bresenham<UserData>(
    ctx: &mut GraphContext<UserData>,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    color: u32,
) {
    if x0 == x1 && y0 != y1 {
        let start = Point::new(x0, y0.min(y1));
        let length = (y1 - y0).abs() as u32;
        vertical(ctx, &start, length, Some(color));
        return;
    }
    if y0 == y1 && x0 != x1 {
        let start = Point::new(x0.min(x1), y0);
        let length = (x1 - x0).abs() as u32;
        horizontal(ctx, &start, length, Some(color));
        return;
    }

    let mut x = x0;
    let mut y = y0;
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        write_pixel(ctx, x, y, color, 255);
        if x == x1 && y == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}

/// Draws a line between two points using the context's stroke width and anti-aliasing settings.
/// Falls back to Bresenham if stroke width is 1 and anti-aliasing is off.
pub fn between_two_points<UserData>(
    ctx: &mut GraphContext<UserData>,
    start: &Point<i32>,
    end: &Point<i32>,
    color: Option<u32>,
) {
    if !is_line_visible(ctx) {
        return;
    }

    let color = color.unwrap_or_else(|| ctx.win.foreground_color);


    // This is an `let-else` statement, stabilised in Rust 1.65.
    // It does the same as
    // let (p0, p1) = match clipped {
    //     Some(pair) => pair,
    //     None => return,
    // };
    let Some((p0, p1)) = clip_line(ctx, start, end) else {
        return;
    };

    let p0 = p0.convert::<i32>();
    let p1 = p1.convert::<i32>();

    if should_use_bresenham(ctx) {
        draw_line_bresenham(ctx, p0.x, p0.y, p1.x, p1.y, color);
    } else if ctx.line.rasterization.is_float() {
        draw_float_line(ctx, &p0, &p1, color);
    } else {
        draw_integer_line(ctx, &p0, &p1, color);
    }
}

/// Determines whether the line should be rendered based on the context configuration.
/// A line is not rendered if its thickness is invalid (e.g., 0).
fn is_line_visible<UserData>(ctx: &GraphContext<UserData>) -> bool {
    let line = &ctx.line;
    let no_int = line.rasterization.is_int() && line.stroke_width_int < 1;
    let no_float = line.rasterization.is_float() && line.stroke_width_float == 0.0;
    !(no_int && no_float)
}

fn should_use_bresenham<UserData>(ctx: &GraphContext<UserData>) -> bool {
    ctx.line.stroke_width_int == 1 && !ctx.line.anti_aliasing.enabled
}

fn clip_line<UserData>(
    ctx: &GraphContext<UserData>,
    start: &Point<i32>,
    end: &Point<i32>,
) -> Option<(Point<u32>, Point<u32>)> {
    match ctx.line.clipping {
        LineClippingStyle::ElasticSlide => {
            clip::line::to_area_elastic_slide(start, end, &ctx.win.rect_area)
        }
        LineClippingStyle::CohenSutherland => {
            clip::line::to_area_cohen_sutherland(start, end, &ctx.win.rect_area)
        }
        LineClippingStyle::LiangBarsky => {
            clip::line::to_area_liang_barsky(start, end, &ctx.win.rect_area)
        }
    }
}
/// TODO: Add proper documentation to the function!
/// TODO: Check if it is possible to call `vertical()` and `horizontal()` from here for optization when a line needs to be drawn along one of the axis.
fn draw_float_line<UserData>(
    ctx: &mut GraphContext<UserData>,
    p0: &Point<i32>,
    p1: &Point<i32>,
    color: u32,
) {
    // Compute directional deltas as floats
    let dx = (p1.x - p0.x) as f32;
    let dy = (p1.y - p0.y) as f32;

    // Compute line length (Euclidean distance)
    let len = (dx * dx + dy * dy).sqrt();
    if len == 0.0 {
        return;
    }

    // Iterate over points on the line from p0 to p1
    for i in 0..=len.ceil() as i32 {
        let t = i as f32 / len;
        let x = p0.x as f32 + t * dx;
        let y = p0.y as f32 + t * dy;

        // Round to nearest integer pixel position
        let ix = x.round() as i32;
        let iy = y.round() as i32;

        // Draw center pixel
        write_pixel(ctx, ix, iy, color, 255);

        // Get thickness and draw additional horizontal copies
        let thickness = ctx.line.stroke_width_float.round() as i32;
        for offset in 1..=(thickness / 2) {
            write_pixel(ctx, ix + offset, iy, color, 255);
            write_pixel(ctx, ix - offset, iy, color, 255);
        }
    }
}

/// TODO: Add proper documentation to the function!
/// TODO: Check if it is possible to call `vertical()` and `horizontal()` from here for optization when a line needs to be drawn along one of the axis.
fn draw_integer_line<UserData>(
    ctx: &mut GraphContext<UserData>,
    p0: &Point<i32>,
    p1: &Point<i32>,
    color: u32,
) {
    // Delta X and Delta Y: the difference in horizontal and vertical direction between the two points
    let dx = p1.x - p0.x;
    let dy = p1.y - p0.y;

    // Length: Euclidean distance between p0 and p1, rounded to nearest integer
    let len = (((dx * dx + dy * dy) as f64).sqrt()) as i32;
    if len == 0 {
        return;
    }

    // Scale factor used to simulate subpixel precision using fixed-point math
    let scale = 512;

    // Loop over each step along the line length
    for i in 0..=len {
        // t: interpolation parameter (scaled)
        let t = (i * scale) / len;

        // x and y: current point along the line, in fixed-point coordinates
        let x = p0.x * scale + t * dx;
        let y = p0.y * scale + t * dy;

        // Convert fixed-point back to integer screen coordinates by rounding
        let ix = ((x + scale / 2) / scale) as i32;
        let iy = ((y + scale / 2) / scale) as i32;

        // Draw the center pixel
        write_pixel(ctx, ix, iy, color, 255);

        // Draw additional pixels based on thickness rule
        let thickness = ctx.line.stroke_width_int as i32;

        // For thickness > 1, replicate pixels symmetrically left and right (on X axis only)
        for offset in 1..=(thickness / 2) {
            // Draw to the right
            write_pixel(ctx, ix + offset, iy, color, 255);
            // Draw to the left (only if within bounds of the rule)
            write_pixel(ctx, ix - offset, iy, color, 255);
        }
    }
}


/// Optimized horizontal line renderer with thickness and optional anti-aliasing.
pub fn horizontal<UserData>(
    ctx: &mut GraphContext<UserData>,
    start: &Point<i32>,
    length: u32,
    color: Option<u32>,
) {
    if length == 0 {
        return;
    }
    let color = color.unwrap_or_else(|| ctx.win.foreground_color);
    let alpha_method = ctx.alpha.enabled.then_some(ctx.alpha.method);
    let half_thickness = (ctx.line.stroke_width_int.max(1) / 2) as i32;
    for offset in -half_thickness..=half_thickness {
        let y = start.y + offset;
        if y < 0 || y >= ctx.win.h as i32 {
            continue;
        }
        let mut x0 = start.x;
        let mut x1 = start.x + length as i32;
        if x1 <= 0 || x0 >= ctx.win.w as i32 {
            continue;
        }
        x0 = x0.max(0);
        x1 = x1.min(ctx.win.w as i32);
        if x1 <= x0 {
            continue;
        }
        let mut buf_index = (y as u32 * ctx.win.w + x0 as u32) as usize;
        for _ in x0..x1 {
            write_pixel_with_blending(&mut ctx.frame_buf[buf_index], color, alpha_method);
            buf_index += 1;
        }
    }
}

/// Optimized vertical line renderer with thickness and optional anti-aliasing.
pub fn vertical<UserData>(
    ctx: &mut GraphContext<UserData>,
    start: &Point<i32>,
    length: u32,
    color: Option<u32>,
) {
    if length == 0 {
        return;
    }
    let color = color.unwrap_or_else(|| ctx.win.foreground_color);
    let alpha_method = ctx.alpha.enabled.then_some(ctx.alpha.method);
    let half_thickness = (ctx.line.stroke_width_int.max(1) / 2) as i32;
    for offset in -half_thickness..=half_thickness {
        let x = start.x + offset;
        if x < 0 || x >= ctx.win.w as i32 {
            continue;
        }
        let mut y0 = start.y;
        let mut y1 = start.y + length as i32;
        if y1 <= 0 || y0 >= ctx.win.h as i32 {
            continue;
        }
        y0 = y0.max(0);
        y1 = y1.min(ctx.win.h as i32);
        if y1 <= y0 {
            continue;
        }
        let mut buf_index = (y0 as u32 * ctx.win.w + x as u32) as usize;
        for _ in y0..y1 {
            write_pixel_with_blending(&mut ctx.frame_buf[buf_index], color, alpha_method);
            buf_index += ctx.win.w_usize;
        }
    }
}
