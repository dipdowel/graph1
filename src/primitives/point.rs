use crate::primitives::numeric::Numeric;
use crate::primitives::Pixel;
use std::ops::{Add, Sub};

/// A generic 2D-point: `(x: u32, y:u32)`. Other `Numeric` types can be used instead of `u32`.
///
/// `T` represents the numeric type for `x` and `y`, limited to `u32`, `i32`, `f32`, or `f64`.<br />
/// Use the `convert` method to convert between different numeric types of Point,<br />
/// e.g. `let point_f64: Point<f64> = point_i32.convert();`<br />
/// Be aware that precision loss may occur during conversion!<br />
/// **NB:** For a note on performance see documentation for `convert()` method.
#[derive(Debug, Clone, Copy, PartialEq)]

pub struct Point<T: Numeric = u32> {
    pub x: T,
    pub y: T,
}

impl<T: Numeric> Point<T> {
    /// Converts the coordinates of a 2D-point to a different numeric type `U`.
    ///
    /// This method allows conversion between `Point<T>` types.<br />
    /// - **NB 1:** Precision loss may occur during conversion. <br />
    /// - **NB 2:** When a type with negative coordinates is converted to an unsigned type,<br />
    /// the negative values are truncated to 0. <br />
    /// - **NB 3:** When converting a point between different `Numeric` types using `convert()`,<br />
    /// keep in mind that it is a two-step conversion for every coordinate: `T -> f64 -> U`. <br />
    /// This may result in slower performance, so use it only in non-performance-critical code. <br />
    /// If performance is critical, consider implementing your own faster direct conversion.
    pub fn convert<U: Numeric>(self) -> Point<U> {
        Point {
            x: U::from_f64(self.x.to_f64()),
            y: U::from_f64(self.y.to_f64()),
        }
    }

    /// Converts the point to a `Pixel` with the given `color`.
    /// Please use this function only in non-performance-critical code.
    /// If performance is critical, consider implementing a faster  conversion yourself.
    /// The `x` and `y` coordinates are rounded to the nearest integer.
    /// # Parameters
    /// - `color`: The color of the pixel.
    pub fn to_pixel(self, color: u32) -> Pixel {
        Pixel {
            x: f64::round(self.x.to_f64()) as u32,
            y: f64::round(self.y.to_f64()) as u32,
            color,
        }
    }

    /// Creates a new `Point` with the given `x` and `y` coordinates.
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    /// Calculates the Euclidian distance to another point.
    pub fn distance_to(&self, other: &Point<T>) -> f64 {
        let dx = self.x.to_f64() - other.x.to_f64();
        let dy = self.y.to_f64() - other.y.to_f64();
        (dx * dx + dy * dy).sqrt()
    }


    /// Computes the interior collinear point along the line segment from `self` to `other`.
    ///
    /// This method computes a new point `c` on the line segment from `self` to `other`,
    /// such that the distance from `self` to `c` is a given fraction `t` (clamped to [0.0, 1.0])
    /// of the full segment length.
    ///
    /// For `t == 0.0`, returns a clone of `self`.
    /// For `t == 1.0`, returns a clone of `other`.
    ///
    /// # Parameters
    /// - `other`: The other point that forms the segment with `self`.
    /// - `t`: A fraction representing the relative position between the two points.
    ///         Values outside [0.0, 1.0] are clamped.
    ///
    /// # Returns
    /// - A new `Point<T>` located `t`-fraction along the segment from `self` to `other`.
    pub fn collinear_interior(&self, other: &Point<T>, t: f64) -> Point<T> {
        let t_clamped = t.clamp(0.0, 1.0);

        if t_clamped == 0.0 {
            self.clone()
        } else if t_clamped == 1.0 {
            other.clone()
        } else {
            let x = self.x.to_f64() + t_clamped * (other.x.to_f64() - self.x.to_f64());
            let y = self.y.to_f64() + t_clamped * (other.y.to_f64() - self.y.to_f64());
            Point {
                x: T::from_f64(x),
                y: T::from_f64(y),
            }
        }
        // TODO:
        // TODO: WRITE UNIT TESTS FOR THIS METHOD!
        // TODO: FOR REAL...
        // TODO:
    }

}

impl<T: Numeric> Default for Point<T> {
    fn default() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
        }
    }
}

impl<T: Numeric> Add for Point<T> {
    type Output = Self;

