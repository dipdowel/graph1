__kernel void fade_kernel(
    __global uint* buf,
    uint width,
    uint height,
    uint color_operand,
    int op_code, // 0 = Add, 1 = Subtract
    int use_alpha
) {
    uint gid = get_global_id(0);
    if (gid >= width * height) return;

    uint px = buf[gid];

    uint r1 = (px >> 24) & 0xFF;
    uint g1 = (px >> 16) & 0xFF;
    uint b1 = (px >> 8) & 0xFF;
    uint a1 = px & 0xFF;

    uint r2 = (color_operand >> 24) & 0xFF;
    uint g2 = (color_operand >> 16) & 0xFF;
    uint b2 = (color_operand >> 8) & 0xFF;
    uint a2 = color_operand & 0xFF;

    int r_sum = (int)r1 + (int)r2;
    int g_sum = (int)g1 + (int)g2;
    int b_sum = (int)b1 + (int)b2;
    int a_sum = (int)a1 + (int)a2;

    int r_diff = (int)r1 - (int)r2;
    int g_diff = (int)g1 - (int)g2;
    int b_diff = (int)b1 - (int)b2;
    int a_diff = (int)a1 - (int)a2;

    int rf, gf, bf, af;

    if (op_code == 0) { // Add
        rf = r_sum > 255 ? 255 : r_sum;
        gf = g_sum > 255 ? 255 : g_sum;
        bf = b_sum > 255 ? 255 : b_sum;
        af = use_alpha ? (a_sum > 255 ? 255 : a_sum) : a1;
    } else { // Subtract
        rf = r_diff < 0 ? 0 : r_diff;
        gf = g_diff < 0 ? 0 : g_diff;
        bf = b_diff < 0 ? 0 : b_diff;
        af = use_alpha ? (a_diff < 0 ? 0 : a_diff) : a1;
    }

    buf[gid] = ((uint)rf << 24) | ((uint)gf << 16) | ((uint)bf << 8) | ((uint)af);
}
