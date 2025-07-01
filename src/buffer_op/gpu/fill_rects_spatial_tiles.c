__kernel void fill_rects_tiles(
    __global uint *out_buf,
    uint width, uint height,
    __global const uint *flat_rects,
    __global const uint *tile_offsets,
    __global const uint *tile_counts,
    uint tiles_x,
    uint tiles_y,
    uint tile_w,
    uint tile_h
) {
    uint gid = get_global_id(0);
    if (gid >= width * height) return;

    uint x = gid % width;
    uint y = gid / width;

    uint tx = x / tile_w;
    uint ty = y / tile_h;
    if (tx >= tiles_x) tx = tiles_x - 1;
    if (ty >= tiles_y) ty = tiles_y - 1;
    uint tile_idx = ty * tiles_x + tx;

    uint offset = tile_offsets[tile_idx];
    uint count  = tile_counts[tile_idx];

    uint color = out_buf[gid];
    for (uint i = 0; i < count; ++i) {
        uint base = (offset + i) * 5;
        uint rx = flat_rects[base + 0];
        uint ry = flat_rects[base + 1];
        uint rw = flat_rects[base + 2];
        uint rh = flat_rects[base + 3];
        uint rcolor = flat_rects[base + 4];

        if (x >= rx && x < rx + rw && y >= ry && y < ry + rh) {
            color = rcolor;
        }
    }
    out_buf[gid] = color;
}
