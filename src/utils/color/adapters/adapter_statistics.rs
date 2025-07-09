#[derive(Debug)]
pub struct AdapterStatistics {
    /// Average color (ABGR) of the pixels that were converted
    pub average_color: u32,
    /// Average value of the red channel
    pub average_red: u32,
    /// Average value of the green channel
    pub average_green: u32,
    /// Average value of the blue channel
    pub average_blue: u32,
    /// Number of pixels that were converted
    pub num_pixels: u32,
}
