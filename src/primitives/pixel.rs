use crate::primitives::point::Point;

#[derive(Debug, Clone, Copy)]
/// A pixel with coordinates and color.
/// `x` and `y` can be only positive integers, since they represent a physical pixel on the screen.
pub struct Pixel {
    pub x: u32,
    pub y: u32,
    pub color: u32,
}

impl From<Pixel> for Point {
    fn from(p: Pixel) -> Self {
        Point { x: p.x, y: p.y }
    }
}
