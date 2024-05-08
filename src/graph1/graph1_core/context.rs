
pub struct ContextWindow {
    pub w: u32,
    pub h: u32,
    pub w_usize: usize,
    pub h_usize: usize,
}

pub struct GraphContext<'rendering> {
    pub win: &'rendering ContextWindow,
    pub buf_view: &'rendering mut[u32],
    /// Use this to pass a color around when no other means are available, e.g.
    /// can be used to render visual shapes if no `Pixel` is passed
    pub default_color: u32
}


