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
            return write!(f, "{}:{}", self.numerator, self.denominator);
        }
        write!(
            f,
            "{:.4}...:{:.4}...",
            self.numerator.to_f64(),
            self.denominator.to_f64()
        )
    }
}

