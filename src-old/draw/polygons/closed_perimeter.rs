use crate::draw::line::between_two_points;
use crate::core::context::GraphContext;
use crate::primitives::primitives::{Pixel, Point};

/// Draws straight lines between consecutive points to form a closed path
///
/// # Parameters
///
/// * `ctx` - A mutable reference to the `GraphContext`
/// * `vertices` - Dots to connect
/// * `color` - A color as `0rgb`
///
pub fn render(ctx: &mut GraphContext, vertices: &Vec<Point>, color: u32) {

    if let Some(first_vertex) = vertices.first() {
        let mut previous_vertex = first_vertex;
        for vertex in vertices.iter().skip(1) {
            between_two_points(
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
        between_two_points(
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
