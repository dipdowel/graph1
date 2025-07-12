__kernel void scanline_fx(
    __global uint *buf,
    uint intensity,
    uint line_flipper,
    uint pixel_count,
    uint width
) {
    uint gid = get_global_id(0);
    if (gid >= pixel_count) return;

    // Apply effect only to selected scanlines
    if (((gid / line_flipper) % 2) == 0) {
        uint c = buf[gid];

        // Extract channels
        uint r = (c >> 24) & 0xff;
        uint g = (c >> 16) & 0xff;
        uint b = (c >> 8) & 0xff;
        uint a = c & 0xff;

        // Clamp subtractions to avoid underflow
        r = r > ((intensity >> 24) & 0xff) ? r - ((intensity >> 24) & 0xff) : 0;
        g = g > ((intensity >> 16) & 0xff) ? g - ((intensity >> 16) & 0xff) : 0;
        b = b > ((intensity >> 8) & 0xff) ? b - ((intensity >> 8) & 0xff) : 0;

        // Repack and store
        buf[gid] = (r << 24) | (g << 16) | (b << 8) | a;
    }
}
