use crate::primitives::data_structs::variant::Variant;
use crate::primitives::plane::{Dimensions2d, RectArea};
use crate::primitives::point::Point;
use crate::text::font::PixelFont;
use crate::text::helpers::is_font_monospaced;
use crate::utils::grid::uniform::UniformGrid;


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


    glyph_dims.w = glyph_dims.w + kerning ;
    glyph_dims.h = glyph_dims.h + leading ;

    Some(glyph_dims)
}


/// Create a uniform grid of character cells based on a monospaced font.
/// Each cell in the grid corresponds to a character position, with dimensions based on the font's
/// glyph size plus any additional spacing (kerning and leading).
/// # Arguments
/// * `font` - The pixel font to be used (must be monospaced)
/// * `dimensions_input` - Either a list of strings (rows of characters) or explicit dimensions
/// * `top_left` - The top-left position of the grid
/// * `color` - Optional color to apply to all cells in the grid
pub fn make_monospaced_char_grid(
    font: &PixelFont,
    dimensions_input: Variant<&[&str], Dimensions2d>,
    top_left: Point,
    color: Option<u32>,
) -> Option<UniformGrid<u32>> {

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
        Some(grid)
}