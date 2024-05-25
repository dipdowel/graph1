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

The default fonts are stored in the Graph1 sourcecode and are encoded in RBF format (see below).
They are embedded directly into a compiled application using `include_bytes!()`. 

It is also possible to load and use your custom pixel fonts (see section "Using custom pixel fonts").

### RBF font format
"RBF" stands for "Raw Bitmap/Binary Font". It is a simple non-compressed binary format for storing pixel fonts as raw binary data.
An RBF-file contains a header with metadata and a body with the font itself.  

#### The header
The font header consists of 4 u16 values.
- `[0]` - font image width
- `[1]` - font  image height
- `[2]` - `0x00_00` -- reserved for the future
- `[3]` - `0x00_00` -- reserved for the future
- 
#### The body
The font body consists of u8 values, encoding black with `0x00` and white with any other value. It is recommended, however,
to use `0xff` to encode white.

### RBF parsing
Graph1 parses an

### Creating and/or using your own pixel fonts
- `TODO:` Come up with how to create and addcustom fonts 
- `TODO:` Explain how to add custom fonts, etc. 
 