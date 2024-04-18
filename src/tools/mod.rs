// pub mod draw;
pub mod debug;

pub mod init_screen_buffer;
pub mod fill;
pub mod primitives;

mod color_math;
pub use color_math::operations;

pub mod pixel_copy;
