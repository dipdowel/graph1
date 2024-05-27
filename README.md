# Graph1

**Application code** is code that uses Graph1 library.

## What Graph1 is and what it does
- Graph1 is a zero-dependency library for producing and manipulating graphical primitives, e.g. lines, curves,
  simple geometric shapes. It also can render texts in pixel fonts.
- Graph1 can be used for creating static images, animations, 2D computer game graphics (non-GPU).

## What Graph1 is not and what it does not
- Graph1 does not read or write files, but it can accept data from files read by the application code 
- Keeping any static state is outside Graph1 scope. This should be done in the application code 

## Working with color
At the moment of writing, `Graph1` uses `0RGB` encoding for a pixel, which means that the upper 8-bits are ignored, 
the next 8 bits are for the R channel, then 8 bits for the G channel, and the last 8 bits for the B channel. This helps
to avoid any extra conversion while working with library `minifb`.

Eventually, we may move to `ARGB` model, but that's not the case now.



## Fonts

### Default embedded fonts
By default, the following pixel fonts are available in Graph1 framework:
- `c_c_red_alert_inet0` by [N3tRunn3r](https://forums.cncnet.org/profile/30740-n3trunn3r/)
- `c_c_red_alert_inet1 (LAN)` by [N3tRunn3r](https://forums.cncnet.org/profile/30740-n3trunn3r/)

The default fonts are stored in the Graph1 sourcecode and are encoded in CBF format (see below).
They are embedded directly into a compiled application using `include_bytes!()`. 

It is also possible to load and use your custom pixel fonts (see section "Using custom pixel fonts").

### CBF font format
"CBF" stands for "Compact Bitmap Font". It is a simple non-compressed binary format for storing pixel fonts as raw binary data.
An CBF-file contains a header with metadata and some data sizes, and a body with some textual information 
and the bits representing the font itself.  

#### The header
Header consists of a bunch of `u16` values:
-----------------------------------------------------------------------------------
- [0x0] - magic number `CBF0` for "Compact Bitmap F0nt"
- [0x1] - version of CBF format
- [0x2] - size of `font_name`, size of the name of the font
- [0x3] - size of `author_signature`, size of the author's name
- [0x4] - size of `char_order`
- [0x5] - size of `char_width`
- [0x6] - font image width
- [0x7] - font image height
- [0x8] - spacing props: lower byte -- kerning, higher byte -- leading.
- [0x9] - UTF8 default char: lower 2 bytes.
- [0xa] - UTF8 default char: higher 2 bytes.
- [0xb] - version of the font
- [0xc] - date: year
- [0xd] - date: lower byte -- day, higher byte -- month.
-----------------------------------------------------------------------------------

 
#### The body
The font body consists of a few fields of variable length. All the lengths are listed in the header.

Font body fields:
- `font_name` - A string with the name of the font.
- `author_signature` - A string with the name of the author of the font.
- `char_order` - order of characters in the font.
- `char_widths` - how many pixels wide each char is. The order of values matches the order of chars in `char_order`
- `font_pixel_data` - 1-bit image data of the font (0 for black, 1 for white)
  
### CBF parsing
Graph1 parses an

### Creating and/or using your own pixel fonts
- `TODO:` Come up with how to create and add custom fonts 
- `TODO:` Explain how to add custom fonts, etc. 
 