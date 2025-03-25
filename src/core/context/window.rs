use crate::core::default_colors;
use crate::primitives::plane::{Dimensions2d, RectArea};
use crate::primitives::point::Point;
use crate::utils::math::geometry::region::Region;

#[derive(Debug)]
pub struct Quadrants {
    pub top_left: Region<u32>,
    pub top_right: Region<u32>,
    pub bottom_left: Region<u32>,
    pub bottom_right: Region<u32>,
}


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
    pub center:Point<u32>,
    /// TODO: write the documentation
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

        let full_region = Region::new(RectArea::new(0, 0, w, h, Some(fg_color)));

        let (top, bottom) = full_region.split_horizontal();
        let (top_left, top_right) = top.split_vertical();
        let (bottom_left, bottom_right) = bottom.split_vertical();

        let quadrants = Quadrants {
            top_left,
            top_right,
            bottom_left,
            bottom_right,
        };


        Self {
            w,
            h,
            w_usize: w as usize,
            h_usize: h as usize,
            w_i32: w as i32,
            h_i32: h as i32,
            dimensions: Dimensions2d { w, h },
            dimensions_usize: Dimensions2d { w: w as usize, h: h as usize },
            rect_area: RectArea::new(0, 0, w, h, Some(fg_color)),
            background_color: background_color_rgba.unwrap_or(default_colors::BACKGROUND),
            foreground_color: fg_color,
            center: Point::new(w / 2, h / 2),
            quadrants
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

    /// FIXME: Move to `Quadrants` struct!!!
    /// FIXME: Move to `Quadrants` struct!!!
    /// FIXME: Move to `Quadrants` struct!!!
    /// FIXME: Move to `Quadrants` struct!!!
    pub fn update_quadrants(&mut self) {

        let full_region = Region::new(RectArea::new(0, 0, self.w, self.h, Some(self.foreground_color)));

        let (top, bottom) = full_region.split_horizontal();
        let (top_left, top_right) = top.split_vertical();
        let (bottom_left, bottom_right) = bottom.split_vertical();

        self.quadrants = Quadrants {
            top_left,
            top_right,
            bottom_left,
            bottom_right,
        };


    }

}
