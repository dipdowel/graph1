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
