use crate::primitives::numeric::Numeric;

/// A generic 2D-point, defaults to `u32` for `x` and `y`.
///
/// `T` represents the numeric type for `x` and `y`, limited to `u32`, `i32`, `f32`, or `f64`.<br />
/// Use the `convert` method to convert between different numeric types of Point,<br />
/// e.g. `let point_f64: Point<f64> = point_i32.convert();`<br />
/// Be aware that precision loss may occur during conversion!<br />
/// **NB:** For a note on performance see documentation for `convert()` method.
#[derive(Debug, Clone, Copy)]
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
    /// If performance is critical, consider implementing a faster direct conversion by yourself.
    pub fn convert<U: Numeric>(self) -> Point<U> {
        Point {
            x: U::from_f64(self.x.to_f64()),
            y: U::from_f64(self.y.to_f64()),
        }
    }

    /// Creates a new `Point` with the given `x` and `y` coordinates.
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

/// An often-used constant for a point at the origin (0, 0).
pub const POINT_ZERO: Point = Point { x: 0, y: 0 };



/// A generic 3D-point, defaults to `u32` for `x`, `y` and `z`.
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
    /// If performance is critical, consider implementing a faster direct conversion by yourself.
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
    fn test_u32_to_other_types() {
        let point_u32 = Point {
            x: 10_u32,
            y: 20_u32,
        };

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
        let point_i32 = Point {
            x: 10_i32,
            y: -20_i32,
        };

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
        let point_f32 = Point {
            x: 10.5_f32,
            y: -20.7_f32,
        };

        // Convert to u32 (fractional and negative values truncated)
        let point_u32: Point<u32> = point_f32.convert();
        assert_eq!(point_u32.x, 10); // 10.5 truncated to 10
        assert_eq!(point_u32.y, 0); // -20.7 truncated to 0

        // Convert to i32
        let point_i32: Point<i32> = point_f32.convert();
        assert_eq!(point_i32.x, 10); // 10.5 truncated to 10
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
        let point_f64 = Point {
            x: 10.9_f64,
            y: -20.1_f64,
        };

        // Convert to u32 (fractional and negative values truncated)
        let point_u32: Point<u32> = point_f64.convert();
        assert_eq!(point_u32.x, 10); // 10.9 truncated to 10
        assert_eq!(point_u32.y, 0); // -20.1 truncated to 0

        // Convert to i32
        let point_i32: Point<i32> = point_f64.convert();
        assert_eq!(point_i32.x, 10); // 10.9 truncated to 10
        assert_eq!(point_i32.y, -20); // -20.1 truncated to -20

        // Convert to f32
        let point_f32: Point<f32> = point_f64.convert();
        assert!((point_f32.x - 10.9).abs() < f32::EPSILON);
        assert!((point_f32.y + 20.1).abs() < f32::EPSILON);
    }
}
