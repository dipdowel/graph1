use image::{DynamicImage, GenericImageView};
use image::io::Reader as ImageReader;
use crate::about::AboutApp;

/// Reads a file from an image, converts it to 0RGB format and writes to a provided `buf`.
/// - On success returns `true`.
/// - On failure writes an error to `stderr` and returns `false`.
/// `buf` is cleared before it gets populated by the pixels
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

    // println!("{:?}", image);

    convert_to_0rgb(image, buf);
    return true;
}

/// Converts a `DynamicImage` instance to 0RGB model and writes the result to `buf`
fn convert_to_0rgb(image: DynamicImage, buf: &mut Vec<u32>) {

    // Make sure there's no pre-existing garbage in the buffer
    buf.clear();

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

            buf.push(pixel_0rgb);

            // println!("Pixel at ({}, {}) is {:?}", x, y, pixel);
            // println!("pixel_0RGB at ({}, {}) is 0x{:08X}", x, y, pixel_0rgb);
        }
    }
}

// Unit tests module
#[cfg(test)]
mod tests {
    use image::{ColorType, ImageBuffer, Rgba, RgbaImage};

    use super::*;

    #[test]
    fn test_convert_to_0rgb_right_size() {
        let mut image_buf: Vec<u32> = Vec::new();

        let dynamic_image = DynamicImage::new(12, 1, ColorType::Rgba8);
        let result = convert_to_0rgb(dynamic_image, &mut image_buf);
        assert_eq!(image_buf.len(), 12);
    }

    #[test]
    fn test_convert_to_0rgb_conversion() {
        // Initialise the image
        let mut rgba_img: RgbaImage = ImageBuffer::new(5, 1);

        let mut pixels = rgba_img.pixels_mut();

        let val = pixels.next().unwrap();
        *val = Rgba([0xff, 0x00, 0x00, 0xff]);
        let val = pixels.next().unwrap();
        *val = Rgba([0x00, 0xff, 0x00, 0xff]);
        let val = pixels.next().unwrap();
        *val = Rgba([0x00, 0x00, 0xff, 0xff]);
        let val = pixels.next().unwrap();
        *val = Rgba([0xff, 0x00, 0xff, 0xff]);
        let val = pixels.next().unwrap();
        *val = Rgba([0xff, 0xff, 0xff, 0xff]);

        let dynamic_image = DynamicImage::from(rgba_img);
        // println!(">>>> dynamic_image: {:?}", dynamic_image);

        let mut image_buf: Vec<u32> = Vec::new();
        convert_to_0rgb(dynamic_image, &mut image_buf);

        let expected_values = vec![
            0x00_ff_00_00,
            0x00_00_ff_00,
            0x00_00_00_ff,
            0x00_ff_00_ff,
            0x00_ff_ff_ff,
        ];

        let mut count = 0;
        for pixel in image_buf {
            // println!(">>>> test image_buf[{count}]: {:08X}, {:08X}", pixel, expected_values[count]);
            assert_eq!(pixel, expected_values[count]);
            count += 1;
        }
    }
}
