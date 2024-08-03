#[derive(Debug, Clone, Copy)]
pub struct Dimensions2d {
    pub w: u32,
    pub h: u32,
}

// TODO:
// TODO:
// TODO: Review this whole module. It may need some good refactoring!
// TODO:
// TODO:

// [ START ] ///////////////////////////////////////////////////////////////////////////////////////
// ========= Point 2D, + conversions from u32 to f32 coordinates and back ==========================

#[derive(Clone, Copy, Debug)]
pub struct PointI32 {
    pub x: i32,
    pub y: i32,
}
// TODO: Add a unit test!
impl From<Point> for PointI32 {
    fn from(p: Point) -> Self {
        PointI32 {
            x: p.x as i32,
            y: p.y as i32,
        }
    }
}


#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

// TODO: Add a unit test!
impl From<Pixel> for Point {
    fn from(p: Pixel) -> Self {
        Point { x: p.x, y: p.y }
    }
}
// TODO: Add a unit test!
impl From<PointI32> for Point {
    fn from(p: PointI32) -> Self {
        Point {
            x: i32::min(p.x, 0) as u32,
            y: i32::min(p.y, 0) as u32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_from_pixel() {
        let pixel: Pixel = Pixel {
            x: 10,
            y: 20,
            color: 0x00_ff_33_33,
        };

        let point: Point = Point::from(pixel);
        assert_eq!(point.x, 10);
        assert_eq!(point.y, 20);
    }

    #[test]
    fn point_from_point_i32() {
        let point_i32: PointI32 = PointI32 { x: -10, y: -20 };

        let point: Point = Point::from(point_i32);
        assert_eq!(point.x, 0);
        assert_eq!(point.y, 0);

        let point_i32: PointI32 = PointI32 { x: 10, y: 20 };

        let point: Point = Point::from(point_i32);
        assert_eq!(point.x, 10);
        assert_eq!(point.y, 20);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PointF32 {
    pub x: f32,
    pub y: f32,
}

// Implementing conversion from Point to PointF32
impl From<Point> for PointF32 {
    fn from(p: Point) -> Self {
        PointF32 {
            x: p.x as f32,
            y: p.y as f32,
        }
    }
}

// Implementing conversion from PointF32 to Point
impl From<PointF32> for Point {
    fn from(p: PointF32) -> Self {
        Point {
            x: p.x as u32,
            y: p.y as u32,
        }
    }
}
// ========= Point 2D, + conversions from u32 to f32 coordinates and back ==========================
// [ END ] ///////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct RectArea {
    pub top_left: Point,
    pub dimensions: Dimensions2d,
}

#[derive(Debug)]
pub struct Point3D {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

#[derive(Debug)]
pub struct Point3DF32 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Pixel {
    pub x: u32,
    pub y: u32,
    pub color: u32,
}

/// Array of pixels. Each pixel has the `0RGB` model.
pub type ImageData0RGB<'a> = &'a mut [u32];

pub const POINT_ZERO: Point = Point { x: 0, y: 0 };

/// A function that transforms a pixel's color based on its value and coordinates.
///
/// This type alias defines a function type that takes a pixel's color value,
/// its (x, y) coordinates, and the width and height of the whole image data. The function
/// returns a new pixel color value based on the input parameters.
///
/// # Parameters
///
/// - `color`: The original color of the pixel as 0RGB
/// - `x`: The x-coordinate of the pixel in the image.
/// - `y`: The y-coordinate of the pixel in the image.
/// - `w`: The width of the image data in pixels.
/// - `h`: The height of the image data in pixels.
/// - `data`: Any extra u32 values that need to be passed to the color transformer function.
///
/// # Returns
///
/// A 0RGB value representing the transformed pixel
pub type PixelColorTransformerFn =
    fn(color: u32, x: u32, y: u32, w: u32, h: u32, data: Option<&Vec<u32>>) -> u32;

//
// #[derive(Debug)]
// pub struct ImageBuffer<'a> {
//     pub dimensions: &'a  Dimensions2d,
//     pub buf:  VecImageData0RGB<'a>,
// }
