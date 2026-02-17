/// Create a pixel from RGBA channels as u32
#[inline(always)]
pub fn of_u32(r: u32, g: u32, b: u32, a: u32) -> u32 {
    ((r & 0xFF) << 24) | ((g & 0xFF) << 16) | ((b & 0xFF) << 8) | (a & 0xFF)
}

/// Create a pixel from RGBA channels as u8
#[inline(always)]
pub fn of_u8(r: u8, g: u8, b: u8, a: u8) -> u32 {
    ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | (a as u32)
}

/// Create a pixel from RGBA channels as u16
#[inline(always)]
pub fn of_u16(r: u16, g: u16, b: u16, a: u16) -> u32 {
    (((r & 0xFF) as u32) << 24) | (((g & 0xFF) as u32) << 16) | (((b & 0xFF) as u32) << 8) | ((a & 0xFF) as u32)
}

/// Create a pixel from RGBA channels as f32 containing packed RRGGBBAA values
#[inline(always)]
pub fn of_f32(r: f32, g: f32, b: f32, a: f32) -> u32 {
    let r = (r as u32) & 0xFF;
    let g = (g as u32) & 0xFF;
    let b = (b as u32) & 0xFF;
    let a = (a as u32) & 0xFF;
    (r << 24) | (g << 16) | (b << 8) | a
}

/// Create a pixel from RGBA channels as f64 containing packed RRGGBBAA values
#[inline(always)]
pub fn of_f64(r: f64, g: f64, b: f64, a: f64) -> u32 {
    let r = (r as u32) & 0xFF;
    let g = (g as u32) & 0xFF;
    let b = (b as u32) & 0xFF;
    let a = (a as u32) & 0xFF;
    (r << 24) | (g << 16) | (b << 8) | a
}

/// Create a pixel from RGBA channels as f32 (normalized from 0.0..1.0)
#[inline(always)]
pub fn of_f32_normalized(r: f32, g: f32, b: f32, a: f32) -> u32 {
    let r = (r * 255.0).clamp(0.0, 255.0) as u32;
    let g = (g * 255.0).clamp(0.0, 255.0) as u32;
    let b = (b * 255.0).clamp(0.0, 255.0) as u32;
    let a = (a * 255.0).clamp(0.0, 255.0) as u32;
    (r << 24) | (g << 16) | (b << 8) | a
}

/// Create a pixel from RGBA channels as f64 (normalized from 0.0..1.0)
#[inline(always)]
pub fn of_f64_normalized(r: f64, g: f64, b: f64, a: f64) -> u32 {
    let r = (r * 255.0).clamp(0.0, 255.0) as u32;
    let g = (g * 255.0).clamp(0.0, 255.0) as u32;
    let b = (b * 255.0).clamp(0.0, 255.0) as u32;
    let a = (a * 255.0).clamp(0.0, 255.0) as u32;
    (r << 24) | (g << 16) | (b << 8) | a
}

