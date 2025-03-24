use crate::primitives::plane::{Dimensions2d, RectArea};
use crate::primitives::point::Point;
use crate::primitives::Pixel;
use crate::primitives::numeric::Numeric;

/// A trait representing anything that can be tested for containment in a `RectArea`.
pub trait Containable<T: Numeric> {
    fn is_contained_in(&self, rect: &RectArea<T>) -> bool;
}

impl<T:Numeric> Containable<T> for Point<T> {
    fn is_contained_in(&self, rect: &RectArea<T>) -> bool {
        let x = self.x;
        let y = self.y;
        x >= rect.top_left.x && x < rect.top_left.x + rect.dimensions.w &&
            y >= rect.top_left.y && y < rect.top_left.y + rect.dimensions.h
    }
}

impl<T: Numeric> Containable<T> for Pixel {

    fn is_contained_in(&self, rect: &RectArea<T>) -> bool {
        let &Pixel{x, y, ..} = self;
        x >= rect.top_left.x.to_u32() && x < rect.top_left.x.to_u32() + rect.dimensions.w.to_u32() &&
            y >= rect.top_left.y.to_u32() && y < rect.top_left.y.to_u32() + rect.dimensions.h.to_u32()
    }
}

impl Containable<u32> for RectArea<u32> {
    fn is_contained_in(&self, rect: &RectArea<u32>) -> bool {
        let x0 = self.top_left.x;
        let y0 = self.top_left.y;
        let x1 = x0 + self.dimensions.w;
        let y1 = y0 + self.dimensions.h;

        x0 >= rect.top_left.x && y0 >= rect.top_left.y &&
            x1 <= rect.top_left.x + rect.dimensions.w &&
            y1 <= rect.top_left.y + rect.dimensions.h
    }
}

/// Represents a rectangular region within a 2D space.
/// Provides access to useful spatial reference points such as:
/// - the center of the region
/// - midpoints of each edge (top, bottom, left, right)
/// - corners (top-left, top-right, bottom-left, bottom-right)
/// These are useful for layout, animation effects, transformations, and subdivision.
#[derive(Debug, Clone, Copy)]
pub struct Region<T: Numeric = u32> {
    rect_area: RectArea<T>,
    center: Point<T>,
    top: Point<T>,
    bottom: Point<T>,
    left: Point<T>,
    right: Point<T>,
    top_left: Point<T>,
    top_right: Point<T>,
    bottom_left: Point<T>,
    bottom_right: Point<T>,
}

impl<T: Numeric> Region<T> {
    /// Creates a new `Region` from the given `RectArea`, calculating all relevant points.
    pub fn new(area: RectArea<T>) -> Self {
        let x0 = area.top_left.x;
        let y0 = area.top_left.y;
        let w = area.dimensions.w;
        let h = area.dimensions.h;
        let x1 = x0 + w;
        let y1 = y0 + h;

        let two = T::from_f64(2.0);
        let cx = (x0 + x1) / two;
        let cy = (y0 + y1) / two;

        Self {
            rect_area: area,
            center: Point::new(cx, cy),
            top: Point::new(cx, y0),
            bottom: Point::new(cx, y1),
            left: Point::new(x0, cy),
            right: Point::new(x1, cy),
            top_left: Point::new(x0, y0),
            top_right: Point::new(x1, y0),
            bottom_left: Point::new(x0, y1),
            bottom_right: Point::new(x1, y1),
        }
    }

    /// Updates the region with a new `RectArea` and recalculates all derived points.
    pub fn update(&mut self, area: RectArea<T>) {
        self.rect_area = area;

        let x0 = area.top_left.x;
        let y0 = area.top_left.y;
        let w = area.dimensions.w;
        let h = area.dimensions.h;
        let x1 = x0 + w;
        let y1 = y0 + h;

        let two = T::from_f64(2.0);
        let cx = (x0 + x1) / two;
        let cy = (y0 + y1) / two;

        self.center = Point::new(cx, cy);
        self.top = Point::new(cx, y0);
        self.bottom = Point::new(cx, y1);
        self.left = Point::new(x0, cy);
        self.right = Point::new(x1, cy);
        self.top_left = Point::new(x0, y0);
        self.top_right = Point::new(x1, y0);
        self.bottom_left = Point::new(x0, y1);
        self.bottom_right = Point::new(x1, y1);
    }

    /// Returns the region's full area.
    pub fn rect_area(&self) -> RectArea<T> {
        self.rect_area
    }

    /// Returns true if the given item is fully contained within the region.
    pub fn contains<I: Containable<T>>(&self, item: I) -> bool {
        let rect = self.rect_area.convert::<T>();
        item.is_contained_in(&rect)
    }

    /// Returns the center point of the region.
    pub fn center(&self) -> Point<T> {
        self.center
    }

    pub fn top(&self) -> Point<T> {
        self.top
    }

    pub fn bottom(&self) -> Point<T> {
        self.bottom
    }

    pub fn left(&self) -> Point<T> {
        self.left
    }

    pub fn right(&self) -> Point<T> {
        self.right
    }

    pub fn top_left(&self) -> Point<T> {
        self.top_left
    }

    pub fn top_right(&self) -> Point<T> {
        self.top_right
    }

    pub fn bottom_left(&self) -> Point<T> {
        self.bottom_left
    }

    pub fn bottom_right(&self) -> Point<T> {
        self.bottom_right
    }
}
