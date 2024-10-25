use crate::primitives::point::Point;

#[derive(Debug, Clone, Copy)]
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
