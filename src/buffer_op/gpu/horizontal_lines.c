// a kinda okay working kernel
__kernel void scanlines(
    __global uint *buf,
    uint buf_width,
    uint buf_height,
    __global const uint *flat_scanline_data,
    __global const uint *flat_data_ptrs,
    __global const uint *scanline_sizes
) {
    uint gid = get_global_id(0);
    if (gid >= buf_width * buf_height) return;

    uint x = gid % buf_width;
    uint y = gid / buf_width;

    uint size = scanline_sizes[y];
    if (size == 0) return;

    uint ptr = flat_data_ptrs[y];


    // Iterate segments from the end (last wins, and return immediately)
    for (int i = size - 3; i >= 0; i -= 3) {
        uint x_start = flat_scanline_data[ptr + i];
        uint x_end   = flat_scanline_data[ptr + i + 1];
        if (x >= x_start && x <= x_end) {
            buf[gid] = flat_scanline_data[ptr + i + 2]; // this sets color
            return;
        }
    }
    // else: leave pixel as is
}