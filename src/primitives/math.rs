use crate::primitives::numeric::Numeric;
use crate::primitives::point::Point;

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

/// A structure representing a 2D displacement vector.
/// It holds horizontal (`dx`) and vertical (`dy`) displacement components.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Displacement<T: Numeric> {
    pub dx: T,
    pub dy: T,
}

impl<T: Numeric> Displacement<T> {
    pub fn new(dx: T, dy: T) -> Self {
        Self { dx, dy }
    }

    /// Applies the displacement to a given point, returning a new point.
    /// The original point remains unchanged.
    pub fn displace(&self, point:&Point<T>) -> Point<T> {
        Point    {
            x: point.x + self.dx,
            y: point.y + self.dy,
        }
    }


    /// Applies the displacement to a given point, modifying it in place.
    /// (The original point is changed)
    pub fn displace_mut(&self, point:&mut Point<T>) {
        point.x = point.x + self.dx;
        point.y = point.y + self.dy;
    }
}

/// A type alias for grid coordinates represented as a point with `usize` components.
pub type GridCoord = Point<usize>;

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

    /// Smoothstep interpolation (3t² - 2t³), smooth at both ends.
    SmoothStep,

    /// Inverse smoothstep interpolation (1 - (3t² - 2t³)), fast at the ends and slow in the middle.
    InverseSmoothStep,

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

/// TODO: check if this belongs here
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorPair(pub u32, pub u32);



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_displacement() {
        let disp = Displacement::new(5, -3);
        let point = Point { x: 10, y: 10 };
        let new_point = disp.displace(&point);
        assert_eq!(new_point, Point { x: 15, y: 7 });

        let mut point_mut = Point { x: 10, y: 10 };
        disp.displace_mut(&mut point_mut);
        assert_eq!(point_mut, Point { x: 15, y: 7 });
    }
    #[test]
    fn test_min_max_contains() {
        let range = MinMax::new(10, 20);
        assert!(range.contains(15, false));
        assert!(!range.contains(10, false));
        assert!(!range.contains(20, false));
        assert!(range.contains(10, true));
        assert!(range.contains(20, true));
    }
    #[test]
    fn test_min_max() {
        let range = MinMax::new(5, 15);
        assert_eq!(range.delta(), 10);
    }
    #[test]
    fn test_range() {
        let range = Range::new(3, 8);
        assert_eq!(range.delta(), 5);
    }
    #[test]
    fn test_shell() {
        let shell = Shell { inner: 2, outer: 5 };
        assert_eq!(shell.inner, 2);
        assert_eq!(shell.outer, 5);
    }
    #[test]
    fn test_bound() {
        let bound = Bound { lower: -1, upper: 1 };
        assert_eq!(bound.lower, -1);
        assert_eq!(bound.upper, 1);
    }
    #[test]
    fn test_color_pair() {
        let colors = ColorPair(0xff0000, 0x00ff00);
        assert_eq!(colors.0, 0xff0000);
        assert_eq!(colors.1, 0x00ff00);
    }
}