__kernel void draw_rectangle(
    __global uint *buf,
    const uint buf_width,
    const uint buf_height,
    const uint rect_x,
    const uint rect_y,
    const uint rect_w,
    const uint rect_h,
    const uint color
) {
    uint local_x = get_global_id(0);
    uint local_y = get_global_id(1);

    uint x = rect_x + local_x;
    uint y = rect_y + local_y;

    if (x < buf_width && y < buf_height) {
        uint idx = y * buf_width + x;
        buf[idx] = color;
    }
}
