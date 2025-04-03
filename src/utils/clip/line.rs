use crate::primitives::plane::RectArea;
use crate::primitives::point::Point;

//
// -----------------------------------------------------------------------------
// Clipping Helper: Elastic Slide
// -----------------------------------------------------------------------------

/// "Elastic Slide" clipping method clamps both line endpoints to the boundary rectangle.
///
/// This method is *not* geometrically accurate — it doesn't preserve line intersection
/// points with the rectangle. Instead, it simply **pulls both endpoints inside** the area
/// by clamping them to the range defined by the rectangle's bounds.
///
/// This strategy is fast and forgiving, making it suitable for prototyping or for use cases
/// where strict correctness is not required. It can also be used to create a simple elastic
/// deformation effect (hence the name).
///
/// Returns:
/// - `Some((clamped_start, clamped_end))` if both endpoints can be clamped
/// - Always returns `Some` (never rejects lines), but output may be degenerate
pub fn to_area_elastic_slide(
    start: &Point<i32>,
    end: &Point<i32>,
    area: &RectArea<u32>,
) -> Option<(Point<u32>, Point<u32>)> {
    let x_min = area.top_left.x as i32;
    let y_min = area.top_left.y as i32;
    let x_max = x_min + area.dimensions.w as i32 - 1;
    let y_max = y_min + area.dimensions.h as i32 - 1;

    Some((
        Point::new(start.x.clamp(x_min, x_max), start.y.clamp(y_min, y_max)).convert(),
        Point::new(end.x.clamp(x_min, x_max), end.y.clamp(y_min, y_max)).convert(),
    ))
}

//
// -----------------------------------------------------------------------------
// Clipping Helper: Cohen–Sutherland
// -----------------------------------------------------------------------------

/// Cohen–Sutherland line clipping algorithm.
///
/// A classic region-code-based algorithm that splits the 2D space into 9 regions
/// using the rectangular bounds. Each point gets a 4-bit outcode representing
/// its position relative to the rectangle (LEFT, RIGHT, TOP, BOTTOM).
///
/// The algorithm iteratively shortens the line by replacing out-of-bounds endpoints
/// with their intersections with the rectangle until the line is fully inside
/// or trivially rejected.
///
/// Returns:
/// - `Some((clipped_start, clipped_end))` if line intersects with or lies within the area
/// - `None` if line is completely outside
pub fn to_area_cohen_sutherland(
    start: &Point<i32>,
    end: &Point<i32>,
    area: &RectArea<u32>,
) -> Option<(Point<u32>, Point<u32>)> {
    let x_min = area.top_left.x as i32;
    let y_min = area.top_left.y as i32;
    let x_max = x_min + area.dimensions.w as i32 - 1;
    let y_max = y_min + area.dimensions.h as i32 - 1;

    let mut x0 = start.x;
    let mut y0 = start.y;
    let mut x1 = end.x;
    let mut y1 = end.y;

    // Bitfield region encoding
    const INSIDE: u8 = 0;
    const LEFT: u8 = 1;
    const RIGHT: u8 = 2;
    const BOTTOM: u8 = 4;
    const TOP: u8 = 8;

    // Computes the region code for a point
    fn compute_code(x: i32, y: i32, x_min: i32, y_min: i32, x_max: i32, y_max: i32) -> u8 {
        let mut code = INSIDE;
        if x < x_min {
            code |= LEFT;
        } else if x > x_max {
            code |= RIGHT;
        }
        if y < y_min {
            code |= BOTTOM;
        } else if y > y_max {
            code |= TOP;
        }
        code
    }

    let mut code0 = compute_code(x0, y0, x_min, y_min, x_max, y_max);
    let mut code1 = compute_code(x1, y1, x_min, y_min, x_max, y_max);

    // Main loop: clip endpoints until fully inside or rejected
    while code0 != 0 || code1 != 0 {
        if (code0 & code1) != 0 {
            return None; // Trivially reject: both endpoints outside same region
        }

        let out = if code0 != 0 { code0 } else { code1 };
        let (x, y) = if out & TOP != 0 {
            let x = x0 + (x1 - x0) * (y_max - y0) / (y1 - y0);
            (x, y_max)
        } else if out & BOTTOM != 0 {
            let x = x0 + (x1 - x0) * (y_min - y0) / (y1 - y0);
            (x, y_min)
        } else if out & RIGHT != 0 {
            let y = y0 + (y1 - y0) * (x_max - x0) / (x1 - x0);
            (x_max, y)
        } else {
            let y = y0 + (y1 - y0) * (x_min - x0) / (x1 - x0);
            (x_min, y)
        };

        // Replace the clipped endpoint
        if out == code0 {
            x0 = x;
            y0 = y;
            code0 = compute_code(x0, y0, x_min, y_min, x_max, y_max);
        } else {
            x1 = x;
            y1 = y;
            code1 = compute_code(x1, y1, x_min, y_min, x_max, y_max);
        }
    }

    Some((Point::new(x0, y0).convert(), Point::new(x1, y1).convert()))
}

//
// -----------------------------------------------------------------------------
// Clipping Helper: Liang–Barsky
// -----------------------------------------------------------------------------

