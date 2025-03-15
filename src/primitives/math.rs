use crate::primitives::numeric::Numeric;


pub struct Range<T: Numeric> {
    pub start: T,
    pub end: T,    
}

impl <T: Numeric> Range<T> {
    pub fn new(start: T, end: T) -> Self {
        Self { start, end }
    }
}



pub struct MinMax<T: Numeric> {
    pub min: T,
    pub max: T,
}
impl <T: Numeric> MinMax<T> {
    pub fn new(min: T, max: T) -> Self {
        Self { min, max }
    }
}