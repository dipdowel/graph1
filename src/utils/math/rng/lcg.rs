/// Configuration for a Linear Congruential Generator (LCG).
#[derive(Debug, Clone, Copy)]
pub  struct LcgConfig {
    /// The modulus (`m`) defines the range of generated numbers.
    ///  Popular values:
    ///   - Use `2^32` (`4_294_967_296`) for 32-bit platforms.
    ///   - Use `2^64` (`18_446_744_073_709_551_616`) for 64-bit platforms.
    ///   - A large **prime number** can improve randomness for some applications.
    pub  m_modulus: u64,
    /// The multiplier (`a`) determines how new numbers are generated from previous ones.
    /// Popular values:
    ///   - `1664525` (Numerical Recipes, good for 32-bit LCGs)
    ///   - `1103515245` (glibc `rand()`)
    ///   - `22695477` (Old Microsoft RNG)
    ///   - `6364136223846793005` (64-bit, PCG-inspired)
    pub  a_multiplier: u64,
    /// The increment (`c`) helps control the periodicity and distribution of numbers.
    /// Popular values:
    ///   - `1013904223` (glibc `rand()`, ensures long cycle)
    ///   - `12345` (Old Microsoft, decent but shorter period)
    ///   - `1442695040888963407` (64-bit, used in some PCG variants)
    pub  c_increment: u64,
}

/// Predefined LCG configurations
#[cfg(target_pointer_width = "64")]
pub const LCG_DEFAULT_64: LcgConfig = LcgConfig {
    // m_modulus: 1u64 << 64,
    m_modulus: u64::MAX.wrapping_add(1), //  2^64
    a_multiplier: 6364136223846793005,
    c_increment: 1442695040888963407,
};

#[cfg(target_pointer_width = "32")]
/// Common for 32-bit systems, based on the book "Numerical Recipes: The Art of Scientific Computing"
pub const LCG_NUMERICAL_RECIPES_32: LcgConfig = LcgConfig {
    m_modulus: 1u64 << 32,
    a_multiplier: 1664525,
    c_increment: 1013904223,
};


/// Old Microsoft RNG (Used in `rand()` from older Windows versions)
pub const LCG_MICROSOFT: LcgConfig = LcgConfig {
    m_modulus: 1u64 << 32,
    a_multiplier: 22695477,
    c_increment: 1,
};

/// glibc `rand()` RNG (Used in many C standard libraries)
pub const LCG_GLIBC: LcgConfig = LcgConfig {
    m_modulus: 1u64 << 32,
    a_multiplier: 1103515245,
    c_increment: 12345,
};


/// A simple Linear Congruential Generator (LCG) for pseudo-random number generation.
///
/// This RNG is **not cryptographically secure**, but it is **fast** and suitable for simulations.
/// It supports different LCG configurations.
///
/// ### Example Usage
/// ```
/// let mut rng = LcgRng::new(42, None); // Use default config
/// let random_value = rng.next_u32();
/// ```
///
pub struct LcgRng<'a> {
    state: u64,
    config: &'a LcgConfig,
}

impl<'a> LcgRng<'a> {
    /// Creates a new LCG RNG with a seed.
    ///
    /// - If `config` is `None`, it defaults to the best LCG settings for the current architecture.
    /// - Otherwise, you can provide a custom `LcgRngConfig` reference.
    ///
    /// ### Example
    /// ```
    /// let mut rng = LcgRng::new(42, Some(&LCG_GLIBC));
    /// let value = rng.next_u32();
    /// ```
    pub fn new(seed: u64, config: Option<&'a LcgConfig>) -> Self {
        Self {
            state: seed,
            config: config.unwrap_or_else(|| {
                #[cfg(target_pointer_width = "64")]
                { &LCG_DEFAULT_64 }
                #[cfg(target_pointer_width = "32")]
                { &LCG_NUMERICAL_RECIPES_32 }
            }),
        }
    }

    /// Generates a random `u32` integer.
    pub fn next_u32(&mut self) -> u32 {
        self.state = self
            .state
            .wrapping_mul(self.config.a_multiplier)
            .wrapping_add(self.config.c_increment)
            % self.config.m_modulus;

        (self.state >> 16) as u32
    }

    /// Generates a random `u32` in the range `[min, max)`.
    ///
    /// - **`min`**: The lower bound (inclusive).
    /// - **`max`**: The upper bound (exclusive).
    ///
    /// ### Example
    /// ```
    /// let mut rng = LcgRng::new(42, None);
    /// let value = rng.next_ranged_u32(10, 100); // Random number in [10, 100)
    /// ```
    pub fn next_ranged_u32(&mut self, min: u32, max: u32) -> u32 {
        assert!(min < max, "min must be less than max");

        // Generate a random number in the range [0, range)
        let range = max - min;
        let rand_within_range = self.next_u32() % range;

        // Shift it to the desired range [min, max)
        min + rand_within_range
    }
    
    /// Generates a random `f64` in the range `[0, 1)`.
    pub fn next_f64(&mut self) -> f64 {
        self.next_u32() as f64 / (u32::MAX as f64)
    }
}

//TODO: write at least some unit tests!
