use std::fmt::{Debug, Display};
use std::ops::{Add, Div, Mul, Rem, RemAssign, Sub};

/// Enum representing supported numeric types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumericType {
    U16,
    U8,
    U32,
    U64,
    I32,
    F32,
    F64,
    Usize,
}

/// A trait for abstracting over some primitive numeric types (`u32`, `i32`, `f64`, etc.).
///
/// This trait provides:
/// - Standard arithmetic operations
/// - Identity values (`zero`, `one`)
/// - Type classification (`is_unsigned`, `is_nan`, `is_integer`)
/// - Type conversions to/from `f64`, `u32`, and `u64`
///
/// Implementors must define core behavior. Some methods (e.g., conversions from `f64`)
/// have default implementations that panic and should be overridden where supported.
pub trait Numeric:
    PartialEq
    + PartialOrd
    // + Ord  // TODO: Ord is not implemented! Implement it if needed.
    + Clone
    + Copy
    + Debug
    + Display
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    // TODO: Think about adding *Assing traits, like AddAssign, SubAssign, etc.
    + Rem<Output = Self>
    + RemAssign
{
    /// Convert this value to `f64`.
    fn to_f64(self) -> f64;

    /// Convert from an `f64` into this type.
    ///
    /// # Panics
    /// The default implementation will panic. Override where supported.
    fn from_f64(_value: f64) -> Self {
        panic!("from_f64 not supported for this type")
    }

    /// Convert this value to `u32`, possibly clamping or saturating.
    fn to_u32(self) -> u32;

    /// Convert from a `u64` into this type.
    ///
    /// # Panics
    /// The default implementation will panic. Override where supported.
    fn from_u32(_value: u32) -> Self {
        panic!("from_u32 not supported for this type")
    }

    /// Convert this value to `u64`, possibly clamping or saturating.
    fn to_u64(self) -> u64;

    /// Convert from a `u64` into this type.
    ///
    /// # Panics
    /// The default implementation will panic. Override where supported.
    fn from_u64(_value: u64) -> Self {
        panic!("from_u64 not supported for this type")
    }

    /// Convert to i32, with clamping or saturating if needed.
    fn to_i32(self) -> i32 {
        self.to_f64().round().clamp(i32::MIN as f64, i32::MAX as f64) as i32
    }

    /// Returns the additive identity (zero).<br />
    /// 🤓 The additive identity is the element in a number system (or algebraic structure) that,
    /// when added to any other element, leaves that element unchanged.
    /// In most systems, this element is 0.
    fn zero() -> Self;

    /// Returns the multiplicative identity (one).<br />
    /// 🤓 The multiplicative identity is the element that, when multiplied by any other element,
    /// does not change that element. In most systems, this element is 1.
    fn one() -> Self;

    /// Returns `true` if the value is equal to zero.
    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }

    /// Returns `true` if the value is equal to one.
    fn is_one(&self) -> bool {
        *self == Self::one()
    }

    /// Returns `true` if the type is unsigned.
    fn is_unsigned() -> bool;

    /// Returns `true` if the value is NaN (not a number).
    ///
    /// Default implementation returns `false`.
    fn is_nan(&self) -> bool {
        false
    }

    /// Returns `true` if the value represents an integer.
    ///
    /// Default implementation returns `true`.
    fn is_integer(&self) -> bool {
        true
    }

    /// Returns the corresponding `NumericType` enum variant for the implementing type.
    fn get_type() -> NumericType;

    /// TODO: Verify that it actually works as expected!
    fn mini(&self, rhs:Self) -> Self {
        if *self < rhs {
            *self
        } else {
            rhs
        }
    }

    /// TODO: Verify that it actually works as expected!
    fn maxi(&self, rhs: Self) -> Self {
        if *self > rhs {
            *self
        } else {
            rhs
        }
    }

    /// Saturating subtraction.
    /// For integers, saturates at the numeric bounds (never panics).
    /// For floats, just plain subtraction (`self - rhs`).
    fn saturating_sub(self, rhs: Self) -> Self;


}
// TODO: unit tests for u8!
// TODO: unit tests for u8!
// TODO: unit tests for u8!
impl Numeric for u8 {
    fn to_f64(self) -> f64 {
        self as f64
    }
    fn from_f64(value: f64) -> Self {
        value.round().clamp(u8::MIN as f64, u8::MAX as f64) as u8
    }
    fn to_u32(self) -> u32 {
        self as u32
    }
    fn from_u32(value: u32) -> Self {
        value.clamp(u8::MIN as u32, u8::MAX as u32) as u8
    }
    fn to_u64(self) -> u64 {
        self as u64
    }
    fn from_u64(value: u64) -> Self {
        value.clamp(u8::MIN as u64, u8::MAX as u64) as u8
    }
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
    fn is_unsigned() -> bool {
        true
    }
    fn get_type() -> NumericType {
        NumericType::U8
    }
    fn saturating_sub(self, rhs: Self) -> Self {
        u8::saturating_sub(self, rhs)
    }
}

