use crate::core::context::alpha::AlphaMethod;
use crate::core::context::line_context::LineContext;
use crate::core::context::{AlphaContext, BezierContext, WindowContext};
use crate::core::default_rng_seeds::{DEFAULT_SEED, DEFAULT_SEED_64};
use crate::draw::tools::brush::Brush;
use crate::utils::math::rng::XorShiftRng;
use std::ptr;

use crate::core::context::gpu::GpuContext;

#[derive(Debug)]
pub struct GraphContext<UserData = Vec<i32>> {
    /// Reference to the window context
    pub win: WindowContext,

    /// The currently active renderable buffer
    pub frame_buf: Vec<u32>,

    /// A vector of the back buffers
    back_bufs: Vec<Vec<u32>>,

    /// A vector of indices of back buffers that were swapped with the frame buffer
    /// This is used to undo the swaps in the reverse order compared to how they were done.
    back_buf_swap_indices: Vec<usize>,

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
    /// TODO: update the documentation!
    /// TODO: update the documentation!
    /// TODO: update the documentation!
    /// TODO: update the documentation!
    /// # Returns
    /// A new `GraphContext` instance
    pub fn new(
        win: WindowContext,
        use_alpha: bool,
        num_back_bufs: usize,
        user_data: Option<UserData>,
        num_threads: usize,
        line: Option<LineContext>,
    ) -> GraphContext<UserData> {
        // How many pixels are in the frame buffer
        let num_pixels = win.get_num_pixels();
        let bg_color = win.background_color;
        
        
        
        let back_bufs = if num_back_bufs > 0 {
            println!("Creating {} back buffers", num_back_bufs);
            vec![vec![bg_color; num_pixels]; num_back_bufs]
        } else {
            println!("No back buffers created");
            vec![]
        };

        GraphContext {
            win,
            frame_buf: vec![bg_color; num_pixels],
            back_bufs,
            back_buf_swap_indices: Vec::new(), // No back buffer swapped yet
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

        // resize the back buffers, if any
        if self.back_bufs.len() > 0 {
            for buf in &mut self.back_bufs {
                buf.resize(num_pixels, self.win.background_color);
            }
        }
    }

    /// Sets the pixel at (x, y) in the frame buffer to the specified color.
    /// If the coordinates are out of bounds, the pixel will not be set.
    /// # Arguments
    /// * `x` - The x-coordinate of the pixel
    /// * `y` - The y-coordinate of the pixel
    /// * `color` - The color to set the pixel to, in RGBA format (0xRRGGBBAA)
    /// * `back_buf_index` - Optional index of the back buffer to set the pixel in. If `None`, sets the pixel in the active frame buffer.
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

    /// Sets the pixel at (x, y) in the specified back buffer to the specified color.
    /// If the coordinates are out of bounds or the back buffer index is invalid, the pixel will not be set.
    /// /// # Arguments
    /// /// * `x` - The x-coordinate of the pixel
    /// /// * `y` - The y-coordinate of the pixel
    /// /// * `color` - The color to set the pixel to, in RGBA format (0xRRGGBBAA)
    /// /// * `back_buf_index` - The index of the back buffer to set the pixel in
    pub fn set_pixel_back_buf(&mut self, x: u32, y: u32, color: u32, back_buf_index: usize) {
        if back_buf_index < self.back_bufs.len() && x < self.win.w && y < self.win.h {
            // Set the pixel in the specified back buffer
            self.back_bufs[back_buf_index][(x + y * self.win.w) as usize] = color;
        }
        // TODO: if context internal log will be introduced, log the back_buf_index out of bounds error
    }

    /// Reads the pixel color from the specified back buffer at (x, y).
    /// /// # Arguments
    /// /// * `x` - The x-coordinate of the pixel
    /// /// * `y` - The y-coordinate of the pixel
    /// /// * `back_buf_index` - The index of the back buffer to read from
    /// /// # Returns
    /// /// An `Option<u32>` containing the pixel color if the coordinates are valid and the back buffer index is within bounds,
    /// /// or `None` if the coordinates are out of bounds or the back buffer index is invalid.
    pub fn get_pixel_back_buf(&self, x: u32, y: u32, back_buf_index: usize) -> Option<u32> {
        if back_buf_index < self.back_bufs.len() && x < self.win.w && y < self.win.h {
            Some(self.back_bufs[back_buf_index][(x + y * self.win.w) as usize])
        } else {
            None
        }
    }

    /// Swaps the frame buffer with a specified back buffer.
    /// # Arguments
    /// * `back_buf_index` - The index of the back buffer to swap with the frame buffer.
    pub fn swap_frame_buf_with(&mut self, back_buf_index: usize) {
        if back_buf_index < self.back_bufs.len() {
            std::mem::swap(&mut self.frame_buf, &mut self.back_bufs[back_buf_index]);
            self.back_buf_swap_indices.push(back_buf_index);

        }
    }

    /// Undoes the last frame buffer swap.
    /// Tha by swapping the frame buffer back with the last swapped back buffer.
    /// If there are no swaps to undo, this function does nothing.
    pub fn undo_last_frame_buf_swap(&mut self) {
        if self.back_buf_swap_indices.is_empty() {
            return; // No swaps to undo
        }
        let back_buf_index = self.back_buf_swap_indices.pop().expect("back_buf_swap is unexpectedly empty!");
        if back_buf_index < self.back_bufs.len() {
            std::mem::swap(&mut self.frame_buf, &mut self.back_bufs[back_buf_index]);
        }
    }

    /// Copies data from a back buffer to the frame buffer.
    /// # Arguments
    /// * `back_buf_index` - The index of the back buffer to copy from.
    pub fn copy_to_frame_buf_from(&mut self, back_buf_index: usize) {
        if back_buf_index < self.back_bufs.len() {
            unsafe {
                ptr::copy_nonoverlapping(
                    self.back_bufs[back_buf_index].as_ptr(),
                    self.frame_buf.as_mut_ptr(),
                    self.frame_buf.len(),
                );
            }
        }
    }

    /// Copies data from a back buffer to the frame buffer.
    /// # Arguments
    /// * `back_buf_index` - The index of the back buffer to copy from.
    pub fn copy_from_frame_buf_to(&mut self, back_buf_index: usize) {
        if back_buf_index < self.back_bufs.len() {
            unsafe {
                ptr::copy_nonoverlapping(
                    self.frame_buf.as_ptr(),
                    self.back_bufs[back_buf_index].as_mut_ptr(),
                    self.frame_buf.len(),
                );
            }
        }
    }
}
