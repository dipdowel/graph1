use crate::primitives::numeric::Numeric;
use crate::primitives::point::Point;


#[derive(Debug, Clone, Copy)]
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
pub struct RectArea<T: Numeric = u32> {
    pub top_left: Point<T>,
    pub dimensions: Dimensions2d<T>,
    pub color: Option<u32>,
}

impl<T: Numeric>  RectArea<T> {
    pub fn new(x:T, y:T, w:T, h:T, color: Option<u32>) -> Self {
        Self {
            top_left: Point { x, y },
            dimensions: Dimensions2d { w, h },
            color,
        }
    }

    pub fn square(x:T, y:T, side:T, color: Option<u32>) -> Self {
        Self {
            top_left: Point { x, y },
            dimensions: Dimensions2d { w: side.clone(), h: side.clone() },
            color,
        }
    }

}