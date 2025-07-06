use crate::primitives::numeric::Numeric;
use crate::primitives::point::Point;


#[derive(Debug, Clone, Copy, PartialEq)]
/// A line segment defined by its start and end points, with an optional color.
pub struct LineSegment<T: Numeric = u32> {
    pub start: Point<T>,
    pub end: Point<T>,
    pub color: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// A combination of width and height on a 2D plane.
pub struct Dimensions2d<T: Numeric = u32> {
    pub w: T,
    pub h: T,
}

impl<T: Numeric> Dimensions2d<T> {
    /// Converts the Dimensions2D to a different numeric type `U`.
    ///
    /// This method allows conversion between `Dimensions2d<T>` types.
    /// Precision loss may occur during conversion.
    pub fn convert<U: Numeric>(self) -> Dimensions2d<U> {
        Dimensions2d {
            w: U::from_f64(self.w.to_f64()),
            h: U::from_f64(self.h.to_f64()),
        }
    }
    /// Creates a new `Dimensions2d` with the given `w` and `h` values.
    pub fn new(w: T, h: T) -> Self {
        Self { w, h }
    }

    /// Creates a new square `Dimensions2d`, with the given side length (i.e. w == h == side).
    pub fn square(side: T) -> Self {
        Self { w: side, h: side }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// A rectangular area with a top-left point, dimensions, and color.
/// It is used to represent a rectangle on the screen, so the coordinates can only be positive integers.
pub struct RectArea<T: Numeric = u32> {
    pub top_left: Point<T>,
    pub dimensions: Dimensions2d<T>,
    pub color: Option<u32>,
}
impl<T: Numeric + std::ops::Add<Output = T>> RectArea<T> {
    /// Creates a new `RectArea` with the given top-left point, dimensions, and color.
    ///
    /// # Parameters
    ///
    /// - `x`: The x-coordinate of the top-left corner.
    /// - `y`: The y-coordinate of the top-left corner.
    /// - `w`: The width of the rectangle.
    /// - `h`: The height of the rectangle.
    /// - `color`: An optional color value (RGBA).
    pub fn new(x: T, y: T, w: T, h: T, color: Option<u32>) -> Self {
        Self {
            top_left: Point { x, y },
            dimensions: Dimensions2d { w, h },
            color,
        }
    }

    /// Creates a new square `RectArea`, with the given top-left point, side length, and color.
    pub fn square(x: T, y: T, side: T, color: Option<u32>) -> Self {
        Self {
            top_left: Point { x, y },
            dimensions: Dimensions2d {
                w: side.clone(),
                h: side.clone(),
            },
            color,
        }
    }

    pub fn convert<U: Numeric>(self) -> RectArea<U> {
        RectArea {
            top_left: self.top_left.convert(),
            dimensions: self.dimensions.convert(),
            color: self.color,
        }
    }

    /// Checks whether a given point lies within the bounds of this rectangle.
    pub fn contains_point(&self, point: &Point<T>) -> bool {
        point.x >= self.top_left.x
            && point.x < self.top_left.x + self.dimensions.w
            && point.y >= self.top_left.y
            && point.y < self.top_left.y + self.dimensions.h
    }

    /// Checks whether the given rectangle is fully contained within this rectangle.
    pub fn contains(&self, other: &RectArea<T>) -> bool {
        other.top_left.x >= self.top_left.x
            && other.top_left.y >= self.top_left.y
            && other.top_left.x + other.dimensions.w <= self.top_left.x + self.dimensions.w
            && other.top_left.y + other.dimensions.h <= self.top_left.y + self.dimensions.h
    }


        /// Checks whether this rectangle overlaps with another rectangle.
        /// (a general AABB (axis-aligned bounding box) overlap test)
        /// # Parameters
        /// - `other`: The other rectangle to check for overlap.
        pub fn overlaps(&self, other: &RectArea<T>) -> bool {
            let self_x1 = self.top_left.x;
            let self_y1 = self.top_left.y;
            let self_x2 = self.top_left.x + self.dimensions.w;
            let self_y2 = self.top_left.y + self.dimensions.h;

            let other_x1 = other.top_left.x;
            let other_y1 = other.top_left.y;
            let other_x2 = other.top_left.x + other.dimensions.w;
            let other_y2 = other.top_left.y + other.dimensions.h;

            self_x1 < other_x2 &&
                self_x2 > other_x1 &&
                self_y1 < other_y2 &&
                self_y2 > other_y1
        }


    /// Returns the bottom-right point of this rectangle.
    pub fn get_bottom_right(&self) -> Point<T> {
        Point {
            x: self.top_left.x + self.dimensions.w /* - T::one() */,
            y: self.top_left.y + self.dimensions.h /* - T::one() */,
        }
    }

    /// Checks whether a given line segment is completely outside this `RectArea`.
    ///
    /// This method performs a fast rejection test based on the axis-aligned bounding box (AABB)
    /// of the rectangle. It assumes that the segment does not intersect the rectangle if both endpoints
    /// are completely to one side (left, right, above, or below).
    ///
    /// # Parameters
    /// - `start`: The starting point of the line segment.
    /// - `end`: The ending point of the line segment.
    ///
    /// # Returns
    /// - `true` if the segment is entirely outside the rectangle and does not intersect it.
    /// - `false` if the segment intersects or lies inside the rectangle.
    pub fn is_line_segment_outside(&self, start: &Point<i32>, end: &Point<i32>) -> bool {
        
        let a: RectArea<i32> = self.convert();
        let x_min = a.top_left.x;
        let y_min = a.top_left.y;
        let x_max = x_min + a.dimensions.w - 1;
        let y_max = y_min + a.dimensions.h - 1;

        // Line is completely left, right, above, or below the area
        (start.x < x_min && end.x < x_min) || // left
            (start.x > x_max && end.x > x_max) || // right
            (start.y < y_min && end.y < y_min) || // above
            (start.y > y_max && end.y > y_max) // below
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_u32_to_i32() {
        let rect_u32: RectArea<u32> = RectArea::new(10, 20, 30, 40, Some(0x0000ffff));
        let rect_i32: RectArea<i32> = rect_u32.convert();

        assert_eq!(rect_i32.top_left.x, 10);
        assert_eq!(rect_i32.top_left.y, 20);
        assert_eq!(rect_i32.dimensions.w, 30);
        assert_eq!(rect_i32.dimensions.h, 40);
        assert_eq!(rect_i32.color, Some(0x0000ffff));
    }

    #[test]
    fn test_convert_i32_to_f32() {
        let rect_i32: RectArea<i32> = RectArea::new(10, -20, 30, 40, Some(0x0000ffff));
        let rect_f32: RectArea<f32> = rect_i32.convert();
        assert_eq!(rect_f32.top_left.x, 10.0);
        assert_eq!(rect_f32.top_left.y, -20.0);
        assert_eq!(rect_f32.dimensions.w, 30.0);
        assert_eq!(rect_f32.dimensions.h, 40.0);
        assert_eq!(rect_f32.color, Some(0x0000ffff));
    }

    #[test]
    fn test_convert_f32_to_u32() {
        let rect_f32: RectArea<f32> = RectArea::new(10.0, -20.0, 30.0, 40.0, Some(0x0000ffff));
        let rect_u32: RectArea<u32> = rect_f32.convert();
        assert_eq!(rect_u32.top_left.x, 10);
        assert_eq!(rect_u32.top_left.y, 0);
        assert_eq!(rect_u32.dimensions.w, 30);
        assert_eq!(rect_u32.dimensions.h, 40);
        assert_eq!(rect_u32.color, Some(0x0000ffff));
    }

    // #[test]
    // fn test_convert_f32_to_f64() {
    //     let rect_f32:RectArea<f32> = RectArea::new(10.5, -20.7, 30.3, 40.4, Some(0x0000ffff));
    //     let rect_f64: RectArea<f64> = rect_f32.convert();
    //     let tolerance = f64::EPSILON;
    //     assert!((rect_f64.top_left.x - 10.5).abs() < tolerance);
    //     assert!((rect_f64.top_left.y + 20.6).abs() < tolerance);
    //     assert!((rect_f64.dimensions.w - 30.3).abs() < tolerance);
    //     // assert!((rect_f64.dimensions.h - 40.4).abs() < tolerance);
    //     assert_eq!(rect_f64.color, Some(0x0000ffff));
    // }

    #[test]
    fn test_convert_f64_to_u32() {
        let rect_f64: RectArea<f64> = RectArea::new(10.9, -20.1, 30.8, 40.2, Some(0x0000ffff));
        let rect_u32: RectArea<u32> = rect_f64.convert();
        assert_eq!(rect_u32.top_left.x, 11);
        assert_eq!(rect_u32.top_left.y, 0); // -20.1 truncated to 0
        assert_eq!(rect_u32.dimensions.w, 31); // 30.8 truncated to 30
        assert_eq!(rect_u32.dimensions.h, 40); // 40.2 truncated to 40
        assert_eq!(rect_u32.color, Some(0x0000ffff));
    }

    #[test]
    fn test_contains_point() {
        let rect = RectArea::new(10, 20, 100, 50, None);

        // Inside
        assert!(rect.contains_point(&Point::new(10, 20))); // top-left corner
        assert!(rect.contains_point(&Point::new(109, 69))); // bottom-right edge (exclusive)
        assert!(rect.contains_point(&Point::new(50, 40))); // center area

        // Outside
        assert!(!rect.contains_point(&Point::new(9, 20))); // left
        assert!(!rect.contains_point(&Point::new(10, 70))); // bottom
        assert!(!rect.contains_point(&Point::new(110, 69))); // right
        assert!(!rect.contains_point(&Point::new(10, 100))); // below
    }

    #[test]
    fn test_contains_rect() {
        let outer = RectArea::new(10, 10, 100, 100, None);

        // Fully inside
        let inner = RectArea::new(20, 20, 50, 50, None);
        assert!(outer.contains(&inner));

        // Edges match
        let exact = RectArea::new(10, 10, 100, 100, None);
        assert!(outer.contains(&exact));

        // Touches right edge
        let touching = RectArea::new(110, 10, 10, 10, None);
        assert!(!outer.contains(&touching));

        // Overflows bottom
        let outside = RectArea::new(50, 50, 60, 100, None);
        assert!(!outer.contains(&outside));
    }

    #[test]
    fn test_rect_fits_rect_within_bounds() {
        let rect_1 = RectArea::new(0, 0, 500, 500, None);
        let rect_2 = RectArea::new(100, 100, 200, 200, None);
        assert!(rect_1.contains(&rect_2));
    }

    #[test]
    fn test_rect_fits_rect_out_of_bounds_x() {
        let rect_1 = RectArea::new(0, 0, 500, 500, None);
        let rect_2 = RectArea::new(400, 100, 200, 200, None);
        assert!(!rect_1.contains(&rect_2));
    }

    #[test]
    fn test_rect_fits_rect_out_of_bounds_y() {
        let rect_1 = RectArea::new(0, 0, 500, 500, None);
        let rect_2 = RectArea::new(100, 400, 200, 200, None);
        assert!(!rect_1.contains(&rect_2));
    }

    #[test]
    fn test_rect_fits_rect_too_wide() {
        let rect_1 = RectArea::new(0, 0, 500, 500, None);
        let rect_2 = RectArea::new(100, 100, 600, 200, None);
        assert!(!rect_1.contains(&rect_2));
    }

    #[test]
    fn test_rect_fits_rect_too_tall() {
        let rect_1 = RectArea::new(0, 0, 500, 500, None);
        let rect_2 = RectArea::new(100, 100, 200, 600, None);

        assert!(!rect_1.contains(&rect_2));
    }

    #[test]
    fn test_rect_fits_rect_exact_fit() {
        let rect_1 = RectArea::new(0, 0, 500, 500, None);
        let rect_2 = RectArea::new(0, 0, 500, 500, None);
        assert!(rect_1.contains(&rect_2));
    }

    // === tests for a line being outside the rectangle ===
    #[test]
    fn test_line_segment_completely_outside_left() {
        let rect = RectArea::new(10, 10, 100, 100, None);
        let start = Point::new(0, 50);
        let end = Point::new(5, 70);
        assert!(rect.is_line_segment_outside(&start, &end));
    }

    #[test]
    fn test_line_segment_completely_outside_right() {
        let rect = RectArea::new(10, 10, 100, 100, None);
        let start = Point::new(120, 20);
        let end = Point::new(130, 30);
        assert!(rect.is_line_segment_outside(&start, &end));
    }

    #[test]
    fn test_line_segment_completely_outside_above() {
        let rect = RectArea::new(10, 10, 100, 100, None);
        let start = Point::new(50, 0);
        let end = Point::new(60, 5);
        assert!(rect.is_line_segment_outside(&start, &end));
    }

    #[test]
    fn test_line_segment_completely_outside_below() {
        let rect = RectArea::new(10, 10, 100, 100, None);
        let start = Point::new(30, 120);
        let end = Point::new(40, 130);
        assert!(rect.is_line_segment_outside(&start, &end));
    }

    #[test]
    fn test_line_segment_inside_rect() {
        let rect = RectArea::new(10, 10, 100, 100, None);
        let start = Point::new(20, 20);
        let end = Point::new(90, 90);
        assert!(!rect.is_line_segment_outside(&start, &end));
    }

    #[test]
    fn test_line_segment_intersecting_left_to_inside() {
        let rect = RectArea::new(10, 10, 100, 100, None);
        let start = Point::new(0, 50);
        let end = Point::new(20, 50);
        assert!(!rect.is_line_segment_outside(&start, &end));
    }

    #[test]
    fn test_line_segment_intersecting_diagonally() {
        let rect = RectArea::new(10, 10, 100, 100, None);
        let start = Point::new(0, 0);
        let end = Point::new(120, 120);
        assert!(!rect.is_line_segment_outside(&start, &end));
    }

    #[test]
    fn test_line_segment_touching_border_not_outside() {
        let rect = RectArea::new(10, 10, 100, 100, None);
        let start = Point::new(10, 10); // top-left corner
        let end = Point::new(10, 110); // vertical edge
        assert!(!rect.is_line_segment_outside(&start, &end));
    }


    /////  AABB overlap tests

    #[test]
    fn test_overlaps_when_no_overlap() {
        let a = RectArea::new(0, 0, 10, 10, None);
        let b = RectArea::new(20, 20, 10, 10, None);
        assert!(!a.overlaps(&b));
        assert!(!b.overlaps(&a));
    }

    #[test]
    fn test_overlaps_when_overlap_by_area() {
        let a = RectArea::new(0, 0, 10, 10, None);
        let b = RectArea::new(5, 5, 10, 10, None);
        assert!(a.overlaps(&b));
        assert!(b.overlaps(&a));
    }

    #[test]
    fn test_overlaps_when_touching_by_edge() {
        let a = RectArea::new(0, 0, 10, 10, None);
        let b = RectArea::new(10, 0, 10, 10, None);
        assert!(!a.overlaps(&b));
        assert!(!b.overlaps(&a));
    }

    #[test]
    fn test_overlaps_when_touching_by_corner() {
        let a = RectArea::new(0, 0, 10, 10, None);
        let b = RectArea::new(10, 10, 10, 10, None);
        assert!(!a.overlaps(&b));
        assert!(!b.overlaps(&a));
    }

    #[test]
    fn test_overlaps_when_one_inside_another() {
        let a = RectArea::new(0, 0, 20, 20, None);
        let b = RectArea::new(5, 5, 5, 5, None);
        assert!(a.overlaps(&b));
        assert!(b.overlaps(&a));
    }

    #[test]
    fn test_overlaps_when_identical() {
        let a = RectArea::new(0, 0, 10, 10, None);
        let b = RectArea::new(0, 0, 10, 10, None);
        assert!(a.overlaps(&b));
        assert!(b.overlaps(&a));
    }


}
