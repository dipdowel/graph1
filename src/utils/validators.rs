use crate::core::context::GraphContext;
use crate::primitives::numeric::Numeric;
use crate::primitives::plane::RectArea;


/// Validates whether `rect_2` fits within `rect_1`.
///
/// # Arguments
/// * `rect_1` - The outer rectangle.
/// * `rect_2` - The inner rectangle candidate.
///
/// # Returns
/// `true` if `rect_2` fits within `rect_1`, `false` otherwise.
pub fn rect_fits_rect(rect_1: &RectArea, rect_2: &RectArea) -> bool {
    rect_2.top_left.x >= rect_1.top_left.x
        && rect_2.top_left.y >= rect_1.top_left.y
        && (rect_2.top_left.x + rect_2.dimensions.w) <= (rect_1.top_left.x + rect_1.dimensions.w)
        && (rect_2.top_left.y + rect_2.dimensions.h) <= (rect_1.top_left.y + rect_1.dimensions.h)
}

/// validate whether a given rectangle fits within the window bounds
///
/// # Arguments
/// * `ctx` - The graph context
/// * `rect` - The rectangle to validate.
///
/// # Returns
/// `true` if the rectangle is within the window bounds, `false` otherwise.

pub fn rect_fits_window<UserData>(ctx: &mut GraphContext<UserData>, rect: &RectArea) -> bool {
    rect_fits_rect(&ctx.win.rect_area, rect)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::plane::RectArea;
    use crate::test::mock_contexts::{get_mock_graph_context};

    //======== [ RECT FITS WINDOW ] ================================================================
    #[test]
    fn test_rect_fits_window_within_bounds() {
        let rect = RectArea::new(100, 100, 200, 150, None);
        assert!(rect_fits_window(&mut get_mock_graph_context(800, 600), &rect));
    }

    #[test]
    fn test_rect_fits_window_out_of_bounds_x() {
        let rect = RectArea::new(900, 100, 200, 150, None);
        assert!(!rect_fits_window(&mut get_mock_graph_context(800, 600), &rect));
    }

    #[test]
    fn test_rect_fits_window_out_of_bounds_y() {
        let rect = RectArea::new(100, 900, 200, 150, None);
        assert!(!rect_fits_window(&mut get_mock_graph_context(800, 600), &rect));
    }

    #[test]
    fn test_rect_fits_window_too_wide() {
        let rect = RectArea::new(10, 10, 1200, 150, None);
        assert!(!rect_fits_window(&mut get_mock_graph_context(800, 600), &rect));
    }


    #[test]
    fn test_rect_fits_window_too_tall() {
        let rect = RectArea::new(10, 10, 120, 1500, None);
        assert!(!rect_fits_window(&mut get_mock_graph_context(800, 600), &rect));
    }

    #[test]
    fn test_rect_fits_window_exact_fit() {
        let rect = RectArea::new(0, 0, 800, 600, None);
        assert!(rect_fits_window(&mut get_mock_graph_context(800, 600), &rect));
    }


    //======== [ RECT FITS ANOTHER RECT ] ==========================================================

    #[test]
    fn test_rect_fits_rect_within_bounds() {
        let rect_1 = RectArea::new(0, 0, 500, 500, None);
        let rect_2 = RectArea::new(100, 100, 200, 200, None);
        assert!(rect_fits_rect(&rect_1, &rect_2));
    }

    #[test]
    fn test_rect_fits_rect_out_of_bounds_x() {
        let rect_1 = RectArea::new(0, 0, 500, 500, None);
        let rect_2 = RectArea::new(400, 100, 200, 200, None);
        assert!(!rect_fits_rect(&rect_1, &rect_2));
    }

    #[test]
    fn test_rect_fits_rect_out_of_bounds_y() {
        let rect_1 = RectArea::new(0, 0, 500, 500, None);
        let rect_2 = RectArea::new(100, 400, 200, 200, None);
        assert!(!rect_fits_rect(&rect_1, &rect_2));
    }

    #[test]
    fn test_rect_fits_rect_too_wide() {
        let rect_1 = RectArea::new(0, 0, 500, 500, None);
        let rect_2 = RectArea::new(100, 100, 600, 200, None);
        assert!(!rect_fits_rect(&rect_1, &rect_2));
    }

    #[test]
    fn test_rect_fits_rect_too_tall() {
        let rect_1 = RectArea::new(0, 0, 500, 500, None);
        let rect_2 = RectArea::new(100, 100, 200, 600, None);
        assert!(!rect_fits_rect(&rect_1, &rect_2));
    }

    #[test]
    fn test_rect_fits_rect_exact_fit() {
        let rect_1 = RectArea::new(0, 0, 500, 500, None);
        let rect_2 = RectArea::new(0, 0, 500, 500, None);
        assert!(rect_fits_rect(&rect_1, &rect_2));
    }
}
