use crate::core::context::GraphContext;
use crate::draw::line;
use crate::draw::rectangle;
use crate::filters::image;
use crate::primitives::plane::{Dimensions2d, RectArea};
use crate::primitives::point::Point;

/// Calculates a point on a cubic Bezier curve
///
/// # Parameters
/// - `t`: Parameter along the curve, in the range [0.0, 1.0]
/// - `p0`: Starting point
/// - `p1`: First control point
/// - `p2`: Second control point
/// - `p3`: Ending point
///
/// # Returns
/// A `Point` on the curve corresponding to the parameter `t`
fn bezier_point(t: f32, p0: &Point, p1: &Point<i32>, p2: &Point<i32>, p3: &Point) -> Point {
    let t2 = t * t;
    let t3 = t2 * t;
    let mt = 1.0 - t;
    let mt2 = mt * mt;
    let mt3 = mt2 * mt;

    let x = mt3 * p0.x as f32
        + 3.0 * mt2 * t * p1.x as f32
        + 3.0 * mt * t2 * p2.x as f32
        + t3 * p3.x as f32;

    let y = mt3 * p0.y as f32
        + 3.0 * mt2 * t * p1.y as f32
        + 3.0 * mt * t2 * p2.y as f32
        + t3 * p3.y as f32;

    Point::new(x.round() as u32, y.round() as u32)
}

/// This function draws Bézier curves based on the provided control points.
/// ## Parameters
/// - `ctx`: Graph context to draw to
/// - `start_end_points`: The starting and ending points of the curve segments
/// - `control_points`: Each pair of these points controls the curviness of the segment
/// - `colors`: Colors for each segment of the curve. If there are more segments than colors, the last color is used for the remaining segments
/// - `resolution_delta`: The delta value for the resolution of the curve. The smaller the value, the smoother the curve. A value of `0.05` is a good starting point.
/// ## Important Notes
/// ### 1. Every segment of the curve is defined by 4 points:
///     1. The starting point
///     2. Control point #1
///     3. Control point #2
///     4. The ending point of this segment of the curve (which can also be the starting point of the next segment)
/// ### 2. The starting/ending points and control points are passed to the function as 2 separate slices:
/// `start_end_points` and `control_points`.
/// #### The first segment has the following layout:
///     1. The starting point: `start_end_points[0]`
///     2. Control point #1: `control_points[0]`
///     3. Control point #2: `control_points[1]`
///     4. The ending point: `start_end_points[1]`
///
/// #### The second segment has the following layout:
///     1. The starting point `start_end_points[1]`
///     2. Control point #1 `control_points[2]`
///     3. Control point #2 `control_points[3]`
///     4. The ending point: `start_end_points[2]`
///     Etc.
///
/// ### Performance
/// This function recalculates points dynamically and draws many lines, which can be computationally
/// expensive for very high resolutions or very complex paths. Adjust resolution_delta to balance
/// between performance and smoothness.
///
pub fn bezier<UserData>(
    ctx: &mut GraphContext<UserData>,
    start_end_points: &[Point],
    control_points: &[Point<i32>],
    colors: &[u32],
    resolution_delta: f32,
) {
    if !ctx.bezier.enabled {
        return;
    }

    let num_segments = start_end_points.len().saturating_sub(1);
    if control_points.len() != num_segments * 2 || num_segments < 1 {
        println!("Invalid input: need at least 2 start/end points and control point pairs");
        return;
    }

    for i in 0..num_segments {
        let color = *colors.get(i).unwrap_or_else(|| colors.last().unwrap());
        let p0 = &start_end_points[i];
        let p1 = &control_points[i * 2];
        let p2 = &control_points[i * 2 + 1];
        let p3 = &start_end_points[i + 1];

        let mut t = 0.0;
        let mut current_point = *p0;
        while t <= 1.0 {
            let next_point = bezier_point(t, p0, p1, p2, p3);
            line::between_two_points(
                ctx,
                &current_point.convert(),
                &next_point.convert(),
                Some(color),
            );
            current_point = next_point;
            t += resolution_delta;
        }
        line::between_two_points(ctx, &current_point.convert(), &p3.convert(), Some(color));
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

const DIMENSIONS: Dimensions2d = Dimensions2d { w: 4, h: 4 };

/// Renders a large point using a solid color at the specified coordinates
fn render_point_color<UserData>(ctx: &mut GraphContext<UserData>, p: &Point, color: u32) {
    if p.x < 2 || p.y < 2 || p.x >= ctx.win.dimensions.w - 2 || p.y >= ctx.win.dimensions.h - 2 {
        return;
    }

    rectangle::filled(
        ctx,
        &RectArea {
            top_left: Point::new(p.x - 2, p.y - 2),
            dimensions: DIMENSIONS,
            color: Some(color),
        },
    );
}

/// Renders a large point by inverting the pixel colors at the specified coordinates
fn render_point_inverted<UserData>(ctx: &mut GraphContext<UserData>, p: &Point) {
    if p.x < 2 || p.y < 2 || p.x >= ctx.win.dimensions.w - 2 || p.y >= ctx.win.dimensions.h - 2 {
        return;
    }

    image::transform_colors(
        &mut ctx.frame_buf,
        &ctx.win.dimensions,
        &RectArea {
            top_left: Point::new(p.x - 2, p.y - 2),
            dimensions: DIMENSIONS,
            color: None,
        },
        &image::ImageFilter::Invert,
    );
}

/// Renders a list of points either as solid colored points or inverted pixels, based on `is_color`
fn render_points<UserData>(
    ctx: &mut GraphContext<UserData>,
    points: &[Point],
    is_color: bool,
    color: u32,
) {
    for p in points {
        if is_color {
            render_point_color(ctx, p, color);
        } else {
            render_point_inverted(ctx, p);
        }
    }
}

/// Draws control points and start/end points for a Bezier curve.
///
/// # Parameters
/// - `ctx`: Mutable reference to drawing context
/// - `control_points`: Optional reference to control points
/// - `control_color`: Optional color override for control points
/// - `start_end_points`: Optional reference to start/end points
/// - `start_end_points_color`: Optional color override for start/end points
///
/// If color is not specified, points are drawn using pixel inversion for contrast.
/// Levers between control points are drawn if `ctx.bezier.render_levers` is true.
fn draw_controls<UserData>(
    ctx: &mut GraphContext<UserData>,
    control_points: Option<&[Point<i32>]>,
    control_color: Option<u32>,
    start_end_points: Option<&[Point]>,
    start_end_points_color: Option<u32>,
) {
    if let Some(points) = start_end_points {
        render_points(
            ctx,
            points,
            start_end_points_color.is_some(),
            start_end_points_color.unwrap_or(ctx.win.foreground_color),
        );
    }

    if let Some(points) = control_points {
        let normalized: Vec<Point> = points
            .iter()
            .map(|p| Point::new(i32::max(p.x, 0) as u32, i32::max(p.y, 0) as u32))
            .collect();

        if ctx.bezier.render_levers {
            for pair in normalized.windows(2).step_by(2) {
                if let [p1, p2] = pair {
                    line::between_two_points(
                        ctx,
                        &p1.convert(),
                        &p2.convert(),
                        Some(control_color.unwrap_or(ctx.win.foreground_color)),
                    );
                }
            }
        }
        render_points(
            ctx,
            &normalized,
            control_color.is_some(),
            control_color.unwrap_or(ctx.win.foreground_color),
        );
    }
}
