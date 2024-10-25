// use graph1::primitives::primitives::{Dimensions2d, ImageData0RGB};

use crate::primitives::primitives::{Dimensions2d, ImageData0RGB};

#[derive(Debug)]
pub struct WindowContext {
    pub w: u32,
    pub h: u32,
    pub w_usize: usize,
    pub h_usize: usize,
    /// Size of the buffer needed to represent the window
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
    pub win: &'c WindowContext,
    pub buf_view: ImageData0RGB<'c>, // TODO: consider replacing with `ImageBuffer`

    /// Use this to pass a color around when no other means are available, e.g.
    /// can be used to render visual shapes if no `Pixel` is passed
    pub default_color: u32,
    /// Settings for rendering controls for Bezier curves
    pub bezier: BezierContext,
    // image_buffer: ImageBuffer<'c>,
}

// impl<'c> GraphContext<'c> {
//
//     pub fn new(
//         win: &'c ContextWindow,
//         mut buf_view: &'c VecImageData0RGB<'c>,
//         default_color: u32,
//     ) -> Self {
//         let image_buffer = ImageBuffer {
//             buf:   buf_view,
//             // buf: &mut buf_view,
//             dimensions: Dimensions2d {
//                 w: win.w,
//                 h: win.h,
//             },
//         };
//
//         Self{
//             win,
//             default_color,
//             image_buffer
//         }
//
//     }
//     pub fn get_image_buffer(&'c mut self) -> &mut ImageBuffer {
//         &mut self.image_buffer
//     }
//
//     // pub fn get_buf_view(&'c mut self) -> &mut VecImageData0RGB {
//     //     &mut self.image_buffer.buf
//     // }
// }
