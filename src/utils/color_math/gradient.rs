/// Calculates a simple gradient between two 0RGB colors.
///
/// # Arguments
/// * `c1` - The starting color (0RGB)
/// * `c2` - The ending color (0RGB)
/// * `steps` - The number of steps in the gradient.
///
/// # Returns
/// A vector of colors in 0xAARRGGBB format representing the gradient.
pub fn simple(c1: u32, c2: u32, steps: usize) -> Vec<u32> {

    // Extract the R, G, B components from both of the colors
    let r1 = ((c1 >> 16) & 0xFF) as f32;
    let g1 = ((c1 >> 8) & 0xFF) as f32;
    let b1 = (c1 & 0xFF) as f32;

    let r2 = ((c2 >> 16) & 0xFF) as f32;
    let g2 = ((c2 >> 8) & 0xFF) as f32;
    let b2 = (c2 & 0xFF) as f32;

    // Vector for the calculated gradient
    let mut gradient = Vec::with_capacity(steps);

    // Calculate the step increments for each component
    let r_step = (r2 - r1) / (steps - 1) as f32;
    let g_step = (g2 - g1) / (steps - 1) as f32;
    let b_step = (b2 - b1) / (steps - 1) as f32;

    // Generate the gradient
    for i in 0..steps {
        let r = (r1 + r_step * i as f32) as u32;
        let g = (g1 + g_step * i as f32) as u32;
        let b = (b1 + b_step * i as f32) as u32;

        // Combine the components back into a single color and add to the gradient
        let color = (0x00 << 24) | (r << 16) | (g << 8) | b;
        gradient.push(color);
    }

    // Debug the gradient colors
    // for &num in &gradient {
    //     println!("{:#010X}", num);
    // }
    gradient
}

