/// RGB tint color representation for multiplicative tinting operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tint {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Tint {
    /// Creates a new tint with the specified RGB values.
    #[inline]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Creates a white tint (no color modification).
    #[inline]
    pub const fn white() -> Self {
        Self { r: 255, g: 255, b: 255 }
    }
}

impl From<(u8, u8, u8)> for Tint {
    #[inline]
    fn from(tuple: (u8, u8, u8)) -> Self {
        Self::new(tuple.0, tuple.1, tuple.2)
    }
}

impl From<u32> for Tint {
    #[inline]
    fn from(color: u32) -> Self {
        let r = ((color >> 24) & 0xFF) as u8;
        let g = ((color >> 16) & 0xFF) as u8;
        let b = ((color >> 8) & 0xFF) as u8;
        Self::new(r, g, b)
    }
}

