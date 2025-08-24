use crate::core::context::GraphContext;
use crate::draw;
use crate::primitives::numeric::Numeric;
use crate::primitives::plane::RectArea;
use crate::utils::math::geometry::region::Region;

/// Trait that abstracts over grid types that can provide rectangular regions of their cells.
pub trait GridLike<T: Numeric> {
    /// Returns the rectangular regions of all cells in the grid.
    fn regions(&self) -> Vec<Region<T>>;
}


/// Generic grid renderer that works with any `GridLike` grid type.
pub fn render<UserData, T: Numeric, G: GridLike<T>>(
    ctx: &mut GraphContext<UserData>,
    grid: &G,
    use_outline: bool,
) {
    if use_outline {
        grid.regions().into_iter().for_each(|cell_t| {
            let cell: Region<u32> = cell_t.convert();
            let rect_area: RectArea<u32> = cell.rect_area();
            draw::rectangle::outline(ctx, &rect_area);
        });
    } else {
        grid.regions().into_iter().for_each(|cell_t| {
            let cell: Region<u32> = cell_t.convert();
            let rect_area: RectArea<u32> = cell.rect_area();
            draw::rectangle::filled(ctx, &rect_area);
        });
    }
}
