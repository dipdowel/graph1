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
