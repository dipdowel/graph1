use crate::primitives::numeric::Numeric;

/// Integer-only fast triangle wave oscillator.
///
/// This function does not use floating-point or frequency.
/// It assumes `counter` increments by 1 each call (e.g. frame count).
///
/// # Parameters
/// - `counter`: Integer counter.
/// - `lower_bound`: Minimum value (inclusive).
/// - `upper_bound`: Maximum value (inclusive).
///
/// # Returns
/// Integer oscillating value.
pub fn linear_fast(
    counter: isize,
    lower_bound: isize,
    upper_bound: isize,
) -> isize {
    let range = upper_bound - lower_bound;
    let full_period = 2 * range;
    let phase = counter % full_period;
    if phase < range {
        lower_bound + phase
    } else {
        upper_bound - (phase - range)
    }
}


#[cfg(test)]
mod tests {
    use super::{ linear_fast};


    #[test]
    fn test_linear_fast_oscillation() {
        let lb:isize = 3;
        let ub:isize = 7;

        let expected:[isize;11] = [3,4,5,6,7,6,5,4,3,4,5];
        for i in 0..expected.len() {
            assert_eq!(linear_fast(lb, ub, i as isize), expected[i] );
        }
    }
}
