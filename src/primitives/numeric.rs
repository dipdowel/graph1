use std::fmt::{Debug, Display};
use std::ops::{Add, Div, Mul, Rem, RemAssign, Sub};

/// Enum representing concrete numeric types supported by the `Numeric` trait.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumericType {
    U32,
    U64,
    I32,
    F32,
    F64,
    Usize,
}

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
    + Debug
    + Display
{
    fn to_f64(self) -> f64;
    fn from_f64(value: f64) -> Self;

    fn to_u32(self) -> u32;
    fn from_u32(value: u64) -> Self;

    fn to_u64(self) -> u64;
    fn from_u64(value: u64) -> Self;

    fn zero() -> Self;
    fn one() -> Self;

    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }

    fn is_one(&self) -> bool {
        *self == Self::one()
    }

    fn is_unsigned() -> bool;

    fn is_nan(&self) -> bool {
        false
    }

    fn is_integer(&self) -> bool {
        true
    }

    fn get_type() -> NumericType;
}

// === Macro for simple, safe numeric types ===

macro_rules! impl_numeric {
    (
        $t:ty, $variant:expr,
        unsigned = $is_unsigned:expr,
        nan = $is_nan_fn:expr,
        is_integer = $is_integer_fn:expr
    ) => {
        impl Numeric for $t {
            fn to_f64(self) -> f64 {
                self as f64
            }

            fn from_f64(value: f64) -> Self {
                f64::round(value).max(0.0) as Self
            }

            fn from_u32(value: u64) -> Self {
                value as Self
            }

            fn to_u32(self) -> u32 {
                self as u32
            }

            fn to_u64(self) -> u64 {
                self as u64
            }

            fn from_u64(value: u64) -> Self {
                value as Self
            }

            fn zero() -> Self {
                0 as Self
            }

            fn one() -> Self {
                1 as Self
            }

            fn is_unsigned() -> bool {
                $is_unsigned
            }

            fn is_nan(&self) -> bool {
                $is_nan_fn(*self)
            }

            fn is_integer(&self) -> bool {
                $is_integer_fn(*self)
            }

            fn get_type() -> NumericType {
                $variant
            }
        }
    };
}

// === Implementations for safe types ===

impl_numeric!(
    u32,
    NumericType::U32,
    unsigned = true,
    nan = |_| false,
    is_integer = |_| true
);

impl_numeric!(
    u64,
    NumericType::U64,
    unsigned = true,
    nan = |_| false,
    is_integer = |_| true
);

impl_numeric!(
    usize,
    NumericType::Usize,
    unsigned = true,
    nan = |_| false,
    is_integer = |_| true
);

// === Manual impls for types with edge-case logic ===

impl Numeric for i32 {
    fn to_f64(self) -> f64 {
        self as f64
    }

    fn from_f64(value: f64) -> Self {
        f64::round(value) as i32
    }

    fn to_u32(self) -> u32 {
        if self < 0 {
            0
        } else {
            self as u32
        }
    }

    fn from_u32(value: u64) -> Self {
        value as i32
    }

    fn to_u64(self) -> u64 {
        if self < 0 {
            0
        } else {
            self as u64
        }
    }

    fn from_u64(value: u64) -> Self {
        value as i32
    }

    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }

    fn is_unsigned() -> bool {
        false
    }

    fn get_type() -> NumericType {
        NumericType::I32
    }
}

impl Numeric for f32 {
    fn to_f64(self) -> f64 {
        self as f64
    }

    fn from_f64(value: f64) -> Self {
        value as f32
    }

    fn to_u32(self) -> u32 {
        if self < 0.0 {
            0
        } else {
            self as u32
        }
    }

    fn from_u32(value: u64) -> Self {
        value as f32
    }

    fn to_u64(self) -> u64 {
        if self < 0.0 {
            0
        } else {
            self as u64
        }
    }

    fn from_u64(value: u64) -> Self {
        value as f32
    }

    fn zero() -> Self {
        0.0
    }

    fn one() -> Self {
        1.0
    }

    fn is_unsigned() -> bool {
        false
    }

    fn is_nan(&self) -> bool {
        f32::is_nan(*self)
    }

    fn is_integer(&self) -> bool {
        self.fract() == 0.0
    }

    fn get_type() -> NumericType {
        NumericType::F32
    }
}

impl Numeric for f64 {
    fn to_f64(self) -> f64 {
        self
    }

    fn from_f64(value: f64) -> Self {
        value
    }

    fn to_u32(self) -> u32 {
        if self < 0.0 {
            0
        } else {
            self as u32
        }
    }

    fn from_u32(value: u64) -> Self {
        value as f64
    }

    fn to_u64(self) -> u64 {
        if self < 0.0 {
            0
        } else {
            self as u64
        }
    }

    fn from_u64(value: u64) -> Self {
        value as f64
    }

    fn zero() -> Self {
        0.0
    }

    fn one() -> Self {
        1.0
    }

    fn is_unsigned() -> bool {
        false
    }

    fn is_nan(&self) -> bool {
        f64::is_nan(*self)
    }

    fn is_integer(&self) -> bool {
        self.fract() == 0.0
    }

    fn get_type() -> NumericType {
        NumericType::F64
    }
}

// === Tests ===

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

    #[test]
    fn test_numeric_type_identification() {
        assert_eq!(u32::get_type(), NumericType::U32);
        assert_eq!(u64::get_type(), NumericType::U64);
        assert_eq!(i32::get_type(), NumericType::I32);
        assert_eq!(f32::get_type(), NumericType::F32);
        assert_eq!(f64::get_type(), NumericType::F64);
        assert_eq!(usize::get_type(), NumericType::Usize);
    }

    #[test]
    fn test_is_nan_behavior() {
        let x: f64 = f64::NAN;
        assert!(x.is_nan());
        assert!(<f64 as Numeric>::is_nan(&x));

        let y: u32 = 123;
        assert!(!<u32 as Numeric>::is_nan(&y));
    }

    #[test]
    fn test_zero_and_one() {
        assert!(u32::zero().is_zero());
        assert!(u64::one().is_one());
        assert!(!i32::one().is_zero());
        assert!(!f32::zero().is_one());
    }

    #[test]
    fn test_is_integer_for_floats() {
        let whole: f64 = 42.0;
        let frac: f64 = 42.5;
        assert!(whole.is_integer());
        assert!(!frac.is_integer());
    }

    #[test]
    fn test_signed_to_unsigned_safety() {
        let a: i32 = -10;
        assert_eq!(a.to_u32(), 0);
        assert_eq!(a.to_u64(), 0);

        let b: f64 = -123.4;
        assert_eq!(b.to_u32(), 0);
        assert_eq!(b.to_u64(), 0);
    }
}
