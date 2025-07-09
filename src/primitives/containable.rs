use crate::primitives::numeric::Numeric;
use crate::primitives::plane::RectArea;
use crate::primitives::point::Point;
use crate::primitives::Pixel;

/// A trait representing anything that can be tested for containment in a `RectArea`.
pub trait Containable<T: Numeric> {
    fn is_contained_in(&self, rect: &RectArea<T>) -> bool;
}

impl<T: Numeric> Containable<T> for Point<T> {
    fn is_contained_in(&self, rect: &RectArea<T>) -> bool {
        let x = self.x;
        let y = self.y;
        x >= rect.top_left.x
            && x < rect.top_left.x + rect.dimensions.w
            && y >= rect.top_left.y
            && y < rect.top_left.y + rect.dimensions.h
    }
}

impl<T: Numeric> Containable<T> for Pixel {
    fn is_contained_in(&self, rect: &RectArea<T>) -> bool {
        let &Pixel { x, y, .. } = self;
        x >= rect.top_left.x.to_u32()
            && x < rect.top_left.x.to_u32() + rect.dimensions.w.to_u32()
            && y >= rect.top_left.y.to_u32()
            && y < rect.top_left.y.to_u32() + rect.dimensions.h.to_u32()
    }
}

impl Containable<u32> for RectArea<u32> {
    fn is_contained_in(&self, rect: &RectArea<u32>) -> bool {
        let x0 = self.top_left.x;
        let y0 = self.top_left.y;
        let x1 = x0 + self.dimensions.w;
        let y1 = y0 + self.dimensions.h;

        x0 >= rect.top_left.x
            && y0 >= rect.top_left.y
            && x1 <= rect.top_left.x + rect.dimensions.w
            && y1 <= rect.top_left.y + rect.dimensions.h
    }
}
