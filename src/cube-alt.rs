use crate::draw::line;
use crate::init::init_window::{WIN_HEIGHT, WIN_WIDTH};
use crate::tools::primitives::{Pixel, Point, Point3DF32};
use std::f32::consts::PI;
use std::mem::size_of;

// TODO: NB: This file is here just for reference. Don't call stuff here!

#[rustfmt::skip]
static EDGES: [[usize; 2]; 12] = [
[0, 1], [1, 2], [2, 3], [3, 0], // back face
[4, 5], [5, 6], [6, 7], [7, 4], // front face
[0, 4], [1, 5], [2, 6], [3, 7] // connecting sides
];

pub struct CubeAlt {
    #[rustfmt::skip]
    /// X-rotation speed (RPS)
    speed_x: f32,
    /// Y-rotation speed (RPS)
    speed_y: f32,
    /// Z-rotation speed (RPS)
    speed_z: f32,
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
            speed_x: speed.x,
            speed_y: speed.y,
            speed_z: speed.z,

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

        // TODO: If we want to change the size during rendering (in runtime),
        // TODO: `vertices` will have to be re-calculated.
        // TODO: Should the code below go into a separate function
        // TODO: that can be called also from `render_frame()`?

        #[rustfmt::skip]
            let  vertices: Vec<Point3DF32> = vec![
            Point3DF32 { x: 0_f32, y: 0_f32, z: 0_f32 },
            Point3DF32 { x: 0_f32, y: 0_f32, z: 0_f32 },
            Point3DF32 { x: 0_f32, y: 0_f32, z: 0_f32},
            Point3DF32 { x: 0_f32, y: 0_f32, z: 0_f32 },
            Point3DF32 { x: 0_f32, y: 0_f32, z: 0_f32 },
            Point3DF32 { x: 0_f32, y: 0_f32, z: 0_f32 },
            Point3DF32 { x: 0_f32, y: 0_f32, z: 0_f32 },
            Point3DF32 { x: 0_f32, y: 0_f32, z: 0_f32 },
        ];

        // FIXME: the initialisation above is too verbose.
        // TODO: add default initialisation of (0,0,0) to `Point3DF32` !

        cube.vertices = vertices;
        cube.update_vertices();

        return cube;
    }
    fn update_vertices(&mut self) {
        self.vertices[0].x = self.cx - self.size;
        self.vertices[0].y = self.cy - self.size;
        self.vertices[0].z = self.cz - self.size;

        self.vertices[1].x = self.cx + self.size;
        self.vertices[1].y = self.cy - self.size;
        self.vertices[1].z = self.cz - self.size;

        self.vertices[2].x = self.cx + self.size;
        self.vertices[2].y = self.cy + self.size;
        self.vertices[2].z = self.cz - self.size;

        self.vertices[3].x = self.cx - self.size;
        self.vertices[3].y = self.cy + self.size;
        self.vertices[3].z = self.cz - self.size;

        self.vertices[4].x = self.cx - self.size;
        self.vertices[4].y = self.cy - self.size;
        self.vertices[4].z = self.cz + self.size;

        self.vertices[5].x = self.cx + self.size;
        self.vertices[5].y = self.cy - self.size;
        self.vertices[5].z = self.cz + self.size;

        self.vertices[6].x = self.cx + self.size;
        self.vertices[6].y = self.cy + self.size;
        self.vertices[6].z = self.cz + self.size;

        self.vertices[7].x = self.cx - self.size;
        self.vertices[7].y = self.cy + self.size;
        self.vertices[7].z = self.cz + self.size;
    }

    pub fn render_frame(
        &mut self,
        buf_view: &mut [u32],
        frame_count: f32,
        translation: &Option<Point3DF32>,
    ) {
        //
        // // apply translations only if provided.
        match translation {
            Some(delta) => {
                self.cx = self.cx + delta.x;
                self.cy = self.cy + delta.y;
                self.cz = self.cz + delta.z;
                self.update_vertices();
            }
            None => (),
        }

        self.time_now = frame_count;

        // calculate the time difference
        self.time_delta = self.time_now - self.time_last;
        self.time_last = self.time_now;

        // rotate the cube along the Z axis
        let angle = self.time_delta * 0.001 * self.speed_z * PI * 2_f32;
        for v in &mut self.vertices {
            let dx: f32 = v.x - self.cx;
            let dy = v.y - self.cy;
            let x = dx * f32::cos(angle) - dy * f32::sin(angle);
            let y = dx * f32::sin(angle) + dy * f32::cos(angle);
            v.x = x + self.cx;
            v.y = y + self.cy;
        }

        // rotate the cube along the X axis
        let angle = self.time_delta * 0.001 * self.speed_x * PI * 2_f32;
        for v in &mut self.vertices {
            let dy = v.y - self.cy;
            let dz = v.z - self.cz;
            let y = dy * f32::cos(angle) - dz * f32::sin(angle);
            let z = dy * f32::sin(angle) + dz * f32::cos(angle);
            v.y = y + self.cy;
            v.z = z + self.cz;
        }

        // rotate the cube along the Y axis
        let angle = self.time_delta * 0.001 * self.speed_y * PI * 2_f32;
        for v in &mut self.vertices {
            let dx = v.x - self.cx;
            let dz = v.z - self.cz;
            let x = dz * f32::sin(angle) + dx * f32::cos(angle);
            let z = dz * f32::cos(angle) - dx * f32::sin(angle);
            v.x = x + self.cx;
            v.z = z + self.cz;
        }

        // draw each edge
        for edge in EDGES {
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
