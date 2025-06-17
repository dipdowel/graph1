use crate::core::context::GraphContext;
use crate::draw::tools::brush::Brush;
use crate::draw::tools::spray::circular::circular_spray;
use crate::draw::tools::spray::rectangular::rectangular_spray;

pub fn simple<UserData>(
    ctx: &mut GraphContext<UserData>,
    x: u32,
    y: u32,
    density: u32,
    colors: &Vec<u32>,
) {
    // TODO: Consider adding support for Alpha!
    // TODO: Consider adding support for Alpha!
    // TODO: Consider adding support for Alpha!
    // TODO: Consider adding support for Alpha!

    match ctx.brush {
        Brush::Circle { radius } => {
            circular_spray(ctx, x, y, density, colors, radius);
        }
        Brush::Rectangle { size } => {
            rectangular_spray(ctx, x, y, density, colors, &size);
        }
    }
}