    /// Adds two points component-wise.
    /// Returns a new point where x = self.x + rhs.x and y = self.y + rhs.y.
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Numeric> Sub for Point<T> {
    type Output = Self;

    /// Subtracts two points component-wise.
    /// Returns a new point where x = self.x - rhs.x and y = self.y - rhs.y.
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Numeric> Point<T> {
    pub fn saturating_sub(self, rhs: Point<T>) -> Point<T> {
        Point {
            x: self.x.saturating_sub(rhs.x),
            y: self.y.saturating_sub(rhs.y),
        }
    }
}

/// An often-used constant for a point at the origin (0, 0).
pub const POINT_ZERO: Point = Point { x: 0, y: 0 };
pub const POINT_ONE: Point = Point { x: 1, y: 1 };

/// A generic 3D-point: `(x: u32, y:u32, z:u32)`. Other `Numeric` types can be used instead of `u32`.
///
/// `T` represents the numeric type for `x`, `y` and `z`, limited to `u32`, `i32`, `f32`, or `f64`.<br />
/// Use the `convert` method to convert between different numeric types of Point3D,<br />
/// e.g. `let point3d_f64: Point3d<f64> = point3d_i32.convert();`<br />
/// Be aware that precision loss may occur during conversion!<br />
/// **NB:** For a note on performance see documentation for `convert()` method.
#[derive(Debug, Clone, Copy)]
pub struct Point3D<T: Numeric = u32> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Numeric> Point3D<T> {
    /// Converts the coordinates of a 3D-point to a different numeric type `U`.
    ///
    /// This method allows conversion between `Point<T>` types.<br />
    /// - **NB 1:** Precision loss may occur during conversion. <br />
    /// - **NB 2:** When a type with negative coordinates is converted to an unsigned type,<br />
    /// the negative values are truncated to 0. <br />
    /// - **NB 3:** When converting a point between different `Numeric` types using `convert()`,<br />
    /// keep in mind that it is a two-step conversion for every coordinate: `T -> f64 -> U`. <br />
    /// This may result in slower performance, so use it only in non-performance-critical code. <br />
    /// If performance is critical, consider implementing your own faster direct conversion.
    pub fn convert<U: Numeric>(self) -> Point3D<U> {
        Point3D {
            x: U::from_f64(self.x.to_f64()),
            y: U::from_f64(self.y.to_f64()),
            z: U::from_f64(self.z.to_f64()),
        }
    }

    /// Creates a new `Point3D` with the given `x`, `y` and `z` coordinates.
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

/// An often-used constant for a 3D-point at the origin (0, 0, 0).
pub const POINT_3D_ZERO: Point3D = Point3D { x: 0, y: 0, z: 0 };

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_points() {
        let a = Point::new(1, 2);
        let b = Point::new(3, 4);
        let expected = Point::new(4, 6);
        assert_eq!(a + b, expected);
    }
    #[test]
    fn test_add_points_float() {
        let a = Point::new(1.0, 2.0);
        let b = Point::new(3.1, 4.1);
        let expected = Point::new(4.1, 6.1);
        assert_eq!(a + b, expected);
    }

    #[test]
    fn test_sub_points() {
        let a = Point::new(5, 7);
        let b = Point::new(2, 3);
        let expected = Point::new(3, 4);
        assert_eq!(a - b, expected);
    }
    #[test]
    fn test_sub_points_float() {
        let a = Point::new(5.2, 7.2);
        let b = Point::new(2.2, 3.2);
        let expected = Point::new(3.0, 4.0);
        assert_eq!(a - b, expected);
    }

    #[test]
    fn test_to_pixel() {
        let mut pixel: Pixel;
        pixel = Point {
            x: 10.8_f64,
            y: -20.1_f64,
        }
        .to_pixel(0xff_ff_ff_ff);
        assert_eq!(
            pixel,
            Pixel {
                x: 11,
                y: 0,
                color: 0xff_ff_ff_ff
            }
        );

        pixel = Point {
            x: 10.8_f32,
            y: -20.1_f32,
        }
        .to_pixel(0xff_ff_ff_ff);
        assert_eq!(
            pixel,
            Pixel {
                x: 11,
                y: 0,
                color: 0xff_ff_ff_ff
            }
        );

        pixel = Point {
            x: -20_i32,
            y: 30_i32,
        }
        .to_pixel(0xff_ff_00_ff);
        assert_eq!(
            pixel,
            Pixel {
                x: 0,
                y: 30,
                color: 0xff_ff_00_ff
            }
        );

        pixel = Point {
            x: 33_u32,
            y: 88_u32,
        }
        .to_pixel(0xff_ff_00_ff);
        assert_eq!(
            pixel,
            Pixel {
                x: 33,
                y: 88,
                color: 0xff_ff_00_ff
            }
        );
    }
    #[test]
    fn test_convert_negative_i32_to_u32() {
        let p: Point<i32> = Point::new(-5, -10);
        let converted: Point<u32> = p.convert();
        assert_eq!(converted, Point::new(0, 0)); // negatives get clamped
    }
    #[test]
    fn test_convert_f64_to_f32_precision_loss() {
        let p = Point::new(1.123456789_f64, 2.987654321_f64);
        let converted: Point<f32> = p.convert();
        assert!((converted.x - 1.1234567).abs() < 1e-6);
        assert!((converted.y - 2.987654).abs() < 1e-6);
    }

    #[test]
    fn test_to_pixel_rounding_behavior() {
        let p = Point::new(2.49_f64, 3.51);
        let pix = p.to_pixel(0xff00ff);
        assert_eq!(pix.x, 2);
        assert_eq!(pix.y, 4);
    }
}
