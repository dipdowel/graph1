use crate::animation_context::AnimationContext;
use crate::graph1::draw::star;
use crate::graph1::draw::star::StarProperties;
use crate::graph1::graph1_core::context::GraphContext;
use crate::graph1::primitives::primitives::Pixel;
use crate::graph1::utils::color_math::argb_math::argb_math;
use crate::graph1::utils::color_math::operations::ColorOperation;

pub fn render(ctx: &mut GraphContext,
              ani_ctx: &AnimationContext){

    let oscillator  = ani_ctx.oscillators.o1;
    let frame_count = ani_ctx.frame_count;
    ///////////// STAR START ////////////////////

    // Example usage
    let mut center_pixel = Pixel {
        x: 300 + oscillator as u32,
        y: 300 - oscillator as u32,
        color: 0xff_00_ff_dd,
    };
    center_pixel.color = argb_math(
        &0xff_ff_ff_ff,
        &(frame_count * oscillator as u32),
        &ColorOperation::Subtract,
    );

    let vertex_num = frame_count % 60;

    let star_props_1 = StarProperties {
        center: center_pixel,
        num_vertices: vertex_num, // e.g., a 10-point star
        outer_radius: 50 + (oscillator / 2) as u32,
        inner_radius: 40,
        rotation_angle: frame_count as f64 * 1.5,
    };

    star::render(ctx, &star_props_1);

    center_pixel.color = argb_math(
        &0xff_22_22_99,
        &(frame_count * oscillator as u32),
        &ColorOperation::Add,
    );
    let star_props_2 = StarProperties {
        center: center_pixel,
        num_vertices: vertex_num, // e.g., a 10-point star
        outer_radius: 40 + (oscillator / 4) as u32,
        inner_radius: 30,
        rotation_angle: frame_count as f64 * 1.5,
    };

    star::render(ctx, &star_props_2);

    ///////////// STAR END ////////////////////


}