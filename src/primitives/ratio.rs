use crate::primitives::numeric::Numeric;
use crate::utils::math::gcd::gcd;
use std::fmt;

/// A struct representing a simple ratio of two `Numeric` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ratio<T: Numeric> {
    pub numerator: T,
    pub denominator: T,
}

impl<T: Numeric> Ratio<T> {
    /// Creates a new `Ratio`, panicking if denominator is zero.
    pub fn new(numerator: T, denominator: T) -> Self {
        assert_ne!(denominator, T::zero(), "Denominator cannot be zero");
        Self {
            numerator,
            denominator,
        }
    }

    /// Returns the decimal representation of the ratio.
    pub fn as_f64(&self) -> f64 {
        self.numerator.to_f64() / self.denominator.to_f64()
    }

    /// Returns the percentage representation (0.0–100.0).
    pub fn as_percent(&self) -> f64 {
        self.as_f64() * 100.0
    }

    /// Returns a simplified version of the ratio using the GCD.
    pub fn simplified(&self) -> Self {
        let gcd = gcd(self.numerator, self.denominator);
        Self {
            numerator: self.numerator / gcd,
            denominator: self.denominator / gcd,
        }
    }

    /// Returns a new `Ratio` scaled by the given factor.
    pub fn scaled(&self, factor: T) -> Self {
        Self {
            numerator: self.numerator * factor,
            denominator: self.denominator * factor,
        }
    }

    /// Returns the inverse of the ratio (flips numerator and denominator).
    pub fn inverted(&self) -> Self {
        assert_ne!(self.numerator, T::zero(), "Cannot invert a zero numerator");
        Self {
            numerator: self.denominator,
            denominator: self.numerator,
        }
    }

    /// Compares this ratio to another.
    pub fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.numerator.to_u64() * other.denominator.to_u64())
            .cmp(&(other.numerator.to_u64() * self.denominator.to_u64()))
    }

    /// Converts the ratio to a (width, height) pair based on a base denominator value.
    pub fn to_dimensions(&self, base: T) -> (T, T) {
        let width = self.numerator * base / self.denominator;
        (width, base)
    }

    /// Returns whether this ratio matches the given width and height.
    pub fn matches_aspect(&self, width: T, height: T) -> bool {
        let simplified = self.simplified();
        let other = Self::new(width, height).simplified();
        simplified == other
    }

    /// Returns whether this is a square ratio (1:1).
    pub fn is_square(&self) -> bool {
        self.numerator == self.denominator
    }


}

impl<T: Numeric> fmt::Display for Ratio<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.numerator.is_integer() {
            return write!(f, "Ratio {}:{}", self.numerator, self.denominator);
        }
        write!(
            f,
            "{:.4}...:{:.4}...",
            self.numerator.to_f64(),
            self.denominator.to_f64()
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::primitives::ratio::Ratio;

    #[test]
    fn test_creation_and_display() {
        let r = Ratio::new(4u32, 3u32);
        assert_eq!(format!("{}", r), "4:3");
    }

    #[test]
    #[should_panic(expected = "Denominator cannot be zero")]
    fn test_creation_zero_denominator() {
        let _ = Ratio::new(5u32, 0u32);
    }

    #[test]
    fn test_as_f64_and_percent() {
        let r = Ratio::new(1u32, 4u32);
        assert_eq!(r.as_f64(), 0.25);
        assert_eq!(r.as_percent(), 25.0);
    }

    #[test]
    fn test_simplified() {
        let r = Ratio::new(10u32, 20u32);
        let simplified = r.simplified();
        assert_eq!(simplified.numerator, 1u32);
        assert_eq!(simplified.denominator, 2u32);
    }

    #[test]
    fn test_scaled() {
        let r = Ratio::new(2u32, 3u32);
        let scaled = r.scaled(2u32);
        assert_eq!(scaled.numerator, 4u32);
        assert_eq!(scaled.denominator, 6u32);
    }

    #[test]
    fn test_inverted() {
        let r = Ratio::new(2u32, 5u32);
        let inv = r.inverted();
        assert_eq!(inv.numerator, 5u32);
        assert_eq!(inv.denominator, 2u32);
    }

    #[test]
    #[should_panic(expected = "Cannot invert a zero numerator")]
    fn test_inverted_panics_on_zero_numerator() {
        let r = Ratio::new(0u32, 1u32);
        let _ = r.inverted();
    }

    #[test]
    fn test_cmp_ordering() {
        let a = Ratio::new(1u32, 3u32);
        let b = Ratio::new(2u32, 3u32);
        assert!(a.cmp(&b).is_lt());
        assert!(b.cmp(&a).is_gt());
        assert_eq!(a.cmp(&a), std::cmp::Ordering::Equal);
    }

    #[test]
    fn test_to_dimensions() {
        let r = Ratio::new(4u32, 3u32);
        let dims = r.to_dimensions(6u32);
        assert_eq!(dims, (8u32, 6u32));
    }

    #[test]
    fn test_matches_aspect() {
        let r = Ratio::new(16u32, 9u32);
        assert!(r.matches_aspect(1920u32, 1080u32));
        assert!(!r.matches_aspect(1920u32, 1200u32));
    }

    #[test]
    fn test_is_square() {
        let square = Ratio::new(5u32, 5u32);
        let non_square = Ratio::new(4u32, 3u32);
        assert!(square.is_square());
        assert!(!non_square.is_square());
    }

}