__kernel void fill_rects(
    __global uint *out_buf,
    uint width, uint height,
    __global const uint *rects, // packed: [x, y, w, h, color, x, y, ...]
    uint num_rects,
    uint default_color
) {
    uint gid = get_global_id(0);
    if (gid >= width * height) return;

    uint x = gid % width;
    uint y = gid / width;

    uint color = default_color;
    for (uint i = 0; i < num_rects; ++i) {
        uint rx = rects[i*5+0];
        uint ry = rects[i*5+1];
        uint rw = rects[i*5+2];
        uint rh = rects[i*5+3];
        uint rcolor = rects[i*5+4];

        if (x >= rx && x < rx+rw && y >= ry && y < ry+rh) {
            color = rcolor; // last rect wins
        }
    }
    out_buf[gid] = color;
}
