use crate::primitives::containable::Containable;
use crate::primitives::numeric::Numeric;
use crate::primitives::plane::{Dimensions2d, RectArea};
use crate::primitives::point::Point;
use crate::primitives::ratio::Ratio;
use crate::primitives::Pixel;

/// Represents a rectangular region on a 2D-plane.
/// Provides access to some spatial reference points such as:
/// - the center of the region
/// - midpoints of each edge (top, bottom, left, right)
/// - corners (top-left, top-right, bottom-left, bottom-right)
/// These could be used in layout implementations, animation effects, transformations, and screen subdivisions.
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
    /// Creates a new `Region` from the given `RectArea`, calculates all relevant points.
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

    /// Updates the region with a new `RectArea` and recalculates all relevant points.
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

        self.center.x = cx;
        self.center.y = cy;
        self.top.x = cx;
        self.top.y = y0;
        self.bottom.x = cx;
        self.bottom.y = y1;
        self.left.x = x0;
        self.left.y = cy;
        self.right.x = x1;
        self.right.y = cy;
        self.top_left.x = x0;
        self.top_left.y = y0;
        self.top_right.x = x1;
        self.top_right.y = y0;
        self.bottom_left.x = x0;
        self.bottom_left.y = y1;
        self.bottom_right.x = x1;
        self.bottom_right.y = y1;
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

    //----------------------------------------------------------------------------------------------

    pub fn width(&self) -> T {
        self.rect_area.dimensions.w
    }

    pub fn height(&self) -> T {
        self.rect_area.dimensions.h
    }

    pub fn size(&self) -> Dimensions2d<T> {
        self.rect_area.dimensions
    }

    pub fn area(&self) -> T {
        self.width() * self.height()
    }

    /// Returns a deterministic pseudorandom point inside the region, based on the given seed.
    pub fn random_point_inside(&self, seed: u32) -> Point<u32> {
        let min_x = self.rect_area.top_left.x.to_u32();
        let max_x = (self.rect_area.top_left.x + self.rect_area.dimensions.w).to_u32();
        let x = hash_random_u32!(seed, min_x, max_x);

        let min_y = self.rect_area.top_left.y.to_u32();
        let max_y = (self.rect_area.top_left.y + self.rect_area.dimensions.h).to_u32();
        let y = hash_random_u32!(seed.wrapping_add(1), min_y, max_y);

        Point::new(x, y)
    }

    /// Returns a deterministic pseudorandom pixel inside the region with the given color, based on the given seed.
    pub fn random_pixel_inside(&self, seed: u32, color: u32) -> Pixel {
        self.random_point_inside(seed).to_pixel(color)
    }

    pub fn split_horizontal(&self, ratio: Option<Ratio<T>>) -> (Region<T>, Region<T>) {
        let h = self.height();
        let w = self.width();
        let (top_h, bottom_h) = match ratio {
            Some(r) => {
                let r = r.simplified();
                let top_h = h * r.numerator / (r.numerator + r.denominator);
                let bottom_h = h - top_h;
                (top_h, bottom_h)
            }
            None => {
                let half = h / T::from_u32(2);
                (half, h - half)
            }
        };

        let top = RectArea::new(
            self.rect_area.top_left.x,
            self.rect_area.top_left.y,
            w,
            top_h,
            self.rect_area.color,
        );

        let bottom = RectArea::new(
            self.rect_area.top_left.x,
            self.rect_area.top_left.y + top_h,
            w,
            bottom_h,
            self.rect_area.color,
        );

        (Region::new(top), Region::new(bottom))
    }

    pub fn split_vertical(&self, ratio: Option<Ratio<T>>) -> (Region<T>, Region<T>) {
        let w = self.width();
        let h = self.height();
        let (left_w, right_w) = match ratio {
            Some(r) => {
                let r = r.simplified();
                let left_w = w * r.numerator / (r.numerator + r.denominator);
                let right_w = w - left_w;
                (left_w, right_w)
            }
            None => {
                let half = w / T::from_u32(2);
                (half, w - half)
            }
        };

        let left = RectArea::new(
            self.rect_area.top_left.x,
            self.rect_area.top_left.y,
            left_w,
            h,
            self.rect_area.color,
        );

        let right = RectArea::new(
            self.rect_area.top_left.x + left_w,
            self.rect_area.top_left.y,
            right_w,
            h,
            self.rect_area.color,
        );

        (Region::new(left), Region::new(right))
    }

    /// Returns a point near the center of the region, jittered by up to `max_offset` in each direction.
    pub fn jittered_center(&self, seed: u32, max_offset: u32) -> Point<u32> {
        let dx = (hash_random_u32!(seed, 0, 2 * max_offset + 1) as i32) - max_offset as i32;
        let dy = (hash_random_u32!(seed.wrapping_add(1), 0, 2 * max_offset + 1) as i32)
            - max_offset as i32;

        let x = (self.center.x.to_u32() as i32 + dx).to_u32();
        let y = (self.center.y.to_u32() as i32 + dy).to_u32();

        Point { x, y }
    }

    /// Returns a pixel near the center of the region, jittered by up to `max_offset`, and colored with the given value.
    pub fn jittered_center_pixel(&self, seed: u32, max_offset: u32, color: u32) -> Pixel {
        self.jittered_center(seed, max_offset).to_pixel(color)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::plane::{Dimensions2d, RectArea};
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

        let inside = Pixel {
            x: 5,
            y: 5,
            color: 0xFF0000FF,
        };
        let outside = Pixel {
            x: 12,
            y: 8,
            color: 0xFF0000FF,
        };

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

    #[test]
    fn test_region_size_and_area() {
        let region = Region::new(RectArea::new(10, 20, 30, 40, None));

        assert_eq!(region.width(), 30);
        assert_eq!(region.height(), 40);
        assert_eq!(region.size(), Dimensions2d::new(30, 40));
        assert_eq!(region.area(), 1200);
    }

    #[test]
    fn test_random_point_inside_is_within_bounds() {
        let region = Region::new(RectArea::new(100, 200, 50, 50, None));

        let pt = region.random_point_inside(42);
        assert!(pt.x >= 100 && pt.x < 150, "x out of bounds: {}", pt.x);
        assert!(pt.y >= 200 && pt.y < 250, "y out of bounds: {}", pt.y);
    }

    #[test]
    fn test_random_pixel_inside_color_and_bounds() {
        let color = 0xDEADBEEF;
        let region = Region::new(RectArea::new(0, 0, 10, 10, None));

        let px = region.random_pixel_inside(123, color);

        assert!(px.x < 10);
        assert!(px.y < 10);
        assert_eq!(px.color, color);
    }

    #[test]
    fn test_split_horizontal_ratio() {
        let region = Region::new(RectArea::new(0, 0, 10, 10, None));
        let ratio = Ratio::new(1, 3).unwrap();
        let (top, bottom) = region.split_horizontal(Some(ratio));

        assert_eq!(top.height(), 2);
        assert_eq!(bottom.height(), 8);
    }

    #[test]
    fn test_split_vertical_ratio() {
        let region = Region::new(RectArea::new(0, 0, 20, 10, None));
        let ratio = Ratio::new(3, 1).unwrap();
        let (left, right) = region.split_vertical(Some(ratio));

        assert_eq!(left.width(), 15);
        assert_eq!(right.width(), 5);
    }

    #[test]
    fn test_jittered_center_stays_near_center() {
        let region = Region::new(RectArea::new(100, 100, 20, 20, None));
        let max_offset = 5;
        let center = region.center().convert::<u32>();

        for seed in 0..10 {
            let jittered = region.jittered_center(seed, max_offset);
            let dx = jittered.x as i32 - center.x as i32;
            let dy = jittered.y as i32 - center.y as i32;

            assert!(dx.abs() <= max_offset as i32, "dx too large: {}", dx);
            assert!(dy.abs() <= max_offset as i32, "dy too large: {}", dy);
        }
    }

    #[test]
    fn test_jittered_center_pixel_correct_color() {
        let region = Region::new(RectArea::new(0, 0, 10, 10, None));
        let color = 0xABCDEF;
        let px = region.jittered_center_pixel(99, 3, color);
        assert_eq!(px.color, color);
    }
}
