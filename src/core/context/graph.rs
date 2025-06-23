use crate::core::context::alpha::AlphaMethod;
use crate::core::context::line_context::LineContext;
use crate::core::context::{AlphaContext, BezierContext, WindowContext};
use crate::core::default_rng_seeds::{DEFAULT_SEED, DEFAULT_SEED_64};
use crate::draw::tools::brush::Brush;
use crate::utils::math::rng::XorShiftRng;
use std::ptr;

#[cfg(feature = "gpu")]
use crate::core::context::gpu::GpuContext;



#[derive(Debug, PartialEq, Eq)]
pub enum FrameBuffer {
    /// The main frame buffer, used for rendering the final image
    Primary,
    /// The draft buffer, used for off-screen rendering
    Draft,
}


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

    /// A random number generator (XorShiftRng) seeded with default values.
    /// If needed, reseed using:
    /// - `rng.set_seed_32(seed_u32)`
    /// - `rng.set_seed_64(seed_u64)`
    pub rng: XorShiftRng,

    /// Settings for line drawing
    pub line: LineContext,
    
    /// The brush used for paint-brush operations.
    pub brush: Brush,

    cur_buf_type: FrameBuffer,
    /*
    // TODO: Consider implementing the following feature:
    /// Autodetect when it's cheaper to perform an operation on just one thread (e.g. due to a small buffer size)
    /// and auto-switch to single-threaded mode and then back to multithreaded mode, once the operation is finished.
    pub num_threads_autoadjust:bool,
     */

    /// Experimental GPU rendering context.
    /// **NB:** Use only if you know what you are doing.
    #[cfg(feature = "gpu")]
    pub gpu_context: GpuContext,

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
            rng: XorShiftRng::new(DEFAULT_SEED, DEFAULT_SEED_64),
            line: line.unwrap_or_default(),
            brush: Brush::default(),
            cur_buf_type: FrameBuffer::Primary,

            #[cfg(feature = "gpu")]
            gpu_context: GpuContext::create(true).expect("Failed to create gpu context"),
        }
    }



}


impl<UserData> GraphContext<UserData> {

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
    
    /// Sets the pixel at (x, y) in the frame buffer to the specified color.
    /// If the coordinates are out of bounds, the pixel will not be set.
    /// # Arguments
    /// * `x` - The x-coordinate of the pixel
    /// * `y` - The y-coordinate of the pixel
    /// * `color` - The color to set the pixel to, in RGBA format (0xRRGGBBAA)
    pub fn set_pixel(&mut self, x: u32, y: u32, color: u32) {
        if x < self.win.w && y < self.win.h {
            self.frame_buf[(x + y * self.win.w) as usize] = color;
        }
    }

    /// Reads the pixel color from the frame buffer at (x, y).
    /// If the coordinates are out of bounds, returns `None`.
    /// # Arguments
    /// * `x` - The x-coordinate of the pixel
    /// * `y` - The y-coordinate of the pixel
    pub fn get_pixel(&self, x: u32, y: u32) -> Option<u32> {
        if x < self.win.w && y < self.win.h {
            Some(self.frame_buf[(x + y * self.win.w) as usize])
        } else {
            None
        }
    }

    /// Sets the current frame buffer to the specified type.
    /// If the requested buffer type is already the current one, no action is taken.
    /// # Arguments
    /// * `buf_type` - The type of the frame buffer to set (either `Primary` or `Draft`)
    pub fn set_frame_buf_to(&mut self, buf_type: FrameBuffer) {

        if buf_type == self.cur_buf_type {
            // No need to swap if the requested buffer is already the current one
            return;
        }
        std::mem::swap(&mut self.frame_buf, &mut self.draft_buf);
        self.cur_buf_type = buf_type;
    }


    /// Sets the GPU state to enabled or disabled. **!EXPERIMENTAL!**
    /// If the `gpu` feature is not enabled, this function does nothing and returns `None`.
    /// # Arguments
    /// * `enabled` - A boolean indicating whether to enable or disable the GPU context
    /// # Returns
    /// An `Option<bool>` indicating the previous state of the GPU context.
    /// If the `gpu` feature is not enabled, returns `None`.
    pub fn set_gpu_state(&mut self, enabled: bool) -> Option<bool> {

        #[allow(unused_mut)]
        let mut result:Option<bool> = None;

        #[cfg(feature = "gpu")]
        {
            self.gpu_context.enabled = enabled;
            result = Some(enabled)
        }
        result
    }

    /// Copies data between the primary and draft frame buffers.
    /// This function allows you to copy `ctx.frame_buf` to `ctx.draft_buf` or vice versa.
    /// /// # Arguments
    /// /// * `src` - The source frame buffer type (either `Primary` or `Draft`)
    /// /// * `dst` - The destination frame buffer type (either `Primary` or `Draft`)
    /// /// # Notes
    /// /// - If `src` and `dst` are the same, no action is taken.
    pub fn copy_frame_buf (&mut self, src: FrameBuffer, dst: FrameBuffer) {

        if src == dst {
            // No need to copy if the source and destination buffers are the same
            return;
        }

        match (src, dst) {
            (FrameBuffer::Primary, FrameBuffer::Draft) => {
                self.draft_buf.copy_from_slice(&self.frame_buf);
                unsafe {
                    ptr::copy_nonoverlapping(self.frame_buf.as_ptr(), self.draft_buf.as_mut_ptr(), self.frame_buf.len());
                }

            }
            (FrameBuffer::Draft, FrameBuffer::Primary) => {
                unsafe {
                    ptr::copy_nonoverlapping(self.draft_buf.as_ptr(), self.frame_buf.as_mut_ptr(), self.draft_buf.len());
                }
            }
            _ => {
                panic!("Error: `copy_frame_buf()` failed!");
            }
        }
    }

}