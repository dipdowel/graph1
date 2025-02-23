use crate::primitives::numeric::Numeric;
use crate::primitives::point::Point;

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
/// A rectangle area with a top-left point, dimensions, and color.
/// It is used to represent a rectangle on the screen, so the coordinates can only be positive integers.
pub struct RectArea<T: Numeric = u32> {
    pub top_left: Point<T>,
    pub dimensions: Dimensions2d<T>,
    pub color: Option<u32>,
}

impl<T: Numeric> RectArea<T> {
    /// Creates a new `RectArea` with the given top-left point, dimensions, and color.
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
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_u32_to_i32() {
        let rect_u32:RectArea<u32> = RectArea::new(10, 20, 30, 40, Some(0x0000ffff));
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
        let rect_f64:RectArea<f64> = RectArea::new(10.9, -20.1, 30.8, 40.2, Some(0x0000ffff));
        let rect_u32: RectArea<u32> = rect_f64.convert();
        assert_eq!(rect_u32.top_left.x, 10);
        assert_eq!(rect_u32.top_left.y, 0); // -20.1 truncated to 0
        assert_eq!(rect_u32.dimensions.w, 30); // 30.8 truncated to 30
        assert_eq!(rect_u32.dimensions.h, 40); // 40.2 truncated to 40
        assert_eq!(rect_u32.color, Some(0x0000ffff));
    }
}


