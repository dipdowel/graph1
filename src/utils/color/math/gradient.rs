/// Calculates a simple gradient between two RGBA colors.
///
/// # Arguments
/// * `c1` - The starting color (RGBA)
/// * `c2` - The ending color (RGBA)
/// * `steps` - The number of steps in the gradient.
///
/// # Returns
/// A vector of colors as `0xRR_GG_BB_AA` representing the gradient.
pub fn simple(c1: u32, c2: u32, steps: usize) -> Vec<u32> {
    assert!(steps > 1, "There must be at least two steps in the gradient.");

    // Extract the RGBA components from both of the colors
    let r1 = ((c1 >> 24) & 0xFF) as f32;
    let g1 = ((c1 >> 16) & 0xFF) as f32;
    let b1 = ((c1 >> 8) & 0xFF) as f32;
    let a1 = (c1 & 0xFF) as f32;

    let r2 = ((c2 >> 24) & 0xFF) as f32;
    let g2 = ((c2 >> 16) & 0xFF) as f32;
    let b2 = ((c2 >> 8) & 0xFF) as f32;
    let a2 = (c2 & 0xFF) as f32;

    // Vector for the calculated gradient
    let mut gradient = Vec::with_capacity(steps);

    // Calculate the step increments for each component
    let r_step = (r2 - r1) / (steps - 1) as f32;
    let g_step = (g2 - g1) / (steps - 1) as f32;
    let b_step = (b2 - b1) / (steps - 1) as f32;
    let a_step = (a2 - a1) / (steps - 1) as f32;

    // Generate the gradient
    for i in 0..steps {
        let r = (r1 + r_step * i as f32) as u32;
        let g = (g1 + g_step * i as f32) as u32;
        let b = (b1 + b_step * i as f32) as u32;
        let a = (a1 + a_step * i as f32) as u32;

        // Combine the components back into a single color and add to the gradient
        let color = (r << 24) | (g << 16) | (b << 8) | a;
        gradient.push(color);
    }

    // Debug the gradient colors
    // for &num in &gradient {
    //     println!("{:#010X}", num);
    // }
    gradient
}


/// Calculates and returns a specified step (color) in the gradient between two RGBA colors.
///
/// # Arguments
/// * `c1` - The starting color (RGBA)
/// * `c2` - The ending color (RGBA)
/// * `steps` - The number of steps in the gradient.
/// * `step` - The specific step in the gradient to calculate.
///
/// # Returns
/// Color of the specified step of the gradient, as `0xRR_GG_BB_AA`

pub fn single_step(c1: u32, c2: u32, steps: usize, step: usize) -> u32 {
    assert!(steps > 1, "There must be at least two steps in the gradient.");
    assert!(step < steps, "Step must be within the range of steps.");

    // Extract the RGBA components from both of the colors
    let r1 = ((c1 >> 24) & 0xFF) as f32;
    let g1 = ((c1 >> 16) & 0xFF) as f32;
    let b1 = ((c1 >> 8) & 0xFF) as f32;
    let a1 = (c1 & 0xFF) as f32;

    let r2 = ((c2 >> 24) & 0xFF) as f32;
    let g2 = ((c2 >> 16) & 0xFF) as f32;
    let b2 = ((c2 >> 8) & 0xFF) as f32;
    let a2 = (c2 & 0xFF) as f32;

    // Calculate the step increments for each component
    let r_step = (r2 - r1) / (steps - 1) as f32;
    let g_step = (g2 - g1) / (steps - 1) as f32;
    let b_step = (b2 - b1) / (steps - 1) as f32;
    let a_step = (a2 - a1) / (steps - 1) as f32;

    // Calculate the specific color at the given step
    let r = (r1 + r_step * step as f32) as u32;
    let g = (g1 + g_step * step as f32) as u32;
    let b = (b1 + b_step * step as f32) as u32;
    let a = (a1 + a_step * step as f32) as u32;

    // Combine the components back into a single color
    (r << 24) | (g << 16) | (b << 8) | a
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_and_single_step() {
        let c1: u32 = 0x11_11_22_44;
        let c2: u32 = 0x11_44_88_FF;
        let gradient = simple(c1, c2, 4);
        // println!(">>> c1: {:#010X}, c2: {:#010X}",c1,c2);
        // println!(">>> gradient: {:#010X?}",gradient);
        for i in 0..4 {
            let color = single_step(c1, c2, 4, i);
            assert_eq!(color, gradient[i]);     
        }

    }


    #[test]
    fn test_simple() {
        let c1: u32 = 0x11_ff_11_11;
        let c2: u32 = 0x22_ff_88_ff;
        let gradient = simple(c1, c2, 4);
        println!(">>> c1: {:#010X}, c2: {:#010X}",c1,c2);
        println!(">>> gradient: {:#010X?}",gradient);
        assert_eq!(gradient[0], 0x11_FF_11_11);
        assert_eq!(gradient[1], 0x16_FF_38_60);
        assert_eq!(gradient[2], 0x1C_FF_60_AF);
        assert_eq!(gradient[3], 0x22_FF_88_FF);
    }
}
