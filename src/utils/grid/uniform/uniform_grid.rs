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
    ///
    /// # Parameters
    /// - `proto_cell`: use as a template for a grid cell.
    ///     - The top-left corner of `proto_cell` is used as the top-left corner for the whole grid
    ///     - The default color of `proto_cell` is assigned to each created cell (can be changed later)
    /// - `rows`: Number of rows in the grid
    /// - `cols`: Number of columns in the grid
    /// - `colors`: Optional vector of colors for the cells. It is treated as a ring buffer, 
    /// the first color from `colors` is applied to the first cell, the second color to the second cell, and so on.
    /// When the end of the vector is reached, the color pointer is reset to the beginning of the vector and the coloring
    /// continues from there.
    ///
    /// # Returns
    /// A new `UniformGrid` instance with the specified number of rows and columns.
    pub fn new(
        proto_cell: RectArea<T>,
        rows: usize,
        cols: usize,
        colors:Option<Vec<u32>>,
    ) -> Self  {
        let mut cells = Vec::with_capacity(rows * cols);
        let w = proto_cell.dimensions.w;
        let h = proto_cell.dimensions.h;
        let base_x = proto_cell.top_left.x;
        let base_y = proto_cell.top_left.y;


        let colors:Vec<Option<u32>> = colors
            .map(|v| v.into_iter().map(Some).collect())
            .unwrap_or(vec![proto_cell.color]);

        // Create an infinite circular iterator over `result`
        let mut circular_iter = colors.iter().cycle();

        for row in 0..rows {
            for col in 0..cols {
                let x = base_x + T::from_u32(col as u32) * w; // FIXME! will break on huge grids
                let y = base_y + T::from_u32(row as u32) * h; // FIXME! will break on huge grids
                // let rect = RectArea::new(x, y, w, h, circular_iter.next().unwrap_or(proto_cell.color));
                let color = *circular_iter.next().unwrap_or(&proto_cell.color);

                let rect = RectArea::new(x, y, w, h, color);
                cells.push(Region::new(rect));
            }
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
        let mut new_cells = Vec::with_capacity(new_rows * new_cols);
        let w = self.proto_cell.dimensions.w;
        let h = self.proto_cell.dimensions.h;
        let base_x = self.proto_cell.top_left.x;
        let base_y = self.proto_cell.top_left.y;

        for row in 0..new_rows {
            for col in 0..new_cols {
                let x = base_x + T::from_u32(col as u32) * w; // FIXME! will break on huge grids
                let y = base_y + T::from_u32(row as u32) * h; // FIXME! will break on huge grids
                let idx = row * self.cols + col;

                if row < self.rows && col < self.cols {
                    let region = self.cells[idx].clone();
                    new_cells.push(region);
                } else {
                    let rect = RectArea::new(x, y, w, h, color.or(self.proto_cell.color));
                    new_cells.push(Region::new(rect));
                }
            }
        }

        self.cells = new_cells;
        self.rows = new_rows;
        self.cols = new_cols;
    }

    /// Resizes all cells with the new width and height, keeping grid layout, preserving Region:id of the cells
    pub fn resize_cells(&mut self, new_cell_size: Dimensions2d<T>) {
        let base_x = self.proto_cell.top_left.x;
        let base_y = self.proto_cell.top_left.y;

        for row in 0..self.rows {
            for col in 0..self.cols {
                let x = base_x + T::from_u32(col as u32) * new_cell_size.w; // FIXME! will break on huge grids
                let y = base_y + T::from_u32(row as u32) * new_cell_size.h; // FIXME! will break on huge grids
                let idx = row * self.cols + col;
                let new_rect = RectArea::new(x, y, new_cell_size.w, new_cell_size.h, self.proto_cell.color);
                self.cells[idx].update(new_rect);
            }
        }

        self.proto_cell.dimensions = new_cell_size;
    }

    /// Returns the full width of the grid in numeric units
    pub fn total_width(&self) -> T {
        self.proto_cell.dimensions.w * T::from_u32(self.cols as u32)
    }

    /// Returns the full height of the grid in numeric units
    pub fn total_height(&self) -> T {
        self.proto_cell.dimensions.h * T::from_u32(self.rows as u32)
    }
    
    /// Returns the total number of cells in the grid
    pub fn num_cells(&self) -> usize {
        self.rows * self.cols
    }
}

