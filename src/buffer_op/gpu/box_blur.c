__kernel void box_blur_kernel(
    __global uint* buf,
    uint width,
    uint height,
    uint radius
) {
    uint gid = get_global_id(0);
    uint x = gid % width;
    uint y = gid / width;

    if (x >= width || y >= height) return;

    uint sum_r = 0, sum_g = 0, sum_b = 0, sum_a = 0;
    uint count = 0;

    for (int ky = -((int)radius); ky <= (int)radius; ++ky) {
        int ny = clamp((int)y + ky, 0, (int)(height - 1));
        for (int kx = -((int)radius); kx <= (int)radius; ++kx) {
            int nx = clamp((int)x + kx, 0, (int)(width - 1));
            uint idx = ny * width + nx;
            uint pixel = buf[idx];
            sum_r += (pixel >> 24) & 0xFF;
            sum_g += (pixel >> 16) & 0xFF;
            sum_b += (pixel >> 8) & 0xFF;
            sum_a += pixel & 0xFF;
            ++count;
        }
    }

    uint avg_r = (sum_r / count) & 0xFF;
    uint avg_g = (sum_g / count) & 0xFF;
    uint avg_b = (sum_b / count) & 0xFF;
    uint avg_a = (sum_a / count) & 0xFF;

    buf[gid] = (avg_r << 24) | (avg_g << 16) | (avg_b << 8) | avg_a;
}
