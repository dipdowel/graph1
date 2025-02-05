use std::f64::consts::PI;

use crate::draw::polygons::closed_perimeter;
use crate::core::context::GraphContext;
use crate::primitives::primitives::{Pixel, Point};

#[derive(Debug, Clone, Copy)]
pub struct StarProperties {
    /// Location of the central point of the star
    pub center: Pixel,

    /// How many rays the star has
    pub num_rays: u32,

    /// Distance from the `center` after which every N+1th  vertex of the star lies
    pub inner_radius: u32,

    /// Distance from the `center` after which every N+2th  vertex of the star lies
    pub outer_radius: u32,

    /// Rotation angle in degrees
    pub rotation_angle: f64,

    /// If `true`, the star will not be rendered to the buffer, only the vertices will be returned
    pub skip_rendering: bool,
}


/// Renders a star with the specified properties into a given `GraphContext`
/// The min allowed value of `StarProperties -> num_rays` is 2.
/// # Parameters
/// * `ctx` - A mutable reference to the `GraphContext`
/// * `props` - Properties of the star to render
/// # Returns
/// A vector of `Point`s representing the vertices of the star.
pub fn render(ctx: &mut GraphContext, props: &StarProperties) -> Vec<Point> {
    // Too few rays, won't really render anything nice
    if props.num_rays < 2 {
        return Vec::new();
    }

    let num_vertices:usize = (props.num_rays * 2) as usize;
    let num_sides = props.num_rays as f64;

    // This correction allows to render the star properly standing flat on its lower side
    let angular_correction = (num_sides - 2.0) * 180.0 / num_sides / 2.0;

    let angle_step = 2.0 * PI / num_vertices as f64; // Angle between each vertex
    let rotation_radians = (props.rotation_angle - angular_correction) * PI / 180.0; // Convert rotation angle to radians

    // ************************************************************************
    // Calculate all the vertex positions
    // ************************************************************************
    let mut vertices = Vec::with_capacity(num_vertices);
    vertices.resize(num_vertices, Point { x: 0, y: 0 });

    for i in 0..num_vertices  {
        let radius = if i % 2 == 0 {
            props.inner_radius
        } else {
            props.outer_radius
        };

        let angle = i as f64 * angle_step + rotation_radians; // Current angle adjusted for rotation

        // Calculate vertex position and write directly to the vector (let's skip some extra local vars)
        vertices[i].x = (props.center.x as f64 + radius as f64 * angle.cos()) as u32;
        vertices[i].y = (props.center.y as f64 + radius as f64 * angle.sin()) as u32;
    }

    if !props.skip_rendering {
        // Draw lines between consecutive vertices
        closed_perimeter::render(ctx, &vertices, props.center.color);
    }

    return vertices;
}