// TODO: unit tests for u16!
// TODO: unit tests for u16!
// TODO: unit tests for u16!
impl Numeric for u16 {
    fn to_f64(self) -> f64 {
        self as f64
    }
    fn from_f64(value: f64) -> Self {
        value.round().clamp(u16::MIN as f64, u16::MAX as f64) as u16
    }
    fn to_u32(self) -> u32 {
        self as u32
    }
    fn from_u32(value: u32) -> Self {
        value.clamp(u16::MIN as u32, u16::MAX as u32) as u16
    }
    fn to_u64(self) -> u64 {
        self as u64
    }
    fn from_u64(value: u64) -> Self {
        value.clamp(u16::MIN as u64, u16::MAX as u64) as u16
    }
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
    fn is_unsigned() -> bool {
        true
    }
    fn get_type() -> NumericType {
        NumericType::U16
    }
    fn saturating_sub(self, rhs: Self) -> Self {
        u16::saturating_sub(self, rhs)
    }
}

impl Numeric for u32 {
    fn to_f64(self) -> f64 {
        self as f64
    }

    fn from_f64(value: f64) -> Self {
        f64::round(value).max(0.0) as u32
    }

    fn to_u32(self) -> u32 {
        self
    }

    fn from_u32(value: u32) -> Self {
        value as u32
    }

    fn to_u64(self) -> u64 {
        self as u64
    }

    fn from_u64(value: u64) -> Self {
        value as u32
    }

    fn to_i32(self) -> i32 {
        self.min(i32::MAX as u32) as i32
    }

    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }

    fn is_unsigned() -> bool {
        true
    }

    fn get_type() -> NumericType {
        NumericType::U32
    }
    fn saturating_sub(self, rhs: Self) -> Self {
        u32::saturating_sub(self, rhs)
    }
}

impl Numeric for u64 {
    fn to_f64(self) -> f64 {
        self as f64
    }

    fn from_f64(value: f64) -> Self {
        f64::round(value).max(0.0) as u64
    }

    fn to_u32(self) -> u32 {
        self as u32
    }

    fn from_u32(value: u32) -> Self {
        value as u64
    }

    fn to_u64(self) -> u64 {
        self
    }

    fn from_u64(value: u64) -> Self {
        value
    }

    fn to_i32(self) -> i32 {
        self.min(i32::MAX as u64) as i32
    }

    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }

    fn is_unsigned() -> bool {
        true
    }

    fn get_type() -> NumericType {
        NumericType::U64
    }

    fn saturating_sub(self, rhs: Self) -> Self {
        u64::saturating_sub(self, rhs)
    }
}

impl Numeric for usize {
    fn to_f64(self) -> f64 {
        self as f64
    }

    fn from_f64(value: f64) -> Self {
        f64::round(value).max(0.0) as usize
    }

    fn to_u32(self) -> u32 {
        self as u32
    }

    fn from_u32(value: u32) -> Self {
        value as usize
    }

    fn to_u64(self) -> u64 {
        self as u64
    }

    fn from_u64(value: u64) -> Self {
        value as usize
    }

    fn to_i32(self) -> i32 {
        self.min(i32::MAX as usize) as i32
    }
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }

    fn is_unsigned() -> bool {
        true
    }

    fn get_type() -> NumericType {
        NumericType::Usize
    }

    fn saturating_sub(self, rhs: Self) -> Self {
        usize::saturating_sub(self, rhs)
    }
}

impl Numeric for i32 {
    fn to_f64(self) -> f64 {
        self as f64
    }

    fn from_f64(value: f64) -> Self {
        f64::round(value) as i32
    }

    /// NB: Negative values are clamped to 0 to avoid underflow.
    fn to_u32(self) -> u32 {
        if self < 0 {
            0
        } else {
            self as u32
        }
    }

    fn from_u32(value: u32) -> Self {
        value.min(i32::MAX as u32) as i32
    }

    /// NB: Negative values are clamped to 0 to avoid underflow.
    fn to_u64(self) -> u64 {
        if self < 0 {
            0
        } else {
            self as u64
        }
    }

    fn from_u64(value: u64) -> Self {
        value.min(i32::MAX as u64) as i32
    }

