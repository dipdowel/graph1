use crate::core::context::GraphContext;
use crate::draw::tools::brush::Brush;
use crate::primitives::math::MinMax;

pub fn simple<UserData>(
    ctx: &mut GraphContext<UserData>,
    x: u32,
    y: u32,
    density: u32,
    colors:&Vec<u32>
) {
    
    // TODO: Consider adding support for Alpha!
    // TODO: Consider adding support for Alpha!
    // TODO: Consider adding support for Alpha!
    // TODO: Consider adding support for Alpha!
    
    match ctx.brush {
        Brush::Circle { radius } => {
            println!("! NOT IMPLEMENTED ! Circle with radius: {}", radius);
        }
        Brush::Rectangle { size } =>  {
            println!("Rectangle with dimensions: {:?}", size);

            let random_xs = ctx.rng.get_vec_u32(density as usize, &MinMax{min: 0, max: size.w - 1});
            let random_ys = ctx.rng.get_vec_u32(density as usize, &MinMax{min: 0, max: size.h - 1});

            let mut color_count:usize = 0;
            let colors_len:usize = colors.len();

            random_xs.iter().zip(random_ys.iter()).for_each(|(&rnd_x, &rnd_y)| {

                let rnd_x:i32 = rnd_x as i32 - size.w as i32 / 2;
                let rnd_y:i32 = rnd_y as i32 - size.h as i32 / 2;
                ctx.set_pixel((x as i32 + rnd_x) as u32  , (y as i32 + rnd_y) as u32, colors[color_count % colors_len]);

                color_count+= 1;

            });
        }
    }



    // let alpha_context = ctx.alpha_context();
    // let alpha_enabled = alpha_context.enabled;
    // let alpha_method = alpha_context.method;

    // Use the context's default brush to draw the spray
    // ctx.default_brush().spray(
    //     x, y, radius, color, alpha_enabled, alpha_method,
    // );
}

/*
pub fn simple_buffer(
    buf: &mut [u32],
    buf_dimensions: &Dimensions2d,
    rng: &mut XorShiftRng,
    x: u32,
    y: u32,
    brush: &Brush,
    density: u32,
    color: u32
) {
    // Use the context's default brush to draw the spray
    let alpha_context = AlphaContext::default();
    let alpha_enabled = alpha_context.enabled;
    let alpha_method = alpha_context.method;

    crate::brushes::default_brush().spray(
        buf, buf_dimensions, x, y, radius, color, alpha_enabled, alpha_method,
    );
}

 */