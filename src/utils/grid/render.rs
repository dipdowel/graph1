use crate::core::context::GraphContext;
use crate::draw;
use crate::primitives::numeric::Numeric;
use crate::primitives::plane::RectArea;

/// Trait that abstracts over grid types that can provide rectangular cell areas for rendering.
pub trait GridLike<T: Numeric> {
    /// An associated iterator type over cell RectArea-s
    type RectIter<'a>: Iterator<Item = RectArea<T>> where T: 'a, Self: 'a;

    /// Returns an iterator over cell RectArea-s of all cells in the grid.
    fn cells_as_rects<'a>(&'a self) -> Self::RectIter<'a>;
}

/// Generic grid renderer that works with any `GridLike` grid type.
pub fn render<UserData, T: Numeric, G: GridLike<T>>(
    ctx: &mut GraphContext<UserData>,
    grid: &G,
    use_outline: bool,
) {
    grid.cells_as_rects().for_each(|rect_area| {
        let rect: RectArea<u32> = rect_area.convert();
        if use_outline {
            draw::rectangle::outline(ctx, &rect);
        } else {
            draw::rectangle::filled(ctx, &rect);
        }
    });
}
