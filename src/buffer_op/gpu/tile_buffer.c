// This kernel copies a small tile image repeatedly across a larger 2D output buffer.

__kernel void tile_pattern(
    __read_only image2d_t tile,      // Input tile pattern (RGBA image)
    __global uchar4* output,         // Output buffer to be filled with tiled pattern
    const int width,                 // Width of output image
    const int height,                // Height of output image
    const int tile_width,            // Width of tile pattern
    const int tile_height            // Height of tile pattern
) {
    // Compute global coordinates for this work item (pixel position)
    int x = get_global_id(0);  // Horizontal coordinate in output
    int y = get_global_id(1);  // Vertical coordinate in output

    // Guard: ensure we're within bounds of the output image
    if (x >= width || y >= height) return;

    // Calculate corresponding coordinate inside the tile pattern
    // This is what makes the pattern repeat, we "wrap around" the tile dimensions
    int tx = x % tile_width;
    int ty = y % tile_height;

    // Define a sampler to fetch image data
    // - Unnormalized coords: use pixel-based access (not [0,1] range)
    // - Repeat address mode ensures wraparound (can also use CLAMP if preferred)
    // - Nearest filter: pick nearest pixel (no interpolation)
    sampler_t smp = CLK_NORMALIZED_COORDS_FALSE |
                    CLK_ADDRESS_REPEAT |
                    CLK_FILTER_NEAREST;

    // Read a pixel from the tile pattern at (tx, ty)
    // We use read_imageui for integer-typed RGBA values
    uint4 pixel = read_imageui(tile, smp, (int2)(tx, ty));

    // Calculate the 1D index in the output buffer for writing
    int index = y * width + x;

    // Write the pixel (converted to uchar4) into the output buffer
    output[index] = convert_uchar4(pixel);
}
