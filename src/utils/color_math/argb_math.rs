use super::operations::ColorOperation;

/// Allows adding or subtracting ARGB color values, color by color.
/// - `c1` 1st color operand (in case of subtraction, `c2` is subtracted from `c1`
/// - `c2` 2nd color operand
/// - `op` operation to apply to `c1` and `c2`
pub fn argb_math(c1: &u32, c2: &u32, op: &ColorOperation) -> u32 {

        let a = match op {
            ColorOperation::Add =>{
                let result = ((c1 >> 24) & 0xff).wrapping_add((c2 >> 24) & 0xff);
                (result.min(0xff) ) as u32
            },
            ColorOperation::Subtract => {
                let result = ((c1 >> 24) & 0xff) as i32 - ((c2 >> 24) & 0xff) as i32;
                (result.max(0) & 0xff) as u32
            },
        };
        let r = match op {
            ColorOperation::Add => {
                let result = ((c1 >> 16) & 0xff).wrapping_add((c2 >> 16) & 0xff);
                (result.min(0xff) ) as u32
            },
            ColorOperation::Subtract => {
                let result = ((c1 >> 16) & 0xff) as i32 - ((c2 >> 16) & 0xff) as i32;
                (result.max(0) & 0xff) as u32
            },
        };
        let g = match op {
            ColorOperation::Add => {let result = ((c1 >> 8) & 0xff).wrapping_add((c2 >> 8) & 0xff);
                (result.min(0xff) ) as u32
            },
            ColorOperation::Subtract => {
                let result = ((c1 >> 8) & 0xff) as i32 - ((c2 >> 8) & 0xff) as i32;
                (result.max(0) & 0xff) as u32
            },
        };
        let b = match op {
            ColorOperation::Add => {let result = (c1 & 0xff).wrapping_add(c2 & 0xff);
                (result.min(0xff) ) as u32},
            ColorOperation::Subtract => {
                let result = (c1 & 0xff) as i32 - (c2 & 0xff) as i32;
                (result.max(0) & 0xff) as u32
            },
        };

        // Recombine using bitwise OR, no checks
        (a << 24) | (r << 16) | (g << 8) | b

}




// Unit tests module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_colors() {
        let color1 = 0xff_11_22_33;
        let color2 = 0xff_22_55_11;
        assert_eq!(argb_math(&color1, &color2, &ColorOperation::Add), 0xff_33_77_44);

        let color1 = 0x22_aa_22_33;
        let color2 = 0x33_99_55_ee;
        assert_eq!(argb_math(&color1, &color2, &ColorOperation::Add), 0x55_ff_77_ff);
    }

    #[test]
    fn test_subtract_colors() {
        let color1 = 0xff_50_70_90;
        let color2 = 0xff_10_20_30;
        assert_eq!(argb_math(&color1, &color2, &ColorOperation::Subtract), 0x00_40_50_60);

        let color1 = 0x00_10_20_00;
        let color2 = 0xff_50_60_ff;
        assert_eq!(argb_math(&color1, &color2, &ColorOperation::Subtract), 0x0);
    }

}