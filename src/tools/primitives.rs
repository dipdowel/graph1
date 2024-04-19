#[derive(Debug)]
pub struct Dimensions2d {
    pub w: u32,
    pub h: u32,
}

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug)]
pub struct RectArea {
    pub top_left:Point,
    pub dimensions:Dimensions2d
}


pub struct StarProperties {
    /// Location of the central point of the polygon
    pub center: Pixel,

    /// How many angles the polygon has
    pub num_vertices: u32,

    /// Distance from the `center` after which every N+1th  vertex of the polygon lies
    pub inner_radius:u32,

    /// Distance from the `center` after which every N+2th  vertex of the polygon lies
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
