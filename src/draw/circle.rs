use crate::draw::line;
use crate::tools::primitives::Pixel;


/// Tries to draw a circle
/// TODO: add more details to the RustDoc here
pub fn circle(buf_view: &mut [u32], center: &Pixel, radius: u32, skip_every:u32) {
    let radius_sq = radius.pow(2) as i32;

    let mut skip_every = skip_every;
    if skip_every==0 {
        skip_every = 1;
    }

    let mut lines_count = 0;

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

        // Adjust start_x for when it's negative.
        let adjusted_start_x = if start_x < 0 { 0 } else { start_x } as u32;

        // Provides the "80s" effect of  skipping every `skip_every` lines while drawing the circle
        let skip: bool = skip_every > 1 && lines_count % skip_every != 0;

        if !skip {
            // Draw the upper half of the circle.
            line::horizontal(buf_view, &Pixel{x:adjusted_start_x, y:center.y + delta_y, color:center.color}, length);
            // Draw the lower half of the circle, avoiding the central line being drawn twice.
            if delta_y > 0 {
                line::horizontal(buf_view, &Pixel{x:adjusted_start_x, y:center.y - delta_y, color:center.color}, length);
            }
        }
        lines_count += 1;
    }
}
