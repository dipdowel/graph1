__kernel void horizontal_lines_x3(
    __global uint *buf,
    uint buf_width,
    uint buf_height,
    __global const uint *lines,
    uint num_lines
) {
    uint gid = get_global_id(0);
    if (gid >= num_lines) return;

    // Each line: [color, x_start, x_end, y]
    uint base = gid * 4;
    uint color   = lines[base + 0];
    uint x_start = lines[base + 1];
    uint x_end   = lines[base + 2];
    uint y       = lines[base + 3];

    if (y >= buf_height) return;

    uint max_x = buf_width - 1;
    x_start = min(max_x, x_start);
    x_end = min(max_x, x_end);
    if (x_start > x_end) return;

    uint buf_offset = y * buf_width;
    for (uint x = x_start; x <= x_end; ++x) {
        buf[buf_offset + x] = color;
    }
}
