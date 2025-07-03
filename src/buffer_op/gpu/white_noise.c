__kernel void apply_noise_kernel(
    __global uint *target_buf,
    __global uint *noise_buf,
    uint len,
    uint noise_len,
    uchar operation,
    uchar use_alpha,
    uint step
) {
    uint gid = get_global_id(0);
    if (gid >= len || (gid % step) != 0) return;

    uint c1 = target_buf[gid];
    uint c2 = noise_buf[(gid / step) % noise_len];

    uint r1 = (c1 >> 24) & 0xff;
    uint g1 = (c1 >> 16) & 0xff;
    uint b1 = (c1 >> 8) & 0xff;
    uint a1 = c1 & 0xff;

    uint r2 = (c2 >> 24) & 0xff;
    uint g2 = (c2 >> 16) & 0xff;
    uint b2 = (c2 >> 8) & 0xff;
    uint a2 = c2 & 0xff;

    uint r, g, b, a;
    if (operation == 0) { // Add
        r = min(r1 + r2, (uint)255);
        g = min(g1 + g2, (uint)255);
        b = min(b1 + b2, (uint)255);
        a = use_alpha ? min(a1 + a2, (uint)255) : a1;
    } else { // Subtract
        r = r1 > r2 ? r1 - r2 : 0;
        g = g1 > g2 ? g1 - g2 : 0;
        b = b1 > b2 ? b1 - b2 : 0;
        a = use_alpha ? (a1 > a2 ? a1 - a2 : 0) : a1;
    }

    target_buf[gid] = (r << 24) | (g << 16) | (b << 8) | a;
}
