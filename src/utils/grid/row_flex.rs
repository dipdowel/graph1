use crate::primitives::numeric::Numeric;
use crate::primitives::plane::RectArea;
use crate::utils::math::geometry::region::Region;
use crate::primitives::align::Align;
use crate::primitives::data_structs::ring_buffers::{DynamicRingBuffer, RingBuffer};
use crate::primitives::point::Point;
use crate::utils::grid::render::GridLike;

/// A single row in a `RowFlexGrid`.
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

    /// Creates a new `FlexRow` to be used in `RowFlexGrid`.
    /// # Parameters
    /// - `height`: Height of the row.
    /// - `widths`: Widths of the cells in this row.
    /// - `colors`: Colors of the cells (RGBA packed as u32).
    /// - `align`: Row alignment if the row’s total width is smaller than the grid
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
pub struct RowFlexGrid<T: Numeric> {
    /// Rows of the grid.
    pub rows: Vec<FlexRow<T>>,
    /// Optional fixed grid width. If `None`, width is determined by the widest row.
    pub grid_width: Option<T>,
    /// Origin point of the grid (top-left corner).
    pub origin: Point<T>,
}

impl<T: Numeric> RowFlexGrid<T> {
    /// Creates a new `RowFlexGrid` with optional rows and an origin.
    pub fn new(origin: Point<T>, rows: Option<Vec<FlexRow<T>>>, grid_width: Option<T>) -> Self {
        Self {
            rows: rows.unwrap_or_default(),
            grid_width,
            origin,
        }
    }

    /// Adds a new row to the grid.
    pub fn add_row(&mut self, row: FlexRow<T>) {
        self.rows.push(row);
    }

    /// Computes the total height of the grid.
    pub fn total_height(&self) -> T {
        self.rows.iter().map(|r| r.height).fold(T::zero(), |acc, h| acc + h)
    }

    /// Computes the effective grid width (either user-defined or max row width).
    pub fn effective_width(&self) -> T {
        if let Some(w) = self.grid_width {
            w
        } else {
            self.rows
                .iter()
                .map(|r| r.total_width())
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or(T::zero())
        }
    }

    /// Returns the rectangular regions of all cells in the grid.
    fn to_regions(&self) -> Vec<Region<T>> {
        let mut regions = Vec::new();
        let mut current_y = self.origin.y;

        for row in &self.rows {
            let row_width = row.total_width();
            let grid_width = self.effective_width();

            // Compute starting x based on alignment
            let mut current_x = match row.align {
                Align::Left => self.origin.x,
                Align::Center => self.origin.x + (grid_width - row_width) / T::from_f64(2.0),
                Align::Right => self.origin.x + (grid_width - row_width),
            };

            let color_count = row.colors.len();

            for (i, &cell_w) in row.widths.iter().enumerate() {
                let color_index = i % color_count.max(1);
                let color = row.colors.get(color_index).copied();

                println!(">> color: {:?}", color);
                let rect = RectArea::new(current_x, current_y, cell_w, row.height, color);
                regions.push(Region::new(rect));
                current_x = current_x + cell_w;
            }

            current_y = current_y + row.height;
        }

        regions
    }

    // TODO:
    // TODO: 1. Think through and implement:
    // TODO:    - Resizing of the whole grid
    // TODO:    - Resizing of 2 neighbouring cells (width-wise)
    // TODO:    - Resizing of 2 neighbouring rows (height-wise)
    // TODO:
    // TODO: 2. Do we want a bounding boxHow to render the bounding box?
    // TODO: 2. How to render the bounding box?
    // TODO: 2. Does vertical alignment make sense here?
    // TODO:
    // TODO:

}

impl<T: Numeric> GridLike<T> for RowFlexGrid<T> {
    fn regions(&self) -> Vec<Region<T>> {
        self.to_regions()
    }
}