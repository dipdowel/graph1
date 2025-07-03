
use crate::primitives::point::Point;
use std::fmt;

#[derive(Clone, Copy, PartialEq)]
/// A pixel with coordinates and color.
/// `x` and `y` can be only positive integers, since they represent a physical pixel on the screen.

pub struct Pixel {
    pub x: u32,
    pub y: u32,
    pub color: u32,
}


impl Pixel {
    pub fn new(x: u32, y: u32, color: u32) -> Self {
        Pixel { x, y, color }
    }
}


impl fmt::Debug for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Pixel {{ x: {}, y: {}, color: {:#x} }}", self.x, self.y, self.color)
    }
}



// FIXME: Can the 4 implementations below be merged into one?

impl From<Pixel> for Point<u32> {
    fn from(p: Pixel) -> Self {
        Point { x: p.x, y: p.y }
    }
}

impl From<Pixel> for Point<i32> {
    fn from(p: Pixel) -> Self {
        Point {
            x: p.x as i32,
            y: p.y as i32,
        }
    }
}

impl From<Pixel> for Point<f32> {
    fn from(p: Pixel) -> Self {
        Point {
            x: p.x as f32,
            y: p.y as f32,
        }
    }
}

impl From<Pixel> for Point<f64> {
    fn from(p: Pixel) -> Self {
        Point {
            x: p.x as f64,
            y: p.y as f64,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion() {
        let pixel: Pixel = Pixel {
            x: 10,
            y: 20,
            color: 0xff_ff_ff_ff,
        };

        let point_u32: Point<u32> = pixel.into();
        let point_i32: Point<i32> = pixel.into();
        let point_f32: Point<f32> = pixel.into();
        let point_f64: Point<f64> = pixel.into();

        assert_eq!(point_u32, Point { x: 10, y: 20 });
        assert_eq!(point_i32, Point { x: 10, y: 20 });
        assert_eq!(point_f32, Point { x: 10.0, y: 20.0 });
        assert_eq!(point_f64, Point { x: 10.0, y: 20.0 });
    }
}
