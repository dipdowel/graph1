use crate::core::context::alpha::AlphaMethod;
use crate::core::context::GraphContext;
use crate::primitives::plane::RectArea;
use crate::utils::color::alpha::{blend_pixel_f32, blend_pixel_int};
use std::cmp::PartialEq;

impl PartialEq for AlphaMethod {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (AlphaMethod::Int, AlphaMethod::Int) => true,
            (AlphaMethod::Float, AlphaMethod::Float) => true,
            _ => false,
        }
    }
}

/// Draws a rectangle with dimensions and filled with a color specified in the `RectArea` struct.
pub fn filled<UserData>(ctx: &mut GraphContext<UserData>, rect: &RectArea) {
    let color = rect.color.unwrap_or(ctx.win.foreground_color);

    // Dereference the options
    let start_x = rect.top_left.x;
    let start_y = rect.top_left.y;
    let width = rect.dimensions.w;
    let height = rect.dimensions.h;

    // Nothing to draw here
    if width == 0 || height == 0 {
        return;
    }

    let end_x = start_x + width;
    let end_y = start_y + height;

    let mut x = start_x;
    let mut y = start_y;

    // Which pixel in the vector should be filled in next.
    let mut pixel_index: usize;

    let mut resulting_color: u32 = color;

    loop {
        pixel_index = (y * ctx.win.w + x) as usize;

        // If alpha blending is enabled, blend the new color with the existing pixel color
        // according to the alpha channel.
        if ctx.alpha.enabled {
            if ctx.alpha.method == AlphaMethod::Int {
                resulting_color = blend_pixel_int(ctx.frame_buf[pixel_index], color);
            }
            if ctx.alpha.method == AlphaMethod::Float {
                resulting_color = blend_pixel_f32(ctx.frame_buf[pixel_index], color);
            }
        }
        ctx.frame_buf[pixel_index] = resulting_color;

        x += 1;

        if x == end_x || x == ctx.win.w {
            y += 1;
            x = start_x;
        };

        if y == end_y || y == ctx.win.h {
            break;
        }
    }
}
