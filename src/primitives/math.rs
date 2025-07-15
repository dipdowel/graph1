use crate::primitives::numeric::Numeric;

/// A structure that holds the minimum and maximum values of a type.
///
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MinMax<T: Numeric> {
    pub min: T,
    pub max: T,
}
impl<T: Numeric> MinMax<T> {
    pub fn new(min: T, max: T) -> Self {
        Self { min, max }
    }
    pub fn delta(&self) -> T {
        self.max - self.min
    }
    pub fn contains(&self, value: T, inclusive:bool) -> bool {
        if inclusive {
            value >= self.min && value <= self.max
        } else {
            value > self.min && value < self.max
        }
    }
}

/// Interpolation methods for shaping a scaled transition curve.
/// Optional parameters allow customizing the curve steepness or shape.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Interpolation {
    /// Linear interpolation (no easing).
    Linear,

    /// Exponential "ease-in" curve (starts slow, ends fast).
    /// `factor` controls steepness. Default = 2.0 (quadratic).
    ExpIn(Option<f64>),

    /// Exponential "ease-out" curve (starts fast, ends slow).
    /// `factor` controls steepness. Default = 2.0 (quadratic).
    ExpOut(Option<f64>),

    /// Symmetric exponential ease-in-out.
    /// Starts slow, speeds up in the middle, then slows down again.
    /// `factor` controls steepness. Default = 2.0.
    ExpInOut(Option<f64>),

    /// Smoothstep interpolation (`3t² - 2t³`), smooth at both ends.
    SmoothStep,

    /// Sigmoid interpolation (S-shaped), models perceptual tapering.
    /// `steepness` controls sharpness. Default = 12.0.
    Sigmoid(Option<f64>),
}


pub const MIN_MAX_U64: MinMax<u64> = MinMax {
    min: 0,
    max: u64::MAX,
};
pub const MIN_MAX_U32: MinMax<u32> = MinMax {
    min: 0,
    max: u32::MAX,
};
pub const MIN_MAX_I32: MinMax<i32> = MinMax {
    min: i32::MIN,
    max: i32::MAX,
};
pub const MIN_MAX_F32: MinMax<f32> = MinMax {
    min: f32::MIN,
    max: f32::MAX,
};
pub const MIN_MAX_F64: MinMax<f64> = MinMax {
    min: f64::MIN,
    max: f64::MAX,
};

/// A range of values.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Range<T: Numeric> {
    pub start: T,
    pub end: T,
}

impl<T: Numeric> Range<T> {
    pub fn new(start: T, end: T) -> Self {
        Self { start, end }
    }
    pub fn delta(&self) -> T {
        self.end - self.start
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Shell<T: Numeric> {
    pub inner: T,
    pub outer: T,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bound<T: Numeric> {
    pub lower: T,
    pub upper: T,
}

/// A range of values.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorPair(pub u32, pub u32);
