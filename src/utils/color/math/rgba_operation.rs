
/// Operations on RGBA color values
pub enum ColorOperation {
    Add,
    Subtract,
}

/// Allows adding or subtracting RGBA color values, channel by channel.
/// - `c1` 1st color operand (in case of subtraction, `c2` is subtracted from `c1`)
/// - `c2` 2nd color operand
/// - `op` operation to apply to `c1` and `c2`
/// - `use_alpha` if false, the alpha channel is ignored (to save CPU cycles), `ctx.use_alpha` is a good default here.
pub fn rgba_operation(c1: &u32, c2: &u32, op: &ColorOperation, use_alpha: bool) -> u32 {
    let r = match op {
        ColorOperation::Add => {
            let result = ((c1 >> 24) & 0xff).wrapping_add((c2 >> 24) & 0xff);
            result.min(0xff) as u32
        }
        ColorOperation::Subtract => {
            let result = ((c1 >> 24) & 0xff) as i32 - ((c2 >> 24) & 0xff) as i32;
            (result.max(0) & 0xff) as u32
        }
    };

    let g = match op {
        ColorOperation::Add => {
            let result = ((c1 >> 16) & 0xff).wrapping_add((c2 >> 16) & 0xff);
            result.min(0xff) as u32
        }
        ColorOperation::Subtract => {
            let result = ((c1 >> 16) & 0xff) as i32 - ((c2 >> 16) & 0xff) as i32;
            (result.max(0) & 0xff) as u32
        }
    };

    let b = match op {
        ColorOperation::Add => {
            let result = ((c1 >> 8) & 0xff).wrapping_add((c2 >> 8) & 0xff);
            result.min(0xff) as u32
        }
        ColorOperation::Subtract => {
            let result = ((c1 >> 8) & 0xff) as i32 - ((c2 >> 8) & 0xff) as i32;
            (result.max(0) & 0xff) as u32
        }
    };

    let mut a = c1 & 0xff;
    if use_alpha {
        a = match op {
            ColorOperation::Add => {
                let result = (c1 & 0xff).wrapping_add(c2 & 0xff);
                result.min(0xff) as u32
            }
            ColorOperation::Subtract => {
                let result = (c1 & 0xff) as i32 - (c2 & 0xff) as i32;
                (result.max(0) & 0xff) as u32
            }
        };
    }
    // Recombine channels into a single u32
    (r << 24) | (g << 16) | (b << 8) | a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_colors() {
        let color1 = 0x11_22_33_ff;
        let color2 = 0x22_55_11_ff;
        assert_eq!(
            rgba_operation(&color1, &color2, &ColorOperation::Add, true),
            0x33_77_44_ff
        );

        let color1 = 0x22_aa_22_33;
        let color2 = 0x33_99_55_ee;
        assert_eq!(
            rgba_operation(&color1, &color2, &ColorOperation::Add, true),
            0x55_ff_77_ff
        );
    }
    #[test]
    fn test_add_colors_no_alpha() {
        let color1 = 0x11_22_33_ff;
        let color2 = 0x22_55_11_ff;
        assert_eq!(
            rgba_operation(&color1, &color2, &ColorOperation::Add, false),
            0x33_77_44_ff
        );

        let color1 = 0x22_aa_22_33;
        let color2 = 0x33_99_55_ee;
        assert_eq!(
            rgba_operation(&color1, &color2, &ColorOperation::Add, false),
            0x55_ff_77_33
        );
    }

    #[test]
    fn test_subtract_colors() {
        let color1 = 0x50_70_90_ff;
        let color2 = 0x10_20_30_ff;
        assert_eq!(
            rgba_operation(&color1, &color2, &ColorOperation::Subtract, true),
            0x40_50_60_00
        );

        let color1 = 0x10_20_00_00;
        let color2 = 0x50_60_ff_ff;
        assert_eq!(
            rgba_operation(&color1, &color2, &ColorOperation::Subtract, true),
            0x0
        );
    }

    #[test]
    fn test_subtract_colors_no_alpha() {
        let color1 = 0x50_70_90_ff;
        let color2 = 0x10_20_30_ff;
        assert_eq!(
            rgba_operation(&color1, &color2, &ColorOperation::Subtract, false),
            0x40_50_60_ff
        );

        let color1 = 0x10_20_00_11;
        let color2 = 0x50_60_ff_ff;
        assert_eq!(
            rgba_operation(&color1, &color2, &ColorOperation::Subtract, false),
            0x00_00_00_11
        );
    }
}
