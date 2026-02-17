use crate::buffer_op::color::cheap::{ColorTransform};


pub fn color_transform_buffer(buffer: &mut [u32], transform: ColorTransform) {

}


/*
TODO:
      1. Analyze `brightness_buf.rs`, `contrast_buf.rs`, `hue_buf.rs`, and `tint_buf.rs` to identify common patterns and opportunities for code reuse.
        - E.g. In each of them we extract color channels like this:
        ```
            // Extract channels
                let r = ((pixel >> 24) & 0xFF) as i32;
                let g = ((pixel >> 16) & 0xFF) as i32;
                let b = ((pixel >> 8)  & 0xFF) as i32;
                let a =  (pixel        & 0xFF) as u32;
         ```
         That can probably be done just once
      2. Implement color_transform_buffer in such a way that there's only one loop for processing the buffer,
           and inside that loop we apply the necessary transformations based on the fields of `ColorTransform`.
           This way we can combine brightness, contrast, hue, and tint adjustments in a single pass over the pixel data.
      3. Make sure that each of the transformations `brightness_buffer()`, `contrast_buffer()`, `hue_buffer()`, `tint_buffer()`
         can still be used separately for cases where only one adjustment is needed.
      4. Struct `struct Hue` contains a matrix. Should we add similar color transformation matrices to `Brightness`, `Contrast`, and `Tint`
         to pre-compute their contributions to the overall color transform as well? Can we then just do some simple matrix operations
         to get the resulting color transform matrix for the combined adjustments?

*/