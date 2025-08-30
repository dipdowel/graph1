use crate::primitives::align::Align;
use crate::primitives::numeric::Numeric;
use crate::primitives::plane::RectArea;
use crate::primitives::point::Point;
use crate::utils::grid::render::GridLike;
use crate::utils::math::geometry::region::Region;

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
        self.widths
            .iter()
            .copied()
            .fold(T::zero(), |acc, w| acc + w)
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
    /// Cannot be less than the widest row.
    bounding_width: T,

    /// Cached width of the widest row.
    widest_row_width: T,

    total_height: T,

    /// Origin point of the grid (top-left corner).
    pub origin: Point<T>,
}

impl<T: Numeric> FlexRowGrid<T> {
    /// Creates a new `FlexRowGrid` with optional row descriptors and an origin.
    /// If `bounding_width` is provided, it sets a width for the "container" for the grid.
    /// If not provided, the grid width is determined by the widest row.
    /// If `bounding_width` is smaller than the widest row, `bounding_width` is forced set to the width of the widest row.
    /// If a row's total width is less than `bounding_width`, it will be aligned according to its `align` property.
    /// # Arguments
    /// * `origin` - The top-left corner point of the grid.
    /// * `rows` - Optional vector of `FlexRow` descriptors to initialize the grid
    /// * `bounding_width` - Optional fixed width for the grid.
    /// * If the total height of all rows equals zero, the grid is still created.

    pub fn new(origin: Point<T>, rows: Option<Vec<FlexRow<T>>>, bounding_width: Option<T>) -> Self {

        let mut grid = Self {
            rows: Vec::new(),
            bounding_width: T::zero(),
            widest_row_width: T::zero(),
            total_height: T::zero(),
            origin,
        };

        // If no bounding width and no rows provided, simply return the empty grid.
        if bounding_width.is_none() && rows.is_none() {
            return grid;
        }

        grid.bounding_width = bounding_width.unwrap_or(T::zero());

        if let Some(flex_rows) = rows {
            for flex_row in flex_rows {
                grid.add_row(flex_row);
            }
        }

        // Ensure bounding width is at least as wide as the widest row
        if grid.bounding_width < grid.widest_row_width {
            grid.bounding_width = grid.widest_row_width;
        }

        grid
    }

    /// Goes through all the rows in the grid and aligns them according to their `align` property.
    fn align_all_rows(&mut self)  {

    }

    /// Adds a new row from a `FlexRow` descriptor to the grid.
    pub fn add_row(&mut self, row: FlexRow<T>) {
        let row_width = row.total_width();
        let row_height = row.height;

        self.widest_row_width = self.widest_row_width.maxi(row_width);
        self.widest_row_width = self.widest_row_width.maxi(row_width);

        let effective_width = self.bounding_width;

        let mut current_x = match row.align {
            Align::Left => self.origin.x,
            Align::Center => self.origin.x + (effective_width.saturating_sub(row_width)) / T::from_f64(2.0),
            Align::Right => self.origin.x + (effective_width.saturating_sub(row_width)),
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


        self.total_height = self.total_height + row_height;
        self.rows.push(region_row);
    }

    /// Returns a reference to a cell by row and column.
    pub fn get_cell(&self, row: usize, col: usize) -> Option<&Region<T>> {
        self.rows.get(row).and_then(|r| r.get(col))
    }

    /// Computes the total height of the grid.
    pub fn total_height(&self) -> T {
        self.total_height
    }

    pub fn widest_row_width(&self) -> T {
        self.widest_row_width
    }

    pub fn bounding_width(&self) -> T {
        self.bounding_width
    }


}

impl<T: Numeric> GridLike<T> for FlexRowGrid<T> {
    type RectIter<'a>
        = std::iter::Map<
        std::iter::Flatten<std::slice::Iter<'a, Vec<Region<T>>>>,
        fn(&Region<T>) -> RectArea<T>,
    >
    where
        T: 'a,
        Self: 'a;

    fn cells_as_rects<'a>(&'a self) -> Self::RectIter<'a> {
        fn to_rect<T: Numeric>(region: &Region<T>) -> RectArea<T> {
            region.rect_area()
        }
        self.rows.iter().flatten().map(to_rect::<T>)
    }
}
