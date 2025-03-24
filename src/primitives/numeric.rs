use std::ops::{Add, Div, Mul, Rem, RemAssign, Sub};

/// A custom trait that serves as a marker for allowed types.
// pub trait Numeric: Clone + Copy + PartialOrd + PartialEq + Sub + Add + Mul + Div + Rem + RemAssign  {
pub trait Numeric:
    PartialEq
    + PartialOrd
    + Clone
    + Copy
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + RemAssign

{
    fn to_f64(self) -> f64;
    fn from_f64(value: f64) -> Self;

    fn to_u64(self) -> u64;
    fn from_u64(value: u64) -> Self;
}

// Implement `Numeric` for the desired types, using `to_f64` and `from_f64` for conversions.

impl Numeric for u32 {
    fn to_f64(self) -> f64 {
        self as f64
    }
    fn from_f64(value: f64) -> Self {
        f64::round(value) as u32
    }
    fn to_u64(self) -> u64 {
        self as u64
    }
    fn from_u64(value: u64) -> Self {
        value as u32
    }
}

impl Numeric for usize {
    fn to_f64(self) -> f64 {
        self as f64
    }
    fn from_f64(value: f64) -> Self {
        f64::round(value) as usize
    }
    fn to_u64(self) -> u64 {
        self as u64
    }
    fn from_u64(value: u64) -> Self {
        value as usize
    }
}

impl Numeric for i32 {
    fn to_f64(self) -> f64 {
        self as f64
    }
    fn from_f64(value: f64) -> Self {
        f64::round(value) as i32
    }

    /// NB: Negative values are truncated to 0.
    fn to_u64(self) -> u64 {
        if self < 0 {
            return 0;
        }
        self as u64
    }
    fn from_u64(value: u64) -> Self {
        value as i32
    }
}

impl Numeric for f32 {
    fn to_f64(self) -> f64 {
        self as f64
    }
    fn from_f64(value: f64) -> Self {
        value as f32
    }

    /// NB: Negative values are truncated to 0.
    fn to_u64(self) -> u64 {
        if self < 0.0 {
            return 0;
        }
        self as u64
    }
    fn from_u64(value: u64) -> Self {
        value as f32
    }
}

impl Numeric for f64 {
    fn to_f64(self) -> f64 {
        self
    }
    fn from_f64(value: f64) -> Self {
        value
    }
    /// NB: Negative values are truncated to 0.
    fn to_u64(self) -> u64 {
        if self < 0.0 {
            return 0;
        }

        self as u64
    }
    fn from_u64(value: u64) -> Self {
        value as f64
    }
}

impl Numeric for u64 {
    fn to_f64(self) -> f64 {
        self as f64
    }
    fn from_f64(value: f64) -> Self {
        f64::round(value) as u64
    }
    fn to_u64(self) -> u64 {
        self
    }
    fn from_u64(value: u64) -> Self {
        value
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arithmetic_u32() {
        let a: u32 = 10;
        let b: u32 = 3;
        assert_eq!(a + b, 13);
        assert_eq!(a - b, 7);
        assert_eq!(a * b, 30);
        assert_eq!(a / b, 3);
        assert_eq!(a % b, 1);

        let mut r = a;
        r %= 4;
        assert_eq!(r, 2);
    }

    #[test]
    fn test_arithmetic_f32() {
        let a: f32 = 10.0;
        let b: f32 = 4.0;
        assert!((a + b - 14.0).abs() < f32::EPSILON);
        assert!((a - b - 6.0).abs() < f32::EPSILON);
        assert!((a * b - 40.0).abs() < f32::EPSILON);
        assert!((a / b - 2.5).abs() < f32::EPSILON);
        assert!((a % b - 2.0).abs() < f32::EPSILON);

        let mut r = a;
        r %= 3.0;
        assert!((r - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_numeric_u32_conversions() {
        let value: u32 = 10;

        let as_f64 = value.to_f64();
        assert_eq!(as_f64, 10.0);

        let from_f64 = u32::from_f64(10.7);
        assert_eq!(from_f64, 11);

        let from_negative_f64 = u32::from_f64(-3.2);
        assert_eq!(from_negative_f64, 0); // negative -> 0
    }

    #[test]
    fn test_numeric_i32_conversions() {
        let value: i32 = -20;

        let as_f64 = value.to_f64();
        assert_eq!(as_f64, -20.0);

        let from_f64 = i32::from_f64(-20.6);
        assert_eq!(from_f64, -21);

        let to_u64 = value.to_u64();
        assert_eq!(to_u64, 0); // negative -> 0

        let from_u64 = i32::from_u64(123);
        assert_eq!(from_u64, 123);
    }

    #[test]
    fn test_numeric_f32_conversions() {
        let value: f32 = 10.5;

        let as_f64 = value.to_f64();
        assert!((as_f64 - 10.5).abs() < f64::EPSILON);

        let from_f64 = f32::from_f64(42.25);
        assert!((from_f64 - 42.25).abs() < f32::EPSILON);

        let from_negative = f32::to_u64(-1.1);
        assert_eq!(from_negative, 0);
    }

    #[test]
    fn test_numeric_f64_conversions() {
        let value: f64 = -20.1;

        let as_f64 = value.to_f64();
        assert_eq!(as_f64, -20.1);

        let to_u64 = value.to_u64();
        assert_eq!(to_u64, 0); // negative → 0

        let from_u64 = f64::from_u64(100);
        assert_eq!(from_u64, 100.0);
    }

}
