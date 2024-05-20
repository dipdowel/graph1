pub fn is_power_of_two(n: u32) -> bool {
    n != 0 && (n & (n - 1)) == 0
}

#[cfg(test)]
mod tests {
    use crate::graph1::utils::misc::is_power_of_two;

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
}
