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
pub enum FrameBufferStatus {
    /// An error occurred while trying to set the active frame buffer
    ErrorBadBufferIndex = 100,
    /// An error occurred while trying to copy frame buffers
    ErrorPixelOutOfBounds = 200,
    /*
    /// Attempt to perform an indirect operation on the active buffer
    ErrorBufferIsActive = 300,
    */
    /// Warning: the buffer was resized to zero pixels.
    WarningResizedToZero = 1000,
    /// Warning: the source and destination frame buffers are the same.
    WarningSameSourceAndDestination = 1001,
}

#[derive(Debug)]
/// A struct that holds an immutable reference to a frame buffer
pub struct ImmutableFrameBuffer<'a> {
    /// The index of the frame buffer in the vector of frame buffers
    pub frame_buf_index: usize,
    /// A reference to the frame buffer itself
    pub frame_buf: &'a Vec<u32>,
}

#[derive(Debug)]
/// A struct that holds a mutable reference to the currently active frame buffer
/// and a vector of immutable references to other frame buffers.
pub struct MultipleFrameBuffers<'a> {
    /// A mutable reference to the currently active frame buffer
    pub active: &'a mut Vec<u32>,
    /// A vector of immutable references to other frame buffers
    pub immut: Vec<ImmutableFrameBuffer<'a>>,
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
    /// * `num_frame_bufs` - How many frame buffers to create (including the active one).
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

        let frame_buf: Vec<u32>;
        let mut frame_bufs: Vec<Vec<u32>> = Vec::new();

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
    /// If the new size is zero, `FrameBufferError::ResizedToZero` is returned.
    /// # Arguments
    /// * `w` - The new width of the window
    /// * `h` - The new height of the window
    pub fn resize(&mut self, w: u32, h: u32) -> Result<(), FrameBufferStatus> {
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
            return Err(FrameBufferStatus::WarningResizedToZero);
        }
        Ok(())
    }

    /// Sets the pixel at (x, y) in the frame buffer to the specified color.
    /// If the coordinates are out of bounds, the pixel will not be set,
    /// and `FrameBufferError::PixelOutOfBounds` is returned.
    /// # Arguments
    /// * `x` - The x-coordinate of the pixel
    /// * `y` - The y-coordinate of the pixel
    /// * `color` - The color to set the pixel to, in RGBA format (0xRRGGBBAA)
    /// * `back_buf_index` - Optional index of the back buffer to set the pixel in. If `None`, sets the pixel in the active frame buffer.
    pub fn set_pixel(&mut self, x: u32, y: u32, color: u32) -> Result<(), FrameBufferStatus> {
        if x < self.win.w && y < self.win.h {
            self.frame_buf[(x + y * self.win.w) as usize] = color;
            return Ok(());
        }
        Err(FrameBufferStatus::ErrorPixelOutOfBounds)
    }

    /// Reads the pixel color from the frame buffer at (x, y).
    /// If the coordinates are out of bounds, returns `None`,
    /// and `FrameBufferError::PixelOutOfBounds` is returned.
    /// # Arguments
    /// * `x` - The x-coordinate of the pixel
    /// * `y` - The y-coordinate of the pixel
    pub fn get_pixel(&mut self, x: u32, y: u32) -> Result<u32, FrameBufferStatus> {
        if x < self.win.w && y < self.win.h {
            return Ok(self.frame_buf[(x + y * self.win.w) as usize]);
        }
        Err(FrameBufferStatus::ErrorPixelOutOfBounds)
    }

    /// Returns index of a frame buffer that is currently active.
    pub fn get_active_frame_buf_index(&self) -> usize {
        self.active_frame_buf_index
    }

    /// Sets the active frame buffer to the one specified by `frame_buf_index`.
    /// If the specified index is the same as the currently active one, no operation is performed
    /// and `FrameBufferError::BadBufferIndex` is returned.
    /// # Arguments
    /// * `frame_buf_index` - The index of the frame buffer to set as active.
    pub fn set_active_frame_buf(
        &mut self,
        frame_buf_index: usize,
    ) -> Result<(), FrameBufferStatus> {
        if frame_buf_index == self.active_frame_buf_index {
            // No operation needed, the requested buffer is already active
            return Err(FrameBufferStatus::WarningSameSourceAndDestination);
        }

        if frame_buf_index < self.frame_bufs.len() {
            // Return the currently active buffer to its place
            std::mem::swap(
                &mut self.frame_buf,
                &mut self.frame_bufs[self.active_frame_buf_index],
            );
            // Set the requested frame buffer as the active one
            std::mem::swap(&mut self.frame_buf, &mut self.frame_bufs[frame_buf_index]);
            self.active_frame_buf_index = frame_buf_index;
            return Ok(());
        }
        Err(FrameBufferStatus::ErrorBadBufferIndex)
    }

    /// Copies the contents of the source frame buffer to the destination frame buffer.
    /// If the source and destination indices are the same, no operation is performed,
    /// and `FrameBufferError::BadBufferIndex` is returned.
    ///
    /// # Arguments
    /// * `src_index` - The index of the source frame buffer to copy from.
    /// * `dst_index` - The index of the destination frame buffer to copy to.
    ///
    pub fn frame_buf_copy(
        &mut self,
        src_index: usize,
        dst_index: usize,
    ) -> Result<(), FrameBufferStatus> {
        if src_index == dst_index {
            // Can't copy from a buffer to itself, no operation needed
            return Err(FrameBufferStatus::WarningSameSourceAndDestination);
        }

        // Ensure the source and destination indices are within bounds and not the same
        if src_index < self.frame_bufs.len() && dst_index < self.frame_bufs.len() {
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
            return Ok(());
        }
        Err(FrameBufferStatus::ErrorBadBufferIndex)
    }

    /// Copies the contents of the currently active frame buffer to the frame buffer specified by `frame_buf_index`.
    /// If the destination frame buffer and  the active one are the same, no operation is performed,
    /// and `FrameBufferError::BadBufferIndex` is returned.
    /// # Arguments
    /// * `frame_buf_index` - The index of the frame buffer to copy to.
    pub fn copy_to_active_frame_buf_from(
        &mut self,
        frame_buf_index: usize,
    ) -> Result<(), FrameBufferStatus> {
        if frame_buf_index == self.active_frame_buf_index {
            // Can't copy from a buffer to itself, no operation needed
            return Err(FrameBufferStatus::WarningSameSourceAndDestination);
        }

        if frame_buf_index < self.frame_bufs.len() {
            unsafe {
                ptr::copy_nonoverlapping(
                    self.frame_bufs[frame_buf_index].as_ptr(),
                    self.frame_buf.as_mut_ptr(),
                    self.frame_buf.len(),
                );
            }
            return Ok(());
        }
        Err(FrameBufferStatus::ErrorBadBufferIndex)
    }

    /// Copies the contents of the specified frame buffer to the currently active frame buffer.
    /// If the source frame buffer is the active one, no operation is performed,
    /// and `FrameBufferError::BadBufferIndex` is returned.
    /// # Arguments
    /// * `frame_buf_index` - The index of the frame buffer to copy from.
    pub fn copy_from_active_frame_buf_to(
        &mut self,
        frame_buf_index: usize,
    ) -> Result<(), FrameBufferStatus> {
        if frame_buf_index == self.active_frame_buf_index {
            // Can't copy from a buffer to itself, no operation needed
            return Err(FrameBufferStatus::WarningSameSourceAndDestination);
        }

        if frame_buf_index < self.frame_bufs.len() {
            unsafe {
                ptr::copy_nonoverlapping(
                    self.frame_buf.as_ptr(),
                    self.frame_bufs[frame_buf_index].as_mut_ptr(),
                    self.frame_buf.len(),
                );
            }
            return Ok(());
        }
        Err(FrameBufferStatus::ErrorBadBufferIndex)
    }

    /// Returns a mutable reference to the currently active frame buffer and
    /// a vector of immutable references to frame buffers specified by the caller.
    /// # Arguments
    /// * `frame_buf_indices` - Indices of the frame buffers to return as immutable.
    /// # Returns
    /// A `MultipleFrameBuffers` struct containing:
    /// * `active` - A mutable reference to the currently active frame buffer.
    /// * `immut` - A vector of immutable references to the frame buffers specified by `frame_buf_indices`.
    /// # Note
    /// - **NB:** If you want `immut` to maintain the order of the frame buffers specified in `frame_buf_indices`,
    /// **do not** include the active frame buffer index in `frame_buf_indices`.
    /// - The active frame buffer will not be included in the `immut` vector as it is returned as mutable in `active`.
    pub fn get_multi_frame_bufs(
        &mut self,
        frame_buf_indices: &[usize],
    ) -> Result<MultipleFrameBuffers, FrameBufferStatus> {
        let mut immut_frame_bufs: Vec<ImmutableFrameBuffer> = Vec::new();

        for buf_index in frame_buf_indices {
            let buf_index = *buf_index;

            if buf_index >= self.frame_bufs.len() {
                return Err(FrameBufferStatus::ErrorBadBufferIndex);
            }

            // Skip the active frame buffer as it'll be returned as mutable.
            if buf_index == self.active_frame_buf_index {
                continue;
            }

            immut_frame_bufs.push(ImmutableFrameBuffer {
                frame_buf_index: buf_index,
                frame_buf: &self.frame_bufs[buf_index],
            });
        }
        Ok(MultipleFrameBuffers {
            active: &mut self.frame_buf,
            immut: immut_frame_bufs,
        })
    }
}

