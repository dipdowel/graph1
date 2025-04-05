use crate::core::context::alpha::AlphaMethod;
use crate::core::context::GraphContext;
use crate::utils::color::alpha::{blend_pixel_f32, blend_pixel_int};

/// TODO: Add proper documentation to the function!
/// Blends a color into the framebuffer at (x, y) with integer-based alpha (0..=255).
#[inline(always)]
pub(crate) fn write_pixel_with_blending(dst: &mut u32, src: u32, method: Option<AlphaMethod>) {
    *dst = match method {
        None => src,
        Some(AlphaMethod::Int) => blend_pixel_int(*dst, src),
        Some(AlphaMethod::Float) => blend_pixel_f32(*dst, src),
    };
}

pub(crate) fn write_pixel(ctx: &mut GraphContext<impl Sized>, x: i32, y: i32, color: u32, alpha: u8) {
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
pub(crate) fn write_pixel_f32(ctx: &mut GraphContext<impl Sized>, x: i32, y: i32, color: u32, alpha: f32) {
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

/// Applies integer alpha (0..=255) to a 32-bit RGBA color.
#[inline(always)]
pub(crate) fn apply_alpha_u8(color: u32, alpha: u8) -> u32 {
    let a = ((color >> 0) & 0xFF) * alpha as u32 / 255;
    let b = ((color >> 8) & 0xFF) * alpha as u32 / 255;
    let g = ((color >> 16) & 0xFF) * alpha as u32 / 255;
    let r = ((color >> 24) & 0xFF) * alpha as u32 / 255;
    (r << 24) | (g << 16) | (b << 8) | a
}
