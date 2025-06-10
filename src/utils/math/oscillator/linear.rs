use crate::primitives::numeric::Numeric;

/// Calculates a linearly oscillating value between `lower_bound` and `upper_bound`.
///
/// Unlike sine-based oscillators, this function moves linearly in one direction
/// and reverses when hitting bounds, creating a triangle wave pattern.
///
/// # Parameters
/// - `counter`: A steadily increasing counter (e.g. frame number or time step).
/// - `frequency`: Speed factor. Larger values make the oscillation progress faster.
/// - `lower_bound`: Minimum value of the oscillation range.
/// - `upper_bound`: Maximum value of the oscillation range.
///
/// # Returns
/// A `f64` value oscillating linearly between `lower_bound` and `upper_bound`.
pub fn linear<C: Numeric, LB: Numeric, UB: Numeric>(
    counter: C,
    frequency: f64,
    lower_bound: LB,
    upper_bound: UB,
) -> f64 {
    let counter = counter.to_f64();
    let lower_bound = lower_bound.to_f64();
    let upper_bound = upper_bound.to_f64();
    let range = upper_bound - lower_bound;

    // The progress linearly increases with counter * frequency
    let scaled = (counter * frequency) % (2.0 * range);

    // Reflect if we're in the descending part of the triangle wave
    if scaled < range {
        lower_bound + scaled
    } else {
        upper_bound - (scaled - range)
    }
}

#[cfg(test)]
mod tests {
    use super::linear;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < 0.01
    }

    #[test]
    fn test_linear_oscillation() {
        let lb = 10.0;
        let ub = 20.0;
        let freq = 1.0;

        assert!(approx_eq(linear(0.0, freq, lb, ub), 10.0));
        assert!(approx_eq(linear(5.0, freq, lb, ub), 15.0));
        assert!(approx_eq(linear(10.0, freq, lb, ub), 20.0));
        assert!(approx_eq(linear(15.0, freq, lb, ub), 15.0));
        assert!(approx_eq(linear(20.0, freq, lb, ub), 10.0));
        assert!(approx_eq(linear(25.0, freq, lb, ub), 15.0));
    }
}