/***************************************************************************************************
----------------------------------------------------------------------------------------------------
                   [████████]  [██████]  [██████]  [████████]  [██████]
                      [██]     [██]      [██]         [██]     [██]
                      [██]     [████]    [██████]     [██]     [██████]
                      [██]     [██]          [██]     [██]         [██]
                      [██]     [██████]  [██████]     [██]     [██████]
....................................................................................................
****************************************************************************************************/

#[cfg(test)]
mod tests {
    use super::*;

    fn make_ctx(width: u32, height: u32, num_bufs: usize) -> GraphContext<()> {
        let background = Some(0xFF0000FF); // Red, fully opaque
        let foreground = Some(0xFFFFFFFF); // White, fully opaque
        let win = WindowContext::new(width, height, background, foreground);
        GraphContext::new(win, false, num_bufs, None, 1, None)
    }

    #[test]
    fn test_new_single_buffer() {
        let width = 4;
        let height = 3;
        let ctx = make_ctx(width, height, 1);
        assert_eq!(ctx.frame_buf.len(), (width * height) as usize);
        assert_eq!(ctx.get_active_frame_buf_index(), 0);

        // Check that the second buffer is initialized with the background color
        assert!(ctx.frame_buf.iter().all(|&c| c == ctx.win.background_color));
    }

