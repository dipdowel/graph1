use crate::primitives::numeric::Numeric;
use crate::utils::math::gcd::gcd;
use std::cmp::Ord;
use std::cmp::Ordering;
use std::fmt;

/// A struct representing a ratio of two `Numeric` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ratio<T: Numeric> {
    pub numerator: T,
    pub denominator: T,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RatioError {
    /// Denominator was zero during ratio creation.
    ZeroDenominator,

    /// Tried to invert a ratio with zero numerator.
    ZeroNumeratorInversion,
}

impl<T: Numeric + Eq> Ord for Ratio<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        // NB: cross-multiplication comparison
        // We evaluate (a/b) ? (c/d) as (a * d) ? (c * b),
        // which is equivalent but avoids floating-point imprecision and division by zero!
        // E.g. compare 3/4 vs 2/3:
        //    3 * 3 = 9
        //    2 * 4 = 8
        //
        //    => 9 > 8 ⇒ 3/4 > 2/3
        //
        (self.numerator.to_u64() * other.denominator.to_u64())
            .cmp(&(other.numerator.to_u64() * self.denominator.to_u64()))
    }
}

impl<T: Numeric + Eq> PartialOrd for Ratio<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl<T: Numeric> Ratio<T> {
    /// Creates a new `Ratio`, returns an error if denominator is zero.
    pub fn new(numerator: T, denominator: T) -> Result<Self, RatioError> {
        if denominator == T::zero() {
            return Err(RatioError::ZeroDenominator);
        }
        Ok(Self {
            numerator,
            denominator,
        })
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

    /// Returns the inverse of the ratio (flips numerator and denominator), or an error if numerator is zero.
    pub fn inverted(&self) -> Result<Self, RatioError> {
        if self.numerator == T::zero() {
            return Err(RatioError::ZeroNumeratorInversion);
        }
        Ok(Self {
            numerator: self.denominator,
            denominator: self.numerator,
        })
    }

    /// Converts the ratio to a (width, height) pair based on a base denominator value.
    pub fn to_dimensions(&self, base: T) -> (T, T) {
        let width = self.numerator * base / self.denominator;
        (width, base)
    }

    /// Returns whether this ratio matches the given width and height.
    pub fn has_same_aspect_as(&self, width: T, height: T) -> bool {
        let simplified = self.simplified();
        let other = match Self::new(width, height) {
            Ok(ratio) => ratio.simplified(),
            Err(_) => return false,
        };
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
            "Ratio {:.6}\u{2026}:{:.6}\u{2026}",
            self.numerator.to_f64(),
            self.denominator.to_f64()
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::primitives::ratio::{Ratio, RatioError};

    #[test]
    fn test_creation_and_display() {
        let r = Ratio::new(4_u32, 3_u32).unwrap();
        assert_eq!(format!("{}", r), "Ratio 4:3");
    }

    #[test]
    fn test_creation_zero_denominator() {
        let err = Ratio::new(5_u32, 0_u32).unwrap_err();
        assert_eq!(err, RatioError::ZeroDenominator);
    }

    #[test]
    fn test_as_f64_and_percent() {
        let r = Ratio::new(1_u32, 4_u32).unwrap();
        assert_eq!(r.as_f64(), 0.25_f64);
        assert_eq!(r.as_percent(), 25.0_f64);
    }

    #[test]
    fn test_simplified() {
        let r = Ratio::new(10_u32, 20_u32).unwrap();
        let simplified = r.simplified();
        assert_eq!(simplified.numerator, 1_u32);
        assert_eq!(simplified.denominator, 2_u32);
    }

    #[test]
    fn test_scaled() {
        let r = Ratio::new(2_u32, 3_u32).unwrap();
        let scaled = r.scaled(2_u32);
        assert_eq!(scaled.numerator, 4_u32);
        assert_eq!(scaled.denominator, 6_u32);
    }

    #[test]
    fn test_inverted() {
        let r = Ratio::new(2_u32, 5_u32).unwrap();
        let inv = r.inverted().unwrap();
        assert_eq!(inv.numerator, 5_u32);
        assert_eq!(inv.denominator, 2_u32);
    }

    #[test]
    fn test_inverted_zero_numerator() {
        let r = Ratio::new(0_u32, 1_u32).unwrap();
        let result = r.inverted();
        assert_eq!(result.unwrap_err(), RatioError::ZeroNumeratorInversion);
    }

    #[test]
    fn test_cmp_ordering() {
        let a = Ratio::new(1_u32, 3_u32).unwrap();
        let b = Ratio::new(2_u32, 3_u32).unwrap();
        assert!(a.cmp(&b).is_lt());
        assert!(b.cmp(&a).is_gt());
        assert_eq!(a.cmp(&a), std::cmp::Ordering::Equal);
    }

    #[test]
    fn test_to_dimensions() {
        let r = Ratio::new(4_u32, 3_u32).unwrap();
        let dims = r.to_dimensions(6_u32);
        assert_eq!(dims, (8_u32, 6_u32));
    }

    #[test]
    fn test_has_same_aspect_as() {
        let r = Ratio::new(16_u32, 9_u32).unwrap();
        assert!(r.has_same_aspect_as(1920_u32, 1080_u32));
        assert!(!r.has_same_aspect_as(1920_u32, 1200_u32));
    }

    #[test]
    fn test_is_square() {
        let square = Ratio::new(5_u32, 5_u32).unwrap();
        let non_square = Ratio::new(4_u32, 3_u32).unwrap();
        assert!(square.is_square());
        assert!(!non_square.is_square());
    }

    #[test]
    fn test_large_values_and_overflow_safety() {
        let a = Ratio::new(u32::MAX, 1_u32).unwrap();
        let b = Ratio::new(u32::MAX - 1, 1_u32).unwrap();
        assert!(a.cmp(&b).is_gt());
        assert_eq!(a.simplified().numerator, u32::MAX);
    }

    #[test]
    fn test_floating_point_like_display() {
        let r = Ratio::new(3_u32, 7_u32).unwrap();
        let display = format!("{}", r);
        assert!(display.contains(":"));
        assert!(display.contains("...") || display.contains("Ratio"));
    }

    #[test]
    fn test_identity_scaling() {
        let r = Ratio::new(3_u32, 5_u32).unwrap();
        let scaled = r.scaled(1_u32);
        assert_eq!(scaled, r);
    }

    #[test]
    fn test_to_dimensions_inverse() {
        let r = Ratio::new(4_u32, 3_u32).unwrap();
        let (w, h) = r.to_dimensions(6_u32);
        let r2 = Ratio::new(w, h).unwrap().simplified();
        assert_eq!(r2, r.simplified());
    }

    #[test]
    fn test_matches_aspect_symmetry() {
        let r1 = Ratio::new(1280_u32, 720_u32).unwrap();
        let r2 = Ratio::new(1920_u32, 1080_u32).unwrap();
        assert!(r1.has_same_aspect_as(1920_u32, 1080_u32));
        assert!(r2.has_same_aspect_as(1280_u32, 720_u32));
    }

    #[test]
    fn test_edge_case_one_pixel() {
        let r = Ratio::new(1_u32, 1_u32).unwrap();
        assert_eq!(r.as_f64(), 1.0_f64);
        assert_eq!(r.as_percent(), 100.0_f64);
        assert!(r.is_square());
    }

    #[test]
    fn test_zero_numerator_ratio() {
        let r = Ratio::new(0_u32, 5_u32).unwrap();
        assert_eq!(r.as_f64(), 0.0_f64);
        assert_eq!(r.as_percent(), 0.0_f64);
        assert_eq!(format!("{}", r), "Ratio 0:5");
    }
}
