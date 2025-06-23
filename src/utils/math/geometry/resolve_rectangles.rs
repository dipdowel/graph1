use crate::primitives::plane::RectArea;
use crate::primitives::numeric::Numeric;

/// Resolves a list of rectangles by removing those fully occluded by later rectangles.
///
/// # Parameters
/// - `rects`: A vector of rectangles ordered by Z-index (back to front).
///
/// # Returns
/// - A simplified vector with occluded rectangles removed.
pub fn resolve_rectangles<T: Numeric + std::ops::Add<Output = T>>(rects: Vec<RectArea<T>>) -> Vec<RectArea<T>> {
    let mut visible: Vec<RectArea<T>> = Vec::new();

    // Iterate in reverse: higher Z-index comes later
    for rect in rects.into_iter().rev() {
        // Check if rect is completely covered by any visible rectangle
        let is_occluded = visible.iter().any(|v| v.contains(&rect));

        if !is_occluded {
            visible.push(rect);
        }
    }

    // Restore front-to-back Z-index order
    visible.reverse();
    visible
}
