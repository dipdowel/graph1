use crate::core::context::GraphContext;
use crate::core::misc::line_clipping_style::LineClippingStyle;
use crate::primitives::point::Point;
use crate::utils::clip;


/// Draws a horizontal line starting from a signed point.
/// Clips the line manually against the window boundaries.
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

    // Reject line if it's vertically off-screen
    if start.y < 0 || start.y >= ctx.win.h as i32 {
        return;
    }

    let mut x0 = start.x;
    let mut x1 = start.x + length as i32;

    // Reject if completely to the left or right
    if x1 <= 0 || x0 >= ctx.win.w as i32 {
        return;
    }

    // Clip horizontally
    x0 = x0.max(0);
    x1 = x1.min(ctx.win.w as i32);

    if x1 <= x0 {
        return;
    }

    let mut buf_index = (start.y as u32 * ctx.win.w + x0 as u32) as usize;
    for _ in x0..x1 {
        ctx.frame_buf[buf_index] = color;
        buf_index += 1;
    }
}


/// Draws a vertical line starting from a signed point.
/// Clips the line manually against the window boundaries.
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

    // Reject line if it's horizontally off-screen
    if start.x < 0 || start.x >= ctx.win.w as i32 {
        return;
    }

    let mut y0 = start.y;
    let mut y1 = start.y + length as i32;

    // Reject if completely above or below screen
    if y1 <= 0 || y0 >= ctx.win.h as i32 {
        return;
    }

    // Clip vertically
    y0 = y0.max(0);
    y1 = y1.min(ctx.win.h as i32);

    if y1 <= y0 {
        return;
    }

    let mut buf_index = (y0 as u32 * ctx.win.w + start.x as u32) as usize;
    for _ in y0..y1 {
        ctx.frame_buf[buf_index] = color;
        buf_index += ctx.win.w_usize;
    }
}


/// Draws a line between two signed points with optional color and full clipping.
pub fn between_two_points<UserData>(
    ctx: &mut GraphContext<UserData>,
    start: &Point<i32>,
    end: &Point<i32>,
    color: Option<u32>,
) {
    let color = color.unwrap_or_else(|| ctx.win.foreground_color);

    let clipped = match ctx.line_clipping {
        LineClippingStyle::ElasticSlide => {
            clip::line::to_area_elastic_slide(start, end, &ctx.win.rect_area)
        }
        LineClippingStyle::CohenSutherland => {
            clip::line::to_area_cohen_sutherland(start, end, &ctx.win.rect_area)
        }
        LineClippingStyle::LiangBarsky => {
            clip::line::to_area_liang_barsky(start, end, &ctx.win.rect_area)
        }
    };

    let Some((p0, p1)) = clipped else {
        return;
    };

    draw_line_bresenham(ctx, p0.x, p0.y, p1.x, p1.y, color);
}

/// Draws a clipped and resolved-color line using Bresenham's algorithm.
fn draw_line_bresenham<UserData>(
    ctx: &mut GraphContext<UserData>,
    x0: u32,
    y0: u32,
    x1: u32,
    y1: u32,
    color: u32,
) {
    let mut x = x0 as i32;
    let mut y = y0 as i32;
    let x1 = x1 as i32;
    let y1 = y1 as i32;

    let dx = (x1 - x).abs();
    let dy = -(y1 - y).abs();
    let sx = if x < x1 { 1 } else { -1 };
    let sy = if y < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    while x != x1 || y != y1 {
        let idx = (y as u32 * ctx.win.w + x as u32) as usize;
        ctx.frame_buf[idx] = color;

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

    // Draw final point
    let idx = (y as u32 * ctx.win.w + x as u32) as usize;
    ctx.frame_buf[idx] = color;
}
