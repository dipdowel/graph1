#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GridPosition {
    pub row: usize,
    pub col: usize,
}

// TODO:
// TODO:
// TODO:
// TODO: Consider using `GridPosition` as argument type in functions in the grids instead of `row` and `column` separately!
// TODO: Consider using `GridPosition` as argument type in functions in the grids instead of `row` and `column` separately!
// TODO: Consider using `GridPosition` as argument type in functions in the grids instead of `row` and `column` separately!
// TODO:
// TODO:
// TODO:

impl GridPosition {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}