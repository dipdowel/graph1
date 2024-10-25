/// Converts the source buffer `src` from RGBA to ABGR and writes the result to `dst`.
/// Can help converting between Rust's native RGBA format (u32) and the ABGR format understood by JS and HTML Canvas.
/// # Arguments
///
/// * `dst` - A mutable slice where the converted ABGR pixels will be stored.
/// * `src` - A slice of RGBA pixels to convert, where each pixel is a `u32`.
///
/// # Panics
///
/// Panics if `dst` and `src` have different lengths.
pub fn rgba_to_abgr(dst: &mut [u32], src: &[u32]) {
    assert_eq!(dst.len(), src.len(), "Source and destination buffers must have the same length!");

    for (dst_pixel, &src_pixel) in dst.iter_mut().zip(src.iter()) {
        // Extract individual color channels from RGBA
        let r = (src_pixel >> 24) & 0xFF;
        let g = (src_pixel >> 16) & 0xFF;
        let b = (src_pixel >> 8) & 0xFF;
        let a = src_pixel & 0xFF;

        // Reassemble in ABGR format and store in dst
        *dst_pixel = (a << 24) | (b << 16) | (g << 8) | r;
    }
}