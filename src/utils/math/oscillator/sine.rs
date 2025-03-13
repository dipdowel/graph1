use std::f64::consts::PI;
use crate::primitives::numeric::Numeric;

/// Calculates an oscillating value using a sine wave, constrained within a specified range.
///
/// # Parameters
/// - `counter`: A continuously increasing value that drives the oscillation (e.g. frame count).
/// - `frequency`: A factor that determines how fast the oscillation progresses.
/// - `lower_bound`: The minimum value of the oscillation range.
/// - `upper_bound`: The maximum value of the oscillation range.
///
/// # Returns
/// Returns a `f64` value that oscillates between `lower_bound` and `upper_bound`.
///
/// # Theory
/// This function generates an oscillating value using a sine wave mapped to a given range:
///
/// 1. **Convert counter to an angle:**
///    The counter is multiplied by `frequency` and `2π` to map it onto the sine function’s period.
///    - The sine function has a natural period of `2π`, meaning it repeats every `2π` units.
///    - By multiplying `counter` by `frequency`, we adjust how fast the oscillation occurs.
///
/// 2. **Compute sine wave:**
///    The `sin(angle)` function produces values in the range `[-1, 1]`, forming a smooth oscillation.
///
/// 3. **Scale sine output to the desired range:**
///    - Compute `amplitude = (upper_bound - lower_bound) / 2`: This determines the half-range of oscillation.
///    - Compute `midpoint = (upper_bound + lower_bound) / 2`: This shifts the sine wave to the desired range.
///    - Compute `scaled = midpoint + (sine_wave * amplitude)`: This ensures the result stays within `[lower_bound, upper_bound]`.
///
/// The resulting value will oscillate smoothly between `lower_bound` and `upper_bound` based on the provided frequency.

pub fn sine<C: Numeric, LB: Numeric, UB: Numeric>(
    counter: C,
    frequency: f64,
    lower_bound: LB,
    upper_bound: UB,
) -> f64 {
    let upper_bound = upper_bound.to_f64();
    let lower_bound = lower_bound.to_f64();
    let angle = counter.to_f64() * frequency * 2.0 * PI; // Scale counter
    let amplitude = (upper_bound - lower_bound) / 2.0; // Half the range
    let midpoint = (upper_bound + lower_bound) / 2.0; // Center of the range
    let scaled = midpoint + (angle.sin() * amplitude); // Scale to [lower_bound, upper_bound]
    scaled
}


#[cfg(test)]
mod tests {
    use super::sine;

    /// Helper function to check if two floating-point numbers are approximately equal
    fn approx_equal(a: f64, b: f64, epsilon: f64) -> bool {
        (a - b).abs() < epsilon
    }

    // TODO: add tests to test use of `Numeric` trait

    #[test]
    fn test_oscillation_at_zero() {
        let value = sine(0.0, 0.01, 80.0, 160.0);
        assert!(approx_equal(value, 120.0, 0.01), "Expected 120.0, got {}", value);
    }

    #[test]
    fn test_oscillation_at_quarter_period() {
        let value = sine(25.0, 0.01, 80.0, 160.0);
        assert!(approx_equal(value, 160.0, 0.01), "Expected 160.0, got {}", value);
    }

    #[test]
    fn test_oscillation_at_half_period() {
        let value = sine(50.0, 0.01, 80.0, 160.0);
        assert!(approx_equal(value, 120.0, 0.01), "Expected 120.0, got {}", value);
    }

    #[test]
    fn test_oscillation_at_three_quarters_period() {
        let value = sine(75.0, 0.01, 80.0, 160.0);
        assert!(approx_equal(value, 80.0, 0.01), "Expected 80.0, got {}", value);
    }

    #[test]
    fn test_oscillation_full_period() {
        let value = sine(100.0, 0.01, 80.0, 160.0);
        assert!(approx_equal(value, 120.0, 0.01), "Expected 120.0, got {}", value);
    }

    #[test]
    fn test_different_bounds() {
        let value = sine(25.0, 0.01, 50.0, 250.0);
        assert!(approx_equal(value, 250.0, 0.01), "Expected 250.0, got {}", value);
    }

    #[test]
    fn test_different_frequency() {
        let value1 = sine(25.0, 0.01, 80.0, 160.0);
        let value2 = sine(25.0, 0.02, 80.0, 160.0);
        assert_ne!(value1, value2, "Expected different values for different frequencies");
    }
}
