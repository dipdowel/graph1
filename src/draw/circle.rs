use crate::draw;
use crate::core::context::GraphContext;
use crate::primitives::Pixel;

/// Draws a filled circle with a specified center, radius, and color.
/// The circle is drawn using an optimized scanline-based circle-drawing algorithm.
///
/// # Parameters
/// - `ctx`: A mutable reference to the `GraphContext` which holds the drawing context.
/// - `center`: A reference to a `Pixel` that specifies the center of the circle and its color.
/// - `radius`: The radius of the circle in pixels.
/// - `skip_every`: The number of lines to skip while drawing. This provides a less dense, stylised circle.
pub fn filled<UserData>(ctx: &mut GraphContext<UserData>, center: &Pixel, radius: u32, skip_every: u32) {

    let radius_sq = radius.pow(2) as i32;

    let mut skip_every = skip_every;
    if skip_every == 0 {
        skip_every = 1;
    }

    let mut lines_count = 0;

    let mut pixel = Pixel {
        x: 0, // needs to be initialised with at least something
        y: 0, // needs to be initialised with at least something
        color: center.color,
    };

    for delta_y in 0..=radius {
        let delta_y_sq = (delta_y.pow(2)) as i32;

        // Calculate the horizontal displacement for the current slice of the circle
        // from the center to one side.
        let width_half = ((radius_sq - delta_y_sq) as f64).sqrt().round() as i32;

        // Calculate start and end points for the current line segment.
        let start_x = center.x as i32 - width_half;
        let end_x = center.x as i32 + width_half;

        // Calculate the length of the line segment. Ensure that we handle cases where start_x could be negative.
        let length = if start_x < 0 { end_x } else { end_x - start_x } as u32;

        // Provides the "80s" effect of  skipping every `skip_every` lines while drawing the circle
        let skip_line: bool = skip_every > 1 && lines_count % skip_every != 0;
        lines_count += 1;

        if skip_line {
            continue;
        }

        // Adjust start_x for when it's negative.
        pixel.x = if start_x < 0 { 0 } else { start_x } as u32;
        pixel.y = center.y + delta_y; // `y` for the upper half-circle

        // Draw the upper half of the circle.
        draw::line::horizontal(ctx, &pixel, length);

        if delta_y > 0 {
            // Draw the lower half of the circle, avoiding the central line being drawn twice.
            pixel.y = (center.y as i32 - delta_y as i32) as u32;
            // NB: due to typecasting `pixel.y` can become really large, but it's okay
            // NB: since `line::horizontal()` checks for `y` being greater than window's height
            draw::line::horizontal(ctx, &pixel, length);
        }
    }
}
