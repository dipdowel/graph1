use crate::core::context::GraphContext;
use crate::draw;
use crate::primitives::numeric::Numeric;
use crate::primitives::plane::RectArea;
use crate::utils::grid::uniform::uniform_grid::UniformGrid;
use crate::utils::math::geometry::region::Region;


/// Renders a `UniformGrid` instance on the screen.
/// NB: This method is very basic and is mostly meant for visual debugging purposes.
/// NB: It is recommended to implement your own rendering if you need something more complex.
///
/// # Parameters
/// - `ctx`: A mutable reference to the drawing context.
/// - `grid`: A `UniformGrid` instance representing the grid to be rendered.
/// - `use_outline`: A boolean flag indicating whether to draw the grid cells as outlines or filled rectangles. Color for each cell is taken from its internal `RectArea`.

pub fn render<UserData, T: Numeric>(
    ctx: &mut GraphContext<UserData>,
    grid: &UniformGrid<T>,
    use_outline: bool,
) {
    if use_outline {
        grid.iter().for_each(|cell_t| {
            let cell: Region<u32> = cell_t.convert();
            let rect_area: RectArea<u32> = cell.rect_area();
            draw::rectangle::outline(ctx, &rect_area);
        });
    } else {
        grid.iter().for_each(|cell_t| {
            let cell: Region<u32> = cell_t.convert();
            let rect_area: RectArea<u32> = cell.rect_area();
            draw::rectangle::filled(ctx, &rect_area);
        });
    }
}
