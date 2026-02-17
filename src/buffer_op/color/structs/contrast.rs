/// Represents a contrast adjustment with a pre-computed fixed-point multiplier.
///
/// Contrast adjustment modifies the difference between pixel values and middle gray (128).
/// - Values > 1.0 increase contrast (darker darks, lighter lights)
/// - Values < 1.0 decrease contrast (more washed out)
/// - Value = 1.0 means no change
/// - Value = 0.0 results in pure gray
///
/// # Fixed-Point Representation
/// The multiplier uses 8.8 fixed-point format (scale 256) for efficient integer
/// arithmetic during pixel operations, avoiding floating-point overhead in tight loops.
///
/// # Examples
/// ```
/// use graph1::buffer_op::color::structs::Contrast;
///
/// let contrast = Contrast::from_f32(1.5); // 50% more contrast
/// let contrast = Contrast::from_f32(0.5); // 50% less contrast
/// let contrast = Contrast::from_f32(1.0); // No change (default)
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Contrast {
    value: f32,
    fixed: i32, // 8.8 fixed-point contrast multiplier (scaled by 256)
}

impl Default for Contrast {
    /// Creates a contrast adjustment of 1.0 (no change).
    fn default() -> Self {
        Self::from_f32(1.0)
    }
}

impl PartialEq for Contrast {
    /// Compares two contrast values for equality within floating-point epsilon.
    fn eq(&self, other: &Self) -> bool {
        (self.value - other.value).abs() < f32::EPSILON
    }
}

impl Contrast {
    /// Fixed-point scaling factor (2^8)
    /// for converting floating-point contrast values to 8.8 fixed-point integers.
    const SCALE: f32 = 256.0; // 1 << 8

    /// Creates a contrast adjustment from a floating-point value.
    ///
    /// # Arguments
    /// * `value` - Contrast multiplier (1.0 = no change, 0.0 = gray, 2.0 = double contrast)
    ///             Values are clamped to [0.0, 10.0] for practical use.
    pub fn from_f32(value: f32) -> Self {
        let value = value.clamp(0.0, 10.0);
        let fixed = (value * Self::SCALE).round() as i32;

        Self { value, fixed }
    }

    /// Returns the stored contrast value as a floating-point number.
    pub fn as_f32(self) -> f32 {
        self.value
    }

    /// Returns the cached fixed-point multiplier.
    pub fn fixed_multiplier(&self) -> i32 {
        self.fixed
    }
}
