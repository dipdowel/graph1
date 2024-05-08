use crate::animation_context::AnimationContext;
use crate::graph1::draw::curves::draw_bezier_curve;
use crate::graph1::graph1_core::context::GraphContext;
use crate::graph1::primitives::primitives::{Point, PointF32};

pub fn render(ctx: &mut GraphContext,
              ani_ctx: &AnimationContext){

    let oscillator  = ani_ctx.oscillators.o1;

    let res_delta: f32 = 0.05;

    // Usage of the continuous Bezier curve!
    let closed_curve = vec![
        Point { x: 50, y: 50 },
        Point { x: 150, y: 150 },
        Point { x: 300, y: 100 },
        Point { x: 350, y: 200 },
        Point { x: 400, y: 400 },
        Point { x: 300, y: 100 },
        Point { x: 150, y: 100 },
        Point { x: 100, y: 140 },
        Point { x: 190, y: 190 },
        Point { x: 50, y: 50 },
    ];
    ctx.default_color = 0x00_ff_00_ff;
    draw_bezier_curve(ctx, &closed_curve, &res_delta);

    // pub fn draw_bezier_curve(ctx: &mut GraphContext, p0: &Point, p1: &Point, p2: &Point, p3: &Point) {
    //******************************************************************************************
    //******************************************************************************************
    let res_delta: f32 = 0.05;

    let string_curve = vec![
        Point { x: 0, y: 0 },
        Point { x: 250, y: 250 - (oscillator) as u32, },
        Point { x: 350 + oscillator as u32, y: 350 + oscillator as u32,},
        Point { x: 800, y: 600 },
    ];
    ctx.default_color = 0x00_ff_ff_ff;
    draw_bezier_curve( ctx, &string_curve, &res_delta);




    let cx = 30_f32 + (oscillator * 5) as f32; // Center x
    let cy = 300_f32  + oscillator as f32; // Center y
    let r = 150_f32 + 1.5 * oscillator as f32; // Radius
    let k = 4.0 / 3.0 * (2.0_f32.sqrt() - 1.0);


    #[rustfmt::skip]
        let circle_points:Vec<Point> = vec![
        // First Quadrant
        Point::from(PointF32{ x: cx + r,     y: cy }),
        Point::from(PointF32{ x: cx + r,     y: cy + r * k }),
        Point::from(PointF32 { x: cx + r * k, y: cy + r }),
        Point::from(PointF32 { x: cx,         y: cy + r }),
        // Second Quadrant
        Point::from(PointF32{ x: cx,         y: cy + r }),
        Point::from(PointF32{ x: cx - r * k, y: cy + r }),
        Point::from(PointF32{ x: cx - r,     y: cy + r * k }),
        Point::from(PointF32{ x: cx - r,     y: cy }),
        // Third Quadrant
        Point::from(PointF32             { x: cx - r,     y: cy }),
        Point::from(PointF32{ x: cx - r,     y: cy - r * k }),
        Point::from(PointF32{ x: cx - r * k, y: cy - r }),
        Point::from(PointF32{ x: cx,         y: cy - r }),
        // Fourth Quadrant
        Point::from(PointF32 { x: cx,         y: cy - r }),
        Point::from(PointF32 { x: cx + r * k, y: cy - r }),
        Point::from(PointF32 { x: cx + r,     y: cy - r * k }),
        Point::from(PointF32 { x: cx + r,     y: cy }),
    ];
    ctx.default_color = 0x00_00_ff_00;
    draw_bezier_curve( ctx, &circle_points[0..4], &res_delta);
    draw_bezier_curve( ctx, &circle_points[4..8], &res_delta);
    draw_bezier_curve( ctx, &circle_points[8..12], &res_delta);
    draw_bezier_curve( ctx, &circle_points[12..16], &res_delta);


}