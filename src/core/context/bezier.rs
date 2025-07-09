use crate::core::default_colors;

/// Settings for rendering controls for Bezier curves
#[derive(Debug, Default, Clone, Copy)]
pub struct BezierContext {
    /// If true, the control points and start-end points will be rendered
    pub render_controls: bool,
    /// If true, each pair of control points will be connected with a line
    pub render_levers: bool,
    /// Color of the control points, if `None` the inverted background color will be used
    pub control_color: Option<u32>,
    /// If true, Bézier curves will be rendered, otherwise not
    pub enabled: bool,
}

impl BezierContext {
    /// Instantiates a new `BezierContext` with default settings
    pub fn new() -> Self {
        Self {
            render_controls: false,
            render_levers: true,
            control_color: Some(default_colors::BEZIER_CONTROL),
            enabled: true,
        }
    }

    pub fn default() -> Self {
        Self::new()
    }
}
