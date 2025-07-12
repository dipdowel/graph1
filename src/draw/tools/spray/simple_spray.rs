use crate::core::context::GraphContext;
use crate::draw::tools::brush::Brush;
use crate::draw::tools::spray::circular::circular_spray;
use crate::draw::tools::spray::rectangular::rectangular_spray;
use crate::primitives::point::Point;

pub fn simple<UserData>(
    ctx: &mut GraphContext<UserData>,
    center: &Point,
    density: u32,
    colors: &[u32],
) {
    // TODO: Consider adding GPU support for spray!
    // TODO: Consider adding support for Alpha!

    match ctx.brush {
        Brush::Circle { radius } => {
            circular_spray(ctx, &center, density, colors, radius);
        }
        Brush::Rectangle { size } => {
            rectangular_spray(ctx, &center, density, colors, &size);
        }
    }
}

//
//
//
// pub fn simple_gpu<UserData>(
//     ctx: &mut GraphContext<UserData>,
//     x: u32,
//     y: u32,
//     density: u32,
//     colors:&Vec<u32>
// ) {
//
//     match ctx.brush {
//         Brush::Circle { radius } => {
//             println!("! NOT IMPLEMENTED ! Circle with radius: {}", radius);
//         }
//         Brush::Rectangle { size } =>  {
//             /*
//
//             let random_xs = ctx.rng.get_vec_u32(density as usize, &MinMax{min: 0, max: size.w - 1});
//             let random_ys = ctx.rng.get_vec_u32(density as usize, &MinMax{min: 0, max: size.h - 1});
//
//             let mut color_count:usize = 0;
//             let colors_len:usize = colors.len();
//
//             random_xs.iter().zip(random_ys.iter()).for_each(|(&rnd_x, &rnd_y)| {
//
//                 let rnd_x:i32 = rnd_x as i32 - size.w as i32 / 2;
//                 let rnd_y:i32 = rnd_y as i32 - size.h as i32 / 2;
//                 ctx.set_pixel((x as i32 + rnd_x) as u32  , (y as i32 + rnd_y) as u32, colors[color_count % colors_len]);
//
//                 color_count+= 1;
//
//             });
//              */
//             ////////////////////////////////////////////////////////////////////////////////////////
//             ////////////////////////////////////////////////////////////////////////////////////////
//
//
//             if ctx.gpu_context.enabled {
//                 // println!("! GPU ATTEMPT !");
//                 use ocl::{Buffer, Kernel};
//
//                 // 1. OpenCL kernel source: generates random positions and color indices
//                 let kernel_src = r#"
//         inline uint xorshift32(uint x) {
//             x ^= x << 13;
//             x ^= x >> 17;
//             x ^= x << 5;
//             return x;
//         }
//
//         __kernel void fill_random_coords(
//             const uint seed,
//             const uint count,
//             const int center_x,
//             const int center_y,
//             const int w,
//             const int h,
//             const uint colors_len,
//             __global uint* out_coords // triplets: x, y, color_idx
//         ) {
//             uint gid = get_global_id(0);
//             if (gid >= count) return;
//
//             uint rnd = xorshift32(seed ^ gid);
//             uint rx = rnd % w;
//             rnd = xorshift32(rnd);
//             uint ry = rnd % h;
//             rnd = xorshift32(rnd);
//             uint color_idx = gid % colors_len;
//
//             int rnd_x = (int)rx - (w / 2);
//             int rnd_y = (int)ry - (h / 2);
//             uint x_out = (uint)((int)center_x + rnd_x);
//             uint y_out = (uint)((int)center_y + rnd_y);
//
//             out_coords[gid * 3 + 0] = x_out;
//             out_coords[gid * 3 + 1] = y_out;
//             out_coords[gid * 3 + 2] = color_idx;
//         }
//     "#;
//
//                 let program_name = "fill_random_coords";
//                 if ctx.gpu_context.get_program(program_name).is_none() {
//                     ctx.gpu_context
//                         .load_program(kernel_src, program_name)
//                         .expect("Failed to build OpenCL program");
//                 }
//                 let program = ctx.gpu_context.get_program(program_name).unwrap();
//
//                 let queue = ctx.gpu_context.queue.as_ref().unwrap().as_ref();
//                 let len = density as usize;
//                 let mut out_coords = vec![0u32; len * 3];
//                 let colors_len = colors.len() as u32;
//
//                 // 2. Create OpenCL buffer for output
//                 let out_buf = Buffer::<u32>::builder()
//                     .queue(queue.clone())
//                     .flags(ocl::flags::MEM_WRITE_ONLY)
//                     .len(len * 3)
//                     .build()
//                     .expect("Failed to create OpenCL buffer for output");
//
//                 // 3. Launch kernel
//                 let kernel = Kernel::builder()
//                     .program(&program)
//                     .name("fill_random_coords")
//                     .queue(queue.clone())
//                     .global_work_size(len)
//                     .arg(ctx.frame_count as u32) // Random seed, can also use real randomness
//                     .arg(len as u32)
//                     .arg(x as i32)
//                     .arg(y as i32)
//                     .arg(size.w as i32)
//                     .arg(size.h as i32)
//                     .arg(colors_len)
//                     .arg(&out_buf)
//                     .build()
//                     .expect("Failed to build kernel");
//                 unsafe { kernel.enq().expect("Failed to enqueue kernel"); }
//
//                 // 4. Read results back
//                 out_buf.read(&mut out_coords).enq().unwrap();
//
//                 // 5. Set pixels on the CPU (reuse your logic)
//                 for i in 0..len {
//                     let rnd_x = out_coords[i * 3 + 0];
//                     let rnd_y = out_coords[i * 3 + 1];
//                     let color_idx = out_coords[i * 3 + 2] as usize % colors.len();
//                     ctx.set_pixel(rnd_x, rnd_y, colors[color_idx]);
//                 }
//             } else {
//                 // CPU fallback, your original code
//                 let random_xs = ctx.rng.get_vec_u32(density as usize, &MinMax{min: 0, max: size.w - 1});
//                 let random_ys = ctx.rng.get_vec_u32(density as usize, &MinMax{min: 0, max: size.h - 1});
//
//                 let mut color_count:usize = 0;
//                 let colors_len:usize = colors.len();
//
//                 random_xs.iter().zip(random_ys.iter()).for_each(|(&rnd_x, &rnd_y)| {
//
//                     let rnd_x:i32 = rnd_x as i32 - size.w as i32 / 2;
//                     let rnd_y:i32 = rnd_y as i32 - size.h as i32 / 2;
//                     ctx.set_pixel((x as i32 + rnd_x) as u32  , (y as i32 + rnd_y) as u32, colors[color_count % colors_len]);
//
//                     color_count+= 1;
//
//                 });
//             }
//
//             ////////////////////////////////////////////////////////////////////////////////////////
//             ////////////////////////////////////////////////////////////////////////////////////////
//
//
//         }
//     }
//
//
//
//     // let alpha_context = ctx.alpha_context();
//     // let alpha_enabled = alpha_context.enabled;
//     // let alpha_method = alpha_context.method;
//
//     // Use the context's default brush to draw the spray
//     // ctx.default_brush().spray(
//     //     x, y, radius, color, alpha_enabled, alpha_method,
//     // );
// }
//
// /*
// pub fn simple_buffer(
//     buf: &mut [u32],
//     buf_dimensions: &Dimensions2d,
//     rng: &mut XorShiftRng,
//     x: u32,
//     y: u32,
//     brush: &Brush,
//     density: u32,
//     color: u32
// ) {
//     // Use the context's default brush to draw the spray
//     let alpha_context = AlphaContext::default();
//     let alpha_enabled = alpha_context.enabled;
//     let alpha_method = alpha_context.method;
//
//     crate::brushes::default_brush().spray(
//         buf, buf_dimensions, x, y, radius, color, alpha_enabled, alpha_method,
//     );
// }
//
// */
