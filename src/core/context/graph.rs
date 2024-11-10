use crate::core::context::{AlphaContext, BezierContext, WindowContext};
use crate::core::context::alpha::AlphaMethod;
use crate::primitives::plane::Dimensions2d;


/// Helps resize the window context
fn resize_window(win: &mut WindowContext, w: u32, h: u32) {
    win.w = w;
    win.h = h;
    win.w_usize = w as usize;
    win.h_usize = h as usize;
    win.w_i32 = w as i32;
    win.h_i32 = h as i32;
    win.dimensions = Dimensions2d { w, h };
}

#[derive(Debug)]
pub struct GraphContext<UserDataType = Vec<i32>> {
    /// Reference to the window context
    pub win: WindowContext,
    /// The main renderable buffer
    pub frame_buf: Vec<u32>,
    /// The off-screen buffer (can be used to prepare graphics in advance)
    pub draft_buf: Vec<u32>,
    /// Whether to initialise and use the draft buffer
    pub use_draft_buf: bool,
    /// A vector of user-defined data. Store any information here that needs to be passed around with the context
    pub user_data: Box<UserDataType>,
    /// Settings for rendering controls for Bezier curves
    pub bezier: Option<BezierContext>,
    /// Current frame in animation. If no animation is needed, can be set to `0`
    pub frame_count: usize,
    ///  Configurations for alpha blending (where applicable)
    pub alpha: AlphaContext,
}

impl<UserDataType: Default> GraphContext<UserDataType> {
    /// Instantiates a new `GraphContext`.
    /// The advanced settings like `alpha` and `bezier` are set to default,
    /// please configure them manually via your context instance.
    /// # Arguments
    /// * `win` - The window context
    /// * `use_alpha` - Whether to enable alpha blending
    /// * `use_draft_buf` - if `true`, create and use the draft buffer (same size as the frame buffer)
    /// * `user_data` - Optional user-defined data
    /// # Returns
    /// A new `GraphContext` instance
    pub fn new(
        win: WindowContext,
        use_alpha: bool,
        use_draft_buf: bool,
        user_data: Option<UserDataType>,
    ) -> GraphContext<UserDataType> {


        // How many pixels are in the frame buffer
        let num_pixels =  win.get_num_pixels();
        let bg_color = win.background_color;

        let draft_buf = if use_draft_buf {
            vec![bg_color; num_pixels]
        } else {
            vec![bg_color; 0]
        };

        GraphContext {
            win,
            frame_buf:vec![bg_color; num_pixels],
            draft_buf,
            use_draft_buf,

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



    /// Resizes the window context, the frame buffer, and the draft buffer (if `use_draft_buf == true`)
    pub fn resize(&mut self, w: u32, h: u32) {
        // resize the window and the frame buffer
        resize_window (&mut self.win,w, h);
        let num_pixels =  self.win.get_num_pixels();

        self.frame_buf.resize(num_pixels, self.win.background_color);

        // resize the draft buffer if it's enabled
        if self.use_draft_buf {
            self.draft_buf.resize(num_pixels, self.win.background_color);
        }
    }

}
