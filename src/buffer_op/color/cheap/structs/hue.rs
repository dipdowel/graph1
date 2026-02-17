/// Represents a hue rotation transformation with a pre-computed fixed-point matrix.
///
/// Hue rotation shifts colors around the color wheel while preserving luminance
/// (perceived brightness). The transformation uses Rec. 601 luminance coefficients
/// (0.299R + 0.587G + 0.114B) for accurate color perception modeling.
/// - https://en.wikipedia.org/wiki/Rec._601
/// - https://en.wikipedia.org/wiki/Luma_(video)
///
/// # Fixed-Point Representation
/// The matrix uses 8.8 fixed-point format (scale 256) for efficient integer
/// arithmetic during pixel operations, avoiding floating-point overhead in tight loops.
///
 
#[derive(Clone, Copy, Debug)]
pub struct Hue {
    radians: f32,
    matrix: [[i32; 3]; 3], // 8.8 fixed-point matrix (scaled by 256)
}

impl Default for Hue {
    /// Creates a hue rotation of 0 radians (no color change).
    fn default() -> Self {
        Self::from_radians(0.0)
    }
}

impl PartialEq for Hue {
    /// Compares two hue rotations for equality within floating-point epsilon.
    fn eq(&self, other: &Self) -> bool {
        (self.radians - other.radians).abs() < f32::EPSILON
    }
}

impl Hue {
    /// Fixed-point scaling factor (2^8)
    /// for converting floating-point matrix values to 8.8 fixed-point integers.
    const SCALE: f32 = 256.0; // 1 << 8

    // Rec. 601 luminance coefficients (ITU-R BT.601)
    const LUM_R: f32 = 0.299;
    const LUM_G: f32 = 0.587;
    const LUM_B: f32 = 0.114;

    // Orthogonal projection constants for YIQ color space transformation
    // These preserve luminance while rotating hue in RGB space
    const PROJ_R_B: f32 = 0.143; // sqrt(LUM_R * LUM_B) ≈ sqrt(0.299 * 0.114) ≈ 0.143
    const PROJ_G_B: f32 = 0.140; // sqrt(LUM_G * LUM_B) ≈ sqrt(0.587 * 0.114) ≈ 0.140
    const PROJ_R_B_SUM: f32 = 0.283; // LUM_R + LUM_B ≈ 0.299 + 0.114 ≈ 0.283

    /// Create a hue rotation from radians.
    pub fn from_radians(radians: f32) -> Self {
        let radians = radians.rem_euclid(std::f32::consts::TAU);
        let matrix = Self::build_matrix(radians);

        Self { radians, matrix }
    }

    /// Create a hue rotation from degrees.
    pub fn from_degrees(degrees: f32) -> Self {
        Self::from_radians(degrees.to_radians())
    }

    /// Create a hue rotation from turns (1.0 = full rotation).
    pub fn from_turns(turns: f32) -> Self {
        Self::from_radians(turns * std::f32::consts::TAU)
    }

    /// Returns the stored angle in radians.
    pub fn as_radians(self) -> f32 {
        self.radians
    }

    /// Returns the stored angle in degrees.
    pub fn as_degrees(self) -> f32 {
        self.radians.to_degrees()
    }

    /// Returns the cached fixed-point matrix as a reference.
    pub fn matrix(&self) -> &[[i32; 3]; 3] {
        &self.matrix
    }

    /// Returns the cached fixed-point matrix by value.
    /// Useful when the caller needs ownership of the matrix values.
    pub fn matrix_values(&self) -> [[i32; 3]; 3] {
        self.matrix
    }

    fn build_matrix(theta: f32) -> [[i32; 3]; 3] {
        let cos = theta.cos();
        let sin = theta.sin();

        // Use pre-defined luminance and projection constants
        let lum_r = Self::LUM_R;
        let lum_g = Self::LUM_G;
        let lum_b = Self::LUM_B;

        // Hue rotation matrix that preserves luminance.
        // The projection constants represent orthogonal color axes in YIQ space.
        let m = [
            [
                lum_r + (1.0 - lum_r) * cos - lum_r * sin,
                lum_g - lum_g * cos - lum_g * sin,
                lum_b - lum_b * cos + (1.0 - lum_b) * sin,
            ],
            [
                lum_r - lum_r * cos + Self::PROJ_R_B * sin,
                lum_g + (1.0 - lum_g) * cos + Self::PROJ_G_B * sin,
                lum_b - lum_b * cos - Self::PROJ_R_B_SUM * sin,
            ],
            [
                lum_r - lum_r * cos - (1.0 - lum_r) * sin,
                lum_g - lum_g * cos + lum_g * sin,
                lum_b + (1.0 - lum_b) * cos + lum_b * sin,
            ],
        ];

        let mut fixed = [[0i32; 3]; 3];

        for i in 0..3 {
            for j in 0..3 {
                fixed[i][j] = (m[i][j] * Self::SCALE).round() as i32;
            }
        }

        fixed
    }
}
