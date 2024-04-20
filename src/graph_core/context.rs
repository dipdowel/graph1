
pub struct ContextWindow {
    pub w: u32,
    pub h: u32,
    pub w_usize: usize,
    pub h_usize: usize,
}

pub struct GraphContext<'a> {
    pub win: &'a ContextWindow,
    pub buf_view: &'a mut[u32]
}


