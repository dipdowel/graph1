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
    // TODO: Detect vertical lines and draw them using a more optimised approach
    // if x0 == x1 && y0 != y1 {
    //     vertical(... parameters ...);
    //     return;
    // }

    // TODO: Detect horizontal lines and draw them using a more optimised approach
    // if y0 == y1 && x0 != x1 {
    //     horizontal( ... parameters ...);
    //     return;
    // }


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

/// TODO: Add proper documentation to the function!
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
    let dx = (p1.x - p0.x) as f32;
    let dy = (p1.y - p0.y) as f32;
    let len = (dx * dx + dy * dy).sqrt();
    if len == 0.0 {
        return;
    }

    let (nx, ny) = (-dy / len, dx / len);
    let is_thin = ctx.line.stroke_width_float <= 1.0;
    let half_thick = if is_thin {
        0.0
    } else {
        ctx.line.stroke_width_float / 2.0
    };

    for i in 0..=len.ceil() as i32 {
        let t = i as f32 / len;
        let x = p0.x as f32 + t * dx;
        let y = p0.y as f32 + t * dy;

        for w in if is_thin {
            0..=0
        } else {
            -half_thick.ceil() as i32..=half_thick.ceil() as i32
        } {
            let ox = x + w as f32 * nx;
            let oy = y + w as f32 * ny;
            let ix = ox.round() as i32;
            let iy = oy.round() as i32;

            if ctx.line.anti_aliasing.enabled && ctx.line.anti_aliasing.method.is_float() {
                let dx = ox - ix as f32;
                let dy = oy - iy as f32;
                let dist2 = dx * dx + dy * dy;
                let alpha = (1.0 - dist2.sqrt()).clamp(0.0, 1.0);
                write_pixel_f32(ctx, ix, iy, color, alpha);
            } else {
                write_pixel(ctx, ix, iy, color, 255);
            }
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
    let dx = p1.x - p0.x;
    let dy = p1.y - p0.y;
    let len = (((dx * dx + dy * dy) as f64).sqrt()) as i32;
    if len == 0 {
        return;
    }
    let scale = 256;
    let nx = (-dy * scale) / len;
    let ny = (dx * scale) / len;
    let half_thick_px = (ctx.line.stroke_width_int.max(1) / 2) as i32;

    for i in 0..=len {
        let t = (i * 256) / len;
        let x = p0.x * 256 + t * dx;
        let y = p0.y * 256 + t * dy;

        for w in -half_thick_px..=half_thick_px {
            let ox = x + w * nx;
            let oy = y + w * ny;
            let ix = ((ox + 128) / 256) as i32;
            let iy = ((oy + 128) / 256) as i32;

            if ctx.line.anti_aliasing.enabled && ctx.line.anti_aliasing.method.is_int() {
                let dx = ox - ix * 256;
                let dy = oy - iy * 256;
                let dist2 = dx * dx + dy * dy;
                let max_dist2 = 2 * 256 * 256;
                let alpha = (((max_dist2 - dist2) * 255) / max_dist2).clamp(0, 255) as u8;
                write_pixel(ctx, ix, iy, color, alpha);
            } else {
                write_pixel(ctx, ix, iy, color, 255);
            }
        }
    }
}

/// TODO: Add proper documentation to the function!
/// TODO: Add support for line thickness (read from the `ctx`) to the function!
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
    if start.y < 0 || start.y >= ctx.win.h as i32 {
        return;
    }
    let mut x0 = start.x;
    let mut x1 = start.x + length as i32;
    if x1 <= 0 || x0 >= ctx.win.w as i32 {
        return;
    }
    x0 = x0.max(0);
    x1 = x1.min(ctx.win.w as i32);
    if x1 <= x0 {
        return;
    }
    let mut buf_index = (start.y as u32 * ctx.win.w + x0 as u32) as usize;
    for _ in x0..x1 {
        write_pixel_with_blending(&mut ctx.frame_buf[buf_index], color, alpha_method);
        buf_index += 1;
    }
}
/// TODO: Add proper documentation to the function!
/// TODO: Add support for line thickness (read from the `ctx`) to the function!
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
    if start.x < 0 || start.x >= ctx.win.w as i32 {
        return;
    }
    let mut y0 = start.y;
    let mut y1 = start.y + length as i32;
    if y1 <= 0 || y0 >= ctx.win.h as i32 {
        return;
    }
    y0 = y0.max(0);
    y1 = y1.min(ctx.win.h as i32);
    if y1 <= y0 {
        return;
    }
    let mut buf_index = (y0 as u32 * ctx.win.w + start.x as u32) as usize;
    for _ in y0..y1 {
        write_pixel_with_blending(&mut ctx.frame_buf[buf_index], color, alpha_method);
        buf_index += ctx.win.w_usize;
    }
}
