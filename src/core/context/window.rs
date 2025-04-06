
use crate::core::context_utils::window_quadrants::Quadrants;
use crate::core::default_colors;
use crate::primitives::plane::{Dimensions2d, RectArea};
use crate::primitives::point::Point;

#[derive(Debug)]
/// A collection of window properties, such as width, height, and background color,
/// Width and height are exposed as `u32`, `usize`, and `i32` for convenience in calculations.
pub struct WindowContext {
    /// Window width
    pub w: u32,
    /// Window height
    pub h: u32,
    /// Window width as `usize`
    pub w_usize: usize,
    /// Window height as `usize`
    pub h_usize: usize,
    /// Window width as `i32`
    pub w_i32: i32,
    /// Window height but as `i32`
    pub h_i32: i32,
    /// Window width and height as a `Dimensions2d`
    pub dimensions: Dimensions2d,
    /// Window width and height as a `Dimensions2d<usize>`
    pub dimensions_usize: Dimensions2d<usize>,
    /// Window as a `RectArea` with a top-left point at (0, 0)
    pub rect_area: RectArea,
    /// Background color of the window, RGBA
    pub background_color: u32,
    /// Foreground color of the window, RGBA
    pub foreground_color: u32,
    /// The central point of the window
    pub center: Point<u32>,
    /// Four equally sized subregions representing the screen's quadrants (top-left, top-right, bottom-left, bottom-right)
    pub quadrants: Quadrants,
}

impl WindowContext {
    /// Instantiates a window context
    pub fn new(
        w: u32,
        h: u32,
        background_color_rgba: Option<u32>,
        foreground_color_rgba: Option<u32>,
    ) -> Self {
        let fg_color = foreground_color_rgba.unwrap_or(default_colors::FOREGROUND);

        let quadrants = Quadrants::from_dimensions(w, h);

        Self {
            w,
            h,
            w_usize: w as usize,
            h_usize: h as usize,
            w_i32: w as i32,
            h_i32: h as i32,
            dimensions: Dimensions2d { w, h },
            dimensions_usize: Dimensions2d {
                w: w as usize,
                h: h as usize,
            },
            rect_area: RectArea::new(0, 0, w, h, Some(fg_color)),
            background_color: background_color_rgba.unwrap_or(default_colors::BACKGROUND),
            foreground_color: fg_color,
            center: Point::new(w / 2, h / 2),
            quadrants,
        }
    }

    /// Instantiates a window context of 320x240 pixels with  default background and foreground colors
    pub fn default() -> Self {
        WindowContext::new(
            320,
            240,
            Some(default_colors::BACKGROUND),
            Some(default_colors::FOREGROUND),
        )
    }

    /// Returns the number of pixels in the window
    pub fn get_num_pixels(&self) -> usize {
        self.w_usize * self.h_usize
    }

    /// Resizes the window context, recalculates all the related window properties
    pub fn resize(&mut self, w: u32, h: u32) {
        self.w = w;
        self.h = h;
        self.w_usize = w as usize;
        self.h_usize = h as usize;
        self.w_i32 = w as i32;
        self.h_i32 = h as i32;
        self.dimensions.w = w;
        self.dimensions.h = h;
        self.rect_area.dimensions.w = w;
        self.rect_area.dimensions.h = h;
        self.center.x = w / 2;
        self.center.y = h / 2;
        self.update_quadrants();
    }

    fn update_quadrants(&mut self) {
        let q = Quadrants::from_dimensions(self.w, self.h);
        // This should avoid reallocation of the regions, hence any potential references should not break
        self.quadrants.top_left.update(q.top_left.rect_area());
        self.quadrants.top_right.update(q.top_right.rect_area());
        self.quadrants.bottom_left.update(q.bottom_left.rect_area());
        self.quadrants
            .bottom_right
            .update(q.bottom_right.rect_area());
    }

    pub fn contains(&self, rect: &RectArea) -> bool {
        self.rect_area.contains(rect)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::plane::RectArea;
    use crate::test::mock_contexts::get_mock_graph_context;

    //======== [ RECT FITS WINDOW ] ================================================================
    #[test]
    fn test_rect_fits_window_within_bounds() {
        let rect = RectArea::new(100, 100, 200, 150, None);
        let ctx = get_mock_graph_context(800, 600);
        assert!(ctx.win.contains(&rect));
    }

    #[test]
    fn test_rect_fits_window_out_of_bounds_x() {
        let rect = RectArea::new(900, 100, 200, 150, None);
        let ctx = get_mock_graph_context(800, 600);
        assert!(!ctx.win.contains(&rect));
    }

    #[test]
    fn test_rect_fits_window_out_of_bounds_y() {
        let rect = RectArea::new(100, 900, 200, 150, None);
        let ctx = get_mock_graph_context(800, 600);
        assert!(!ctx.win.contains(&rect));
    }

    #[test]
    fn test_rect_fits_window_too_wide() {
        let rect = RectArea::new(10, 10, 1200, 150, None);
        let ctx = get_mock_graph_context(800, 600);
        assert!(!ctx.win.contains(&rect));
    }

    #[test]
    fn test_rect_fits_window_too_tall() {
        let rect = RectArea::new(10, 10, 120, 1500, None);
        let ctx = get_mock_graph_context(800, 600);
        assert!(!ctx.win.contains(&rect));
    }

    #[test]
    fn test_rect_fits_window_exact_fit() {
        let rect = RectArea::new(0, 0, 800, 600, None);
        let ctx = get_mock_graph_context(800, 600);
        assert!(ctx.win.contains(&rect));
    }
}
