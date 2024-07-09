

use crate::draw::line::between_two_points;
use crate::graph1_core::context::GraphContext;
use std::f64::consts::PI;
use crate::primitives::primitives::{Pixel, Point};

pub struct PolygonProperties {
    /// Location of the central point of the polygon
    pub center: Pixel,

    /// Number of sides the polygon has
    pub num_sides: u32,

    /// Distance from the `center` to each vertex
    pub radius: u32,

    /// Rotation angle in degrees
    pub rotation_angle: f64,
}

/// Function to draw a polygon based on provided properties
/// The lowest value of `PolygonProperties -> num_sides` is 3.
pub fn render(ctx: &mut GraphContext, props: &PolygonProperties) {

    // Do nothing if it's not even a triangle
    if (props.num_sides<3){
        return;
    }

    let angle_step = 2.0 * PI / props.num_sides as f64; // Angle between each vertex
    let rotation_radians = props.rotation_angle as f64 * PI / 180.0; // Convert rotation angle to radians

    // Calculating all vertex positions
    let mut vertices = Vec::new();
    for i in 0..props.num_sides {
        let angle = i as f64 * angle_step + rotation_radians; // Current angle adjusted for rotation

        // Calculate vertex position
        let x = props.center.x as f64 + props.radius as f64 * angle.cos();
        let y = props.center.y as f64 + props.radius as f64 * angle.sin();
        vertices.push(Point {
            x: x as u32,
            y: y as u32,
        });
    }

    // Draw lines between consecutive vertices
    if let Some(first_vertex) = vertices.first() {
        let mut previous_vertex = first_vertex;
        for vertex in vertices.iter().skip(1) {
            between_two_points(
                ctx,
                &Pixel {
                    x: previous_vertex.x,
                    y: previous_vertex.y,
                    color: props.center.color,
                },
                vertex,
            );
            previous_vertex = vertex;
        }
        // Connect the last vertex to the first to complete the polygon
        between_two_points(
            ctx,
            &Pixel {
                x: previous_vertex.x,
                y: previous_vertex.y,
                color: props.center.color,
            },
            first_vertex,
        );
    }
}