    #[test]
    fn test_new_multi_buffer() {
        let width = 2;
        let height = 2;
        let ctx = make_ctx(width, height, 2);
        assert_eq!(ctx.frame_buf.len(), 4);
        assert_eq!(ctx.get_active_frame_buf_index(), 0);
        assert_eq!(ctx.frame_bufs.len(), 2);
        // Check that the second buffer is initialized
        assert_eq!(ctx.frame_bufs[1].len(), 4);
        // Check that the second buffer is initialized with the background color
        assert!(ctx.frame_bufs[1]
            .iter()
            .all(|&c| c == ctx.win.background_color));
    }

    #[test]
    fn test_resize() {
        let mut ctx = make_ctx(2, 2, 1);
        assert_eq!(ctx.frame_buf.len(), 4);
        ctx.resize(3, 6).unwrap();
        assert_eq!(ctx.frame_buf.len(), 3 * 6);
        assert_eq!(ctx.win.w, 3);
        assert_eq!(ctx.win.h, 6);
    }

    #[test]
    fn test_resize_to_zero() {
        let mut ctx = make_ctx(2, 2, 1);
        let res = ctx.resize(10, 0);
        assert!(matches!(res, Err(FrameBufferStatus::WarningResizedToZero)));

        let res = ctx.resize(4, 4);
        assert!(matches!(res, Ok(())));
        assert_eq!(ctx.frame_buf.len(), 4 * 4);

        let res = ctx.resize(0, 10);
        assert!(matches!(res, Err(FrameBufferStatus::WarningResizedToZero)));
    }

    #[test]
    fn test_set_and_get_pixel() {
        let mut ctx = make_ctx(3, 2, 1);
        let color = 0xAABBCCDD;
        assert!(ctx.set_pixel(1, 1, color).is_ok());
        assert_eq!(ctx.get_pixel(1, 1).unwrap(), color);
        // Out of bounds
        assert!(ctx.set_pixel(10, 10, color).is_err());
        assert!(ctx.get_pixel(10, 10).is_err());
    }

    #[test]
    fn test_active_frame_buf_index() {
        let ctx = make_ctx(2, 2, 2);
        assert_eq!(ctx.get_active_frame_buf_index(), 0);
    }

    #[test]
    fn test_set_active_frame_buf_and_swap() {
        let mut ctx = make_ctx(2, 2, 3);
        // Write unique values to each buffer in turn
        for buf_index in 1..3 {
            ctx.set_active_frame_buf(buf_index).unwrap();
            for i in 0..ctx.frame_buf.len() {
                ctx.frame_buf[i] = (buf_index as u32) * 0x11111111;
            }
        }
        // Now verify that values persist after swaps
        for buf_index in 1..3 {
            ctx.set_active_frame_buf(buf_index).unwrap();
            assert!(ctx
                .frame_buf
                .iter()
                .all(|&v| v == (buf_index as u32) * 0x11111111));
        }
        // Out of bounds should error
        let res = ctx.set_active_frame_buf(100);
        // check that the error is `FrameBufferStatus::ErrorBadBufferIndex`
        assert!(matches!(res, Err(FrameBufferStatus::ErrorBadBufferIndex)));
    }

