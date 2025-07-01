__kernel void horizontal_lines_y_grouped(
    __global uint* buf,
    uint buf_width,
    uint buf_height,
    __global const uint* scanline_data,
    __global const uint* scanline_pointers,
    __global const uint* lookup_table
) {
    uint x = get_global_id(0);
    uint y = get_global_id(1);
    if (x >= buf_width || y >= buf_height) return;

    buf[y * buf_width + x] = x*y/4;

// TODO: DELETE THIS FILE AND ITS RUST WRAPPER!
// TODO: DELETE THIS FILE AND ITS RUST WRAPPER!
// TODO: DELETE THIS FILE AND ITS RUST WRAPPER!
// TODO: DELETE THIS FILE AND ITS RUST WRAPPER!
// TODO: DELETE THIS FILE AND ITS RUST WRAPPER!
// TODO: DELETE THIS FILE AND ITS RUST WRAPPER!
// TODO: DELETE THIS FILE AND ITS RUST WRAPPER!
// TODO: DELETE THIS FILE AND ITS RUST WRAPPER!

//    // Find all segments for this y
//    uint seg_start = lookup_table[y];
//    uint seg_end = lookup_table[y + 1];
//
//    for (uint idx = seg_start; idx < seg_end; ++idx) {
//        uint ptr = scanline_pointers[idx];
//
//        // Defensive: Ensure ptr is not out of scanline_data bounds
//        // Each scanline_data[ptr] = y (redundant), scanline_data[ptr+1] = color
//        // scanline_data[ptr+2..] = x_start, x_end, [x_start, x_end, ...]
//        uint record_y = scanline_data[ptr];
//        if (record_y != y) continue; // Defensive check
//
//        uint color = scanline_data[ptr + 1];
//        // How many segments in this scanline? You need to encode this somewhere.
//        // Let's assume scanline_data[ptr+2] = seg_count:
//        uint seg_count = scanline_data[ptr + 2];
//
//        // Each segment: [x_start, x_end], starting at ptr+3
//        for (uint s = 0; s < seg_count; ++s) {
//            uint base = ptr + 3 + s * 2;
//            uint x_start = scanline_data[base];
//            uint x_end   = scanline_data[base + 1];
//            if (x >= x_start && x <= x_end) {
//                buf[y * buf_width + x] = color;
//                // break; // If "first match wins"
//            }
//        }
//    }
}
