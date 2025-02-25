use crate::core::context::GraphContext;
use crate::draw::line;
use crate::primitives::Pixel;
use crate::primitives::point::Point;

/// Draws straight lines between consecutive points to form a closed path
///
/// # Parameters
///
/// * `ctx` - A mutable reference to the `GraphContext`
/// * `vertices` - Dots to connect
/// * `color` - A color as `RGBA`, if `None`, `ctx.win.foreground_color` will be used
///
pub fn closed_perimeter<UserData>(ctx: &mut GraphContext<UserData>, vertices: &Vec<Point>, color: Option<u32>) {

    let color = color.unwrap_or(ctx.win.foreground_color);

    if let Some(first_vertex) = vertices.first() {

        let mut previous_vertex = first_vertex;

        for vertex in vertices.iter().skip(1) {
            line::between_two_points(
                ctx,
                &Pixel {
                    x: previous_vertex.x,
                    y: previous_vertex.y,
                    color,
                },
                vertex,
            );
            previous_vertex = vertex;
        }
        // Connect the last point to the first to complete the shape
        line::between_two_points(
            ctx,
            &Pixel {
                x: previous_vertex.x,
                y: previous_vertex.y,
                color,
            },
            first_vertex,
        );
    }
}
