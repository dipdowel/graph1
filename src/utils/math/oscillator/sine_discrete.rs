use crate::primitives::numeric::Numeric;

/// Calculates a discrete oscillating value using a sine wave, constrained within a specified integer range.
///
/// # Parameters
/// - `counter`: A continuously increasing value that drives the oscillation (e.g., frame count).
/// - `frequency_divisor`: A factor that slows down the oscillation; larger values slow down oscillation.
/// - `levels`: The maximum number of discrete steps.
/// - `ensure_minimum_one`: If `true`, ensures the minimum output is `1` instead of `0`.
///
/// # Returns
/// A `usize` that oscillates between 0 and `levels` (or between 1 and `levels` if `ensure_minimum_one` is enabled).
pub fn sine_discrete<C: Numeric>(
    counter: C,
    frequency_divisor: f32,
    levels: usize,
    ensure_minimum_one: bool,
) -> usize {
    let counter = counter.to_f64() as f32; // FIXME: maybe add `to_f32` to `Numeric` trait?
    let sine_input = (counter / frequency_divisor).sin();
    let normalized_value = (sine_input + 1.0) / 2.0;
    let mut oscillator = (normalized_value * (levels as f32)) as usize;

    if ensure_minimum_one && oscillator == 0 {
        oscillator = 1;
    }

    oscillator
}

#[cfg(test)]
mod tests {
    use super::sine_discrete;

    #[test]
    fn test_basic_behavior() {
        let result = sine_discrete(0.0, 9.0, 33, true);
        assert_eq!(result, 17); // Near the middle
    }

    #[test]
    fn test_counter_progression() {
        for frame in 1..100 {
            let current = sine_discrete(frame as f32, 9.0, 33, true);
            // The value should oscillate over time, sometimes changing
            assert!(current >= 1 && current <= 33);
        }
    }

    #[test]
    fn test_without_ensure_minimum_one() {
        for frame in 0..100 {
            let current = sine_discrete(frame as f32, 9.0, 33, false);
            assert!(current <= 33);
            // It's allowed to be 0 here
        }
    }

    #[test]
    fn test_with_small_levels() {
        for frame in 0..100 {
            let current = sine_discrete(frame as f32, 5.0, 5, true);
            assert!(current >= 1 && current <= 5);
        }
    }
}

// Example to match original code effect:
//
// let oscillator = sine_discrete(ctx.frame_count, 9.0, 33, true);
