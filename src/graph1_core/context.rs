use crate::graph1_core::default_colors;
use crate::primitives::helper_types::BufferRGBA;
use crate::primitives::plane::Dimensions2d;

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
    /// Background color of the window, RGBA
    pub background_color: u32,
}

impl WindowContext {
    /// Instantiates a window context
    pub fn new(w: u32, h: u32, background_color_rgba: Option<u32>) -> Self {
        Self {
            w,
            h,
            w_usize: w as usize,
            h_usize: h as usize,
            dimensions: Dimensions2d { w, h },
            size: (4 * w * h) as usize,
            background_color: background_color_rgba.unwrap_or(default_colors::BACKGROUND),
        }
    }
    /*
       pub fn default() -> Self {
           WindowContext::new(320, 240)
       }
    */
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
            control_color: Some(default_colors::BEZIER_CONTROL),
            start_end_points_color: Some(default_colors::BEZIER_START_END),
        }
    }
}

#[derive(Debug)]
pub struct GraphContext<'c, UserDataType = Vec<i32>> {
    /// Reference to the window context
    pub win: &'c WindowContext,
    /// Reference to the main renderable buffer
    pub frame_buf: BufferRGBA<'c>,
    /// Reference to the off-screen buffer (can be used to prepare graphics in advance)
    pub draft_buf: Option<BufferRGBA<'c>>,
    /// A vector of user-defined data. Store any information here that needs to be passed around with the context
    pub user_data: Box<UserDataType>,
    /// A default color for rendering
    pub default_color: u32,
    /// Settings for rendering controls for Bezier curves
    pub bezier: Option<BezierContext>,
    /// Current frame in animation. If no animation is needed, can be set to `0`
    pub frame_count: usize,
    /// If false, the alpha channel will be ignored when performing image/color operations and rendering
    /// TODO: implement support for it in functions!
    pub use_alpha: bool,
}

impl<'d, UserDataType: Default> GraphContext<'d, UserDataType> {
    /// TODO: review the implementation of `new` method! It might need some adjustments.
    pub fn new(
        win: &'d WindowContext,
        frame_buf: BufferRGBA<'d>,
        use_alpha: bool,
        default_color: Option<u32>,
        user_data: Option<UserDataType>,
    ) -> GraphContext<'d, UserDataType> {
        GraphContext {
            win,
            frame_buf,
            draft_buf: None,
            default_color: default_color.unwrap_or(default_colors::FOREGROUND),
            // Use the provided `user_data` or default to `UserDataType::default()`
            user_data: Box::new(user_data.unwrap_or_default()),
            bezier: None,
            frame_count: 0,
            use_alpha,
        }
    }
}
