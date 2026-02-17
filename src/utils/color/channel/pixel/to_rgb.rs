/// Extract RGBA channels from a pixel as u32
#[inline(always)]
pub fn as_u32(pixel: u32) -> (u32, u32, u32, u32) {
    let r = (pixel >> 24) & 0xFF;
    let g = (pixel >> 16) & 0xFF;
    let b = (pixel >> 8) & 0xFF;
    let a = pixel & 0xFF;
    (r, g, b, a)
}

/// Extract RGBA channels from a pixel as u8
#[inline(always)]
pub fn as_u8(pixel: u32) -> (u8, u8, u8, u8) {
    let r = ((pixel >> 24) & 0xFF) as u8;
    let g = ((pixel >> 16) & 0xFF) as u8;
    let b = ((pixel >> 8) & 0xFF) as u8;
    let a = (pixel & 0xFF) as u8;
    (r, g, b, a)
}

/// Extract RGBA channels from a pixel as u16
#[inline(always)]
pub fn as_u16(pixel: u32) -> (u16, u16, u16, u16) {
    let r = ((pixel >> 24) & 0xFF) as u16;
    let g = ((pixel >> 16) & 0xFF) as u16;
    let b = ((pixel >> 8) & 0xFF) as u16;
    let a = (pixel & 0xFF) as u16;
    (r, g, b, a)
}

/// Extract RGBA channels from a pixel as f32 containing packed values (0..255)
#[inline(always)]
pub fn as_f32(pixel: u32) -> (f32, f32, f32, f32) {
    let r = ((pixel >> 24) & 0xFF) as f32;
    let g = ((pixel >> 16) & 0xFF) as f32;
    let b = ((pixel >> 8) & 0xFF) as f32;
    let a = (pixel & 0xFF) as f32;
    (r, g, b, a)
}

/// Extract RGBA channels from a pixel as f64 containing packed values (0..255)
#[inline(always)]
pub fn as_f64(pixel: u32) -> (f64, f64, f64, f64) {
    let r = ((pixel >> 24) & 0xFF) as f64;
    let g = ((pixel >> 16) & 0xFF) as f64;
    let b = ((pixel >> 8) & 0xFF) as f64;
    let a = (pixel & 0xFF) as f64;
    (r, g, b, a)
}

/// Extract RGBA channels from a pixel as f32 (normalized to 0.0..1.0)
#[inline(always)]
pub fn as_f32_normalized(pixel: u32) -> (f32, f32, f32, f32) {
    let r = ((pixel >> 24) & 0xFF) as f32 / 255.0;
    let g = ((pixel >> 16) & 0xFF) as f32 / 255.0;
    let b = ((pixel >> 8) & 0xFF) as f32 / 255.0;
    let a = (pixel & 0xFF) as f32 / 255.0;
    (r, g, b, a)
}

/// Extract RGBA channels from a pixel as f64 (normalized to 0.0..1.0)
#[inline(always)]
pub fn as_f64_normalized(pixel: u32) -> (f64, f64, f64, f64) {
    let r = ((pixel >> 24) & 0xFF) as f64 / 255.0;
    let g = ((pixel >> 16) & 0xFF) as f64 / 255.0;
    let b = ((pixel >> 8) & 0xFF) as f64 / 255.0;
    let a = (pixel & 0xFF) as f64 / 255.0;
    (r, g, b, a)
}

