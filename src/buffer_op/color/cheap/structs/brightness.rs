/// Brightness factor in 8.8 fixed-point:
/// - 256 = 1.0x (no change)
/// - 128 = 0.5x (darker)
/// - 512 = 2.0x (brighter)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Brightness {
    factor_8_8: i32,
}

impl Brightness {
    // // Create from an 8.8 fixed-point factor (256 = 1.0x).
    // pub fn from_factor_8_8(factor_8_8: i32) -> Self {
    //     Self { factor_8_8 }
    // }

    /// Create from a percentage (100 = 1.0x, 50 = 0.5x, 200 = 2.0x).
    pub fn from_percent(percent: i32) -> Self {
        // factor_8_8 = percent/100 * 256  => percent * 256 / 100
        // Rounded:
        let factor_8_8 = (percent * 256 + 50) / 100;
        Self { factor_8_8 }
    }

    /// Common preset: no change (1.0x).
    pub const fn identity() -> Self {
        Self { factor_8_8: 256 }
    }

    /// Returns true if the brightness adjustment is effectively an identity operation (no change).
    ///
    /// This checks if the brightness factor is 256 (1.0x in 8.8 fixed-point), which means
    /// applying it would have no effect on the colors.
    #[inline(always)]
    pub fn is_identity(&self) -> bool {
        self.factor_8_8 == 256
    }

    #[inline(always)]
    pub fn factor_8_8(self) -> i32 {
        self.factor_8_8
    }
}

#[inline(always)]
fn clamp_u8_i32(v: i32) -> u32 {
    if v < 0 { 0 } else if v > 255 { 255 } else { v as u32 }
}
