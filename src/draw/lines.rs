use crate::core::context::GraphContext;
use crate::draw::line;
use crate::primitives::point::Point;

/// Draws straight line segments between consecutive points
///
/// This function supports negative coordinates and off-screen vertices,
/// and uses the context's line clipping strategy to crop segments to the screen.
///
/// # Parameters
/// * `ctx` - A mutable reference to the `GraphContext`
/// * `vertices` - A vector of signed 2D points (can contain negative/off-screen coordinates)
/// * `color` - An optional RGBA color; defaults to `ctx.win.foreground_color` if `None`
pub fn between_points<UserData>(
    ctx: &mut GraphContext<UserData>,
    vertices: &[Point<i32>],
    color: Option<u32>,
) {
    if let Some(first_vertex) = vertices.first() {
        let mut previous_vertex = first_vertex;

        for vertex in vertices.iter().skip(1) {
            line::between_two_points(ctx, previous_vertex, vertex, color);
            previous_vertex = vertex;
        }
    }
}

//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::core::context::window::window::WindowContext;
//     use crate::core::context::GraphContext;
//     use crate::primitives::point::Point;
//
//     #[test]
//     fn test_between_points_draws_line_segments() {
//         let mut ctx = GraphContext::new(
//             WindowContext::new(20, 20, Some(0x00000000), Some(0xFFFFFFFF)),
//             false,
//             1,
//             None::<()>,
//             1,
//             None,
//         );
//
//         // Draw a line from (2,2) -> (10,10) -> (15,10)
//         let points = vec![Point::new(2, 2), Point::new(10, 10), Point::new(15, 10)];
//         let color = Some(0xFF112233);
//         between_points(&mut ctx, &points, color);
//
//         // Sample pixels to verify drawing occurred
//         let px1 = ctx.get_pixel(2, 2).unwrap();
//         let px2 = ctx.get_pixel(10, 10).unwrap();
//         let px3 = ctx.get_pixel(15, 10).unwrap();
//
//         assert_eq!(px1, 0xFF112233);
//         assert_eq!(px2, 0xFF112233);
//         assert_eq!(px3, 0xFF112233);
//     }
//
//     #[test]
//     fn test_between_points_empty_input_does_nothing() {
//         let mut ctx = GraphContext::new(
//             WindowContext::new(10, 10, Some(0x00ABCDEF), Some(0xFFFFFFFF)),
//             false,
//             1,
//             None::<()>,
//             1,
//             None,
//         );
//
//         between_points::<()>(&mut ctx, &[], None);
//
//         // Ensure buffer remains with background color
//         for y in 0..10 {
//             for x in 0..10 {
//                 assert_eq!(ctx.get_pixel(x, y).unwrap(), 0x00ABCDEF);
//             }
//         }
//     }
//
//     #[test]
//     fn test_between_points_single_point_does_nothing() {
//         let mut ctx = GraphContext::new(
//             WindowContext::new(10, 10, Some(0x00ABCDEF), Some(0xFFFFFFFF)),
//             false,
//             1,
//             None::<()>,
//             1,
//             None,
//         );
//
//         let pts = vec![Point::new(3, 3)];
//         between_points(&mut ctx, &pts, None);
//
//         // Only one point — nothing should be drawn
//         for y in 0..10 {
//             for x in 0..10 {
//                 assert_eq!(ctx.get_pixel(x, y).unwrap(), 0x00ABCDEF);
//             }
//         }
//     }
// }
