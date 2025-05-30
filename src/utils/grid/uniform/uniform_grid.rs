use crate::primitives::numeric::Numeric;
use crate::primitives::plane::{Dimensions2d, RectArea};
use crate::utils::math::geometry::region::Region;

use crate::primitives::neighborhood::NeighborhoodType;

/// A 2D uniform grid of rectangular `Region` cells
/// The cell indices are positive integers, starting from (0, 0) in the top-left corner.
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
        colors: Option<Vec<u32>>,
    ) -> Self {
        let mut cells = Vec::with_capacity(rows * cols);
        let w = proto_cell.dimensions.w;
        let h = proto_cell.dimensions.h;
        let base_x = proto_cell.top_left.x;
        let base_y = proto_cell.top_left.y;

        let colors: Vec<Option<u32>> = colors
            .map(|v| v.into_iter().map(Some).collect())
            .unwrap_or(vec![proto_cell.color]);

        // Create an infinite circular iterator over `result`
        let mut circular_iter = colors.iter().cycle();

        for row in 0..rows {
            for col in 0..cols {
                let x = base_x + T::from_u32(col as u32) * w; // FIXME! will break on huge grids
                let y = base_y + T::from_u32(row as u32) * h; // FIXME! will break on huge grids
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

    pub fn resize_grid(&mut self, rows: usize, cols: usize, color: Option<u32>) {
        let mut new_cells = Vec::with_capacity(rows * cols);
        let w = self.proto_cell.dimensions.w;
        let h = self.proto_cell.dimensions.h;
        let base_x = self.proto_cell.top_left.x;
        let base_y = self.proto_cell.top_left.y;

        for row in 0..rows {
            for col in 0..cols {
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
        self.rows = rows;
        self.cols = cols;
    }

    /// Resizes all cells with the new width and height, keeping grid layout, preserving Region:id of the cells
    pub fn resize_cells(&mut self, size: Dimensions2d<T>) {
        if size.w == self.proto_cell.dimensions.w &&  size.h == self.proto_cell.dimensions.h {
            // No need to resize if the size is the same
            return;
        }
        
        
        let base_x = self.proto_cell.top_left.x;
        let base_y = self.proto_cell.top_left.y;

        for row in 0..self.rows {
            for col in 0..self.cols {
                let x = base_x + T::from_u32(col as u32) * size.w; // FIXME! will break on huge grids
                let y = base_y + T::from_u32(row as u32) * size.h; // FIXME! will break on huge grids
                let idx = row * self.cols + col;
                let new_rect = RectArea::new(x, y, size.w, size.h, self.proto_cell.color);
                self.cells[idx].update(new_rect);
            }
        }

        self.proto_cell.dimensions = size;
    }

    /// A convenience method that does two things:
    /// 1. Resizes the grid to the specified number of rows and columns.
    /// 2. Resizes cells to the given size.
    ////
    /// # Parameters
    /// - `cell_size`: The new size of each cell in the grid
    /// - `rows`: The new number of rows in the grid
    /// - `cols`: The new number of columns in the grid
    /// - `color`: Optional color to apply to the cells. If not provided, the original color of the `proto_cell` will be used.
    ///
    /// **NB:** This method calls `resize_cells()` and `resize_grid()` internally.
    pub fn resize(&mut self, cell_size: Dimensions2d<T>, rows: usize, cols: usize, color: Option<u32>) {
        self.resize_cells(cell_size);
        self.resize_grid(rows, cols, color);
    }




    /// Resizes the grid to the specified number of rows and columns.
    /// Cells get resized automatically to keep the original width and height of the grid intact.
    /// # Parameters
    /// - `rows`: The new number of rows in the grid
    /// - `cols`: The new number of columns in the grid
    /// - `color`: Optional color to apply to the cells. If not provided, the original color of the `proto_cell` will be used.
    /// **NB:** This method calls `resize_cells()` and `resize_grid()` internally.
    pub fn resize_grid_auto(&mut self, rows: usize, cols: usize, color: Option<u32>) {
        let new_cell_width = self.total_width() / T::from_u64(cols as u64);
        let new_cell_height = self.total_height() / T::from_u64(rows as u64);
        self.resize_cells(Dimensions2d::new(new_cell_width, new_cell_height));
        self.resize_grid(rows, cols, color);
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

//***************************************************************************************************
//***************************************************************************************************
//***************************************************************************************************
//***************************************************************************************************

// WiP WiP WiP WiP WiP WiP WiP WiP WiP WiP WiP WiP WiP WiP WiP WiP WiP WiP WiP WiP WiP WiP WiP WiP WiP

/// A single neighbor: index and reference to the Region cell
#[derive(Debug)]
pub struct Neighbor<'a, T: Numeric> {
    pub row: usize,
    pub col: usize,
    pub cell: &'a Region<T>,
}

// TODO: This implementation is so much a WiP!
// TODO: Refactor:
// TODO: - Better naming
// TODO: - Better structure
// TODO: - Think of DRYing it a bit, maybe?
impl<T: Numeric> UniformGrid<T> {
    /// Returns immutable references to neighboring cells, along with their indices
    ///
    /// # Parameters
    /// - `row`: The row index of the cell for which to find neighbors
    /// - `col`: The column index of the cell for which to find neighbors
    /// - `kind`: The type of neighborhood to consider (e.g., orthogonal, diagonal, etc.)
    pub fn get_neighbors(
        &self,
        row: usize,
        col: usize,
        neighborhood_type: &NeighborhoodType,
    ) -> Vec<Neighbor<T>> {
        let mut result: Vec<Neighbor<T>> = Vec::new();

        // Determine the directions to check based on the neighborhood type
        #[rustfmt::skip]
        let directions: Vec<(isize, isize)> = match neighborhood_type {
            // Orthogonal neighbors: up, right, down, left
            NeighborhoodType::Orthogonal => vec![(0, -1), (1, 0), (0, 1), (-1, 0)],
            // Diagonal neighbors: top-left, top-right, bottom-right, bottom-left
            NeighborhoodType::Diagonal => vec![(-1, -1), (1, -1), (1, 1), (-1, 1)],
            // Immediate neighbors: all 8 surrounding cells
            NeighborhoodType::Immediate => vec![
                (-1, -1), (0, -1), (1, -1),
                (-1, 0),           (1, 0),
                (-1, 1),  (0, 1),  (1, 1),
            ],
            // Square neighborhood: all cells within a square of given distance
            NeighborhoodType::Square { distance } => {
                let distance = *distance as isize;
                let mut dirs = vec![];
                for dy in -distance..=distance {
                    for dx in -distance ..=distance {
                        if dx != 0 || dy != 0 {
                            dirs.push((dx, dy));
                        }
                    }
                }
                dirs
            },
            // Circular neighborhood: all cells within a given radius
            NeighborhoodType::Circle { radius } => {
                let mut dirs = vec![];
                let radius = *radius as isize;
                let r_sq = radius.pow(2);
                for dy in -radius ..=radius {
                    for dx in -radius..=radius {
                        if dx != 0 || dy != 0 {
                            // Include only cells within the circle
                            if dx * dx + dy * dy <= r_sq {
                                dirs.push((dx, dy));
                            }
                        }
                    }
                }
                dirs
            },
            // Diamond neighborhood: all cells within a diamond of given distance
            NeighborhoodType::Diamond { distance } => {
                let mut dirs = vec![];
                let distance = *distance as isize;
                for dy in distance..=distance {
                    for dx in -distance..=distance {
                        if dx != 0 || dy != 0 {
                            // Include only cells within the diamond
                            if dx.abs() + dy.abs() <= distance {
                                dirs.push((dx, dy));
                            }
                        }
                    }
                }
                dirs
            },
        };

        // Iterate over the previously calculated directions
        for (dx, dy) in directions {
            // Calculate the neighbor's row and column
            let (n_row, n_col) = ((row as isize + dy) as usize, (col as isize + dx) as usize);

            // Ensure the neighbors are not negative (those simply don't exist in an on-screen grid)
            if n_row >= 0 && n_col >= 0 {
                let (n_row, n_col) = (n_row as usize, n_col as usize);
                // Retrieve the cell at the neighbor's position, if it exists
                if let Some(region) = self.get_cell(n_row, n_col) {
                    result.push(Neighbor {
                        row: n_row,
                        col: n_col,
                        cell: region,
                    });
                }
            }
        }

        result
    }
}
