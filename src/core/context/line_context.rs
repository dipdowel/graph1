use crate::core::misc::line_clipping_style::LineClippingStyle;

/// Defines the rendering style for lines drawn in the framebuffer.
///
/// This struct groups all configuration options related to line rendering:
/// - Thickness
/// - Anti-aliasing
/// - Rasterization precision
/// - Clipping algorithm
#[derive(Debug, Clone)]
pub struct LineContext {
    /// The clipping algorithm used to constrain lines within the drawable area.
    /// - `ElasticSlide`: fast but imprecise, pulls endpoints into screen bounds
    /// - `CohenSutherland`: classic, region-code based, fairly accurate
    /// - `LiangBarsky`: mathematically precise and fastest of the three
    pub clipping: LineClippingStyle,

    /// The width of the drawn line, in pixels.
    /// This is a floating-point value to allow subpixel thickness.
    /// For example: `1.0` = 1 pixel, `0.5` = half pixel, `2.5` = slightly thicker than 2.
    /// Used when `rasterization` is set to `RasterizationMethod.Float`.
    pub width_float: f32,

    /// The width of the drawn line, in pixels.
    /// Used when `rasterization` is set to `RasterizationMethod.Integer`.
    pub width_int: u16,

    /// Anti-aliasing settings for drawing lines.
    pub anti_aliasing: AntiAliasingConfig,

    /// The type of rasterization used to generate the line path.
    /// - `Integer`: classic integer math, e.g. Bresenham or fixed-point geometry.
    ///    Use it for better speed and simpler math (good for animations or low-end CPUs).
    /// - `Float`: floating-point line interpolation and line width sampling
    ///    Use it for more accurate rendering (especially with subpixel line widths).
    pub rasterization: RasterizationMethod,
}

impl LineContext {
    /// Configures the context to draw without anti-aliasing using integer rasterization.
    ///
    /// - `width`: line width in whole pixels (used with integer rasterization)
    pub fn set_int_no_aa(&mut self, width: u16) {
        self.width_int = width;
        self.anti_aliasing.enabled = false;
        self.rasterization = RasterizationMethod::Int;
    }

    /// Configures the context to draw without anti-aliasing using float rasterization.
    ///
    /// - `width`: subpixel-precise line width (used with float rasterization)
    pub fn set_float_no_aa(&mut self, width: f32) {
        self.width_float = width;
        self.anti_aliasing.enabled = false;
        self.rasterization = RasterizationMethod::Float;
    }

    /// Configures the context to draw with integer anti-aliasing and integer rasterization.
    ///
    /// - `width`: line width in whole pixels (used with integer rasterization)
    pub fn set_int_aa_int(&mut self, width: u16) {
        self.width_int = width;
        self.anti_aliasing.enabled = true;
        self.anti_aliasing.method = AntiAliasingMethod::Int;
        self.rasterization = RasterizationMethod::Int;
    }

    /// Configures the context to draw with integer anti-aliasing and float rasterization.
    ///
    /// - `width`: line width in whole pixels (used with integer AA)
    pub fn set_float_aa_int(&mut self, width: u16) {
        self.width_int = width;
        self.anti_aliasing.enabled = true;
        self.anti_aliasing.method = AntiAliasingMethod::Int;
        self.rasterization = RasterizationMethod::Float;
    }

    /// Configures the context to draw with float anti-aliasing and integer rasterization.
    ///
    /// - `width`: line width in whole pixels (used with integer rasterization)
    pub fn set_int_aa_float(&mut self, width: u16) {
        self.width_int = width;
        self.anti_aliasing.enabled = true;
        self.anti_aliasing.method = AntiAliasingMethod::Float;
        self.rasterization = RasterizationMethod::Int;
    }

    /// Configures the context to draw with float anti-aliasing and float rasterization.
    ///
    /// - `width`: subpixel-precise line width (used with float AA and rasterization)
    pub fn set_float_aa_float(&mut self, width: f32) {
        self.width_float = width;
        self.anti_aliasing.enabled = true;
        self.anti_aliasing.method = AntiAliasingMethod::Float;
        self.rasterization = RasterizationMethod::Float;
    }

    /// Creates a new LineContext with all configurable rendering parameters.
    ///
    /// - `clipping`: how to clip line endpoints
    /// - `width_float`: line width for float rasterization
    /// - `width_int`: line width for integer rasterization
    /// - `anti_aliasing_enabled`: enable or disable AA
    /// - `anti_aliasing_method`: which AA technique to use if enabled
    /// - `rasterization`: float or int rasterization method
    pub fn new(
        clipping: LineClippingStyle,
        width_float: f32,
        width_int: u16,
        anti_aliasing_enabled: bool,
        anti_aliasing_method: AntiAliasingMethod,
        rasterization: RasterizationMethod,
    ) -> Self {
        Self {
            clipping,
            width_float,
            width_int,
            anti_aliasing: AntiAliasingConfig {
                enabled: anti_aliasing_enabled,
                method: anti_aliasing_method,
            },
            rasterization,
        }
    }

    /// Returns `true` if anti-aliasing is enabled.
    pub fn is_anti_aliasing(&self) -> bool {
        self.anti_aliasing.enabled
    }
}

impl Default for LineContext {
    fn default() -> Self {
        Self {
            clipping: LineClippingStyle::ElasticSlide,
            width_float: 1.0,
            width_int: 1,
            anti_aliasing: AntiAliasingConfig::default(),
            rasterization: RasterizationMethod::Int,
        }
    }
}

/// Anti-aliasing configuration state.
#[derive(Debug, Clone)]
pub struct AntiAliasingConfig {
    /// Whether anti-aliasing is enabled for lines.
    /// This enables smooth transitions at the edges of lines using the method below.
    pub enabled: bool,
    /// The algorithm used to calculate anti-aliasing coverage.
    /// - `Integer`: fast fixed-point Xiaolin Wu approximation (integer-only, high performance)
    /// - `Float`: accurate floating-point Xiaolin Wu (best quality, slower)
    pub method: AntiAliasingMethod,
}

impl Default for AntiAliasingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            method: AntiAliasingMethod::Int,
        }
    }
}

/// Selects the anti-aliasing technique to use (if enabled).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AntiAliasingMethod {
    /// Integer-based approximation of Xiaolin Wu's algorithm.
    /// Uses fixed-point math and is suitable for fast rendering.
    Int,
    /// Original floating-point Xiaolin Wu algorithm.
    /// Smoothest appearance, most accurate coverage calculation.
    Float,
}

impl AntiAliasingMethod {
    pub fn is_int(&self) -> bool {
        *self == AntiAliasingMethod::Int
    }

    pub fn is_float(&self) -> bool {
        *self == AntiAliasingMethod::Float
    }
}

/// Selects the rasterization technique used to generate line paths.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RasterizationMethod {
    /// Use integer-based line rasterization.
    /// Includes Bresenham and fixed-point Xiaolin Wu techniques.
    Int,
    /// Use floating-point subpixel rendering.
    /// Required for smooth curves and fractional line widths.
    Float,
}

impl RasterizationMethod {
    pub fn is_int(&self) -> bool {
        *self == RasterizationMethod::Int
    }

    pub fn is_float(&self) -> bool {
        *self == RasterizationMethod::Float
    }
}
