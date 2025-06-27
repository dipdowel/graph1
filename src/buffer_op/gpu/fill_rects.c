__kernel void fill_rects(
    __global uint *out_buf,
    uint width, uint height,
    __global const uint *rects,
    uint num_rects
) {
    uint gid = get_global_id(0);
    if (gid >= width * height) return;

    uint x = gid % width;
    uint y = gid / width;

    // Start with the existing value, not a default fill
    uint color = out_buf[gid];
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
