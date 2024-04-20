pub struct GraphContext<'a> {
    pub win_width: u32,
    pub win_height: u32,
    pub win_width_usize: usize,
    pub win_height_usize: usize,
    pub buf_view: &'a mut[u32]
}


