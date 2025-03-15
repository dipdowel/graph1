use crate::primitives::numeric::Numeric;

/// A range of values.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Range<T: Numeric> {
    pub start: T,
    pub end: T,    
}

impl <T: Numeric> Range<T> {
    pub fn new(start: T, end: T) -> Self {
        Self { start, end }
    }
}



/// A structure that holds the minimum and maximum values of a type.
///
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MinMax<T: Numeric> {
    pub min: T,
    pub max: T,
}
impl <T: Numeric> MinMax<T> {
    pub fn new(min: T, max: T) -> Self {
        Self { min, max }
    }
}

pub const MIN_MAX_U64: MinMax<u64> = MinMax { min: 0, max: u64::MAX };
pub const MIN_MAX_U32: MinMax<u32> = MinMax { min: 0, max: u32::MAX };
pub const MIN_MAX_I32: MinMax<i32> = MinMax { min: i32::MIN, max: i32::MAX };
pub const MIN_MAX_F32: MinMax<f32> = MinMax { min: f32::MIN, max: f32::MAX };
pub const MIN_MAX_F64: MinMax<f64> = MinMax { min: f64::MIN, max: f64::MAX };
