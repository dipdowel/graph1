pub fn is_power_of_two(n: u32) -> bool {
    n != 0 && (n & (n - 1)) == 0
}

/// Finds the nearest power of two which is  less than the given number or equal to it.
/// * `n` - A 32-bit unsigned integer.
///
/// # Returns
/// * The nearest power of two for a given `n`. Returns 0 if `n` is 0. Returns `n` if `n` itself is a power of two.
pub fn nearest_power_of_two_towards_zero(n: u32) -> u32 {
    if n == 0 {
        return 0; // Edge case for zero
    }
    // Compute the highest power of two less than or equal to n
    let mut p = 1;
    while p <= n {
        p <<= 1;
    }
    p >> 1
}




#[cfg(test)]
mod tests {
    use super::*;

    // TESTS FOR `is_power_of_two()`
    //==============================================================================================
    #[test]
    fn test_is_power_of_two() {
        assert_eq!(is_power_of_two(1), true);
        assert_eq!(is_power_of_two(2), true);
        assert_eq!(is_power_of_two(4), true);
        assert_eq!(is_power_of_two(8), true);
        assert_eq!(is_power_of_two(16), true);
        assert_eq!(is_power_of_two(32), true);
        assert_eq!(is_power_of_two(64), true);
        assert_eq!(is_power_of_two(128), true);

        assert_eq!(is_power_of_two(0), false);
        assert_eq!(is_power_of_two(3), false);
        assert_eq!(is_power_of_two(5), false);
        assert_eq!(is_power_of_two(6), false);
        assert_eq!(is_power_of_two(10), false);
        assert_eq!(is_power_of_two(12), false);
        assert_eq!(is_power_of_two(200), false);
    }

    // TESTS FOR `nearest_power_of_two_less_than()`
    //==============================================================================================
    #[test]
    fn test_zero() {
        assert_eq!(nearest_power_of_two_towards_zero(0), 0);
    }

    #[test]
    fn test_one() {
        assert_eq!(nearest_power_of_two_towards_zero(1), 1);
    }

    #[test]
    fn test_two() {
        assert_eq!(nearest_power_of_two_towards_zero(2), 2);
    }

    #[test]
    fn test_power_of_two() {
        assert_eq!(nearest_power_of_two_towards_zero(16), 16);
        assert_eq!(nearest_power_of_two_towards_zero(32), 32);
        assert_eq!(nearest_power_of_two_towards_zero(64), 64);
    }

    #[test]
    fn test_non_power_of_two() {
        assert_eq!(nearest_power_of_two_towards_zero(20), 16);
        assert_eq!(nearest_power_of_two_towards_zero(50), 32);
        assert_eq!(nearest_power_of_two_towards_zero(100), 64);
    }

    #[test]
    fn test_large_number() {
        assert_eq!(nearest_power_of_two_towards_zero(1_000_000), 524_288);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(nearest_power_of_two_towards_zero(3), 2);
        assert_eq!(nearest_power_of_two_towards_zero(5), 4);
        assert_eq!(nearest_power_of_two_towards_zero(17), 16);
        assert_eq!(nearest_power_of_two_towards_zero(1023), 512);
        assert_eq!(nearest_power_of_two_towards_zero(1024), 1024);
    }

}