    fn to_i32(self) -> i32 {
        self
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

    fn saturating_sub(self, rhs: Self) -> Self {
        i32::saturating_sub(self, rhs)
    }
}

impl Numeric for f32 {
    fn to_f64(self) -> f64 {
        self as f64
    }

    fn from_f64(value: f64) -> Self {
        value as f32
    }

    /// NB: Negative values are clamped to 0 to avoid invalid unsigned conversion.
    fn to_u32(self) -> u32 {
        if self < 0.0 {
            0
        } else {
            self as u32
        }
    }

    fn from_u32(value: u32) -> Self {
        value as f32
    }

    /// NB: Negative values are clamped to 0 to avoid invalid unsigned conversion.
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

    fn to_i32(self) -> i32 {
        self.round().clamp(i32::MIN as f32, i32::MAX as f32) as i32
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
        false
    }

    fn get_type() -> NumericType {
        NumericType::F32
    }

    fn saturating_sub(self, rhs: Self) -> Self {
        self - rhs
    }
}

impl Numeric for f64 {
    fn to_f64(self) -> f64 {
        self
    }

    fn from_f64(value: f64) -> Self {
        value
    }

    /// NB: Negative values are clamped to 0 to avoid invalid unsigned conversion.
    fn to_u32(self) -> u32 {
        if self < 0.0 {
            0
        } else {
            self as u32
        }
    }

    fn from_u32(value: u32) -> Self {
        value as f64
    }

    /// NB: Negative values are clamped to 0 to avoid invalid unsigned conversion.
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

    fn to_i32(self) -> i32 {
        self.round().clamp(i32::MIN as f64, i32::MAX as f64) as i32
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
        false
    }

    fn get_type() -> NumericType {
        NumericType::F64
    }

