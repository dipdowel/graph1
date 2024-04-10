#[derive(Debug)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug)]
pub struct Point3D {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

#[derive(Debug)]
pub struct Pixel {
    pub(crate) x: u32,
    pub y: u32,
    pub color: u32,
}
