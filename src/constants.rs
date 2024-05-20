use crate::graph1::primitives::primitives::Point;
use crate::init::init_window::{WIN_HEIGHT, WIN_WIDTH};


pub const DEFAULT_BG_COLOR: u32 = 0x00_33_33_33;

pub const PIXEL_PER_PAGE: usize = (WIN_WIDTH * WIN_HEIGHT) as usize;


/// How many full-screen pages are there in the screen buffer
pub const NUM_OF_PAGES: usize = 4;
pub const TOTAL_BUFFER_SIZE: usize = NUM_OF_PAGES * (WIN_WIDTH * WIN_HEIGHT) as usize;
pub const POINT_ZERO: Point = Point { x: 0, y: 0 };
