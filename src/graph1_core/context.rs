// use graph1::primitives::primitives::{Dimensions2d, ImageData0RGB};

use crate::primitives::primitives::{Dimensions2d, BufferRGBA};

#[derive(Debug)]
/// A collection of pre-computed window properties
/// that can be used to
pub struct WindowContext {
    /// Window width
    pub w: u32,
    /// Window height
    pub h: u32,
    /// Same as `w` but as `usize`
    pub w_usize: usize,
    /// Same as `h` but as `usize`
    pub h_usize: usize,
    /// Size of the framebuffer to render the window, in bytes
    pub size: usize,
    /// Window width and height as a `Dimensions2d`
    pub dimensions: Dimensions2d,
}

impl WindowContext {
    /// Instantiates a window context
    pub fn new(w: u32, h: u32) -> Self {

        Self {
            w,
            h,
            w_usize: w as usize,
            h_usize: h as usize,
            dimensions: Dimensions2d { w, h },
            size: (4 * w * h)  as usize,
        }
    }
}

/// Settings for rendering controls for Bezier curves
#[derive(Debug)]
pub struct BezierContext {
    /// If true, the control points and start-end points will be rendered
    pub render_controls: bool,
    /// If true, each pair of control points will be connected with a line
    pub render_levers: bool,
    /// Color of the control points, if `None` the inverted background color will be used
    pub control_color: Option<u32>,
    /// Color of the start and end points, if `None` the inverted background color will be used
    pub start_end_points_color: Option<u32>,
}

impl BezierContext {
    /// Instantiates a new `BezierContext` with default settings
    pub fn new() -> Self {
        Self {
            render_controls: false,
            render_levers: true,
            control_color: Some(0x00_00_33_ff),
            start_end_points_color: Some(0x00_ff_33_00),
        }
    }
}

#[derive(Debug)]
pub struct GraphContext<'c> {
    /// Reference to the window context
    pub win: &'c WindowContext,
    /// Reference to the main renderable buffer
    pub frame_buf: BufferRGBA<'c>,
    /// Reference to the off-screen buffer (can be used to prepare graphics in advance)
    pub draft_buf: Option<BufferRGBA<'c>>,
    /// A default color for rendering
    pub default_color: u32,
    /// Settings for rendering controls for Bezier curves
    pub bezier: Option<BezierContext>,
    /// Current frame in animation. If no animation is needed, can be set to `0`
    pub frame_count: usize,
}
