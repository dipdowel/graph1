// use crate::primitives::numeric::Numeric;
// use crate::primitives::plane::RectArea;

/*
/// Subtracts `cut` from `rect`, returning the visible regions of `rect` as smaller rectangles.
fn subtract_rect<T: Numeric + std::ops::Add<Output = T> + PartialEq>(
    rect: &RectArea<T>,
    cut: &RectArea<T>,
) -> Vec<RectArea<T>> {

    panic!("subtract_rect() NEEDS TO BE TESTED BEFORE USE!");


    let mut result = Vec::new();

    let x1 = rect.top_left.x;
    let y1 = rect.top_left.y;
    let x2 = x1 + rect.dimensions.w;
    let y2 = y1 + rect.dimensions.h;

    let cx1 = cut.top_left.x.maxi(x1);
    let cy1 = cut.top_left.y.maxi(y1);
    let cx2 = (cut.top_left.x + cut.dimensions.w).mini(x2);
    let cy2 = (cut.top_left.y + cut.dimensions.h).mini(y2);

    // No overlap
    if cx1 >= cx2 || cy1 >= cy2 {
        result.push(*rect);
        return result;
    }

    // Top strip
    if y1 < cy1 {
        result.push(RectArea::new(x1, y1, rect.dimensions.w, cy1 - y1, rect.color));
    }
    // Bottom strip
    if cy2 < y2 {
        result.push(RectArea::new(x1, cy2, rect.dimensions.w, y2 - cy2, rect.color));
    }
    // Left strip
    if x1 < cx1 {
        result.push(RectArea::new(x1, cy1, cx1 - x1, cy2 - cy1, rect.color));
    }
    // Right strip
    if cx2 < x2 {
        result.push(RectArea::new(cx2, cy1, x2 - cx2, cy2 - cy1, rect.color));
    }

    result


}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::plane::RectArea;

    #[test]
    fn test_subtract_no_overlap() {
        let r1 = RectArea::new(10, 10, 20, 20, Some(0x12345678));
        let r2 = RectArea::new(40, 40, 10, 10, Some(0x12345678));
        let result = subtract_rect(&r1, &r2);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], r1);
    }

    #[test]
    fn test_subtract_full_overlap() {
        let r1 = RectArea::new(10, 10, 20, 20, Some(0x12345678));
        let r2 = RectArea::new(10, 10, 20, 20, Some(0x99999999));
        let result = subtract_rect(&r1, &r2);
        assert!(result.is_empty());
    }

    #[test]
    fn test_subtract_partial_overlap_top() {
        let r1 = RectArea::new(10, 10, 20, 20, Some(0xFF));
        let r2 = RectArea::new(15, 5, 10, 10, Some(0xFF));
        let result = subtract_rect(&r1, &r2);
        assert!(result.iter().all(|r| r.color == Some(0xFF)));
        for r in &result {
            assert!(r.top_left.y >= 10);
        }
    }

    #[test]
    fn test_subtract_partial_overlap_bottom() {
        let r1 = RectArea::new(10, 10, 20, 20, Some(0xFF));
        let r2 = RectArea::new(15, 25, 10, 10, Some(0xFF));
        let result = subtract_rect(&r1, &r2);
        assert!(result.iter().all(|r| r.top_left.y < 30));
    }
}

 */
