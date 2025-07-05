use crate::core::context::alpha::AlphaMethod;
use crate::core::context::line_context::LineContext;
use crate::core::context::{AlphaContext, BezierContext, WindowContext};
use crate::core::default_rng_seeds::{DEFAULT_SEED, DEFAULT_SEED_64};
use crate::draw::tools::brush::Brush;
use crate::utils::math::rng::XorShiftRng;
use std::ptr;

use crate::core::context::gpu::GpuContext;
use crate::core::default_colors::TRANSPARENT_BLACK;

#[derive(Debug)]
pub enum FrameBufferError {
    /// No error occurred yet
    NoError,
    /// An error occurred while trying to set the active frame buffer
    BadBufferIndex,
    /// An error occurred while trying to copy frame buffers
    PixelOutOfBounds,
    /// The frame buffer was resized to zero pixels (not an error per se, but a warning)
    ResizedToZero,
}

#[derive(Debug)]
pub struct GraphContext<UserData = Vec<i32>> {
    /// Reference to the window context
    pub win: WindowContext,

    /* start FRAME BUFFER related fields */
    /// The currently active renderable buffer
    pub frame_buf: Vec<u32>,

    /// A vector of initialized frame buffers
    frame_bufs: Vec<Vec<u32>>,

    /// The index of the currently active frame buffer
    active_frame_buf_index: usize,

    /// The status of the latest frame buffer operation
    frame_buf_error: FrameBufferError,

    /* end FRAME BUFFER related fields */
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
    /*
    // TODO: Consider implementing the following feature:
    /// Autodetect when it's cheaper to perform an operation on just one thread (e.g. due to a small buffer size)
    /// and auto-switch to single-threaded mode and then back to multithreaded mode, once the operation is finished.
    pub num_threads_autoadjust:bool,
     */
    /// Context for very experimental GPU rendering.
    /// **NB:** Use only if you know what you are doing!
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
    /// TODO: update the documentation (e.g. on `num_frame_bufs`)!
    /// TODO: update the documentation!
    /// TODO: update the documentation!
    /// TODO: update the documentation!
    /// # Returns
    /// A new `GraphContext` instance
    pub fn new(
        win: WindowContext,
        use_alpha: bool,
        num_frame_bufs: usize,
        user_data: Option<UserData>,
        num_threads: usize,
        line: Option<LineContext>,
    ) -> GraphContext<UserData> {
        // How many pixels are in the frame buffer
        let num_pixels = win.get_num_pixels();
        let bg_color = win.background_color;

        // Ensure at least one frame buffer will be created
        let num_frame_bufs = num_frame_bufs.max(1);

        // TODO: write a unit test that ensures that the number of frame buffers is always at least 1.

        let mut frame_buf: Vec<u32>;
        let mut frame_bufs: Vec<Vec<u32>> = Vec::new();

        println!("num_frame_bufs: {}", num_frame_bufs);

        // Only one frame buffer is needed, so it is created directly as the active frame buffer.
        if num_frame_bufs == 1 {
            frame_buf = vec![bg_color; num_pixels];
        } else {
            // Initialize the active frame buffer with a dummy.
            // TODO: check if we can actually avoid allocating memory for the dummy and still be able to use
            // TODO: `std::mem::swap` (or maybe something like `std::mem::take`?)

            frame_buf = vec![TRANSPARENT_BLACK; num_pixels];
            // Create additional frame buffers
            for _ in 1..=num_frame_bufs {
                frame_bufs.push(vec![bg_color; num_pixels]);
            }
        }

        let mut ctx = GraphContext {
            win,
            frame_buf,
            frame_bufs,
            active_frame_buf_index: 0, // Index of the currently active frame buffer
            frame_buf_error: FrameBufferError::NoError,

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
            gpu_context: GpuContext::create(),
        };

        // Set buffer `0` to be the active frame buffer
        if num_frame_bufs > 1 {
            std::mem::swap(&mut ctx.frame_buf, &mut ctx.frame_bufs[0]);
        }

        ctx
    }
}

impl<UserData> GraphContext<UserData> {
    /// Resizes the window context, the frame buffer, and the back frame buffers, if any.
    /// If the new size is zero, `frame_buf_error` is set to `FrameBufferError::ResizedToZero`.
    /// # Arguments
    /// * `w` - The new width of the window
    /// * `h` - The new height of the window
    pub fn resize(&mut self, w: u32, h: u32) {
        self.frame_buf_error = FrameBufferError::NoError;
        // resize the window (which also resizes the quadrants)
        self.win.resize(w, h);

        // resize the frame buffer
        let num_pixels = self.win.get_num_pixels();
        self.frame_buf.resize(num_pixels, self.win.background_color);

        if self.frame_bufs.len() > 0 {
            // resize all the existing frame buffers
            for frame_buf in &mut self.frame_bufs {
                frame_buf.resize(num_pixels, self.win.background_color);
            }
        }
        if num_pixels == 0 {
            self.frame_buf_error = FrameBufferError::ResizedToZero;
        }
    }

    /// Sets the pixel at (x, y) in the frame buffer to the specified color.
    /// If the coordinates are out of bounds, the pixel will not be set,
    /// and `frame_buf_error` will be set to `FrameBufferError::PixelOutOfBounds`.
    /// # Arguments
    /// * `x` - The x-coordinate of the pixel
    /// * `y` - The y-coordinate of the pixel
    /// * `color` - The color to set the pixel to, in RGBA format (0xRRGGBBAA)
    /// * `back_buf_index` - Optional index of the back buffer to set the pixel in. If `None`, sets the pixel in the active frame buffer.
    pub fn set_pixel(&mut self, x: u32, y: u32, color: u32) {
        self.frame_buf_error = FrameBufferError::NoError;
        if x < self.win.w && y < self.win.h {
            self.frame_buf[(x + y * self.win.w) as usize] = color;
            return;
        }
        self.frame_buf_error = FrameBufferError::PixelOutOfBounds;
    }

