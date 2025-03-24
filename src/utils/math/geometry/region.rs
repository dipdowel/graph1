use crate::primitives::containable::Containable;
use crate::primitives::numeric::Numeric;
use crate::primitives::plane::RectArea;
use crate::primitives::point::Point;


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



#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::plane::{RectArea, Dimensions2d};
    use crate::primitives::point::Point;
    use crate::primitives::Pixel;

    #[test]
    fn test_region_creation_and_accessors() {
        let area = RectArea {
            top_left: Point::new(10, 20),
            dimensions: Dimensions2d { w: 30, h: 40 },
            color: Some(0xFF0000FF),
        };

        let region = Region::new(area);

        assert_eq!(region.rect_area(), area);
        assert_eq!(region.center(), Point::new(25, 40));
        assert_eq!(region.top(), Point::new(25, 20));
        assert_eq!(region.bottom(), Point::new(25, 60));
        assert_eq!(region.left(), Point::new(10, 40));
        assert_eq!(region.right(), Point::new(40, 40));
        assert_eq!(region.top_left(), Point::new(10, 20));
        assert_eq!(region.top_right(), Point::new(40, 20));
        assert_eq!(region.bottom_left(), Point::new(10, 60));
        assert_eq!(region.bottom_right(), Point::new(40, 60));
    }

    #[test]
    fn test_region_update() {
        let mut region = Region::new(RectArea {
            top_left: Point::new(0, 0),
            dimensions: Dimensions2d { w: 10, h: 10 },
            color: Some(0xFF0000FF),
        });

        let new_area = RectArea {
            top_left: Point::new(5, 5),
            dimensions: Dimensions2d { w: 20, h: 20 },
            color: Some(0xFF0000FF),
        };

        region.update(new_area);

        assert_eq!(region.rect_area(), new_area);
        assert_eq!(region.center(), Point::new(15, 15));
        assert_eq!(region.bottom_right(), Point::new(25, 25));
    }

    #[test]
    fn test_contains_point() {
        let region = Region::new(RectArea {
            top_left: Point::new(0, 0),
            dimensions: Dimensions2d { w: 10, h: 10 },
            color: Some(0xFF0000FF),
        });

        assert!(region.contains(Point::new(5, 5)));
        assert!(!region.contains(Point::new(11, 5)));
    }

    #[test]
    fn test_contains_pixel() {
        let region = Region::new(RectArea {
            top_left: Point::new(0, 0),
            dimensions: Dimensions2d { w: 10, h: 10 },
            color: Some(0xFF0000FF),
        });

        let inside = Pixel { x: 5, y: 5, color: 0xFF0000FF };
        let outside = Pixel { x: 12, y: 8, color: 0xFF0000FF };

        assert!(region.contains(inside));
        assert!(!region.contains(outside));
    }

    #[test]
    fn test_contains_rect_area() {
        let region = Region::new(RectArea {
            top_left: Point::new(0, 0),
            dimensions: Dimensions2d { w: 20, h: 20 },
            color: Some(0xFF0000FF),
        });

        let smaller_inside = RectArea {
            top_left: Point::new(5, 5),
            dimensions: Dimensions2d { w: 10, h: 10 },
            color: Some(0xFF0000FF),
        };

        let partially_outside = RectArea {
            top_left: Point::new(15, 15),
            dimensions: Dimensions2d { w: 10, h: 10 },
            color: Some(0xFF0000FF),
        };

        assert!(region.contains(smaller_inside));
        assert!(!region.contains(partially_outside));
    }
}
