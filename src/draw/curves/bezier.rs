use crate::draw::line::between_two_points;
use crate::draw::rectangle_filled;
use crate::graph1_core::context::GraphContext;
use crate::primitives::primitives::{Dimensions2d, Pixel, Point, PointI32, RectArea};
use crate::{draw, filters};

fn bezier_point(t: &f32, p0: &Point, p1: &PointI32, p2: &PointI32, p3: &Point) -> Point {
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

    // let result = Point {
    //     x: x.round() as u32,
    //     y: y.round() as u32,
    // };
    // println!(">>> bezier_point: {:?}", result);
    // result
}

/// This function draws Bezier curves based on the provided control points.
/// ## Important Notes
/// ### Organization of the vector of points
/// - The first and last points are always end points of the resulting curve.
/// - Every 4 points define 1 cubic Bezier curve:
///     - [0] The starting point
///     - [1] Control point #1
///     - [2] Control point #2
///     - [3] The ending point of this segment of the curve / the starting point of the next segment

/// ### Vector Length
/// The vector length should be `3n+1`, where `n` is the number of curves.
/// ### Performance
/// This function recalculates points dynamically and draws many lines, which can be computationally
/// expensive for very high resolutions or very complex paths. Adjust resolution_delta to balance
/// between performance and smoothness.
///
/// ## Parameters
/// - `ctx`: Graph context to draw to
/// - `points`: The points that define the Bezier curve (see above for organization)
/// - `colors`: Colors for each segment of the curve. If there are more segments than colors, the last color is used for the remaining segments
/// - `resolution_delta`: The delta value for the resolution of the curve. The smaller the value, the smoother the curve. A value of `0.05` is a good starting point.
///

