use crate::primitives::data_structs::variant::Variant;
use crate::primitives::plane::{Dimensions2d, RectArea};
use crate::primitives::point::Point;
use crate::text::font::PixelFont;
use crate::utils::grid;
use crate::utils::grid::uniform::UniformGrid;
use crate::utils::math::geometry::region::Region;

/// Provides functionality for managing a grid of monospaced characters.
/// Can be used for rendering a text in a console-like interface.
/// - The grid dimensions are defined by the max number of characters a row and the number of rows.
/// - The grid dimensions are provided either explicitly or are figured out from the provided set of strings,
///   i.e. width of the longest string is used for the width of the grid.
#[derive(Debug, Clone)]
pub struct MonospacedCharGrid {
    grid: UniformGrid<u32>,
}

/// Normalize the input dimensions, whether they are provided as a list of strings or as explicit dimensions.
/// In case of a list of strings, the width is determined by the length of the longest string,
/// and the height is determined by the number of strings.
/// # Arguments
/// * `dimensions_input` - Either a list of strings (rows of characters) or explicit dimensions
/// # Returns
/// The normalized dimensions, or `None` if the input is invalid (e.g., zero width or height)

fn normalize_input(
    dimensions_input: Variant<&[&str], Dimensions2d>,
) -> Option<Dimensions2d<usize>> {

    let result: Dimensions2d<usize> = match dimensions_input {
        Variant::Primary(strs) => {
            let rows = strs.len() as u32;

            let cols = strs
                .iter()
                .map(|s| s.chars().count() as u32)
                .max()
                .unwrap_or(0);

            Dimensions2d {
                h: rows as usize,
                w: cols as usize,
            }
        }
        Variant::Secondary(dims) => dims.convert(),
    };

    if result.w == 0 || result.h == 0 {
        None
    } else {
        Some(result)
    }
}

impl MonospacedCharGrid {
    /// Create a new `MonospacedCharGrid` instance.
    /// # Arguments
    /// * `font` - A monospaced (!) pixel font
    /// * `font_scale_factor` - Scaling factor for the font (must be >=1)
    /// * `dimensions_input` - Either a list of strings (rows of characters) or explicit dimensions
    /// * `top_left` - The starting point (top-left corner) of the grid on the screen
    /// * `color` - Optional color for the grid cells
    pub fn new(
        font: &PixelFont,
        dimensions_input: Variant<&[&str], Dimensions2d>,
        top_left: Point,
        color: Option<u32>,
    ) -> Option<Self> {

        if !is_font_monospaced(font) {
            return None;
        }

        // Figure out a single character cell dimensions
        let cell_dims = get_char_cell_dims(font)?;

        // Get the resulting grid dimensions
        let grid_dimensions = normalize_input(dimensions_input.clone())?;

        let proto_cell: RectArea = RectArea {
            top_left,
            dimensions: cell_dims,
            color,
        };

        let colors = if let Some(c) = color {
            Some(vec![c])
        } else {
            None
        };

        let grid = UniformGrid::new(proto_cell, grid_dimensions.h, grid_dimensions.w, colors);



        Some(MonospacedCharGrid {
            grid,
        })
    }

    /// Render the grid to the provided context
    /// # Arguments
    /// * `ctx` - The context to which the grid will be rendered
    /// * `use_outline` - Whether to render the grid cells as outlines (true) or filled (false)
    pub fn render<UserData>(&self, ctx: &mut crate::core::context::GraphContext<UserData>, use_outline: bool) {
        grid::render(ctx, &self.grid, use_outline);
    }

    /// Get a reference to a specific cell in the grid
    /// # Arguments
    /// * `row` - The row index of the cell
    /// * `col` - The column index of the cell
    pub fn get_cell(&self, row: usize, col: usize ) -> Option<&Region> {
        self.grid.get_cell(row, col)
    }

    /// Get a reference to a specific cell in the grid by its linear index
    pub fn get_cell_by_index(&self, idx: usize ) -> Option<&Region> {
        self.grid.get_cell_by_index(idx)
    }

    /// Get the total number of cells in the grid
    pub fn num_cells(&self) -> usize {
        self.grid.num_cells()
    }

    /// Get the number of columns in the grid
    pub fn num_columns(&self) -> usize {
        self.grid.cols
    }

    /// Get the number of rows in the grid
    pub fn num_rows(&self) -> usize {
        self.grid.rows
    }

    /// Get the dimensions of a single cell in the grid
    pub fn cell_dimensions(&self) -> Dimensions2d {
        self.grid.proto_cell.dimensions.clone()
    }

    /// Get the overall dimensions of the grid (width and height in pixels)
    pub fn grid_dimensions(&self) -> Dimensions2d {
        Dimensions2d {
            w: self.grid.total_width(),
            h: self.grid.total_height(),
        }
    }

    /// Get the top-left position of the grid
    pub fn top_left(&self) -> Point {
        self.grid.proto_cell.top_left.clone()
    }
}

/// Check if a font is monospaced (all glyphs have the same width)
/// # Arguments
/// * `font` - The pixel font to be checked
/// # Returns
/// `true` if the font is monospaced, `false` otherwise
fn is_font_monospaced(font: &PixelFont) -> bool {
    if font.char_order.len() == 0 {
        return false;
    }

    let first_char = font.char_order.chars().nth(0).unwrap();
    let first_glyph = font.get_glyph(&first_char);
    let first_width = first_glyph.dimensions.w;

    for c in font.char_order.chars() {
        let glyph = font.get_glyph(&c);
        if glyph.dimensions.w != first_width {
            return false;
        }
    }

    true
}

/// Calculate the area of a single character cell (glyph + spacing) times scaling
/// # Arguments
/// * `font` - The pixel font to be used
/// * `font_scale_factor` - The scaling factor for the font
/// # Returns
/// The dimensions of a single character cell, or `None` if something goes wrong (e.g., font has no characters, font_scale_factor is zero)

fn get_char_cell_dims(font: &PixelFont) -> Option<Dimensions2d> {

    let kerning = font.spacing.kerning_px as u32;
    let leading = font.spacing.leading_px as u32;

    let first_font_char = font.char_order.chars().nth(0)?;
    let mut glyph_dims = font.get_glyph(&first_font_char).dimensions.clone();


    glyph_dims.w = (glyph_dims.w + kerning);
    glyph_dims.h = (glyph_dims.h + leading);

    Some(glyph_dims)
}
