use std::f64::consts::PI;
use crate::core::context::GraphContext;
use crate::draw::polygons::closed_perimeter;
use crate::primitives::Pixel;
use crate::primitives::point::Point;

#[derive(Debug, Clone, Copy)]
pub struct PolygonProperties {
    /// Location and color of the central point of the polygon
    pub center: Pixel,

    /// Number of sides the polygon has
    pub num_sides: u32,

    /// Distance from the `center` to each vertex
    pub radius: u32,

    /// Rotation angle in degrees
    pub rotation_angle: f64,

    /// If `true`, the polygon will not be rendered to the buffer, only the vertices will be returned
    pub skip_rendering: bool,
}

/// Renders a polygon with the specified properties into a given `GraphContext`.
/// The min allowed value of `PolygonProperties -> num_sides` is 3.
/// # Parameters
/// * `ctx` - A mutable reference to the `GraphContext`
/// * `props` - Properties of the polygon to render
/// # Returns
/// A vector of `Point`s representing the vertices of the polygon.
pub fn polygon<UserData>(ctx: &mut GraphContext<UserData>, props: &PolygonProperties) -> Vec<Point<i32>> {
    // Do nothing if it's not at least a triangle
    if props.num_sides < 3 {
        return Vec::new();
    }

    let num_sides = props.num_sides as f64;

    // This correction allows to render a polygon properly standing flat on its lower side
    let angular_correction = (num_sides - 2.0) * 180.0 / num_sides / 2.0;

    let angle_step = 2.0 * PI / num_sides; // Angle between each vertex
    let rotation_radians = (props.rotation_angle + angular_correction) * PI / 180.0; // Convert rotation angle to radians

    // ************************************************************************
    // Calculate all the vertex positions
    // ************************************************************************
    let num_sides_usize = num_sides as usize;

    let mut vertices:Vec<Point<i32>> = Vec::with_capacity(num_sides_usize);
    vertices.resize(num_sides_usize, Point { x: 0, y: 0 });


    for i in 0..num_sides_usize {
        let angle = i as f64 * angle_step + rotation_radians; // Current angle adjusted for rotation

        // Calculate vertex position
        vertices[i].x = (props.center.x as f64 + props.radius as f64 * angle.cos()) as i32;
        vertices[i].y = (props.center.y as f64 + props.radius as f64 * angle.sin()) as i32;
    }

    if !props.skip_rendering {
        // Draw lines between consecutive vertices
        closed_perimeter::closed_perimeter(ctx, &vertices, Some(props.center.color));
    }

    vertices
}
