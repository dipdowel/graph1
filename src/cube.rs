use crate::draw::line;
use crate::init::init_window::{WIN_HEIGHT, WIN_WIDTH};
use crate::tools::primitives::{Pixel, Point, Point3DF32};
use std::f32::consts::PI;
use std::mem::size_of;

#[rustfmt::skip]
static edges: [[usize; 2]; 12] = [
[0, 1], [1, 2], [2, 3], [3, 0], // back face
[4, 5], [5, 6], [6, 7], [7, 4], // front face
[0, 4], [1, 5], [2, 6], [3, 7] // connecting sides
];

pub struct Cube {
    #[rustfmt::skip]
    /// X-rotation speed (RPS)
    SPEED_X: f32,
    /// Y-rotation speed (RPS)
    SPEED_Y: f32,
    /// Z-rotation speed (RPS)
    SPEED_Z: f32,
    /// X-center (the location of the cube on the X axis)
    cx: f32,
    /// Y-center (the location of the cube on the Y axis)
    cy: f32,
    /// Z-center (the location of the cube on the Z axis)
    cz: f32,
    /// Length of the cube's edge
    size: f32,
    /// vertices
    vertices: Vec<Point3DF32>,

    // Animation values
    time_delta: f32,
    time_last: f32,
    time_now: f32,

    /// Start point of an edge + color
    cube_pixel: Pixel,
    /// End point of an edge
    cube_point: Point,
}

impl Cube {
    pub fn new(speed: Point3DF32, center: Point3DF32, size: f32, color: u32) -> Cube {
        let mut cube = Cube {
            SPEED_X: speed.x,
            SPEED_Y: speed.y,
            SPEED_Z: speed.z,

            cx: center.x,
            cy: center.y,
            cz: center.z,

            // Default initial values
            cube_pixel: Pixel { x: 0, y: 0, color },

            // Default initial values
            cube_point: Point { x: 0, y: 0 },

            size,
            time_delta: 0_f32,
            time_now: 0_f32,
            time_last: 0_f32,
            vertices: Vec::new(),
        };

        let Cube { cx, cy, cz, .. } = cube;

        #[rustfmt::skip]
            let  vertices: Vec<Point3DF32> = vec![
            Point3DF32 { x: cx - size, y: cy - size, z: cz - size },
            Point3DF32 { x: cx + size, y: cy - size, z: cz - size },
            Point3DF32 { x: cx + size, y: cy + size, z: cz - size },
            Point3DF32 { x: cx - size, y: cy + size, z: cz - size },
            Point3DF32 { x: cx - size, y: cy - size, z: cz + size },
            Point3DF32 { x: cx + size, y: cy - size, z: cz + size },
            Point3DF32 { x: cx + size, y: cy + size, z: cz + size },
            Point3DF32 { x: cx - size, y: cy + size, z: cz + size },

        ];

        cube.vertices = vertices;

        return cube;
    }

    pub fn render_frame(&mut self, buf_view: &mut [u32], frame_count: f32) {
        self.time_now = frame_count;

        // calculate the time difference
        self.time_delta = self.time_now - self.time_last;
        self.time_last = self.time_now;

        // rotate the cube along the Z axis
        let angle = self.time_delta * 0.001 * self.SPEED_Z * PI * 2_f32;
        let cx = self.cx; /* + oscillator; */
        let cy = self.cy; /* + oscillator; */
        let cz = self.cz; /*+ oscillator as f32; */

        for mut v in &mut self.vertices {
            let dx: f32 = v.x - cx;
            let dy = v.y - cy;
            let x = dx * f32::cos(angle) - dy * f32::sin(angle);
            let y = dx * f32::sin(angle) + dy * f32::cos(angle);
            v.x = x + cx;
            v.y = y + cy;
        }

        // rotate the cube along the X axis
        let angle = self.time_delta * 0.001 * self.SPEED_X * PI * 2_f32;
        for mut v in &mut self.vertices {
            let dy = v.y - cy;
            let dz = v.z - cz;
            let y = dy * f32::cos(angle) - dz * f32::sin(angle);
            let z = dy * f32::sin(angle) + dz * f32::cos(angle);
            v.y = y + cy;
            v.z = z + cz;
        }

        // rotate the cube along the Y axis
        let angle = self.time_delta * 0.001 * self.SPEED_Y * PI * 2_f32;
        for mut v in &mut self.vertices {
            let dx = v.x - cx;
            let dz = v.z - cz;
            let x = dz * f32::sin(angle) + dx * f32::cos(angle);
            let z = dz * f32::cos(angle) - dx * f32::sin(angle);
            v.x = x + cx;
            v.z = z + cz;
        }

        // draw each edge
        for edge in edges {
            self.cube_pixel.x = self.vertices[edge[0]].x as u32;
            self.cube_pixel.y = self.vertices[edge[0]].y as u32;
            self.cube_point.x = self.vertices[edge[1]].x as u32;
            self.cube_point.y = self.vertices[edge[1]].y as u32;
            // println!("cube_pixel: {:?}", cube_pixel);
            // println!("cube_point: {:?}", cube_point);

            line::between_two_points(buf_view, &self.cube_pixel, &self.cube_point);
        }
    }
}
