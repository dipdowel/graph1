/*
   This module embeds font source images in CBF format into the compiled executable and provides
   a method for instantiating the embedded fonts for usage.

   "CBF" stands for "Compact Bitmap Font". See `/README.md` for details.
*/

use std::collections::HashMap;
use std::io::Read;

use crate::constants::POINT_ZERO;
use crate::graph1::primitives::primitives::{Dimensions2d, RectArea};
use crate::graph1::text::{font, font_constants};
use crate::graph1::text::font::{PixelFont, PixelFontMeta, Spacing};
use crate::graph1::utils::bit_operations;
use crate::graph1::utils::math::nearest_power_of_two_towards_zero;
use crate::graph1::utils::pixel_copy::image_data;
use crate::graph1::utils::text::u16_vec_to_utf8_char;

// Embed fonts data
const DATA_C_C_RED_ALERT_INET: &[u8] = include_bytes!("cbf_data/c_c_red_alert_inet0.cbf");
const DATA_C_C_RED_ALERT_LAN: &[u8] = include_bytes!("cbf_data/cc_red_alert_lan.cbf");

// List of available embedded fonts
pub enum EmbeddedFonts {
    CCRedAlertInet,
    CCRedAlertLan,
}

fn validate_cbf_basics(cbf_magic_number:u16, cbf_ver:u16) {
    if cbf_magic_number != font_constants::CBF_MAGIC_NUMBER {
        panic!("CBF is possibly malformed (wrong magic number: {:08X})", cbf_magic_number);
    }

    if cbf_ver != font_constants::CBF_VERSION {
        panic!("Expected a CBF of version {}", font_constants::CBF_VERSION);
    }

}

