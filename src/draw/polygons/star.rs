use std::f64::consts::PI;
use crate::graph_core::context::GraphContext;
use crate::draw::line::between_two_points;
use crate::tools::primitives::{Pixel, Point, StarProperties};

/// Function to draw a polygon based on provided properties
// pub fn star(buf_view: &mut [u32],props: &StarProperties) {
pub fn star(ctx: &mut GraphContext, props: &StarProperties) {

    let angle_step = 2.0 * PI / props.num_vertices as f64;  // Angle between each vertex
    let rotation_radians = props.rotation_angle as f64 * PI / 180.0;  // Convert rotation angle to radians

    // Calculating all vertex positions
    let mut vertices = Vec::new();
    for i in 0..props.num_vertices {
        let radius = if i % 2 == 0 { props.inner_radius } else { props.outer_radius };

        let angle = i as f64 * angle_step + rotation_radians;  // Current angle adjusted for rotation


        // Calculate vertex position
        let x = props.center.x as f64 + radius as f64 * angle.cos();
        let y = props.center.y as f64 + radius as f64 * angle.sin();
        vertices.push(Point { x: x as u32, y: y as u32 });
    }

    // Draw lines between consecutive vertices
    if let Some(first_vertex) = vertices.first() {
        let mut previous_vertex = first_vertex;
        for vertex in vertices.iter().skip(1) {
            between_two_points(ctx, &Pixel{x:previous_vertex.x, y:previous_vertex.y, color:props.center.color}, vertex);
            previous_vertex = vertex;
        }
        // Connect the last vertex to the first to complete the polygon
        between_two_points(ctx, &Pixel{x:previous_vertex.x, y:previous_vertex.y, color:props.center.color}, first_vertex);

    }
}

