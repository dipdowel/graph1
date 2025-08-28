use crate::primitives::numeric::Numeric;
use crate::primitives::plane::{RectArea};
use crate::utils::math::geometry::region::Region;
use crate::primitives::align::Align;
use crate::primitives::point::Point;
use crate::utils::grid::render::GridLike;

/// A single row descriptor for initializing a `FlexRowGrid`.
#[derive(Debug, Clone)]
pub struct FlexRow<T: Numeric> {
    /// Height of the row in pixels.
    pub height: T,
    /// Widths of the cells in this row.
    pub widths: Vec<T>,
    /// Colors of the cells (RGBA packed as u32).
    pub colors: Vec<u32>,
    /// Row alignment if the row’s total width is smaller than the grid width.
    pub align: Align,
}

impl<T: Numeric> FlexRow<T> {
    /// Computes the total width of the row (sum of cell widths).
    pub fn total_width(&self) -> T {
        self.widths.iter().copied().fold(T::zero(), |acc, w| acc + w)
    }

    /// Returns the number of cells in this row.
    pub fn cell_count(&self) -> usize {
        self.widths.len()
    }

    /// Creates a new `FlexRow` to be used in `FlexRowGrid`.
    pub fn new(height: T, widths: Vec<T>, colors: Vec<u32>, align: Align) -> Self {
        Self {
            height,
            widths,
            colors,
            align,
        }
    }
}

/// A 2D grid where each row can have arbitrary cell widths and height.
#[derive(Debug, Clone)]
pub struct FlexRowGrid<T: Numeric> {
    /// Rows of the grid, each row is a vector of `Region` cells.
    pub rows: Vec<Vec<Region<T>>>,
    /// Optional fixed grid width. If `None`, width is determined by the widest row.
    pub grid_width: Option<T>,
    /// Origin point of the grid (top-left corner).
    pub origin: Point<T>,
}

impl<T: Numeric> FlexRowGrid<T> {
    /// Creates a new `FlexRowGrid` with optional row descriptors and an origin.
    pub fn new(origin: Point<T>, rows: Option<Vec<FlexRow<T>>>, grid_width: Option<T>) -> Self {
        let mut grid = Self {
            rows: Vec::new(),
            grid_width,
            origin,
        };

        if let Some(row_descs) = rows {
            for desc in row_descs {
                grid.add_row(desc);
            }
        }

        grid
    }

    /// Adds a new row from a `FlexRow` descriptor to the grid.
    pub fn add_row(&mut self, row: FlexRow<T>) {
        let row_width = row.total_width();
        let effective_width = self.grid_width.unwrap_or(row_width);

        let mut current_x = match row.align {
            Align::Left => self.origin.x,
            Align::Center => self.origin.x + (effective_width - row_width) / T::from_f64(2.0),
            Align::Right => self.origin.x + (effective_width - row_width),
        };

        let current_y = self.total_height() + self.origin.y;

        let color_count = row.colors.len().max(1);
        let mut region_row: Vec<Region<T>> = Vec::new();

        for (i, &cell_w) in row.widths.iter().enumerate() {
            let color = row.colors.get(i % color_count).copied();
            let rect = RectArea::new(current_x, current_y, cell_w, row.height, color);
            region_row.push(Region::new(rect));
            current_x = current_x + cell_w;
        }

        self.rows.push(region_row);
    }

    /// Returns a reference to a cell by row and column.
    pub fn get_cell(&self, row: usize, col: usize) -> Option<&Region<T>> {
        self.rows.get(row).and_then(|r| r.get(col))
    }

    /// Returns a mutable reference to a cell by row and column.
    pub fn get_cell_mut(&mut self, row: usize, col: usize) -> Option<&mut Region<T>> {
        self.rows.get_mut(row).and_then(|r| r.get_mut(col))
    }

    /// Computes the total height of the grid.
    pub fn total_height(&self) -> T {
        self.rows
            .iter()
            .map(|r| r.first().map(|cell| cell.rect_area().dimensions.h).unwrap_or(T::zero()))
            .fold(T::zero(), |acc, h| acc + h)
    }

    /// Computes the effective grid width (either user-defined or max row width).
    pub fn effective_width(&self) -> T {
        if let Some(w) = self.grid_width {
            w
        } else {
            self.rows
                .iter()
                .map(|r| {
                    r.iter()
                        .map(|cell| cell.rect_area().dimensions.w)
                        .fold(T::zero(), |acc, w| acc + w)
                })
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or(T::zero())
        }
    }

    /// Returns the rectangular regions of all cells in the grid.
    fn to_regions(&self) -> Vec<Region<T>> {
        self.rows.iter().flatten().cloned().collect()
    }
}

impl<T: Numeric> GridLike<T> for FlexRowGrid<T> {
    fn regions(&self) -> Vec<Region<T>> {
        self.to_regions()
    }
}
