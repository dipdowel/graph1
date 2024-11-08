use crate::core::context::{AlphaContext, BezierContext, WindowContext};
use crate::core::context::alpha::AlphaMethod;
use crate::primitives::helper_types::BufferRGBA;

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
    /// Settings for rendering controls for Bezier curves
    pub bezier: Option<BezierContext>,
    /// Current frame in animation. If no animation is needed, can be set to `0`
    pub frame_count: usize,
    ///  Configurations for alpha blending (where applicable)
    pub alpha: AlphaContext,
}

impl<'d, UserDataType: Default> GraphContext<'d, UserDataType> {
    /// Instantiates a new `GraphContext`.
    /// The advanced settings like `alpha` and `bezier` are set to default,
    /// please configure them manually via your context instance.
    pub fn new(
        win: &'d WindowContext,
        frame_buf: BufferRGBA<'d>,
        use_alpha: bool,
        user_data: Option<UserDataType>,
    ) -> GraphContext<'d, UserDataType> {
        GraphContext {
            win,
            frame_buf,
            draft_buf: None,
            // Use the provided `user_data` or default to `UserDataType::default()`
            user_data: Box::new(user_data.unwrap_or_default()),
            bezier: None,
            frame_count: 0,
            alpha: AlphaContext {
                enabled: use_alpha,
                method: AlphaMethod::Int,
            },
        }
    }
}