/// Liang–Barsky line clipping algorithm.
///
/// A mathematically optimal algorithm that uses parametric line equations and
/// inequality comparisons to trim a line segment against a rectangular clip region.
/// It avoids branching by treating all four boundaries (left/right/top/bottom) as
/// constraints on a parameter `t` in the range `[0, 1]`.
///
/// The algorithm efficiently computes intersection points using the line's delta
/// and rejects lines outside the region early.
///
/// Returns:
/// - `Some((clipped_start, clipped_end))` if part of the line intersects the area
/// - `None` if the line lies entirely outside
pub fn to_area_liang_barsky(
    start: &Point<i32>,
    end: &Point<i32>,
    area: &RectArea<u32>,
) -> Option<(Point<u32>, Point<u32>)> {
    // Convert area bounds to f32 for parametric calculations
    let x_min = area.top_left.x as f32;
    let y_min = area.top_left.y as f32;
    let x_max = x_min + area.dimensions.w as f32 - 1.0;
    let y_max = y_min + area.dimensions.h as f32 - 1.0;

    // Decompose line endpoints into floats
    let (x0, y0, x1, y1) = (start.x as f32, start.y as f32, end.x as f32, end.y as f32);

    // Direction vector of the line
    let dx = x1 - x0;
    let dy = y1 - y0;

    // Parametric bounds: start at full range [0, 1]
    let mut t0 = 0.0;
    let mut t1 = 1.0;

    /// Clip helper: restricts t0..t1 range using one edge of the rectangle
    ///
    /// For each rectangle boundary (left, right, bottom, top), we derive a constraint
    /// of the form: `p * t <= q`, and we update t0 and t1 to satisfy all such constraints.
    ///
    /// - If the line is parallel to a boundary (`p == 0`), we check whether it's outside (`q < 0`)
    /// - If the line is entering the boundary, we update `t0`
    /// - If the line is exiting the boundary, we update `t1`
    fn clip(p: f32, q: f32, t0: &mut f32, t1: &mut f32) -> bool {
        if p == 0.0 {
            // Line is parallel to this boundary — accept only if inside
            return q >= 0.0;
        }

        let r = q / p;

        if p < 0.0 {
            // Potentially entering this side — tighten lower bound
            if r > *t1 {
                return false; // Clipped entirely
            }
            if r > *t0 {
                *t0 = r;
            }
        } else {
            // Potentially exiting — tighten upper bound
            if r < *t0 {
                return false; // Clipped entirely
            }
            if r < *t1 {
                *t1 = r;
            }
        }
        true
    }

    // Apply clipping constraints for all 4 sides of the rectangle
    if clip(-dx, x0 - x_min, &mut t0, &mut t1)    // Left
        && clip(dx, x_max - x0, &mut t0, &mut t1) // Right
        && clip(-dy, y0 - y_min, &mut t0, &mut t1) // Bottom
        && clip(dy, y_max - y0, &mut t0, &mut t1)
    // Top
    {
        // Calculate new clipped endpoints using the [t0, t1] segment
        let clipped_start = Point::new((x0 + dx * t0) as i32, (y0 + dy * t0) as i32).convert();

        let clipped_end = Point::new((x0 + dx * t1) as i32, (y0 + dy * t1) as i32).convert();

        Some((clipped_start, clipped_end))
    } else {
        None // Rejected: line lies completely outside the clipping area
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::plane::RectArea;
    use crate::primitives::point::Point;

    #[test]
    fn elastic_slide_inside_line() {
        let area = RectArea::new(5, 5, 10, 10, None);
        let start = Point::new(6, 6);
        let end = Point::new(10, 10);

        let clipped = to_area_elastic_slide(&start, &end, &area).unwrap();
        assert_eq!(clipped.0, start.convert());
        assert_eq!(clipped.1, end.convert());
    }

    #[test]
    fn elastic_slide_crossing_line() {
        let area = RectArea::new(5, 5, 10, 10, None);
        let start = Point::new(0, 0);
        let end = Point::new(20, 20);

        let clipped = to_area_elastic_slide(&start, &end, &area).unwrap();
        assert_eq!(clipped.0, Point::new(5, 5));
        assert_eq!(clipped.1, Point::new(14, 14));
    }

    #[test]
    fn cohen_sutherland_partial_clip() {
        let area = RectArea::new(5, 5, 10, 10, None);
        let start = Point::new(0, 0);
        let end = Point::new(20, 20);

        let clipped = to_area_cohen_sutherland(&start, &end, &area).unwrap();
        assert_eq!(clipped.0, Point::new(5, 5));
        assert_eq!(clipped.1, Point::new(14, 14));
    }

    #[test]
    fn cohen_sutherland_reject_outside() {
        let area = RectArea::new(5, 5, 10, 10, None);
        let start = Point::new(0, 0);
        let end = Point::new(3, 3);

        assert!(to_area_cohen_sutherland(&start, &end, &area).is_none());
    }

    #[test]
    fn liang_barsky_partial_clip() {
        let area = RectArea::new(5, 5, 10, 10, None);
        let start = Point::new(0, 0);
        let end = Point::new(10, 10);

        let clipped = to_area_liang_barsky(&start, &end, &area).unwrap();
        assert_eq!(clipped.0, Point::new(5, 5));
        assert_eq!(clipped.1, Point::new(10, 10));
    }

    #[test]
    fn liang_barsky_fully_outside() {
        let area = RectArea::new(5, 5, 10, 10, None);
        let start = Point::new(-10, -10);
        let end = Point::new(-5, -5);

        assert!(to_area_liang_barsky(&start, &end, &area).is_none());
    }
}
