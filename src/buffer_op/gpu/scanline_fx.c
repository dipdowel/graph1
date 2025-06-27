__kernel void scanline_fx(
    __global uint *buf,
    uint intensity,
    uint line_flipper,
    uint pixel_count,
    uint width
) {
    uint gid = get_global_id(0);
    if (gid >= pixel_count) return;

    // Figure out if this pixel is in a scanline "row"
    if (((gid / line_flipper) % 2) == 0) {
        // Do subtract intensity (on packed RGBA)
        uint c = buf[gid];

        // Subtract the intensity from each color channel except alpha
        
        // Fast path: subtract as a 32-bit value (may overflow/underflow)
        buf[gid] = c - intensity;
    }
}
