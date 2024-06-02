/// Takes in a vector,  divides it into 4 equal parts and returns those parts
/// Panics if vector length cannot be divided into 4 equal parts
pub fn slice_buffer_in_4(
    buffer: &mut Vec<u32>,
) -> (&mut [u32], &mut [u32], &mut [u32], &mut [u32]) {
    if buffer.len() % 4 != 0 {
        panic!("`buffer` length must be a multiple of 4!");
    }

    let chunk_size: usize = buffer.len() / 4;

    // Split the buffer to avoid borrowing conflicts
    let (first_half, second_half) = buffer.split_at_mut(chunk_size * 2);
    let (buf_view_1, buf_view_2) = first_half.split_at_mut(chunk_size);
    let (buf_view_3, buf_view_4) = second_half.split_at_mut(chunk_size);

    (buf_view_1, buf_view_2, buf_view_3, buf_view_4)
}
