use crate::primitives::numeric::Numeric;

/// Returns the greatest common divisor of two `Numeric` values using Euclidean algorithm.
pub fn gcd<T: Numeric>(mut a: T, mut b: T) -> T {
    while b != T::zero() {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd_basic_cases() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(101, 103), 1);
        assert_eq!(gcd(56, 98), 14);
        assert_eq!(gcd(17, 0), 17);
        assert_eq!(gcd(0, 25), 25);
        assert_eq!(gcd(0, 0), 0);
    }

    #[test]
    fn test_gcd_same_numbers() {
        assert_eq!(gcd(7, 7), 7);
        assert_eq!(gcd(123456, 123456), 123456);
    }

    #[test]
    fn test_gcd_large_numbers() {
        assert_eq!(gcd(1_000_000_007_u64, 123456789_u64), 1);
        assert_eq!(gcd(4294967296_u64, 65536_u64), 65536);
    }
}
