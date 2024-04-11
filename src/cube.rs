pub fn cube(buf_view: &mut [u32], frame_count: u32) {
    println!("jajajja");

}
// use crate::init::init_window::{WIN_HEIGHT, WIN_WIDTH};
// use crate::tools::primitives::Point3D;
//
// const SPEED_X: f64 = 0.05; // rps
// const SPEED_Y: f64 = 0.05; // rps
// const SPEED_Z: f64 = 0.02; // rps
//
// pub fn cube(buf_view: &mut [u32], frame_count: u32){
//
//
//
//     /*
//       var cx = w / 2;
//       var cy = h / 2;
//       var cz = 0;
//       var size = h / 4;
//
//       var vertices = [
//                         new POINT3D(cx - size, cy - size, cz - size),
//                         new POINT3D(cx + size, cy - size, cz - size),
//                         new POINT3D(cx + size, cy + size, cz - size),
//                         new POINT3D(cx - size, cy + size, cz - size),
//                         new POINT3D(cx - size, cy - size, cz + size),
//                         new POINT3D(cx + size, cy - size, cz + size),
//                         new POINT3D(cx + size, cy + size, cz + size),
//                         new POINT3D(cx - size, cy + size, cz + size)
//         ];
//
//
//           var edges = [
//             [0, 1], [1, 2], [2, 3], [3, 0], // back face
//             [4, 5], [5, 6], [6, 7], [7, 4], // front face
//             [0, 4], [1, 5], [2, 6], [3, 7] // connecting sides
//           ];
//
//      */
//
//     let cx = WIN_WIDTH / 2;
//     let cy = WIN_HEIGHT / 2;
//     let cz = 0;
//     let size = WIN_HEIGHT / 4;
//
//     let vertices: Vec<Point3D> = vec![
//         Point3D {
//             x: cx + size,
//             y: cy - size,
//             z: cz - size,
//         },
//         Point3D {
//             x: cx + size,
//             y: cy + size,
//             z: cz - size,
//         },
//         Point3D {
//             x: cx - size,
//             y: cy + size,
//             z: cz - size,
//         },
//         Point3D {
//             x: cx - size,
//             y:  cy - size,
//             z: cz + size,
//         },
//         Point3D {
//             x: cx + size,
//             y: cy - size,
//             z: cz + size
//         },
//         Point3D {
//             x: cx + size,
//             y: cy + size,
//             z: cz + size,
//         },
//         Point3D {
//             x: cx - size,
//             y: cy + size,
//             z: cz + size,
//         },
//
//
//     ];
//
//     let edges = [
//         [0, 1], [1, 2], [2, 3], [3, 0], // back face
//         [4, 5], [5, 6], [6, 7], [7, 4], // front face
//         [0, 4], [1, 5], [2, 6], [3, 7] // connecting sides
//     ];
//
//
//     // set up the animation loop
//     let timeDelta: u32;
//     let timeLast:u32  = 0;
//
// }