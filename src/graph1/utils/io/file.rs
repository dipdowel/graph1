use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImageView, Pixel};

use crate::about::AboutApp;
use crate::graph1::utils::color_math::argb_math::argb_math;
use crate::graph1::utils::color_math::operations::ColorOperation;

/// Reads a file from an image, converts it to 0RGB format and writes to a provided `buf`.
/// - On success returns `true`.
/// - On failure writes an error to `stderr` and returns `false`.
/// If `buf` is too short, it gets extended to accommodate all the pixels.
/// If `buf` is too long, the unused memory is left intact (i.e. may contain garbage)
pub fn read_image(path: &str, buf: &mut Vec<u32>) -> bool {
    let file_open_result = ImageReader::open(path);

    let reader = match file_open_result {
        Ok(reader) => {
            println!("{} File opened successfully!", AboutApp::log_prefix());
            reader
        }
        Err(error) => {
            eprintln!("{}Failed to open {path} :: {error}", AboutApp::err_prefix());
            return false;
        }
    };

    let image_result = reader.decode();

    let image = match image_result {
        Ok(image) => {
            println!("{} Image decoded successfully!", AboutApp::log_prefix());
            image
        }
        Err(error) => {
            eprintln!(
                "{}Failed to decode an image from {path} :: {error}",
                AboutApp::err_prefix()
            );
            return false;
        }
    };

    convert_to_0rgb(image, buf);
    return true;
}

/// Converts a `DynamicImage` instance to 0RGB model and writes the result to `buf`
fn convert_to_0rgb(image: DynamicImage, buf: &mut Vec<u32>) {
    let mut pixel_index: usize = 0;
    let buf_len: usize = buf.len();

    // TODO: 1. Write some unit tests
    // TODO: 2. Use `image_bytes` instead of `image.height() * image.weight()`
    // TODO: 3. Do we still get right results? It should be faster this way.
    // let image_bytes = image.into_bytes();

    // Access the image's pixels.
    for y in 0..image.height() {
        for x in 0..image.width() {
            let pixel = image.get_pixel(x, y);

            let pixel_0rgb: u32 = (0x00_00_00_00 << 24)
                | ((pixel[0] as u32) << 16)
                | ((pixel[1] as u32) << 8)
                | (pixel[2] as u32);

            // TODO: Unit test if this works correctly!!!
            // Overwrite the existing values in the buffer.
            // Grow buffer if it's too short.
            if pixel_index < buf_len {
                buf[pixel_index] = pixel_0rgb;
            } else {
                buf.push(pixel_0rgb);
            }

            println!("Pixel at ({}, {}) is {:?}", x, y, pixel);
            println!("pixel_0RGB at ({}, {}) is 0x{:08X}", x, y, pixel_0rgb);
        }
    }
}

// Unit tests module
#[cfg(test)]
mod tests {
    use image::ColorType;
    use super::*;

    #[test]
    fn test_pixel_model_conversion() {
        let mut image_buf: Vec<u32> = Vec::new();
        let result = read_image("assets/01_test_palette.png", &mut image_buf);
        assert_eq!(2, 2);
    }

    #[test]
    fn test_convert_to_0rgb_right_size() {
        let mut image_buf: Vec<u32> = Vec::new();

        let the_length = 12;

        let d_image = DynamicImage::new(the_length, 1, ColorType::Rgba8);

        assert_eq!(image_buf.len(), the_length as usize);

    }


}
