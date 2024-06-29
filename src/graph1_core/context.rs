// use graph1::primitives::primitives::{Dimensions2d, ImageData0RGB};

use crate::primitives::primitives::{Dimensions2d, ImageData0RGB};


pub struct WindowContext {
    pub w: u32,
    pub h: u32,
    pub w_usize: usize,
    pub h_usize: usize,

    /// Window width and height as a `Dimensions2d`
    pub dimensions: Dimensions2d
}



impl WindowContext {
    /// Instantiates a window context
    pub fn new(w:u32, h:u32) -> Self {

        return Self{
            w,
            h,
            w_usize: w as usize,
            h_usize: h as usize,
            dimensions: Dimensions2d{
                w,
                h
            }
        }

    }

}

pub struct GraphContext<'c> {
    pub win: &'c WindowContext,
    pub buf_view: ImageData0RGB<'c>, // TODO: consider replacing with `ImageBuffer`

    /// Use this to pass a color around when no other means are available, e.g.
    /// can be used to render visual shapes if no `Pixel` is passed
    pub default_color: u32,
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
