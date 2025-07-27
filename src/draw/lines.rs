use crate::core::context::GraphContext;
use crate::draw::line;
use crate::primitives::point::Point;

/// Draws straight line segments between consecutive points
///
/// This function supports negative coordinates and off-screen vertices,
/// and uses the context's line clipping strategy to crop segments to the screen.
///
/// # Parameters
/// * `ctx` - A mutable reference to the `GraphContext`
/// * `vertices` - A vector of signed 2D points (can contain negative/off-screen coordinates)
/// * `color` - An optional RGBA color; defaults to `ctx.win.foreground_color` if `None`
pub fn between_points<UserData>(
    ctx: &mut GraphContext<UserData>,
    vertices: &[Point<i32>],
    color: Option<u32>,
) {
    if let Some(first_vertex) = vertices.first() {
        let mut previous_vertex = first_vertex;

        for vertex in vertices.iter().skip(1) {
            line::between_two_points(ctx, previous_vertex, vertex, color);
            previous_vertex = vertex;
        }
    }
}
