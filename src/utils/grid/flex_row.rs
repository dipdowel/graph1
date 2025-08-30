use crate::primitives::align::Align;
use crate::primitives::numeric::Numeric;
use crate::primitives::plane::{Dimensions2d, RectArea};
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

/// Cached internal state of the grid.
/// Must be recalculated on grid updates such as grid resizing, adding or removing rows, etc.
#[derive(Debug, Clone)]
struct CachedGridProps<T: Numeric> {
    /// Caches widths and heights of each row
    row_dims: Vec<Dimensions2d<T>>,
    /// Optional user-provided horizontal grid "container" width.
    /// Allows aligning rows horizontally within the "container"
    /// Cannot be less than the widest row.
    /// If no explicit value was provided, the value is determined by the widest row.
    bounding_width: Option<T>,
    /// Alignment of each row
    row_aligns: Vec<Align>,
    widest_row_width: T,
    narrowest_row_width: T,
    total_grid_height: T,
}

impl<T: Numeric> CachedGridProps<T> {
    fn new() -> Self {
        Self {
            row_dims: Vec::new(),
            row_aligns: Vec::new(),
            bounding_width: None,
            widest_row_width: T::zero(),
            narrowest_row_width: T::zero(),
            total_grid_height: T::zero(),
        }
    }
}

/// A 2D grid where each row can have arbitrary cell widths and height.
#[derive(Debug, Clone)]
pub struct FlexRowGrid<T: Numeric> {
    /// Origin point of the grid (top-left corner).
    origin: Point<T>,

    /// Rows of the grid, each row is a vector of `Region` cells.
    rows: Vec<Vec<Region<T>>>,

    /// Grid props that need to be recalculated on resizing, adding/removing rows, etc.
    props: CachedGridProps<T>,
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
            props: CachedGridProps::new(),
            origin,
        };

        // If no bounding width and no rows provided, simply return the empty grid.
        if bounding_width.is_none() && rows.is_none() {
            return grid;
        }

        grid.props.bounding_width = bounding_width;

        if let Some(flex_rows) = rows {
            for flex_row in flex_rows {
                grid.add_row_internal(flex_row);
            }
        }

        grid.update_props();
        grid.align_all_rows();
        grid
    }

    /// Computes the effective width of the grid,
    /// which is the maximum of the bounding width (if set) and the widest row width.
    fn effective_width(&self) -> T {
        if let Some(bw) = self.props.bounding_width {
            bw.maxi(self.props.widest_row_width)
        } else {
            self.props.widest_row_width
        }
    }


    /// Recalculates cached properties of the grid.
    /// Must be called after any modification to the grid structure (adding/removing rows, resizing, etc.)
    /// to ensure the cached properties are up to date.
    fn update_props(&mut self) {
        let big_dummy = T::from_f64(f64::MAX - 1.0);

        let mut widest_row = T::zero();
        let mut narrowest_row = big_dummy;
        let mut total_height = T::zero();
        let mut row_dims: Vec<Dimensions2d<T>> = Vec::with_capacity(self.rows.len());

        for row in &self.rows {
            let row_height = if let Some(first_cell) = row.first() {
                first_cell.rect_area().dimensions.h
            } else {
                T::zero()
            };
            let row_width: T = row.iter().map(|cell| cell.rect_area().dimensions.w).sum();

            widest_row = widest_row.maxi(row_width);
            narrowest_row = narrowest_row.mini(row_width);
            total_height = total_height + row_height;

            row_dims.push(Dimensions2d::new(row_width, row_height));
        }

        self.props.widest_row_width = widest_row;
        // self.challenge_effective_width(widest_row);
        self.props.narrowest_row_width = if narrowest_row == big_dummy {
            T::zero()
        } else {
            narrowest_row
        };
        self.props.total_grid_height = total_height;
        self.props.row_dims = row_dims;
    }

    /// Goes through all the rows in the grid and aligns them according to their `align` property.
    /// N: Make sure to call `update_props()` before calling this method to ensure the cached properties are up to date!
    fn align_all_rows(&mut self) {
        for row_index in 0..self.rows.len() {
            self.align_row(row_index);
        }
    }

    /// Aligns a single row based on its `align` property and the effective grid width.
    /// N: Make sure to call `update_props()` before calling this method to ensure the cached properties are up to date!
    fn align_row(&mut self, row_index: usize) {
        if row_index >= self.rows.len() {
            return;
        }

        let row_dims = self.props.row_dims[row_index].clone();
        let effective_width = self.effective_width();

        let align = self.props.row_aligns[row_index];
        let row = &mut self.rows[row_index];
        let row_width = row_dims.w;

        if row_width >= effective_width {
            // No alignment needed/possible (?)
            return;
        }

        let margin_left_right = effective_width - row_width;

        let offset_x = match align {
            Align::Left => T::zero(),
            Align::Center => margin_left_right / T::from_f64(2.0),
            Align::Right => margin_left_right,
        };

        let mut current_x = self.origin.x + offset_x;

        // Shift all cells in the row by offset_x
        for cell in row.iter_mut() {
            let mut rect = cell.rect_area();
            // rect.top_left.x = rect.top_left.x + offset_x;
            rect.top_left.x = current_x;
            cell.update(rect);

            current_x = current_x + rect.dimensions.w;
        }
    }

    fn add_row_internal(&mut self, row: FlexRow<T>) {
        let mut region_row: Vec<Region<T>> = Vec::with_capacity(row.widths.len());
        let color_count = row.colors.len().max(1);

        // Save alignment of the row
        self.props.row_aligns.push(row.align);

        let current_y = self.origin.y + self.props.total_grid_height;
        let mut current_x = self.origin.x;

        for (i, &cell_w) in row.widths.iter().enumerate() {
            let color = row.colors.get(i % color_count).copied();
            let rect = RectArea::new(current_x, current_y, cell_w, row.height, color);
            region_row.push(Region::new(rect));
            current_x = current_x + cell_w;
        }
        self.rows.push(region_row);
        self.props.total_grid_height = self.props.total_grid_height + row.height;
        // self.ensure_bounding_width();
    }
    /// Adds a new row from a `FlexRow` descriptor to the grid.
    pub fn add_row(&mut self, row: FlexRow<T>) {
        let row_width = row.total_width();
        self.add_row_internal(row);
        self.update_props();
        self.update_props();
        self.update_props();
        self.update_props();
        self.align_all_rows();
        self.update_props();
        self.align_all_rows();
        self.update_props();
        self.align_all_rows();
        self.update_props();
    }

    /// Returns a reference to a cell by row and column.
    pub fn get_cell(&self, row: usize, col: usize) -> Option<&Region<T>> {
        self.rows.get(row).and_then(|r| r.get(col))
    }

    /// The total height of the grid.
    pub fn grid_height(&self) -> T {
        self.props.total_grid_height
    }

    pub fn widest_row_width(&self) -> T {
        self.props.widest_row_width
    }

    pub fn narrowest_row_width(&self) -> T {
        self.props.narrowest_row_width
    }

    pub fn bounding_width(&self) -> Option<T> {
        self.props.bounding_width
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
