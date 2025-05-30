use crate::primitives::numeric::Numeric;
use crate::primitives::point::Point;

/// Computes an interior point (approximate centroid) of a 4-gon, built from the first 4 points of a vector.
///
/// # Parameters
/// - `points`: A vector reference containing at least 4 `Point<T>` elements
///
/// # Returns
/// - `Some(Point<T>)` -- the approximate centroid
/// - `None` -- if `points` contains fewer than 4 elements
pub fn approximate_center<T: Numeric>(points: &Vec<Point<T>>) -> Option<Point<T>> {
    if points.len() < 4 {
        return None;
    }

    let (sum_x, sum_y) = points.iter().take(4).fold(
        (T::zero(), T::zero()),
        |(sx, sy), p| (sx + p.x, sy + p.y),
    );

    Some(Point {
        x: sum_x / T::from_u32(4),
        y: sum_y / T::from_u32(4),
    })
}