__kernel void fill_buffer(__global uint *buf, uint value, uint len) {
    uint gid = get_global_id(0);
    if (gid < len) buf[gid] = value;
}
