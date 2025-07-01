__kernel void fill_rects_buckets(
    __global uint *out_buf,
    uint width, uint height,
    __global const uint *flat_rects,
    __global const uint *bucket_offsets,
    __global const uint *bucket_counts,
    uint n_buckets,
    uint bucket_w
) {
    uint gid = get_global_id(0);
    if (gid >= width * height) return;

    uint x = gid % width;
    uint y = gid / width;

    uint bucket = x / bucket_w;
    if (bucket >= n_buckets) bucket = n_buckets - 1;

    uint offset = bucket_offsets[bucket];
    uint count  = bucket_counts[bucket];

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