    #[test]
    fn test_frame_buf_copy() {
        let mut ctx = make_ctx(2, 2, 2);
        // Fill buffer 0 with a pattern
        for i in 0..ctx.frame_buf.len() {
            ctx.frame_buf[i] = 0x12345678;
        }
        // Copy from buffer 0 to buffer 1
        assert!(ctx.frame_buf_copy(0, 1).is_ok());
        // Switch to buffer 1 and check values
        ctx.set_active_frame_buf(1).unwrap();
        assert!(ctx.frame_buf.iter().all(|&v| v == 0x12345678));
        // Should fail on same src/dst
        let res = ctx.frame_buf_copy(0, 0);
        assert!(matches!(
            res,
            Err(FrameBufferStatus::WarningSameSourceAndDestination)
        ));
    }

    #[test]
    fn test_copy_to_and_from_active_frame_buf() {
        let mut ctx = make_ctx(2, 2, 2);
        // Write distinct data to buffer 0 and 1

        let res = ctx.set_active_frame_buf(0);
        // this is expected, and it's just a warning
        assert!(matches!(
            res,
            Err(FrameBufferStatus::WarningSameSourceAndDestination)
        ));

        for i in 0..ctx.frame_buf.len() {
            ctx.frame_buf[i] = 0xCAFEBABE;
        }
        ctx.set_active_frame_buf(1).unwrap();
        for i in 0..ctx.frame_buf.len() {
            ctx.frame_buf[i] = 0xDEADBEEF;
        }
        // Copy buffer 1 into buffer 0 (active <- from)
        assert!(ctx.copy_to_active_frame_buf_from(0).is_ok());
        assert!(ctx.frame_buf.iter().all(|&v| v == 0xCAFEBABE));
        // Copy active buffer (1) into buffer 0 (from active -> to)
        ctx.set_active_frame_buf(0).unwrap();
        assert!(ctx.copy_from_active_frame_buf_to(1).is_ok());
        ctx.set_active_frame_buf(1).unwrap();
        assert!(ctx.frame_buf.iter().all(|&v| v == 0xCAFEBABE));
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////
    #[test]
    fn returns_active_and_immut_refs_when_indices_valid() {
        let mut ctx = make_ctx(2, 2, 3);
        let indices = [0, 1, 2];
        let res = ctx.get_multi_frame_bufs(&indices);
        assert!(res.is_ok());
        let bufs = res.unwrap();

        // Should skip index 0 in immut (active), so only [1, 2]
        let immut_indices: Vec<_> = bufs.immut.iter().map(|f| f.frame_buf_index).collect();
        assert_eq!(immut_indices, vec![1, 2]);
        // Active buffer should be mutable
        bufs.active[0] = 0x22222222;

        // The active buffer is 0 by default
        assert_eq!(ctx.get_active_frame_buf_index(), 0);
    }

    #[test]
    fn order_is_preserved_and_active_is_skipped() {
        let mut ctx = make_ctx(2, 2, 4);
        ctx.set_active_frame_buf(2).unwrap();
        let indices = [3, 0, 2, 1];
        let res = ctx.get_multi_frame_bufs(&indices);
        assert!(res.is_ok());
        let bufs = res.unwrap();
        let immut_indices: Vec<_> = bufs.immut.iter().map(|f| f.frame_buf_index).collect();
        // 2 is active, so only [3,0,1] in order
        assert_eq!(immut_indices, vec![3, 0, 1]);
    }

    #[test]
    fn returns_error_for_any_oob_index() {
        let mut ctx = make_ctx(2, 2, 2);
        // Buffers: 0,1; index 2 is invalid
        let indices = [0, 2];
        let res = ctx.get_multi_frame_bufs(&indices);
        assert!(matches!(res, Err(FrameBufferStatus::ErrorBadBufferIndex)));
    }

    #[test]
    fn returns_empty_immut_if_only_active_requested() {
        let mut ctx = make_ctx(2, 2, 2);
        let indices = [0]; // 0 is active
        let res = ctx.get_multi_frame_bufs(&indices);
        assert!(res.is_ok());
        let bufs = res.unwrap();
        assert!(bufs.immut.is_empty());
    }

    #[test]
    fn active_mutation_reflects_on_context() {
        let mut ctx = make_ctx(2, 2, 2);
        let indices = [0, 1];
        let res = ctx.get_multi_frame_bufs(&indices).unwrap();
        for px in res.active.iter_mut() {
            *px = 0xAABBCCDD;
        }
        assert!(ctx.frame_buf.iter().all(|&px| px == 0xAABBCCDD));
    }
}