    /// Reads the pixel color from the frame buffer at (x, y).
    /// If the coordinates are out of bounds, returns `None`,
    /// and sets `frame_buf_error` to `FrameBufferError::PixelOutOfBounds`.
    /// # Arguments
    /// * `x` - The x-coordinate of the pixel
    /// * `y` - The y-coordinate of the pixel
    pub fn get_pixel(&mut self, x: u32, y: u32) -> Option<u32> {
        self.frame_buf_error = FrameBufferError::NoError;
        if x < self.win.w && y < self.win.h {
            Some(self.frame_buf[(x + y * self.win.w) as usize])
        } else {
            self.frame_buf_error = FrameBufferError::PixelOutOfBounds;
            None
        }
    }

    /// Returns index of a frame buffer that is currently active.
    pub fn get_active_frame_buf_index(&self) -> usize {
        self.active_frame_buf_index
    }

    /// Sets the active frame buffer to the one specified by `frame_buf_index`.
    /// If the specified index is the same as the currently active one, no operation is performed
    /// and `frame_buf_error`  is set to `FrameBufferError::BadBufferIndex`.
    /// # Arguments
    /// * `frame_buf_index` - The index of the frame buffer to set as active.
    pub fn set_active_frame_buf(&mut self, frame_buf_index: usize) {
        self.frame_buf_error = FrameBufferError::NoError;
        if frame_buf_index < self.frame_bufs.len() && frame_buf_index != self.active_frame_buf_index
        {
            // Return the currently active buffer to its place
            std::mem::swap(
                &mut self.frame_buf,
                &mut self.frame_bufs[self.active_frame_buf_index],
            );
            // Set the requested frame buffer as the active one
            std::mem::swap(&mut self.frame_buf, &mut self.frame_bufs[frame_buf_index]);
            self.active_frame_buf_index = frame_buf_index;
            return;
        }
        self.frame_buf_error = FrameBufferError::BadBufferIndex;
    }

    /// Copies the contents of the source frame buffer to the destination frame buffer.
    /// If the source and destination indices are the same, no operation is performed,
    /// and `frame_buf_error` is set to `FrameBufferError::BadBufferIndex`.
    ///
    /// # Arguments
    /// * `src_index` - The index of the source frame buffer to copy from.
    /// * `dst_index` - The index of the destination frame buffer to copy to.
    ///
    pub fn frame_buf_copy(&mut self, src_index: usize, dst_index: usize) {
        self.frame_buf_error = FrameBufferError::NoError;
        // Ensure the source and destination indices are within bounds and not the same
        if src_index < self.frame_bufs.len()
            && dst_index < self.frame_bufs.len()
            && src_index != dst_index
        {
            // Return the currently active buffer to its place in the vector for simplicity of indexing
            // Effectively, at this point `frame_buf` must reference the dummy filled with `default_colors::TRANSPARENT_BLACK`
            std::mem::swap(
                &mut self.frame_buf,
                &mut self.frame_bufs[self.active_frame_buf_index],
            );
            unsafe {
                ptr::copy_nonoverlapping(
                    self.frame_bufs[src_index].as_ptr(),
                    self.frame_bufs[dst_index].as_mut_ptr(),
                    self.frame_buf.len(),
                );
            }
            // Restore the active buffer reference
            std::mem::swap(
                &mut self.frame_buf,
                &mut self.frame_bufs[self.active_frame_buf_index],
            );
            return;
        }
        self.frame_buf_error = FrameBufferError::BadBufferIndex;
    }

    /// Copies the contents of the currently active frame buffer to the frame buffer specified by `frame_buf_index`.
    /// If the destination frame buffer and  the active one are the same, no operation is performed,
    /// and `frame_buf_error` is set to `FrameBufferError::BadBufferIndex`.
    /// # Arguments
    /// * `frame_buf_index` - The index of the frame buffer to copy to.
    pub fn copy_to_active_frame_buf_from(&mut self, frame_buf_index: usize) {
        self.frame_buf_error = FrameBufferError::NoError;
        if frame_buf_index < self.frame_bufs.len() && frame_buf_index != self.active_frame_buf_index
        {
            unsafe {
                ptr::copy_nonoverlapping(
                    self.frame_bufs[frame_buf_index].as_ptr(),
                    self.frame_buf.as_mut_ptr(),
                    self.frame_buf.len(),
                );
            }
            return;
        }
        self.frame_buf_error = FrameBufferError::BadBufferIndex;
    }

    /// Copies the contents of the specified frame buffer to the currently active frame buffer.
    /// If the source frame buffer is the active one, no operation is performed,
    /// and `frame_buf_error` is set to `FrameBufferError::BadBufferIndex`.
    /// # Arguments
    /// * `frame_buf_index` - The index of the frame buffer to copy from.
    pub fn copy_from_active_frame_buf_to(&mut self, frame_buf_index: usize) {
        self.frame_buf_error = FrameBufferError::NoError;
        if frame_buf_index < self.frame_bufs.len() && frame_buf_index != self.active_frame_buf_index
        {
            unsafe {
                ptr::copy_nonoverlapping(
                    self.frame_buf.as_ptr(),
                    self.frame_bufs[frame_buf_index].as_mut_ptr(),
                    self.frame_buf.len(),
                );
            }
            return;
        }
        self.frame_buf_error = FrameBufferError::BadBufferIndex;
    }
}
