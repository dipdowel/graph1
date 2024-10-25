/// A custom trait that serves as a marker for allowed types.
pub trait Numeric {
    fn to_f64(self) -> f64;
    fn from_f64(value: f64) -> Self;
}

/// Implement `Numeric` for the desired types, using `to_f64` and `from_f64` for conversions.
impl Numeric for u32 {
    fn to_f64(self) -> f64 {
        self as f64
    }
    fn from_f64(value: f64) -> Self {
        value as u32
    }
}

impl Numeric for i32 {
    fn to_f64(self) -> f64 {
        self as f64
    }
    fn from_f64(value: f64) -> Self {
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
}

impl Numeric for f64 {
    fn to_f64(self) -> f64 {
        self
    }
    fn from_f64(value: f64) -> Self {
        value
    }
}
