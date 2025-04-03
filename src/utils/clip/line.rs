
// -----------------------------------------------------------------------------
// Clipping Helper: Elastic Slide
// -----------------------------------------------------------------------------

use crate::primitives::plane::RectArea;
use crate::primitives::point::Point;

/// "Elastic slide" clips both endpoints by clamping them to the rectangle.
/// Not geometrically accurate, but simple and forgiving.
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
        Point::new(
            start.x.clamp(x_min, x_max),
            start.y.clamp(y_min, y_max),
        )
            .convert(),
        Point::new(end.x.clamp(x_min, x_max), end.y.clamp(y_min, y_max)).convert(),
    ))
}

// -----------------------------------------------------------------------------
// Clipping Helper: Cohen–Sutherland
// -----------------------------------------------------------------------------

/// Region-code-based line clipping algorithm.
/// Efficient for binary inclusion/exclusion of regions.
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

    const INSIDE: u8 = 0;
    const LEFT: u8 = 1;
    const RIGHT: u8 = 2;
    const BOTTOM: u8 = 4;
    const TOP: u8 = 8;

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

    while code0 != 0 || code1 != 0 {
        if (code0 & code1) != 0 {
            return None; // Trivially reject
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

// -----------------------------------------------------------------------------
// Clipping Helper: Liang–Barsky
// -----------------------------------------------------------------------------

/// Liang–Barsky: Efficient, math-based parametric clipping.
/// Uses inequalities to trim line segment against all four sides.
pub fn to_area_liang_barsky(
    start: &Point<i32>,
    end: &Point<i32>,
    area: &RectArea<u32>,
) -> Option<(Point<u32>, Point<u32>)> {
    let x_min = area.top_left.x as f32;
    let y_min = area.top_left.y as f32;
    let x_max = x_min + area.dimensions.w as f32 - 1.0;
    let y_max = y_min + area.dimensions.h as f32 - 1.0;

    let (x0, y0, x1, y1) = (
        start.x as f32,
        start.y as f32,
        end.x as f32,
        end.y as f32,
    );

    let dx = x1 - x0;
    let dy = y1 - y0;

    let mut t0 = 0.0;
    let mut t1 = 1.0;

    fn clip(p: f32, q: f32, t0: &mut f32, t1: &mut f32) -> bool {
        if p == 0.0 {
            return q >= 0.0;
        }
        let r = q / p;
        if p < 0.0 {
            if r > *t1 {
                return false;
            }
            if r > *t0 {
                *t0 = r;
            }
        } else {
            if r < *t0 {
                return false;
            }
            if r < *t1 {
                *t1 = r;
            }
        }
        true
    }

    if clip(-dx, x0 - x_min, &mut t0, &mut t1)
        && clip(dx, x_max - x0, &mut t0, &mut t1)
        && clip(-dy, y0 - y_min, &mut t0, &mut t1)
        && clip(dy, y_max - y0, &mut t0, &mut t1)
    {
        Some((
            Point::new((x0 + dx * t0) as i32, (y0 + dy * t0) as i32).convert(),
            Point::new((x0 + dx * t1) as i32, (y0 + dy * t1) as i32).convert(),
        ))
    } else {
        None
    }
}
