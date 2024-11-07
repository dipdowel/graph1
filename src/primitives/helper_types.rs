
/// Array of pixels. Each pixel has the `RGBA` model.
pub type BufferRGBA<'a> = &'a mut [u32];

/// A function that transforms the color of a pixel based on its initial value and coordinates.
///
/// This type describes a function that takes a pixel's color value,
/// its (x, y) coordinates, and the width and height of the whole image data. The function
/// returns a new pixel color value based on the input parameters.
///
/// # Parameters
///
/// - `color`: The original color of the pixel as 0RGB
/// - `x`: The x-coordinate of the pixel in the image.
/// - `y`: The y-coordinate of the pixel in the image.
/// - `w`: The width of the image data in pixels.
/// - `h`: The height of the image data in pixels.
/// - `data`: Any extra u32 values that need to be passed to the color transformer function.
///
/// # Returns
///
/// An RGBA value representing the transformed pixel
pub type PixelColorTransformerFn =
fn(color: u32, x: u32, y: u32, w: u32, h: u32, data: Option<&Vec<u32>>) -> u32;