    fn saturating_sub(self, rhs: Self) -> Self {
        self - rhs
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

    // === Tests for potential unsafe casts ===

    #[test]
    fn test_u32_from_overflows() {
        let high_f64 = (u32::MAX as f64) + 1000.0;
        let result = u32::from_f64(high_f64);
        assert_eq!(result, u32::MAX);

        let negative_f64 = -123.4;
        let result = u32::from_f64(negative_f64);
        assert_eq!(result, 0);

        let large_u64 = u64::MAX;
        let result = u32::from_u64(large_u64);
        assert_eq!(result, u32::MAX);

        let result = u32::from_u32(u32::MAX);
        assert_eq!(result, u32::MAX);
    }

    #[test]
    fn test_u64_from_overflows() {
        let high_f64 = (u64::MAX as f64) * 2.0;
        let result = u64::from_f64(high_f64);
        assert_eq!(result, u64::MAX);

        let negative_f64 = -9999.0;
        let result = u64::from_f64(negative_f64);
        assert_eq!(result, 0);

        let result = u64::from_u32(u32::MAX);
        assert_eq!(result, u32::MAX as u64);
    }

    #[test]
    fn test_usize_from_overflows() {
        let high_f64 = (usize::MAX as f64) * 2.0;
        let result = usize::from_f64(high_f64);
        assert_eq!(result, usize::MAX);

        let negative = -1.0;
        let result = usize::from_f64(negative);
        assert_eq!(result, 0);

        let large_u64 = u64::MAX;
        let result = usize::from_u64(large_u64);
        assert_eq!(result, usize::MAX);

        let result = usize::from_u32(u32::MAX);
        assert_eq!(result, u32::MAX as usize);
    }

    #[test]
    fn test_i32_from_overflows() {
        let high_f64 = (i32::MAX as f64) + 1000.0;
        let result = i32::from_f64(high_f64);
        assert_eq!(result, i32::MAX);

        let low_f64 = (i32::MIN as f64) - 1000.0;
        let result = i32::from_f64(low_f64);
        assert_eq!(result, i32::MIN);

        let large_u64 = u64::MAX;
        let result = i32::from_u64(large_u64);
        assert_eq!(result, i32::MAX);

        let result = i32::from_u32(u32::MAX);
        assert_eq!(result, i32::MAX);
    }
    #[test]
    fn test_f32_from_u64_overflow() {
        let large_u64 = u64::MAX;
        let result = f32::from_u64(large_u64);
        assert!(result.is_finite());
        assert!(result <= f32::MAX);
    }

    #[test]
    fn test_f64_from_u64_overflow() {
        let large_u64 = u64::MAX;
        let result = f64::from_u64(large_u64);
        assert!(result.is_finite());
        assert!(result <= f64::MAX);
    }
    // === Clamp tests ===
    #[test]
    fn test_u32_from_f64_clamps() {
        assert_eq!(u32::from_f64(-10.0), 0);
        assert_eq!(u32::from_f64(f64::MAX), u32::MAX);
    }

    #[test]
    fn test_u64_from_f64_clamps() {
        assert_eq!(u64::from_f64(-123.4), 0);
        assert_eq!(u64::from_f64(f64::MAX), u64::MAX);
    }

    #[test]
    fn test_usize_from_f64_clamps() {
        assert_eq!(usize::from_f64(-9999.0), 0);
        assert_eq!(usize::from_f64(f64::MAX), usize::MAX);
    }

    #[test]
    fn test_i32_from_f64_clamps() {
        assert_eq!(i32::from_f64(f64::MIN), i32::MIN);
        assert_eq!(i32::from_f64(f64::MAX), i32::MAX);
    }

    #[test]
    fn test_i32_from_u64_clamps() {
        assert_eq!(i32::from_u64(u64::MAX), i32::MAX);
    }

    ///////////////////////////
    #[test]
    fn test_u32_from_f64_fails_without_clamp() {
        let result = <u32 as Numeric>::from_f64(f64::MAX);
        assert_eq!(result, u32::MAX); // without clamp, this returns 0 or garbage
    }

    #[test]
    fn test_u32_from_u64_fails_without_clamp() {
        let result = <u32 as Numeric>::from_u64(u64::MAX);
        assert_eq!(result, u32::MAX); // unchecked cast may wrap
    }

    #[test]
    fn test_i32_from_f64_fails_without_clamp() {
        let result = <i32 as Numeric>::from_f64(f64::MIN);
        assert_eq!(result, i32::MIN); // without clamp, wraps or overflows
    }

    #[test]
    fn test_usize_from_f64_fails_without_clamp() {
        let result = <usize as Numeric>::from_f64(f64::MAX);
        assert_eq!(result, usize::MAX); // may silently wrap
    }

    #[test]
    fn test_i32_from_u64_fails_without_clamp() {
        let result = <i32 as Numeric>::from_u64(u64::MAX);
        assert_eq!(result, i32::MAX); // unchecked, might overflow
    }

    #[test]
    fn test_to_i32_clamping() {
        assert_eq!(123_u32.to_i32(), 123);
        assert_eq!(u64::MAX.to_i32(), i32::MAX);
        assert_eq!((-999.9_f64).to_i32(), -1000);
        assert_eq!(9999999999_f64.to_i32(), i32::MAX);
        assert_eq!((-9999999999_f64).to_i32(), i32::MIN);
        assert_eq!(i32::MAX.to_i32(), i32::MAX);
        assert_eq!(i32::MIN.to_i32(), i32::MIN);
        assert_eq!((123.8_f32).to_i32(), 124);
        assert_eq!((-1.9_f32).to_i32(), -2);
    }
    #[test]
    fn test_numeric_mini_maxi() {
        // Integers
        let a: u32 = 10;
        let b: u32 = 20;
        assert_eq!(a.mini(b), 10);
        assert_eq!(a.maxi(b), 20);
        assert_eq!(b.mini(a), 10);
        assert_eq!(b.maxi(a), 20);
        assert_eq!(a.mini(a), 10);
        assert_eq!(b.maxi(b), 20);

        let x: i32 = -5;
        let y: i32 = 15;
        assert_eq!(x.mini(y), -5);
        assert_eq!(x.maxi(y), 15);

        // Equal signed integers
        let i: i32 = -123;
        assert_eq!(i.mini(i), i);
        assert_eq!(i.maxi(i), i);

        // Floats
        let f1: f32 = 1.5;
        let f2: f32 = 2.5;
        assert!((f1.mini(f2) - 1.5).abs() < f32::EPSILON);
        assert!((f1.maxi(f2) - 2.5).abs() < f32::EPSILON);

        let f3: f64 = -100.0;
        let f4: f64 = -200.0;
        assert!((f3.mini(f4) + 200.0).abs() < f64::EPSILON);
        assert!((f3.maxi(f4) + 100.0).abs() < f64::EPSILON);

        // Equal floats
        let f_equal_f32: f32 = 3.1415;
        assert!((f_equal_f32.mini(f_equal_f32) - f_equal_f32).abs() < f32::EPSILON);
        assert!((f_equal_f32.maxi(f_equal_f32) - f_equal_f32).abs() < f32::EPSILON);

        let f_equal_f64: f64 = -42.42;
        assert!((f_equal_f64.mini(f_equal_f64) - f_equal_f64).abs() < f64::EPSILON);
        assert!((f_equal_f64.maxi(f_equal_f64) - f_equal_f64).abs() < f64::EPSILON);

        // Equal values (unsigned)
        let z: usize = 42;
        assert_eq!(z.mini(z), 42);
        assert_eq!(z.maxi(z), 42);
    }
}
