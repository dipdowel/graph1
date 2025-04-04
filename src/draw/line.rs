use crate::core::context::alpha::AlphaMethod;
use crate::core::context::GraphContext;
use crate::core::misc::line_clipping_style::LineClippingStyle;
use crate::primitives::point::Point;
use crate::utils::clip;
use crate::utils::color::alpha::{blend_pixel_f32, blend_pixel_int};
use std::f32;

/// TODO: Add proper documentation to the function!
#[inline(always)]
fn write_pixel_with_blending(dst: &mut u32, src: u32, method: Option<AlphaMethod>) {
    *dst = match method {
        None => src,
        Some(AlphaMethod::Int) => blend_pixel_int(*dst, src),
        Some(AlphaMethod::Float) => blend_pixel_f32(*dst, src),
    };
}

/// TODO: Add proper documentation to the function!
#[inline(always)]
fn write_pixel(ctx: &mut GraphContext<impl Sized>, x: i32, y: i32, color: u32, alpha: f32) {
    if x < 0 || y < 0 || x >= ctx.win.w as i32 || y >= ctx.win.h as i32 {
        return;
    }
    let idx = (y as u32 * ctx.win.w + x as u32) as usize;
    let dst = &mut ctx.frame_buf[idx];
    let alpha_method = ctx.alpha.method;
    *dst = match alpha_method {
        AlphaMethod::Int => blend_pixel_int(*dst, color),
        AlphaMethod::Float => blend_pixel_f32(*dst, apply_alpha(color, alpha)),
    };
}

/// TODO: Add proper documentation to the function!
fn apply_alpha(color: u32, alpha: f32) -> u32 {
    let a = ((color >> 24) & 0xFF) as f32 * alpha;
    let r = ((color >> 16) & 0xFF) as f32 * alpha;
    let g = ((color >> 8) & 0xFF) as f32 * alpha;
    let b = (color & 0xFF) as f32 * alpha;
    ((a as u32).min(255) << 24)
        | ((r as u32).min(255) << 16)
        | ((g as u32).min(255) << 8)
        | (b as u32).min(255)
}

/// TODO: Add proper documentation to the function!
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
/// TODO: Add proper documentation to the function!
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
/// TODO: Add proper documentation to the function!
fn draw_line_bresenham<UserData>(
    ctx: &mut GraphContext<UserData>,
    x0: u32,
    y0: u32,
    x1: u32,
    y1: u32,
    color: u32,
) {
    let alpha_method = ctx.alpha.enabled.then_some(ctx.alpha.method);
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
        write_pixel_with_blending(&mut ctx.frame_buf[idx], color, alpha_method);
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
    let idx = (y as u32 * ctx.win.w + x as u32) as usize;
    write_pixel_with_blending(&mut ctx.frame_buf[idx], color, alpha_method);
}
/// TODO: Add proper documentation to the function!
// Draw a line with thickness and anti-aliasing.  
pub fn draw_line_thick_aa<UserData>(
    ctx: &mut GraphContext<UserData>,
    start: &Point<i32>,
    end: &Point<i32>,
    color: Option<u32>,
    thickness: f32,
) {
    let color = color.unwrap_or_else(|| ctx.win.foreground_color);
    let half_thick = (thickness / 2.0).max(0.5);
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
    let Some((p0, p1)) = clipped else { return; };
    let (x0, y0) = (p0.x as f32, p0.y as f32);
    let (x1, y1) = (p1.x as f32, p1.y as f32);
    let dx = x1 - x0;
    let dy = y1 - y0;
    let len = (dx * dx + dy * dy).sqrt();
    if len == 0.0 {
        return;
    }
    let (nx, ny) = (-dy / len, dx / len);
    let steps = len.ceil() as i32;
    for i in 0..steps {
        let t = i as f32 / len;
        let x = x0 + t * dx;
        let y = y0 + t * dy;
        for w in -half_thick.ceil() as i32..=half_thick.ceil() as i32 {
            let ox = x + w as f32 * nx;
            let oy = y + w as f32 * ny;
            let ix = ox.floor() as i32;
            let iy = oy.floor() as i32;
            let dist = ((ox - ix as f32).powi(2) + (oy - iy as f32).powi(2)).sqrt();
            let alpha = (1.0 - dist).clamp(0.0, 1.0);
            write_pixel(ctx, ix, iy, color, alpha);
        }
    }
}
