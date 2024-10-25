use crate::primitives::numeric::Numeric;

/// A generic 2D-point, defaults to `u32` for `x` and `y`.
///
/// `T` represents the numeric type for `x` and `y`, limited to `u32`, `i32`, `f32`, or `f64`.
#[derive(Debug, Clone, Copy)]
pub struct Point<T: Numeric = u32> {
    pub x: T,
    pub y: T,
}

impl<T: Numeric> Point<T> {
    /// Converts the point to a different numeric type `U`.
    ///
    /// This method allows conversion between `Point<T>` types.
    /// Precision loss may occur during conversion.
    pub fn convert<U: Numeric>(self) -> Point<U> {
        Point {
            x: U::from_f64(self.x.to_f64()),
            y: U::from_f64(self.y.to_f64()),
        }
    }
}

/// An often-used constant for a point at the origin (0, 0).
pub const POINT_ZERO: Point = Point { x: 0, y: 0 };

/// A generic 2D-point, defaults to `u32` for `x` and `y`.
///
/// `T` represents the numeric type for `x` and `y`, limited to `u32`, `i32`, `f32`, or `f64`.
#[derive(Debug, Clone, Copy)]
pub struct Point3D<T: Numeric = u32> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Numeric> Point3D<T> {
    /// Converts the 3D-point to a different numeric type `U`.
    ///
    /// This method allows conversion between `Point3D<T>` types.
    /// Precision loss may occur during conversion.
    pub fn convert<U: Numeric>(self) -> Point3D<U> {
        Point3D {
            x: U::from_f64(self.x.to_f64()),
            y: U::from_f64(self.y.to_f64()),
            z: U::from_f64(self.z.to_f64()),
        }
    }
}


/// An often-used constant for a 3D-point at the origin (0, 0, 0).
pub const POINT_3D_ZERO: Point3D = Point3D { x: 0, y: 0, z: 0 };


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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u32_to_other_types() {
        let point_u32 = Point { x: 10_u32, y: 20_u32 };

        // Convert to i32
        let point_i32: Point<i32> = point_u32.convert();
        assert_eq!(point_i32.x, 10);
        assert_eq!(point_i32.y, 20);

        // Convert to f32
        let point_f32: Point<f32> = point_u32.convert();
        assert_eq!(point_f32.x, 10.0);
        assert_eq!(point_f32.y, 20.0);

        // Convert to f64
        let point_f64: Point<f64> = point_u32.convert();
        assert_eq!(point_f64.x, 10.0);
        assert_eq!(point_f64.y, 20.0);
    }

    #[test]
    fn test_i32_to_other_types() {
        let point_i32 = Point { x: 10_i32, y: -20_i32 };

        // Convert to u32 (negative values become 0 due to casting)
        let point_u32: Point<u32> = point_i32.convert();
        assert_eq!(point_u32.x, 10);
        assert_eq!(point_u32.y, 0); // -20 truncated to 0

        // Convert to f32
        let point_f32: Point<f32> = point_i32.convert();
        assert_eq!(point_f32.x, 10.0);
        assert_eq!(point_f32.y, -20.0);

        // Convert to f64
        let point_f64: Point<f64> = point_i32.convert();
        assert_eq!(point_f64.x, 10.0);
        assert_eq!(point_f64.y, -20.0);
    }

    #[test]
    fn test_f32_to_other_types() {
        let point_f32 = Point { x: 10.5_f32, y: -20.7_f32 };

        // Convert to u32 (fractional and negative values truncated)
        let point_u32: Point<u32> = point_f32.convert();
        assert_eq!(point_u32.x, 10); // 10.5 truncated to 10
        assert_eq!(point_u32.y, 0);  // -20.7 truncated to 0

        // Convert to i32
        let point_i32: Point<i32> = point_f32.convert();
        assert_eq!(point_i32.x, 10);  // 10.5 truncated to 10
        assert_eq!(point_i32.y, -20); // -20.7 truncated to -20

        // // Convert to f64 (these fail due to floating-point precision)
        // let point_f64: Point<f64> = point_f32.convert();
        // assert!((point_f64.x - 10.5).abs() < f64::EPSILON);
        // assert!((point_f64.y + 20.7).abs() < f64::EPSILON);

        // Convert to f64 (with a tolerance for floating-point precision)
        let point_f64: Point<f64> = point_f32.convert();
        let tolerance = 1e-6; // 0.000001 -- a tolerance value suitable for `f32` to `f64` comparison
        assert!((point_f64.x - 10.5).abs() < tolerance);
        assert!((point_f64.y + 20.7).abs() < tolerance);

    }

    #[test]
    fn test_f64_to_other_types() {
        let point_f64 = Point { x: 10.9_f64, y: -20.1_f64 };

        // Convert to u32 (fractional and negative values truncated)
        let point_u32: Point<u32> = point_f64.convert();
        assert_eq!(point_u32.x, 10); // 10.9 truncated to 10
        assert_eq!(point_u32.y, 0);  // -20.1 truncated to 0

        // Convert to i32
        let point_i32: Point<i32> = point_f64.convert();
        assert_eq!(point_i32.x, 10);  // 10.9 truncated to 10
        assert_eq!(point_i32.y, -20); // -20.1 truncated to -20

        // Convert to f32
        let point_f32: Point<f32> = point_f64.convert();
        assert!((point_f32.x - 10.9).abs() < f32::EPSILON);
        assert!((point_f32.y + 20.1).abs() < f32::EPSILON);
    }
}

