__kernel void tile_pattern(
    __read_only image2d_t tile,
    __global uchar4* output,
    const int width,
    const int height,
    const int tile_width,
    const int tile_height
) {
    int x = get_global_id(0);
    int y = get_global_id(1);

    if (x >= width || y >= height) return;

    int tx = x % tile_width;
    int ty = y % tile_height;

    sampler_t smp = CLK_NORMALIZED_COORDS_FALSE | CLK_ADDRESS_REPEAT | CLK_FILTER_NEAREST;
    uint4 pixel = read_imageui(tile, smp, (int2)(tx, ty));

    int index = y * width + x;
    output[index] = convert_uchar4(pixel);
}
