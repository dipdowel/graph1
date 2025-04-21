use crate::core::context::alpha::AlphaMethod;
use crate::core::context::line_context::LineContext;
use crate::core::context::{AlphaContext, BezierContext, WindowContext};


#[derive(Debug)]
pub struct GraphContext<UserData = Vec<i32>> {
    /// Reference to the window context
    pub win: WindowContext,
    /// The main renderable buffer
    pub frame_buf: Vec<u32>,
    /// The off-screen buffer (can be used to prepare graphics in advance)
    pub draft_buf: Vec<u32>,
    /// Whether to initialise and use the draft buffer
    pub use_draft_buf: bool,
    /// A vector of user-defined data. Store any information here that needs to be passed around with the context
    pub user_data: Box<UserData>,
    /// Settings for rendering controls for Bezier curves
    pub bezier: BezierContext,
    /// Current frame in animation. If no animation is needed, can be set to `0`
    pub frame_count: usize,
    ///  Configurations for alpha blending (where applicable)
    pub alpha: AlphaContext,
    /// Some operations in Graph1 can be sped up using multiple CPUs / CPU cores.
    /// If the target hardware has multiple cores and `num_threads > 1`,
    /// Graph1 will attempt to create `num_threads` threads to parallelize some computations.
    /// If `1`, Graph1 will perform calculations only on the main thread.
    /// If `0`, Graph1 will not perform those operations, that support multithreading. Not recommended for usage.
    pub num_threads: usize,

    /// Settings for line drawing
    pub line: LineContext,
    
    /*
    // TODO: Consider implementing the following feature:
    /// Autodetect when it's cheaper to perform an operation on just one thread (e.g. due to a small buffer size)
    /// and auto-switch to single-threaded mode and then back to multithreaded mode, once the operation is finished.
    pub num_threads_autoadjust:bool,
     */
}

impl<UserData: Default> GraphContext<UserData> {
    /// Instantiates a new `GraphContext`.
    /// The advanced settings like `alpha` and `bezier` are set to default,
    /// please configure them manually via your context instance.
    /// # Arguments
    /// * `win` - The window context
    /// * `use_alpha` - Whether to enable alpha blending
    /// * `use_draft_buf` - if `true`, create and use the draft buffer (same size as the frame buffer)
    /// * `user_data` - Optional user-defined data
    /// * `num_threads` - How many threads to use for rendering.
    ///     * `0` - skip operations that support multithreading (rather should not be used).
    ///     * `1` - use main thread only.
    ///     * `2` - and more - use that many threads.
    /// * `line` - Optional line context. If `None` given, default settings will be used.
    /// # Returns
    /// A new `GraphContext` instance
    pub fn new(
        win: WindowContext,
        use_alpha: bool,
        use_draft_buf: bool,
        user_data: Option<UserData>,
        num_threads: usize,
        line: Option<LineContext>,
    ) -> GraphContext<UserData> {
        // How many pixels are in the frame buffer
        let num_pixels = win.get_num_pixels();
        let bg_color = win.background_color;

        let draft_buf = if use_draft_buf {
            vec![bg_color; num_pixels]
        } else {
            vec![bg_color; 0]
        };

        GraphContext {
            win,
            frame_buf: vec![bg_color; num_pixels],
            draft_buf,
            use_draft_buf,

            // Use the provided `user_data` or default to `UserData::default()`
            user_data: Box::new(user_data.unwrap_or_default()),
            bezier: BezierContext::default(),
            frame_count: 0,
            alpha: AlphaContext {
                enabled: use_alpha,
                method: AlphaMethod::Int,
            },
            num_threads,

            line: line.unwrap_or_default(),
        }
    }

    /// Resizes the window context, the frame buffer, and the draft buffer (if `use_draft_buf == true`)
    pub fn resize(&mut self, w: u32, h: u32) {
        // resize the window (which also resizes the quadrants)
        self.win.resize(w, h);

        // resize the frame buffer
        let num_pixels = self.win.get_num_pixels();
        self.frame_buf.resize(num_pixels, self.win.background_color);

        // resize the draft buffer if it's enabled
        if self.use_draft_buf {
            self.draft_buf.resize(num_pixels, self.win.background_color);
        }
    }
}
