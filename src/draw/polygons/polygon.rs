use std::f64::consts::PI;

use crate::draw::polygons::closed_perimeter;
use crate::graph1_core::context::GraphContext;
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
    if props.num_sides < 3 {
        return;
    }

    let num_sides = props.num_sides as f64;

    // This correction allows to render a polygon properly standing flat on its lower side
    let angular_correction = (num_sides - 2.0) * 180.0 / num_sides / 2.0;

    let angle_step = 2.0 * PI / num_sides; // Angle between each vertex
    let rotation_radians = (props.rotation_angle + angular_correction) * PI / 180.0; // Convert rotation angle to radians

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
    closed_perimeter::render(ctx, &vertices, props.center.color);
}
