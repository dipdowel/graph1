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
    uint x = get_global_id(0);
    uint y = get_global_id(1);

    if (x >= rect_x && x < rect_x + rect_w &&
        y >= rect_y && y < rect_y + rect_h) {

        uint idx = y * buf_width + x;
        buf[idx] = color;
    }
}
