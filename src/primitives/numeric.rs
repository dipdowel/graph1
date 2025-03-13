/// A custom trait that serves as a marker for allowed types.
pub trait Numeric: Clone + Copy /* + PartialOrd + PartialEq */ {
    fn to_f64(self) -> f64;
    fn from_f64(value: f64) -> Self;

    /*
    /// Add two numeric values.
    fn add(self, other: Self) -> Self;
    /// Subtract one numeric value from another.
    fn sub(self, other: Self) -> Self;
    */
}

// Implement `Numeric` for the desired types, using `to_f64` and `from_f64` for conversions.

impl Numeric for u32 {
    fn to_f64(self) -> f64 {
        self as f64
    }
    fn from_f64(value: f64) -> Self {
        f64::round(value)  as u32
    }
    /*
    fn add(self, other: Self) -> Self {
        self + other
    }
    fn sub(self, other: Self) -> Self {
        self - other
    }
     */
}

impl Numeric for usize {
    fn to_f64(self) -> f64 {
        self as f64
    }
    fn from_f64(value: f64) -> Self {
        f64::round(value) as usize
    }
}

impl Numeric for i32 {
    fn to_f64(self) -> f64 {
        self as f64
    }
    fn from_f64(value: f64) -> Self {
        f64::round(value) as i32
    }

    /*
        fn add(self, other: Self) -> Self {
            self + other
        }
        fn sub(self, other: Self) -> Self {
            self - other
        }
    */
}

impl Numeric for f32 {
    fn to_f64(self) -> f64 {
        self as f64
    }
    fn from_f64(value: f64) -> Self {
        value as f32
    }
    /*
        fn add(self, other: Self) -> Self {
            self + other
        }
        fn sub(self, other: Self) -> Self {
            self - other
        }
    */
}

impl Numeric for f64 {
    fn to_f64(self) -> f64 {
        self
    }
    fn from_f64(value: f64) -> Self {
        value
    }
    /*
    fn add(self, other: Self) -> Self {
        self + other
    }
    fn sub(self, other: Self) -> Self {
        self - other
    }
     */
}
