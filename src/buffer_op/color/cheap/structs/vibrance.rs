/// Vibrance adjustment for selectively boosting or reducing color saturation.
///
/// Vibrance preferentially affects less-saturated colors more than highly-saturated ones,
/// making it useful for enhancing dull colors without oversaturating already-vibrant areas.
///
/// # Value Range
/// - Positive values (e.g., 0.5): Boost dull colors more aggressively
/// - Negative values (e.g., -0.5): Mute colors, with greater effect on less saturated colors
/// - 0.0: No change (identity)
/// - Typical range: -1.0 to 1.0
///
/// # Fixed-Point Representation
/// Internally stores the vibrance as an 8.8 fixed-point integer for efficient computation.
///
/// # Examples
/// ```
/// use graph1::buffer_op::color::cheap::Vibrance;
///
/// let vibrance = Vibrance::from_f32(0.5);  // Boost dull colors
/// let vibrance = Vibrance::from_f32(-0.3); // Mute colors
/// let vibrance = Vibrance::identity();     // No change
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vibrance {
    value: f32,
    fixed: i32, // 8.8 fixed-point vibrance value (scaled by 256)
}

impl Vibrance {
    /// Fixed-point scaling factor (2^8)
    const SCALE: f32 = 256.0;

    /// Creates a vibrance adjustment from a floating-point value.
    ///
    /// # Arguments
    /// * `value` - Vibrance adjustment (-1.0 to 1.0 typical range)
    ///             Values are clamped to [-2.0, 2.0] for safety.
    pub fn from_f32(value: f32) -> Self {
        let value = value.clamp(-2.0, 2.0);
        let fixed = (value * Self::SCALE).round() as i32;
        Self { value, fixed }
    }

    /// Returns the stored vibrance value as a floating-point number.
    #[inline(always)]
    pub fn as_f32(self) -> f32 {
        self.value
    }

    /// Returns the cached fixed-point vibrance value.
    #[inline(always)]
    pub fn fixed_value(&self) -> i32 {
        self.fixed
    }

    /// Common preset: no change (0.0).
    pub const fn identity() -> Self {
        Self {
            value: 0.0,
            fixed: 0,
        }
    }

    /// Returns true if the vibrance adjustment is effectively an identity operation (no change).
    #[inline(always)]
    pub fn is_identity(&self) -> bool {
        self.value.abs() < f32::EPSILON
    }
}

impl Default for Vibrance {
    /// Creates a vibrance adjustment of 0.0 (no change).
    fn default() -> Self {
        Self::identity()
    }
}

