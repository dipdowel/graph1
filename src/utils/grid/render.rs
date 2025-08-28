use crate::core::context::GraphContext;
use crate::draw;
use crate::primitives::numeric::Numeric;
use crate::primitives::plane::RectArea;

/// Trait that abstracts over grid types that can provide rectangular cell areas for rendering.
pub trait GridLike<T: Numeric> {
    /// Returns the rectangular areas of all cells in the grid.
    fn cells_as_rects(&self) -> Vec<RectArea<T>>;
}

/// Generic grid renderer that works with any `GridLike` grid type.
pub fn render<UserData, T: Numeric, G: GridLike<T>>(
    ctx: &mut GraphContext<UserData>,
    grid: &G,
    use_outline: bool,
) {
    grid.cells_as_rects().into_iter().for_each(|rect_area| {
        let rect: RectArea<u32> = rect_area.convert();
        if use_outline {
            draw::rectangle::outline(ctx, &rect);
        } else {
            draw::rectangle::filled(ctx, &rect);
        }
    });
}