/// Instantiates an embedded pixel font
///
/// # Parameters
///
/// - `font_name`: The name of the embedded font to instantiate.
/// - `font_scale_factor`: scale up the font size, valid values are powers of two (1,2,4,8, etc,)
/// - `spacing`: Kerning and Leading
/// - `default_char`: An optional character to use as the default character if a specified character is not found.
pub fn instantiate_embedded_font(
    font_name: EmbeddedFonts,
    font_scale_factor: u8,
    spacing: Option<Spacing>,
    default_char: Option<char>,
) -> PixelFont {

    let scale_factor: u8 = nearest_power_of_two_towards_zero(font_scale_factor as u32) as u8;

    let mut font_data: &[u8] = match font_name {
        EmbeddedFonts::CCRedAlertInet => DATA_C_C_RED_ALERT_INET,
        EmbeddedFonts::CCRedAlertLan => DATA_C_C_RED_ALERT_LAN,
        _ => DATA_C_C_RED_ALERT_INET,
    };

    // Buffer to hold the font header
    let mut buffer = vec![0u8; 14*2];
    font_data.read_exact(&mut buffer).unwrap();

    // Read the header bytes into a Vec<u16>
    let font_header: Vec<u16> = buffer
        .chunks(2)
        .map(|chunk| {
            let mut array = [0u8; 2];
            array.copy_from_slice(chunk);
            u16::from_le_bytes(array) // Decode from little endian and return
        })
        .collect();

    //^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    //  Header consists of a bunch of `u16` values:
    // -----------------------------------------------------------------------------------
    // [00] - magic number `CBF0` for "Compact Bitmap F0nt"
    // [01] - version of CBF format
    // [02] - size of `font_name`, size of the name of the font
    // [03] - size of `author_signature`, size of the author's name
    // [04] - size of `char_order`
    // [05] - size of `char_width`
    // [06] - font image width
    // [07] - font image height
    // [08] - spacing props: lower byte -- kerning, higher byte -- leading.
    // [09] - UTF8 default char: lower 2 bytes.
    // [10] - UTF8 default char: higher 2 bytes.
    // [11] - version of the font
    // [12] - date: year
    // [13] - date: lower byte -- day, higher byte -- month.
    // -----------------------------------------------------------------------------------
    // Font body fields:
    //      * font_name -- A string with the name of the font.
    //      * author_signature -- A string with the name of the author of the font.
    //      * char_order -- order of characters in the font.
    //      * char_widths -- how many pixels wide a char is. In the order of `char_order`
    //      * font_pixel_data -- 1-bit image data of the font (0 for black, 1 for white)


    // READ THE HEADER
    // ---------------

    // CBF file identification
    let cbf_magic_number = font_header[0];
    let cbf_version = font_header[1];
    validate_cbf_basics(cbf_magic_number, cbf_version);

    // Sizes of the variable-length data fields
    let font_name_size = font_header[2] as usize;
    let author_signature_size = font_header[3] as usize;
    let char_order_size = font_header[4] as usize;
    let char_widths_size = font_header[5] as usize;

    // Font properties
    let font_image_width = font_header[6] as u32;
    let font_image_height = font_header[7] as u32;
    let spacing_props = font_header[8];

    // Default char
    let default_char_part_1 = font_header[9];
    let default_char_part_2 = font_header[10];

    let font_ver = font_header[11];

    // Font creation date
    let date_year = font_header[12];
    let month_day = font_header[13];


    // Default kerning of the font is stored in the lower byte of `spacing_props`
    let font_native_kerning_px = (spacing_props & 0x00FF) as u8;

    // Default leading of the font is stored in the higher byte of `spacing_props`
    let font_native_leading_px = (spacing_props >> 8) as u8;

    // Reassemble the default char
    let native_default_char = u16_vec_to_utf8_char(vec![default_char_part_1, default_char_part_2]);

    let date_month = (month_day & 0x00FF) as u8;
    let date_day = (month_day >> 8) as u8;

    // println!(">>> [!] FONT CREATION DATE: {}-{}-{}", year, month, day);
    // println!(">>> [!] FONT VER: {font_ver}");

    // READ THE BODY
    // -------------

    // Read the font name
    let mut font_name_buf = Vec::new();
    font_name_buf.resize(font_name_size, 0);
    font_data.read_exact(&mut font_name_buf).unwrap();
    let font_name = String::from_utf8(font_name_buf).unwrap_or_else(|er| "A font with no name".to_string());
     // println!(">>>> font_name: {}", font_name);

    // Read the signature of the author of the font
    let mut author_signature_buf = Vec::new();
    author_signature_buf.resize(author_signature_size, 0);
    font_data.read_exact(&mut author_signature_buf).unwrap();
    let author_signature = String::from_utf8(author_signature_buf).unwrap_or_else(|er| "No author name set".to_string());
    // println!(">>>> author_signature: {}", author_signature);

    // Read character order in the font
    let mut char_order_buf = Vec::new();
    char_order_buf.resize(char_order_size, 0);
    font_data.read_exact(&mut char_order_buf).unwrap();
    let char_order = String::from_utf8(char_order_buf).unwrap_or_else(|er| font::DEFAULT_CHAR_ORDER.to_string());
    // println!(">>>> char_order: {}", char_order);

    // Read widths of characters in the font
    let mut char_widths_buf = Vec::new();
    char_widths_buf.resize(char_widths_size, 0);
    font_data.read_exact(&mut char_widths_buf).unwrap();
    // println!(">>>> chat_widths_buf: {:?}", char_widths_buf);

    let mut char_map: HashMap<char, u8> = HashMap::new();

    let mut char_index:usize  = 0;
    for ch in char_order.chars(){
        char_map.insert(ch, char_widths_buf[char_index]);
        char_index += 1;
    }

    // Read the font bitmap data
    let mut font_pixel_data = vec![];
    font_data.read_to_end(&mut font_pixel_data).unwrap();

    let src_font_image: Vec<u32> = bit_operations::one_bit_image_to_rgb(&font_pixel_data);
    let mut font_image_buf: Vec<u32> = src_font_image.clone();

    let mut font_image_buf_dim: Dimensions2d = Dimensions2d {
        w: font_image_width,
        h: font_image_height,
    };

    // If we need to scale up the font, we first need to scale up its source image
    if scale_factor > 1 {
        let src_font_image_dim: Dimensions2d = Dimensions2d {
            w: font_image_width,
            h: font_image_height,
        };


        for value in char_map.values_mut(){
            *value = *value * scale_factor;
        }



        font_image_buf_dim.w = font_image_buf_dim.w * (scale_factor as u32);
        font_image_buf_dim.h = font_image_buf_dim.h * (scale_factor as u32);

        font_image_buf.resize((font_image_buf_dim.w * font_image_buf_dim.h) as usize, 0);

        image_data::scale_up(
            &mut font_image_buf,
            &font_image_buf_dim,
            &POINT_ZERO,
            &src_font_image,
            &src_font_image_dim,
            &RectArea {
                top_left: POINT_ZERO,
                dimensions: Dimensions2d {
                    w: font_image_width,
                    h: font_image_height,
                },
            },
            scale_factor,
        );
    }

    let spacing = spacing.unwrap_or_else(|| Spacing {
        kerning_px: font_native_kerning_px,
        leading_px: font_native_leading_px,
    });


    let default_char = default_char.unwrap_or_else(|| native_default_char);

    let img_dimensions = Dimensions2d {
        w:  font_image_buf_dim.w,
        h:  font_image_buf_dim.h,
    };

    PixelFont::new(
        font_image_buf,
        img_dimensions,
        char_order,
        default_char,
        spacing,
        char_map,
        1* scale_factor,
        PixelFontMeta{
            font_ver,
            font_name,
            author_signature,
            date_year,
            date_month,
            date_day

        }
    )
}
