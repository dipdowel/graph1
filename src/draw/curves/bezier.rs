use crate::draw::line::between_two_points;
use crate::draw::rectangle_filled;
use crate::filters;
use crate::graph1_core::context::GraphContext;
use crate::primitives::primitives::{Dimensions2d, Pixel, Point, RectArea};

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
    if length < 4 {
        println!("draw_bezier_curve(): Not enough points to draw a curve!");
        return;
    }

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

const DIMENSIONS: Dimensions2d = Dimensions2d { w: 4, h: 4 };

/// Options for rendering Bezier curve control points
pub struct BezierControlShowOptions {
    /// Show the start and end points of each curve segment
    pub show_start_end_points: bool,
    /// Color of the control points, if `None`, the rendered points will be of inverted color of the background (hence always visible)
    pub color: Option<u32>,
}


/// Renders a big visual point of a given color
fn render_point_color(ctx: &mut GraphContext, p: &Point, color: u32) {
    rectangle_filled(
        ctx,
        &RectArea {
            top_left: Point {
                x: p.x - 2,
                y: p.y - 2,
            },
            dimensions: DIMENSIONS,
        },
        color,
    );
}

/// Renders a big visual point by inverting the background
fn render_point_inverted(ctx: &mut GraphContext, p: &Point) {
    filters::image::transform_colors(
        &mut ctx.buf_view,
        &ctx.win.dimensions,
        &RectArea {
            top_left: Point {
                x: p.x - 2,
                y: p.y - 2,
            },
            dimensions: DIMENSIONS,
        },
        &filters::image::ImageFilter::Invert,
    );
}

/// Renders control points for a Bezier curve, helps with visual debugging of the curve
/// ## Important Notes
/// - `points` must be exactly the same as provided to `draw_bezier_curve()`
/// ## Parameters
/// - `ctx`: Graph context to draw to
/// - `points`: The points that define the Bezier curve
/// - `options`: Options for rendering the control points

pub fn draw_bezier_curve_controls(
    ctx: &mut GraphContext,
    points: &[Point],
    options: Option<&BezierControlShowOptions>,
) {
    let default_options = BezierControlShowOptions {
        show_start_end_points: true,
        color: Some(ctx.default_color),
    };

    // let options: &BezierControlShowOptions = match options {
    //     Some(opts) => opts,
    //     None => &default_options,
    // };
    let options: &BezierControlShowOptions = options.unwrap_or_else(|| &default_options);

    let &BezierControlShowOptions {
        show_start_end_points,
        color,
    } = options;

    let is_color = color.is_some();
    let color = color.unwrap_or(ctx.default_color);

    // index of the point in the loop
    let mut point_index: isize = -1;

    for p in points {
        point_index += 1;

        // skip the start/end points
        if !show_start_end_points && point_index % 3 == 0 {
            continue;
        }

        if is_color {
            render_point_color(ctx, p, color);
        } else {
            render_point_inverted(ctx, p);
        }
    }
}
