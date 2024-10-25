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
}



#[derive(Debug, Clone, Copy)]
pub struct RectArea<T: Numeric = u32> {
    pub top_left: Point<T>,
    pub dimensions: Dimensions2d<T>,
}

