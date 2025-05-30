use crate::core::context::GraphContext;
use crate::core::context_utils::context_snapshot::ContextSnapshot;
use crate::draw;
use crate::draw::polygons::closed_perimeter;
use crate::draw::tools::fill;
use crate::primitives::numeric::Numeric;
use crate::primitives::plane::RectArea;
use crate::primitives::point::Point;
use crate::utils::math::geometry::approximate_center;

/// Struct holding customizable properties of the 3D bar
/// @See `bar_3d()`.
#[derive(Debug, Clone, Copy)]
pub struct Bar3DProps {
    /// X-coordinate of the bar base
    pub x: i32,
    /// Y-coordinate of the bar base
    pub y: i32,
    /// Width of the bar front face
    pub width: i32,
    /// Height of the bar (on the screen)
    pub height: i32,
    /// Depth of the 3D bar (isometric projection)
    pub depth: i32,
    /// Color of the front face
    pub color_front: u32,
    /// Color of the top face
    pub color_top: u32,
    /// Color of the side face
    pub color_side: u32,

    /// Defines how much the top and side faces skew to simulate depth.
    /// slant.x and slant.y control horizontal and vertical offset per depth step,
    /// shaping the bar’s 3D tilt visually.
    /// Best works with low values (1..10).
    pub slant: Point<u32>
}


/// Draws a pseudo-3D bar in isometric projection using polygons and flood fill.
/// Each face is drawn using `closed_perimeter()` and filled with `flood()`.
///
/// # Arguments
/// * `ctx` - Mutable reference to the GraphContext
/// * `props` - Configuration for the bar appearance and position
pub fn bar_3d<UserData>(ctx: &mut GraphContext<UserData>, props: &Bar3DProps) {
    let Bar3DProps {
        x,
        y,
        width,
        height,
        depth,
        color_front,
        color_top,
        color_side,
        slant
    } = *props;
    let slant_x = slant.x as i32; // how much the bar shifts right
    let slant_y = slant.y as i32; // how much the bar shifts up

    //
    // FRONT face (rectangle)
    // Front face of the bar (just a flat rectangle)
    let front = RectArea::new(x, y - height, width, height + 1, Some(color_front));
    draw::rectangle::filled(ctx, &front);

    //
    // TOP face (parallelogram) drawn using horizontal lines with slant control
    let line_ctx_state = ctx.line.get_context();
    ctx.line.set_int_no_aa(Some(1));

    for i in 0..=depth {
        let base_x = x + i * slant_x;
        let base_y = y - height - i * slant_y;

        // Fill all y_slant scanlines at this depth level
        for j in 0..slant_y {
            let y_line = base_y + j;
            let start = Point::new(base_x, y_line);
            draw::line::horizontal(ctx, &start, i32::to_u32(width), Some(color_top));
        }
    }
    ctx.line.set_context(line_ctx_state);

    //
    // RIGHT-SIDE face (slanted parallelogram)
    let side = vec![
        Point::new(x + width, (y - height)),
        Point::new(x + width + depth * slant_x, y - height - depth * slant_y),
        Point::new(x + width + depth * slant_x, y - depth * slant_y),
        Point::new(x + width, y),
    ];

    closed_perimeter(ctx, &side, Some(color_side));

    let flood_fill_point = approximate_center(&side).unwrap_or(side[0].clone() + Point::new(1, 1));

    fill::flood(
        &mut ctx.frame_buf,
        &ctx.win.dimensions,
        &flood_fill_point.to_pixel(color_side),
    );
}