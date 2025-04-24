use crate::primitives::numeric::Numeric;
use crate::primitives::point::Point;

pub struct BezierSegment<T: Numeric> {
    pub start: Point<T>,
    pub end: Point<T>,
    pub start_control: Point<T>,
    pub end_control: Point<T>,
    pub color: u32,
}

impl<T: Numeric> BezierSegment<T> {
    pub fn new(
        start: Point<T>,
        end: Point<T>,
        start_control: Point<T>,
        end_control: Point<T>,
        color: u32,
    ) -> Self {
        BezierSegment {
            start,
            end,
            start_control,
            end_control,
            color,
        }
    }
}
