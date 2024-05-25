/*
   This module embeds font source images in rbf format into the compiled executable and provides
   a method for instantiating the embedded fonts for usage.

   "RBF" stands for "Raw Bitmap/Binary Font". See `/README.md` for details.
*/
/*
use crate::constants::POINT_ZERO;
use crate::graph1::primitives::primitives::{Dimensions2d, RectArea};
use crate::graph1::text::char_width_map::get_c_c_red_alert_inet0;
use crate::graph1::text::font;
use std::io::Read;

use crate::graph1::text::font::PixelFont;
use crate::graph1::utils::pixel_copy::image_data;

const DATA_C_C_RED_ALERT_INET0: &[u8] = include_bytes!("./rbf_data/c_c_red_alert_inet0.rbf");

pub enum EmbeddedFonts {
    CCRedAlertInet0,
}

pub fn instantiate_embedded_font(
    mut font_image_buf: Vec<u32>,
    font_name: EmbeddedFonts,
    font_scale_factor: u8,
    char_order: Option<&str>,
) -> PixelFont {

    let scale_factor: u8 =
        crate::graph1::utils::misc::nearest_power_of_two_towards_zero(font_scale_factor as u32)
            as u8;

    let mut font_data: &[u8] = match font_name {
        EmbeddedFonts::CCRedAlertInet0 => DATA_C_C_RED_ALERT_INET0,
        _ => DATA_C_C_RED_ALERT_INET0,
    };

    // Buffer to hold the first 8 bytes, they are the font data header
    let mut buffer = vec![0u8; 8];
    font_data.read_exact(&mut buffer).unwrap();

    // Read the header (first 8 bytes) into a Vec<u16>
    let font_header_raw: Vec<u16> = buffer
        .chunks(2)
        .map(|chunk| {
            let mut array = [0u8; 2];
            array.copy_from_slice(chunk);
            u16::from_le_bytes(array) // Decode from little endian and return
        })
        .collect();

    // Read the body of the font into a Vec<u8>
    let mut font_body_raw = vec![];
    font_data.read_to_end(&mut font_body_raw).unwrap();

    // Normalise the font header to Vec<u32>
    // let font_header: Vec<u32> = font_header_raw.into_iter().map(|x| x as u32).collect();
    let font_header: Vec<u32> = font_header_raw.into_iter().map(u32::from).collect();

    let font_body:  Vec<u32> = font_body_raw
        .into_iter()
        .map(|x| if x == 0x0_u8 { 0x0_u32 } else { 0x00_ff_ff_ff })
        .collect();

    let font_image_width = font_header[0] * scale_factor as u32;
    let font_image_height = font_header[1] * scale_factor as u32;

    // if scale_factor >1 {
    //
    // }

    font_image_buf = font_body.clone();

    let font_image_buf_dim: Dimensions2d = Dimensions2d {
        w: font_image_width,
        h: font_image_height,
    };

    // If we need to scale up the font, we first need to scale up its source image
    if scale_factor > 1 {
        let src_font_image_dim: Dimensions2d = Dimensions2d {
            w: font_header[0],
            h: font_header[1],
        };
        image_data::scale_up(
            &mut font_image_buf,
            &font_image_buf_dim,
            &POINT_ZERO,
            &font_body,
            &src_font_image_dim,
            &RectArea {
                top_left: POINT_ZERO,
                dimensions: Dimensions2d {
                    w: src_font_image_dim.w,
                    h: src_font_image_dim.h,
                },
            },
            scale_factor,
        );
    }
    let char_order: &str = char_order.unwrap_or_else(|| font::DEFAULT_CHAR_ORDER);

    PixelFont::new(
        &font_image_buf,
        font_image_width,
        font_image_height,
        char_order,
        2, // FIXME: should not be hardcoded!
        4, // FIXME: should not be hardcoded!
        get_c_c_red_alert_inet0(scale_factor),
        1 * scale_factor, //TODO: Check if this breaks with values greater than 1!
    )
}
*/