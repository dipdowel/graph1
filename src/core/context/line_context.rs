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

    /// The width of the line stroke, in pixels.
    /// This is a floating-point value to allow subpixel thickness.
    /// For example: `1.0` = 1 pixel, `0.5` = half pixel, `2.5` = slightly thicker than 2.
    /// Used when `rasterization` is set to `RasterizationMethod.Float`.
    pub stroke_width_float: f32,

    /// The width of the line stroke, in pixels.
    /// Used when `rasterization` is set to `RasterizationMethod.Integer`.
    pub stroke_width_int: u16,

    /// Anti-aliasing settings for drawing lines.
    pub anti_aliasing: AntiAliasingConfig,

    /// The type of rasterization used to generate the line path.
    /// - `Integer`: classic integer math, e.g. Bresenham or fixed-point geometry.
    ///    Use it for better speed and simpler math (good for animations or low-end CPUs).
    /// - `Float`: floating-point line interpolation and stroke sampling
    ///    Use it for more accurate rendering (especially with subpixel stroke widths).
    pub rasterization: RasterizationMethod,
}

impl LineContext {
    // TODO: implement proper `new()`
    pub fn new(
        clipping: LineClippingStyle,
        stroke_width_float: f32,
        stroke_width_int: u16,
        anti_aliasing_enabled: bool,
        anti_aliasing_method: AntiAliasingMethod,
        rasterization: RasterizationMethod,
    ) -> Self {
        Self {
            clipping,
            stroke_width_float,
            stroke_width_int,
            anti_aliasing: AntiAliasingConfig {
                enabled: anti_aliasing_enabled,
                method: anti_aliasing_method,
            },
            rasterization,
        }
    }

    // pub fn is_float_rasterization(&self) -> bool {
    //     self.rasterization == RasterizationMethod::Float
    // }
    //
    // pub fn is_int_rasterization(&self) -> bool {
    //     self.rasterization == RasterizationMethod::Integer
    // }

    pub fn is_anti_aliasing(&self) -> bool {
        self.anti_aliasing.enabled
    }

    pub fn is_integer_aa(&self) -> bool {
        self.anti_aliasing.method == AntiAliasingMethod::Int
    }

    pub fn is_float_aa(&self) -> bool {
        self.anti_aliasing.method == AntiAliasingMethod::Float
    }
}

impl Default for LineContext {
    fn default() -> Self {
        Self {
            clipping: LineClippingStyle::ElasticSlide,
            stroke_width_float: 1.0,
            stroke_width_int: 1,
            anti_aliasing: AntiAliasingConfig::default(),
            rasterization: RasterizationMethod::Int,
        }
    }
}

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
    /// Required for smooth curves and fractional stroke widths.
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
