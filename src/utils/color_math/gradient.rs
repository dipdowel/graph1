/// Calculates a simple gradient between two 0RGB colors.
///
/// # Arguments
/// * `c1` - The starting color (0RGB)
/// * `c2` - The ending color (0RGB)
/// * `steps` - The number of steps in the gradient.
///
/// # Returns
/// A vector of colors in 0x00_RR_GG_BB format representing the gradient.
pub fn simple(c1: u32, c2: u32, steps: usize) -> Vec<u32> {
    assert!(steps > 1, "There must be at least two steps in the gradient.");

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


/// Calculates and returns a specified step (color) in the gradient between two 0RGB colors.
///
/// # Arguments
/// * `c1` - The starting color (0RGB)
/// * `c2` - The ending color (0RGB)
/// * `steps` - The number of steps in the gradient.
/// * `step` - The specific step in the gradient to calculate.
///
/// # Returns
/// A color in 0x00_RR_GG_BB format representing the color at the specified step.
pub fn single_step(c1: u32, c2: u32, steps: usize, step: usize) -> u32 {
    assert!(steps > 1, "There must be at least two steps in the gradient.");
    assert!(step < steps, "Step must be within the range of steps.");

    // Extract the R, G, B components from both of the colors
    let r1 = ((c1 >> 16) & 0xFF) as f32;
    let g1 = ((c1 >> 8) & 0xFF) as f32;
    let b1 = (c1 & 0xFF) as f32;

    let r2 = ((c2 >> 16) & 0xFF) as f32;
    let g2 = ((c2 >> 8) & 0xFF) as f32;
    let b2 = (c2 & 0xFF) as f32;

    // Calculate the step increments for each component
    let r_step = (r2 - r1) / (steps - 1) as f32;
    let g_step = (g2 - g1) / (steps - 1) as f32;
    let b_step = (b2 - b1) / (steps - 1) as f32;

    // Calculate the specific color at the given step
    let r = (r1 + r_step * step as f32) as u32;
    let g = (g1 + g_step * step as f32) as u32;
    let b = (b1 + b_step * step as f32) as u32;

    // Combine the components back into a single color
    let color = (0x00 << 24) | (r << 16) | (g << 8) | b;

    color
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_and_single_step() {
        let c1: u32 = 0x00_11_22_33;
        let c2: u32 = 0x00_44_55_66;
        let gradient = simple(c1, c2, 10);
        // println!(">>> c1: {:#010X}, c2: {:#010X}",c1,c2);

        for i in 0..10 {
            let color = single_step(c1, c2, 10, i);
            assert_eq!(color, gradient[i]);
            // println!("color: {:#010X}, gradient color  {:#010X}",color, gradient[i]);
        }

    }

}
