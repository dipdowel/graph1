#[derive(Clone, Copy, Debug)]
pub struct Hue {
    radians: f32,
    matrix: [[i32; 3]; 3], // 8.8 fixed-point matrix
}

impl Hue {
    const SCALE: f32 = 256.0; // 1 << 8

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

    /// Returns the cached fixed-point matrix.
    pub fn matrix(&self) -> &[[i32; 3]; 3] {
        &self.matrix
    }

    fn build_matrix(theta: f32) -> [[i32; 3]; 3] {
        let cos = theta.cos();
        let sin = theta.sin();

        // Rec. 601 luminance coefficients
        let lum_r = 0.299;
        let lum_g = 0.587;
        let lum_b = 0.114;

        let m = [
            [
                lum_r + (1.0 - lum_r) * cos - lum_r * sin,
                lum_g - lum_g * cos - lum_g * sin,
                lum_b - lum_b * cos + (1.0 - lum_b) * sin,
            ],
            [
                lum_r - lum_r * cos + 0.143 * sin,
                lum_g + (1.0 - lum_g) * cos + 0.140 * sin,
                lum_b - lum_b * cos - 0.283 * sin,
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
