// Tilemap renderer kernel with optional rotation and flipping,
// plus a default fill color for regions outside the tilemap.
// Author: Graaf van de Graphics

// tile_descriptor (16-bit):
//  - Bits  0-11: tile_id      (0–4095) — index of tile in atlas
//  - Bits 12-13: rotation     (2-bit code: 00 = 0°, 01 = 90°, 10 = 180°, 11 = 270°)
//  - Bit     14: flip_y       (1 = vertical flip)
//  - Bit     15: flip_x       (1 = horizontal flip)

__kernel void render_tilemap(
    __read_only image2d_t tile_atlas,     // Tile atlas (packed grid of tiles)
    __global const ushort* tile_descriptors, // Tile map buffer (tile descriptors)
    __global uchar4* framebuffer,         // Output framebuffer (1 pixel = uchar4 RGBA)

    const int fb_width,                   // Framebuffer width (in pixels)
    const int fb_height,                  // Framebuffer height (in pixels)

    const int tile_width,                 // Width of a single tile
    const int tile_height,                // Height of a single tile

    const int atlas_tiles_per_row,        // Number of tiles per row in atlas

    const int tilemap_width,              // Width of tilemap (in tiles)
    const int tilemap_height,             // Height of tilemap (in tiles)

    const int enable_transform,           // Boolean flag: apply rotation/flip?

    const uint default_color              // Default color (0xRRGGBBAA) for out-of-bounds pixels
) {
    int x = get_global_id(0); // Pixel X
    int y = get_global_id(1); // Pixel Y

    if (x >= fb_width || y >= fb_height) return;

    // Determine tile cell and pixel offset within the tile
    int tile_col = x / tile_width;
    int tile_row = y / tile_height;

    int local_x = x % tile_width;
    int local_y = y % tile_height;

    // If pixel lies outside the tilemap-defined area, fill with default color
    if (tile_col >= tilemap_width || tile_row >= tilemap_height) {
        framebuffer[y * fb_width + x] = convert_uchar4(default_color);
        return;
    }

    // Read tile descriptor (16-bit value)
    int tile_index = tile_row * tilemap_width + tile_col;
    ushort descriptor = tile_descriptors[tile_index];

    // Decode tile ID (lower 12 bits)
    ushort tile_id = descriptor & 0x0FFF;

    int tx = local_x;
    int ty = local_y;

    if (enable_transform) {
        // Decode transform flags from upper 4 bits
        int rotation = (descriptor >> 12) & 0x03; // bits 12–13
        int flip_y   = (descriptor >> 14) & 0x01; // bit 14
        int flip_x   = (descriptor >> 15) & 0x01; // bit 15

        // Apply flips
        if (flip_x) tx = tile_width - 1 - tx;
        if (flip_y) ty = tile_height - 1 - ty;

        // Apply rotation (0°, 90°, 180°, 270°)
        int rx = tx, ry = ty;
        switch (rotation) {
            case 0:  rx = tx;                    ry = ty;                    break; // 0°
            case 1:  rx = ty;                    ry = tile_width - 1 - tx;   break; // 90°
            case 2:  rx = tile_width - 1 - tx;   ry = tile_height - 1 - ty;  break; // 180°
            case 3:  rx = tile_height - 1 - ty;  ry = tx;                    break; // 270°
        }
        tx = rx;
        ty = ry;
    }

    // Determine top-left position of the tile in the atlas
    int atlas_tile_x = (tile_id % atlas_tiles_per_row) * tile_width;
    int atlas_tile_y = (tile_id / atlas_tiles_per_row) * tile_height;

    // Final sample position in the tile atlas
    int sample_x = atlas_tile_x + tx;
    int sample_y = atlas_tile_y + ty;

    // Create sampler for nearest-neighbor lookup without normalization
    sampler_t smp = CLK_NORMALIZED_COORDS_FALSE |
                    CLK_ADDRESS_CLAMP |
                    CLK_FILTER_NEAREST;

    // Read pixel from atlas
    uint4 pixel = read_imageui(tile_atlas, smp, (int2)(sample_x, sample_y));

    // Write to framebuffer
    framebuffer[y * fb_width + x] = convert_uchar4(pixel);
}
