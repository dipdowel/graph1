use crate::primitives::plane::RectArea;
use crate::primitives::numeric::Numeric;

/// Resolves a list of rectangles by removing those fully occluded by later rectangles
/// and merging adjacent ones of the same color.
///
/// # Parameters
/// - `rects`: A vector of rectangles ordered by Z-index (back to front).
///
/// # Returns
/// - A simplified vector with occluded rectangles removed and adjacent merges applied.
pub fn resolve_rectangles<T: Numeric + std::ops::Add<Output = T> + PartialEq>(rects: Vec<RectArea<T>>) -> Vec<RectArea<T>> {
    let mut visible: Vec<RectArea<T>> = Vec::new();

    // Step 1: Occlusion Culling
    for rect in rects.into_iter().rev() {
        let is_occluded = visible.iter().any(|v| v.contains(&rect));
        if !is_occluded {
            visible.push(rect);
        }
    }
    visible.reverse();

    // Step 2: Merging Adjacent Rectangles
    let mut merged = Vec::new();
    let mut skip = vec![false; visible.len()];

    for i in 0..visible.len() {
        if skip[i] {
            continue;
        }

        let mut current = visible[i];
        for j in (i + 1)..visible.len() {
            if skip[j] {
                continue;
            }

            let other = visible[j];
            let same_color = current.color == other.color;

            // Horizontal merge
            let same_row = current.top_left.y == other.top_left.y;
            let same_height = current.dimensions.h == other.dimensions.h;
            let adjacent_x = current.top_left.x + current.dimensions.w == other.top_left.x
                || other.top_left.x + other.dimensions.w == current.top_left.x;

            if same_color && same_row && same_height && adjacent_x {
                let (left, right) = if current.top_left.x < other.top_left.x {
                    (current, other)
                } else {
                    (other, current)
                };
                current = RectArea::new(
                    left.top_left.x,
                    left.top_left.y,
                    left.dimensions.w + right.dimensions.w,
                    left.dimensions.h,
                    left.color,
                );
                skip[j] = true;
                continue;
            }

            // Vertical merge
            let same_col = current.top_left.x == other.top_left.x;
            let same_width = current.dimensions.w == other.dimensions.w;
            let adjacent_y = current.top_left.y + current.dimensions.h == other.top_left.y
                || other.top_left.y + other.dimensions.h == current.top_left.y;

            if same_color && same_col && same_width && adjacent_y {
                let (top, bottom) = if current.top_left.y < other.top_left.y {
                    (current, other)
                } else {
                    (other, current)
                };
                current = RectArea::new(
                    top.top_left.x,
                    top.top_left.y,
                    top.dimensions.w,
                    top.dimensions.h + bottom.dimensions.h,
                    top.color,
                );
                skip[j] = true;
            }
        }

        merged.push(current);
    }

    merged
}
