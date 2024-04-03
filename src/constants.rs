pub const MONITOR_WIDTH: u32 = 3440;
pub const MONITOR_HEIGHT: u32 = 1440;

pub const WIN_WIDTH: u32 = 640;
pub const WIN_HEIGHT: u32 = 480;

pub const WIN_X: u32 = MONITOR_WIDTH / 2 - WIN_WIDTH / 2;
pub const WIN_Y: u32 = MONITOR_HEIGHT / 2 - WIN_HEIGHT / 2;

pub const WIN_NAME_PREFIX: &str = "GF ";
pub const DEV_WIN_NAME_PREFIX: &str = "[dev] ";

pub const DEFAULT_BG_COLOR: u32 = 0x00_33_33_33;

pub const PIXEL_PER_PAGE: usize = (WIN_WIDTH * WIN_HEIGHT) as usize;


/// How many full-screen pages are there in the screen buffer
pub const NUM_OF_PAGES: usize = 4;
pub const TOTAL_BUFFER_SIZE: usize = NUM_OF_PAGES * (WIN_WIDTH * WIN_HEIGHT) as usize;
