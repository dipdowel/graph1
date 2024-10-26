use crate::primitives::numeric::Numeric;
use crate::primitives::point::Point;


#[derive(Debug, Clone, Copy)]
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
        Self { w:side, h:side }
    }
}



#[derive(Debug, Clone, Copy)]
/// A rectangle area with a top-left point, dimensions, and color.
/// It is used to represent a rectangle on the screen, so the coordinates can only be positive integers.
pub struct RectArea<T: Numeric = u32> {
    pub top_left: Point<T>,
    pub dimensions: Dimensions2d<T>,
    pub color: Option<u32>,
}

impl<T: Numeric>  RectArea<T> {

    /// Creates a new `RectArea` with the given top-left point, dimensions, and color.
    pub fn new(x:T, y:T, w:T, h:T, color: Option<u32>) -> Self {
        Self {
            top_left: Point { x, y },
            dimensions: Dimensions2d { w, h },
            color,
        }
    }

    /// Creates a new square `RectArea`, with the given top-left point, side length, and color.
    pub fn square(x:T, y:T, side:T, color: Option<u32>) -> Self {
        Self {
            top_left: Point { x, y },
            dimensions: Dimensions2d { w: side.clone(), h: side.clone() },
            color,
        }
    }

}