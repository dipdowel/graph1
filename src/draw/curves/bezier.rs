use crate::draw::line::between_two_points;
use crate::graph1_core::context::GraphContext;
use crate::primitives::primitives::{Pixel, Point};

fn bezier_point(t: &f32, p0: &Point, p1: &Point, p2: &Point, p3: &Point) -> Point {
    let x = (1.0 - t).powi(3) * p0.x as f32
        + 3.0 * (1.0 - t).powi(2) * t * p1.x as f32
        + 3.0 * (1.0 - t) * t.powi(2) * p2.x as f32
        + t.powi(3) * p3.x as f32;

    let y = (1.0 - t).powi(3) * p0.y as f32
        + 3.0 * (1.0 - t).powi(2) * t * p1.y as f32
        + 3.0 * (1.0 - t) * t.powi(2) * p2.y as f32
        + t.powi(3) * p3.y as f32;

    Point {
        x: x.round() as u32,
        y: y.round() as u32,
    }
}

/// This function should  handle an arbitrary long vector of points to draw complex and continuous Bezier curves.
/// ## Important Notes
/// ### Point Organization
/// - The first and last points are always end points of the curve.
/// - Every set of 4 points defines 1 cubic Bezier curve:
///     - a start point
///     - two control points
///     - an end point
///
/// This function requires the provided point array to be organized correctly.
/// There should be one starting point and multiple groups of three additional points:
/// 1. control point
/// 2. control point
/// 3. end/start of next curve
/// ### Vector Length
/// The vector length should be `3n+1`, where `n` is the number of curves.
/// ### Performance
/// This function recalculates points dynamically and draws many lines, which can be computationally
/// expensive for very high resolutions or very complex paths. Adjust resolution_delta to balance
/// between performance and smoothness.
pub fn draw_bezier_curve(ctx: &mut GraphContext, points: &[Point], resolution_delta: &f32) {
    let length = points.len();

    for i in (0..length - 3).step_by(3) {
        let p0 = &points[i];
        let p1 = &points[i + 1];
        let p2 = &points[i + 2];
        let p3 = &points[i + 3];

        // FIXME: Come up with a way to pass the color value(s) to this function!
        let color = ctx.default_color;

        let mut t = 0.0;
        let mut current_point = *p0;

        while t <= 1.0 {
            let next_point = bezier_point(&t, &p0, &p1, &p2, &p3);
            between_two_points(
                ctx,
                &Pixel {
                    x: current_point.x,
                    y: current_point.y,
                    color,
                },
                &next_point,
            );
            current_point = next_point;
            t += resolution_delta; // This delta determines the resolution of the curve
        }

        // Draw the final segment to the last control point
        between_two_points(
            ctx,
            &Pixel {
                x: current_point.x,
                y: current_point.y,
                color,
            },
            &p3,
        );
    }
}
