use crate::primitives::math::MinMax;
use crate::primitives::numeric::Numeric;

/// Normalize the `MinMax` range.
/// * If `range.min` is greater than `range.max`, the values are swapped, otherwise the range remains unchanged.
/// # Returns
/// * `MinMax<T>` - The normalized range (cloned).
pub(crate) fn normalize_min_max<T: Numeric>(range: &MinMax<T>) -> MinMax<T> {
    if range.min > range.max {
        return MinMax {
            min: range.max,
            max: range.min,
        };
    }
    range.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_min_max() {
        // Normal case
        let result = normalize_min_max(&MinMax::new(10, 20));
        assert_eq!(result.min, 10, "normal case, min");
        assert_eq!(result.max, 20, "normal case, max");

        // Case: min == max
        let result = normalize_min_max(&MinMax::new(20, 20));
        assert_eq!(result.min, 20, "min == max case, min");
        assert_eq!(result.max, 20, "min == max case, max");

        // Edge case: min > max
        let result = normalize_min_max(&MinMax::new(20, 10));
        assert_eq!(result.min, 10, "edge case, min");
        assert_eq!(result.max, 20, "edge case, max");
    }
}