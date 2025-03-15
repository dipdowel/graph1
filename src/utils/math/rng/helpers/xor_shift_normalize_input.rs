use crate::primitives::math::MinMax;
use crate::primitives::numeric::Numeric;

pub(crate) struct NormalizedInput<T: Numeric> {
    pub(crate) range: MinMax<T>,
    pub(crate) vec: Vec<T>,
    pub(crate) is_constant: bool,
}

/// Normalize the input range for the `XorShiftRng` and prepare an empty vector.
/// * `size` - The size of the vector to generate.
/// * `range` - The provided range of values
/// # Returns
/// * `NormalizedInput<T>` - A structure containing the normalized `range` and a vector.
///     - The vector is filled with `range.min` if `range.min == range.max`.
///     - The vector is empty in all other cases.
///     - `is_constant` is set if the vector is filled with the range's minimum value.
pub(crate) fn normalize_input<T: Numeric>(size: usize, range: &MinMax<T>) -> NormalizedInput<T> {
    // Edge case:  min == max --> a vector of constant values
    if range.min == range.max {
        return NormalizedInput {
            range: range.clone(),
            vec: vec![range.min; size],
            is_constant: true,
        };
    }

    // Edge case: min > max. Swap min and max to ensure min <= max + prepare an empty vector.
    if range.min > range.max {
        return NormalizedInput {
            range: MinMax::new(range.max, range.min),
            vec: Vec::with_capacity(size),
            is_constant: false,
        };
    }

    // Normal case: min <= max. Prepare an empty vector.
    NormalizedInput {
        range: MinMax::new(range.min, range.max),
        vec: Vec::with_capacity(size),
        is_constant: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_input() {
        // Normal case
        let result = normalize_input(10, &MinMax::new(10, 20));
        assert_eq!(result.range.min, 10, "normal case, min");
        assert_eq!(result.range.max, 20, "normal case, max");
        assert_eq!(result.vec.capacity(), 10, "normal case, capacity");
        assert_eq!(result.vec.len(), 0, "normal case, len");
        assert_eq!(result.is_constant, false, "normal case, is_constant");

        // Edge case: min == max
        let result = normalize_input(10, &MinMax::new(10, 10));
        assert_eq!(result.range.min, 10, "mix==max case, min");
        assert_eq!(result.range.max, 10, "mix==max case, max");
        assert_eq!(result.vec.capacity(), 10, "mix==max case, capacity");
        assert_eq!(result.vec.len(), 10, "mix==max case, len");
        assert_eq!(result.vec[0], 10, "mix==max case, vec[0]");
        assert_eq!(result.vec[9], 10, "mix==max case, vec[9]");
        assert_eq!(result.is_constant, true, "mix==max, is_constant");

        // Edge case: min > max
        let result = normalize_input(10, &MinMax::new(20, 10));
        assert_eq!(result.range.min, 10, "min>max case, min");
        assert_eq!(result.range.max, 20, "min>max case, max");
        assert_eq!(result.vec.capacity(), 10, "min>max case, capacity");
        assert_eq!(result.vec.len(), 0, "min>max case, len");
        assert_eq!(result.is_constant, false, "min>max, is_constant");
    }
}
