use crate::core::context::alpha::AlphaMethod;
use crate::core::context::GraphContext;
use crate::core::misc::line_clipping_style::LineClippingStyle;
use crate::primitives::point::Point;
use crate::utils::clip;
use crate::utils::color::alpha::{blend_pixel_f32, blend_pixel_int};

/// Blends a color into the framebuffer at (x, y) with integer-based alpha (0..=255).
#[inline(always)]
fn write_pixel(ctx: &mut GraphContext<impl Sized>, x: i32, y: i32, color: u32, alpha: u8) {
    if x < 0 || y < 0 || x >= ctx.win.w as i32 || y >= ctx.win.h as i32 {
        return;
    }
    let idx = (y as u32 * ctx.win.w + x as u32) as usize;
    let dst = &mut ctx.frame_buf[idx];

    *dst = match ctx.alpha.method {
        AlphaMethod::Int => blend_pixel_int(*dst, apply_alpha_u8(color, alpha)),
        AlphaMethod::Float => blend_pixel_f32(*dst, apply_alpha_u8(color, alpha)),
    };
}


/// Blends a color into the framebuffer using a floating-point alpha [0.0..=1.0].
#[inline(always)]
fn write_pixel_f32(ctx: &mut GraphContext<impl Sized>, x: i32, y: i32, color: u32, alpha: f32) {
    if x < 0 || y < 0 || x >= ctx.win.w as i32 || y >= ctx.win.h as i32 {
        return;
    }
    let idx = (y as u32 * ctx.win.w + x as u32) as usize;
    let dst = &mut ctx.frame_buf[idx];
    // Do NOT pre-multiply alpha — blend_pixel_f32 expects RGBA with alpha in A channel
    let a = (alpha.clamp(0.0, 1.0) * 255.0).round() as u8;
    let color_with_alpha = (color & 0xFFFFFF00) | (a as u32);
    *dst = blend_pixel_f32(*dst, color_with_alpha);
}


/// Applies floating-point alpha [0.0..=1.0] to a 32-bit RGBA color.
#[inline(always)]
fn apply_alpha_f32(color: u32, alpha: f32) -> u32 {
    unreachable!("apply_alpha_f32() is no longer used; see write_pixel_f32")
}

/// Applies integer alpha (0..=255) to a 32-bit RGBA color.
#[inline(always)]
fn apply_alpha_u8(color: u32, alpha: u8) -> u32 {
    let a = ((color >> 0) & 0xFF) * alpha as u32 / 255;
    let b = ((color >> 8) & 0xFF) * alpha as u32 / 255;
    let g = ((color >> 16) & 0xFF) * alpha as u32 / 255;
    let r = ((color >> 24) & 0xFF) * alpha as u32 / 255;
    (r << 24) | (g << 16) | (b << 8) | a
}

/// Fallback: Fast, integer-only line drawing without thickness or anti-aliasing.
fn draw_line_bresenham<UserData>(
    ctx: &mut GraphContext<UserData>,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    color: u32,
) {
    let mut x = x0;
    let mut y = y0;
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        write_pixel(ctx, x, y, color, 255);
        if x == x1 && y == y1 { break; }
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

    let line = &ctx.line;


    // Check if lines are configured to be drawn and return if not
    let no_int_defined = line.rasterization.is_int() && line.stroke_width_int < 1;
    let no_float_defined = line.rasterization.is_float() && line.stroke_width_float == 0.0;
    if no_int_defined && no_float_defined {
        return;
    }

    let color = color.unwrap_or_else(|| ctx.win.foreground_color);

    // First, apply line clipping based on the selected strategy
    let clipped = match ctx.line.clipping {
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
    let p0 = p0.convert::<i32>();
    let p1 = p1.convert::<i32>();

    // Fast path: use Bresenham if stroke is 1px and anti-aliasing is disabled
    if ctx.line.stroke_width_int == 1 && !ctx.line.anti_aliasing.enabled {
        draw_line_bresenham(ctx, p0.x, p0.y, p1.x, p1.y, color);
        return;
    }

    // Float-based rendering
    if ctx.line.rasterization.is_float() {
        let dx = p1.x as f32 - p0.x as f32;
        let dy = p1.y as f32 - p0.y as f32;
        let len = (dx * dx + dy * dy).sqrt();
        if len == 0.0 {
            return;
        }

        let (nx, ny) = (-dy / len, dx / len);
        let is_thin = line.stroke_width_float <= 1.0;
        let half_thick = if is_thin { 0.0 } else { line.stroke_width_float / 2.0 };

        for i in 0..=len.ceil() as i32 {
            let t = i as f32 / len;
            let x = p0.x as f32 + t * dx;
            let y = p0.y as f32 + t * dy;

            for w in if is_thin { 0..=0 } else { -half_thick.ceil() as i32..=half_thick.ceil() as i32 } {
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
        return;
    }

    // Integer-based rendering (fixed-point)
    let dx = p1.x - p0.x;
    let dy = p1.y - p0.y;
    let len = (((dx * dx + dy * dy) as f64).sqrt()) as i32;
    if len == 0 {
        return;
    }
    let scale = 256;
    let nx = (-dy * scale) / len;
    let ny = (dx * scale) / len;
    let half_thick_px = ( line.stroke_width_int.max(1)  / 2) as i32;

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
