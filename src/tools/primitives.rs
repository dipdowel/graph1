#[derive(Debug)]
pub struct Dimensions2d {
    pub w: u32,
    pub h: u32,
}


// [ START ] ///////////////////////////////////////////////////////////////////////////////////////
// ========= Point 2D, + conversions from u32 to f32 coordinates and back ==========================

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

#[derive(Clone, Copy, Debug)]
pub struct PointF32 {
    pub x: f32,
    pub y: f32,
}


// Implementing conversion from Point to PointF32
impl From<Point> for PointF32 {
    fn from(p: Point) -> Self {
        PointF32 {
            x: p.x as f32,
            y: p.y as f32,
        }
    }
}


// Implementing conversion from PointF32 to Point
impl From<PointF32> for Point {
    fn from(p: PointF32) -> Self {
        Point {
            x: p.x as u32,
            y: p.y as u32,
        }
    }
}
// ========= Point 2D, + conversions from u32 to f32 coordinates and back ==========================
// [ END ] ///////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct RectArea {
    pub top_left:Point,
    pub dimensions:Dimensions2d
}


pub struct StarProperties {
    /// Location of the central point of the star
    pub center: Pixel,

    /// How many angles the star has
    pub num_vertices: u32,

    /// Distance from the `center` after which every N+1th  vertex of the star lies
    pub inner_radius:u32,

    /// Distance from the `center` after which every N+2th  vertex of the star lies
    pub outer_radius:u32,

    /// Rotation angle in degrees
    pub(crate) rotation_angle: f64,

}

#[derive(Debug)]
pub struct Point3D {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

#[derive(Debug)]
pub struct Point3DF32 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Pixel {
    pub(crate) x: u32,
    pub y: u32,
    pub color: u32,
}