// FIXME: Update the documentation to reflect the new organization of the points!!!!!
// FIXME: Update the documentation to reflect the new organization of the points!!!!!
// FIXME: Update the documentation to reflect the new organization of the points!!!!!
// FIXME: Update the documentation to reflect the new organization of the points!!!!!
// FIXME: Update the documentation to reflect the new organization of the points!!!!!
// FIXME: Update the documentation to reflect the new organization of the points!!!!!
// FIXME: Update the documentation to reflect the new organization of the points!!!!!
pub fn draw_bezier_curve(
    ctx: &mut GraphContext,
    start_end_points: &[Point],
    control_points: &[PointI32],
    colors: &[u32],
    resolution_delta: &f32,
) {
    //**********************************************************************************************
    //*** [ start-end points and control points validation ] ***************************************
    //**********************************************************************************************

    let start_end_len = start_end_points.len();
    let control_len = control_points.len();

    let rule_1 = start_end_len + control_len >= 4;
    let rule_2 =  control_len == (start_end_len-1)*2;

    if !rule_1 || !rule_2 {
        println!(
            "draw_bezier_curve():\n\t1. Expecting minimum 2 start-end points and 2 control points.\n\t\
            2. `control_len` must be equal to `(start_end_len-1) * 2`\n\t\
            Provided: start_end_len: {}, control_len: {}",start_end_len, control_len
        );
        return;
    }

    //**********************************************************************************************

    if colors.len() < 1 {
        println!(
            "draw_bezier_curve(): Expecting at least 1 color. Provided: {}",
            colors.len()
        );
        return;
    }

    let mut color: u32;
    let mut start_end_index: usize = 0;
    let max_color_index = colors.len() - 1;

    for i in (0..control_points.len() - 1).step_by(2) {
        // TODO: check if the `min` safeguard actually works here
        color = colors[usize::min(start_end_index, max_color_index)];

        let p0: &Point = &start_end_points[start_end_index];
        let p1: &PointI32 = &control_points[i];
        let p2: &PointI32 = &control_points[i + 1];
        let p3: &Point = &start_end_points[start_end_index + 1];
        start_end_index += 1;

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

    if ctx.bezier.render_controls {
        draw_controls(
            ctx,
            Some(control_points),
            ctx.bezier.control_color,
            Some(start_end_points),
            ctx.bezier.start_end_points_color,
        );
    }
}

//
//==================================================================================================
//=== [Code below is for visual debugging of the curves] ===========================================
//==================================================================================================

const DIMENSIONS: Dimensions2d = Dimensions2d { w: 4, h: 4 };

/// Renders a big visual point of a given color
fn render_point_color(ctx: &mut GraphContext, p: &Point, color: u32) {
    // Don't try to draw off-screen
    if p.x < 2 || p.y < 2 || p.x >= ctx.win.dimensions.w - 2 || p.y >= ctx.win.dimensions.h - 2 {
        return;
    };

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
    // Don't try to draw off-screen
    if p.x < 2 || p.y < 2 || p.x >= ctx.win.dimensions.w - 2 || p.y >= ctx.win.dimensions.h - 2 {
        return;
    };

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
/// Renders a slice points as big points on the screen
fn render_points(ctx: &mut GraphContext, points: &[Point], is_color: bool, color: u32) {
    if is_color {
        for p in points {
            render_point_color(ctx, p, color);
        }
    } else {
        for p in points {
            render_point_inverted(ctx, p);
        }
    }
}

/// Renderts controls points and start-end points of a Bezier curve, helps with visual debugging of the curve.
/// ## Parameters
/// - `ctx`: Graph context to draw to
/// - `control_points`: The control points of the Bezier curve
/// - `control_color`: The color of the control points. If `None`, the background color at the point is inverted
/// - `start_end_points`: The start-end points of the Bezier curve
/// - `start_end_points_color`: The color of the start-end points. If `None`, the background color at the point is inverted

fn draw_controls(
    ctx: &mut GraphContext,
    control_points: Option<&[PointI32]>,
    control_color: Option<u32>,
    start_end_points: Option<&[Point]>,
    start_end_points_color: Option<u32>,
) {
    // for i in (0..control_points.len() - 1).step_by(2) {
    // for points in control_points

    // Render start-end points
    if start_end_points.is_some() {
        let is_color = start_end_points_color.is_some();
        render_points(
            ctx,
            start_end_points.unwrap(),
            is_color,
            start_end_points_color.unwrap_or(ctx.default_color),
        );
    }

    // Convert and render control points
    if control_points.is_some() {
        let control_points = control_points.unwrap();
        let is_color = control_color.is_some();
        let control_color: u32 = control_color.unwrap_or(ctx.default_color);

        if ctx.bezier.render_levers {
        for i in (0..control_points.len() - 1).step_by(2) {
            draw::line::between_two_points(
                ctx,
                &Pixel {
                    x: control_points[i].x as u32,
                    y: control_points[i].y as u32,
                    color: control_color,
                },
                &Point {
                    x: control_points[i + 1].x as u32,
                    y: control_points[i + 1].y as u32,
                },
            );
        }
        }

        let mut points: Vec<Point> = Vec::new();

        for control_point in control_points {
            if control_point.x > -1 && control_point.y > -1 {
                &points.push(Point {
                    x: control_point.x as u32,
                    y: control_point.y as u32,
                });
            }
        }

        render_points(ctx, &points, is_color, control_color);
    }
}

/*
/// Renders control points and start-end points of a Bezier curve with default colors
/// ## Parameters
/// - `ctx`: Graph context to draw to
/// - `control_points`: The control points of the Bezier curve
/// - `start_end_points`: The start-end points of the Bezier curve
/// @See `draw_bezier_curve_controls()`
fn draw_controls_red_blue(
    ctx: &mut GraphContext,
    control_points: Option<&[PointI32]>,
    start_end_points: Option<&[Point]>,
) {
    draw_controls(
        ctx,
        control_points,
        Some(0x00_00_33_ff),
        start_end_points,
        Some(0x00_ff_33_00),
    );
}

/// Renders control points and start-end points of a Bezier curve inverting the background color at points
/// ## Parameters
/// - `ctx`: Graph context to draw to
/// - `control_points`: The control points of the Bezier curve
/// - `start_end_points`: The start-end points of the Bezier curve
/// @See `draw_bezier_curve_controls()`
fn draw_controls_invert(
    ctx: &mut GraphContext,
    control_points: Option<&[PointI32]>,
    start_end_points: Option<&[Point]>,
) {
    draw_controls(ctx, control_points, None, start_end_points, None);
}
*/