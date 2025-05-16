use crate::primitives::numeric::Numeric;
use crate::primitives::plane::{Dimensions2d, RectArea};
use crate::utils::math::geometry::region::Region;

/// A 2D uniform grid of rectangular `Region` cells
#[derive(Debug, Clone)]
pub struct UniformGrid<T: Numeric> {
    cells: Vec<Region<T>>,
    /// This `RectArea` is used to construct the grid cells.
    /// The top-left corner of the grid corresponds to the top-left corner of this `RectArea`.
    /// The default color of the grid cells is the same as the color of this `RectArea`.
    pub proto_cell: RectArea<T>,
    /// Number of rows in the grid
    pub rows: usize,
    /// Number of columns in the grid
    pub cols: usize,
}

impl<T: Numeric> UniformGrid<T> {
    /// Constructs a new grid based on the provided props
    pub fn new(
        /// This `RectArea` is used to construct the grid cells.
        /// The top-left corner of the grid corresponds to the top-left corner of this `RectArea`.
        /// The default color of the grid cells is the same as the color of this `RectArea`.
        proto_cell: RectArea<T>,
        /// Number of rows in the grid
        rows: usize,
        /// Number of columns in the grid
        cols: usize,
    ) -> Self {
        let num_cells = rows * cols;
        let mut cells = Vec::with_capacity(num_cells);
        for _ in 0..num_cells {
            cells.push(Region::new(proto_cell.clone()));
        }

        Self {
            cells,
            proto_cell,
            rows,
            cols,
        }
    }

    /// Returns a reference to a cell by row and column
    pub fn get_cell(&self, row: usize, col: usize) -> Option<&Region<T>> {
        if row >= self.rows || col >= self.cols {
            return None;
        }
        Some(&self.cells[row * self.cols + col])
    }

    /// Returns a cell by flat index (row-major order)
    pub fn get_cell_index(&self, index: usize) -> Option<&Region<T>> {
        self.cells.get(index)
    }

    /// Returns an iterator over all cells (immutable)
    pub fn iter(&self) -> impl Iterator<Item = &Region<T>> {
        self.cells.iter()
    }

    /// Returns an iterator over all cells (mutable)
    /// NB: check if this is actually needed
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Region<T>> {
        self.cells.iter_mut()
    }

    /// Resizes the number of rows and columns, keeping cell size and top-left the same.
    /// * If new_rows or new_cols are less than the current ones, the columns and/or rows will be truncated.
    /// * If new_rows or new_cols are greater than the current ones, the new cells will be added correspondingly after the existing ones.
    /// * If `color` is provided, it will be applied to the newly added cells, 
    /// otherwise the original color of the `proto_cell` will be used, @see `new()`.
 
    pub fn resize_grid(&mut self, new_rows: usize, new_cols: usize, color: Option<u32>) {
        //TODO: Implement!
        //TODO: Make sure the cells are correctly shifted per row and column
        //TODO: Probably, re-creating the `self.cells` vector is the easiest way to do this
    }

    /// Resizes all cells with the new width and height, keeping grid layout, preserving Region:id of the cells
    pub fn resize_cells(&mut self, new_cell_size: Dimensions2d<T>) {
        //TODO: Implement!
        //TODO: Make sure to use `Region::update()` to preserve the IDs of the regions!
    }

    /// Returns the full width of the grid in numeric units
    pub fn total_width(&self) -> T {
        self.proto_cell.dimensions.w * self.cols
    }

    /// Returns the full height of the grid in numeric units
    pub fn total_height(&self) -> T {
        self.proto_cell.dimensions.h * self.rows
    }
}
