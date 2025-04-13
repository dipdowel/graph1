use crate::core::context::GraphContext;
use crate::core::context_utils::line_clipping_style::LineClippingStyle;
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

/// Draws a line between two points using the context's line width and anti-aliasing settings.
/// Falls back to Bresenham if line width is 1 and anti-aliasing is off.
///
/// # Arguments
/// * `ctx` - The graph context.
/// * `start` - The starting point of the line.
/// * `end` - The ending point of the line.
/// * `color` - Optional color for the line. If not provided, the context's foreground color is used.
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
    let no_int = line.rasterization.is_int() && line.width_int < 1;
    let no_float = line.rasterization.is_float() && line.width_float == 0.0;
    !(no_int && no_float)
}

#[inline(always)]
fn should_use_bresenham<UserData>(ctx: &GraphContext<UserData>) -> bool {
    ctx.line.width_int == 1 && !ctx.line.anti_aliasing.enabled && ctx.line.rasterization.is_int()
}

#[inline(always)]
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

#[inline(always)]
fn draw_along_axis<UserData>(
    ctx: &mut GraphContext<UserData>,
    p0: &Point<i32>,
    p1: &Point<i32>,
    color: u32,
) -> bool {
    if !ctx.line.anti_aliasing.enabled {
        if p0.x == p1.x && p0.y != p1.y {
            let start = Point::new(p0.x, p0.y.min(p1.y));
            let length = (p1.y - p0.y).abs() as u32;
            vertical(ctx, &start, length, Some(color));
            return true;
        } else if p0.y == p1.y && p0.x != p1.x {
            let start = Point::new(p0.x.min(p1.x), p0.y);
            let length = (p1.x - p0.x).abs() as u32;
            horizontal(ctx, &start, length, Some(color));
            return true;
        }
    }
    false
}

#[inline(always)]
fn draw_integer_line<UserData>(
    ctx: &mut GraphContext<UserData>,
    p0: &Point<i32>,
    p1: &Point<i32>,
    color: u32,
) {
    if draw_along_axis(ctx, p0, p1, color) {
        return;
    }

    let dx = p1.x - p0.x;
    let dy = p1.y - p0.y;
    let len = p1.distance_to(p0) as i32;

    if len == 0 {
        return;
    }



    let line_width = ctx.line.width_int.max(1);
    let radius = line_width as f32 / 2.0;
    let ceil_radius = radius.ceil() as i32;
    let max_dist2 = radius * radius;

    // Used for limiting the length of the line to prevent protrusions due to the line width
    let dst_len = len-(ctx.line.width_int as i32/2);

    for i in 0..=len {

        if i == dst_len {
            break;
        }

        let t = i as f32 / len as f32;
        let x = p0.x as f32 + t * dx as f32;
        let y = p0.y as f32 + t * dy as f32;

        let cx = (x + 0.5).floor() as i32;
        let cy = (y + 0.5).floor() as i32;

        for oy in -ceil_radius..=ceil_radius {
            for ox in -ceil_radius..=ceil_radius {
                let px = cx + ox;
                let py = cy + oy;

                let dist2 = (x - px as f32).powi(2) + (y - py as f32).powi(2);

                if dist2 <= max_dist2 {
                    if ctx.line.anti_aliasing.enabled && ctx.line.anti_aliasing.method.is_int() {
                        let alpha = ((1.0 - dist2 / max_dist2) * 255.0).clamp(0.0, 255.0) as u8;
                        write_pixel(ctx, px, py, color, alpha);
                    } else {
                        write_pixel(ctx, px, py, color, 255);
                    }
                }
            }
        }
    }
}

#[inline(always)]
fn draw_float_line<UserData>(
    ctx: &mut GraphContext<UserData>,
    p0: &Point<i32>,
    p1: &Point<i32>,
    color: u32,
) {
    if draw_along_axis(ctx, p0, p1, color) {
        return;
    }

    let dx = (p1.x - p0.x) as f32;
    let dy = (p1.y - p0.y) as f32;
    let len = p1.distance_to(p0) as f32;

    if len == 0.0 {
        return;
    }


    let radius = ctx.line.width_float.max(1.0) / 2.0;
    let ceil_radius = radius.ceil() as i32;
    let max_dist2 = radius * radius;

    // Used for limiting the length of the line to prevent protrusions due to the line width
    let dst_len = ( len - ctx.line.width_float / 2.0) as i32;

    for i in 0..=len.ceil() as i32 {

        if i == dst_len {
            break;
        }


        let t = i as f32 / len;
        let x = p0.x as f32 + t * dx;
        let y = p0.y as f32 + t * dy;

        let cx = (x + 0.5).floor() as i32;
        let cy = (y + 0.5).floor() as i32;

        for oy in -ceil_radius..=ceil_radius {
            for ox in -ceil_radius..=ceil_radius {
                let px = cx + ox;
                let py = cy + oy;

                let dist2 = (x - px as f32).powi(2) + (y - py as f32).powi(2);

                if dist2 <= max_dist2 {
                    if ctx.line.anti_aliasing.enabled && ctx.line.anti_aliasing.method.is_float() {
                        let alpha = (1.0 - dist2.sqrt() / radius).clamp(0.0, 1.0);
                        write_pixel_f32(ctx, px, py, color, alpha);
                    } else {
                        write_pixel(ctx, px, py, color, 255);
                    }
                }
            }
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
    let half_thickness = (ctx.line.width_int.max(1) / 2) as i32;
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
    let half_thickness = (ctx.line.width_int.max(1) / 2) as i32;
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
