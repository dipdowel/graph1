use crate::buffer_op::gpu::fill_rects::fill_rects_get_kernel;
use crate::buffer_op::gpu::horizontal_lines_y_grouped;
use crate::buffer_op::gpu::kernel_bundle::KernelBundle;
use crate::buffer_op::gpu::kernel_executor::execute_kernels_and_read;
use crate::buffer_op::lines::{horizontal_batch, horizontal_batch_threaded};
use crate::core::context::GraphContext;
use crate::core::context_utils::context_snapshot::ContextSnapshot;
use crate::draw::polygons::closed_perimeter;
use crate::primitives::numeric::Numeric;
use crate::primitives::plane::RectArea;
use crate::primitives::point::Point;
use crate::utils::math::geometry::approximate_center;
use crate::{buffer_op, draw};

/// Struct holding customizable properties of the 3D bar
/// @See `bar_3d()`.
#[derive(Debug, Clone, Copy)]
pub struct Bar3DProps {
    /// X-coordinate of the bar base
    pub x: i32,
    /// Y-coordinate of the bar base
    pub y: i32,
    /// Width of the bar front face
    pub width: i32,
    /// Height of the bar (on the screen)
    pub height: i32,
    /// Depth of the 3D bar (isometric projection)
    pub depth: i32,
    /// Color of the front face
    pub color_front: u32,
    /// Color of the top face
    pub color_top: u32,
    /// Color of the side face
    pub color_side: u32,

    /// Defines how much the top and side faces skew to simulate depth.
    /// slant.x and slant.y control horizontal and vertical offset per depth step,
    /// shaping the bar’s 3D tilt visually.
    /// Best works with low values (1..10).
    pub slant: Point<u32>,

    /// If true, the bar is projected to the right, otherwise -- to the left.
    pub project_to_right: bool,
}

/// Draws a pseudo-3D bar in isometric projection using polygons and flood fill.
/// Each face is drawn using `closed_perimeter()` and filled with `flood()`.
///
/// # Arguments
/// * `ctx` - Mutable reference to the GraphContext
/// * `props` - Configuration for the bar appearance and position
pub fn bar_3d<UserData>(ctx: &mut GraphContext<UserData>, props: &Bar3DProps) {
    let Bar3DProps {
        x,
        y,
        width,
        height,
        depth,
        color_front,
        color_top,
        color_side,
        slant,
        project_to_right: draw_left_face,
    } = *props;
    let slant_x = slant.x as i32; // how much the bar shifts right
    let slant_y = slant.y as i32; // how much the bar shifts up

    //
    // FRONT face (rectangle)
    // Front face of the bar (just a flat rectangle)
    let front = RectArea::new(x, y - height, width, height + 1, Some(color_front));
    draw::rectangle::filled(ctx, &front);

    //
    // TOP face (parallelogram) drawn using horizontal lines with slant control
    //
    // Save the current line context state to restore it after drawing the top face
    let line_ctx_state = ctx.line.get_context();
    ctx.line.set_int_no_aa(Some(1));

    let total_steps = (depth + 1) * slant_y;
    for n in 0..total_steps {
        let i = n / slant_y;
        let j = n % slant_y;

        let dx = if props.project_to_right {
            x + i * slant_x
        } else {
            x - i * slant_x
        };
        let dy = y - height - i * slant_y + j;

        let start = Point::new(dx, dy);
        draw::line::horizontal(ctx, &start, i32::to_u32(width), Some(color_top));
    }

    // Restore the original line context
    ctx.line.set_context(line_ctx_state);

    // RIGHT-SIDE or LEFT-SIDE face (slanted parallelogram)
    let side = if props.project_to_right {
        vec![
            Point::new(x + width, y - height),
            Point::new(x + width + depth * slant_x, y - height - depth * slant_y),
            Point::new(x + width + depth * slant_x, y - depth * slant_y),
            Point::new(x + width, y),
        ]
    } else {
        vec![
            Point::new(x, y - height),
            Point::new(x - depth * slant_x, y - height - depth * slant_y),
            Point::new(x - depth * slant_x, y - depth * slant_y),
            Point::new(x, y),
        ]
    };

    closed_perimeter(ctx, &side, Some(color_side));
    let flood_fill_point = approximate_center(&side).unwrap_or(side[0].clone() + Point::new(1, 1));
    buffer_op::scanline_wavefront(
        &mut ctx.frame_buf,
        &ctx.win.dimensions,
        &flood_fill_point.to_pixel(color_side),
    );
}

/*
/// Draws multiple pseudo-3D bars in isometric projection using batch rectangle drawing for the front face.
///
/// Uses `draw::rectangle::filled_multiple()` for better performance when rendering many bars.
///
/// # Arguments
/// * `ctx` - Mutable reference to the GraphContext
/// * `props` - Vector of bar configurations
pub fn bars_3d_old<UserData>(ctx: &mut GraphContext<UserData>, props: &Vec<Bar3DProps>) {
    use crate::draw;
    use crate::primitives::plane::RectArea;

    // Helper: Collect all horizontal spans for a convex quadrilateral (parallelogram)
    fn parallelogram_horizontal_spans(
        p0: Point<i32>,
        p1: Point<i32>,
        p2: Point<i32>,
        p3: Point<i32>,
        color: u32,
    ) -> Vec<u32> {
        // The points should be in order (top-left, top-right, bottom-right, bottom-left)
        let mut spans: Vec<u32> = Vec::new();
        // Find min/max y
        let min_y = p0.y.min(p1.y).min(p2.y).min(p3.y);
        let max_y = p0.y.max(p1.y).max(p2.y).max(p3.y);

        // For each scanline, find intersection with left and right edges
        for y in min_y..=max_y {
            // Left edge: (p0 to p3)
            let x_left = if p3.y != p0.y {
                p0.x + ((p3.x - p0.x) as f32 * (y - p0.y) as f32 / (p3.y - p0.y) as f32).round() as i32
            } else {
                p0.x
            };
            // Right edge: (p1 to p2)
            let x_right = if p2.y != p1.y {
                p1.x + ((p2.x - p1.x) as f32 * (y - p1.y) as f32 / (p2.y - p1.y) as f32).round() as i32
            } else {
                p1.x
            };
            let x_start = x_left.min(x_right);
            let x_end = x_left.max(x_right);
            spans.push(x_start as u32);
            spans.push(x_end as u32);
            spans.push(y as u32);
        }
        // Prepend color as first element for horizontal_lines_x3_threaded
        let mut batch = Vec::with_capacity(1 + spans.len());
        batch.push(color);
        batch.extend(spans);
        batch
    }

        // Batch all top faces and side faces as lines
        let mut top_face_lines: Vec<Vec<u32>> = Vec::with_capacity(props.len());
        let mut side_face_lines: Vec<Vec<u32>> = Vec::with_capacity(props.len());
        let mut fronts: Vec<RectArea<i32>> = Vec::with_capacity(props.len());

          for bar in props {
              let Bar3DProps {
                  x, y, width, height, depth,
                  color_front, color_top, color_side,
                  slant, project_to_right,
                  ..
              } = *bar;
              let slant_x = slant.x as i32;
              let slant_y = slant.y as i32;

              // --- Top face as parallelogram ---
              let (p0, p1, p2, p3) = if project_to_right {
                  // Right-projected: p0 = top-left, p1 = top-right, p2 = bottom-right, p3 = bottom-left
                  (
                      Point::new(x, y - height), // top-left
                      Point::new(x + width, y - height), // top-right
                      Point::new(x + width + depth * slant_x, y - height - depth * slant_y), // bottom-right
                      Point::new(x + depth * slant_x, y - height - depth * slant_y), // bottom-left
                  )
              } else {
                  // Left-projected
                  (
                      Point::new(x, y - height), // top-left
                      Point::new(x + width, y - height), // top-right
                      Point::new(x + width - depth * slant_x, y - height - depth * slant_y), // bottom-right
                      Point::new(x - depth * slant_x, y - height - depth * slant_y), // bottom-left
                  )
              };

              let spans_top:Vec<u32> = parallelogram_horizontal_spans(p0, p1, p2, p3, color_top);
              top_face_lines.push(spans_top);

              // --- Side face as parallelogram ---
              let (s0, s1, s2, s3) = if project_to_right {
                  // Right side face
                  (
                      Point::new(x + width, y - height),                        // top-left
                      Point::new(x + width + depth * slant_x, y - height - depth * slant_y), // top-right
                      Point::new(x + width + depth * slant_x, y - depth * slant_y),          // bottom-right
                      Point::new(x + width, y),                                 // bottom-left
                  )
              } else {
                  // Left side face
                  (
                      Point::new(x, y - height),                       // top-left
                      Point::new(x - depth * slant_x, y - height - depth * slant_y), // top-right
                      Point::new(x - depth * slant_x, y - depth * slant_y),          // bottom-right
                      Point::new(x, y),                                 // bottom-left
                  )
              };

              let spans_side = parallelogram_horizontal_spans(s0, s1, s2, s3, color_side);
              side_face_lines.push(spans_side);

              // println!("color_front: {color_front:#010x}, color_top: {color_top:#010x}, color_side: {color_side:#010x}");

              // --- Front face batch collect (rectangles) ---
              let front = RectArea::new(x, y - height, width, height + 1, Some(color_front));
              fronts.push(front);
          }


          // Draw all side faces in parallel


          // Draw all top faces in parallel
          draw::lines_batches::horizontal_lines_x3_threaded(ctx, &top_face_lines);

    let front_refs: Vec<&RectArea<i32>> = fronts.iter().collect();

    // Build all kernels operating on the same frame buffer
        let kernel1_res = horizontal_lines_x3_get_kernel(&mut ctx.frame_buf, &ctx.win.dimensions, &side_face_lines, &mut ctx.gpu_context);
        let kernel2_res = horizontal_lines_x3_get_kernel(&mut ctx.frame_buf, &ctx.win.dimensions, &top_face_lines, &mut ctx.gpu_context);
        let kernel3_res = fill_rects_get_kernel(&mut ctx.frame_buf, &ctx.win.dimensions, &front_refs, ctx.win.foreground_color, &mut ctx.gpu_context);

    // If any kernel creation fails, fallback to CPU rendering
    if kernel1_res.is_err() || kernel2_res.is_err() || kernel3_res.is_err() {
        draw::lines_batches::horizontal_lines_x3_threaded(ctx, &side_face_lines);
        draw::lines_batches::horizontal_lines_x3_threaded(ctx, &top_face_lines);
        draw::rectangle::filled_multiple(ctx, &front_refs);
        return;
    }

    // If all kernels are created successfully, execute them
    let KernelBundle{kernel: kernel1, buffers,..} = kernel1_res.expect("kernel1 failed");
    let buffer = buffers.get(0).expect("No frame buffer in kernel bundle");

    let KernelBundle{kernel:kernel2,..} = kernel2_res.expect("kernel2 failed");
    let KernelBundle{kernel:kernel3,..} = kernel3_res.expect("kernel3 failed");

    let kernels = vec![kernel1, kernel2, kernel3];
    let gpu_result = execute_kernels_and_read(&kernels,  &buffer, &mut ctx.frame_buf);

    if gpu_result.is_err() {
        eprintln!("GPU execution failed :( {}", gpu_result.unwrap_err());
    }

}
*/

pub fn bars_3d<UserData>(ctx: &mut GraphContext<UserData>, props: &Vec<Bar3DProps>) {
    use crate::draw;
    use crate::primitives::plane::RectArea;

    /// Generates scanlines for a convex quadrilateral (parallelogram) in the form:
    /// Each scanline is a Vec<u32>: [x_start, x_end, color]
    /// **NB:** The function has a side effect!
    /// **NB:** It modifies `segments_per_scanline` and `scanlines`!
    ///
    /// # Arguments
    /// * `p0`, `p1`, `p2`, `p3` - Points defining the parallelogram vertices in order
    /// * `color` - Color to fill the parallelogram
    /// * `segments_per_scanline` - a registry of how many line segments there are per scanline
    /// * `scanlines` - The scanline data itself
    ///
    fn parallelogram_horizontal_scanlines(
        p0: Point<i32>,
        p1: Point<i32>,
        p2: Point<i32>,
        p3: Point<i32>,
        color: u32,
        segments_per_scanline: &mut Vec<u32>,
        scanlines: &mut Vec<Vec<u32>>,
    ) {
        let min_y = p0.y.min(p1.y).min(p2.y).min(p3.y);
        let max_y = p0.y.max(p1.y).max(p2.y).max(p3.y);

        for y in min_y..max_y {
            let x_left = if p3.y != p0.y {
                p0.x + ((p3.x - p0.x) as f32 * (y - p0.y) as f32 / (p3.y - p0.y) as f32).round()
                    as i32
            } else {
                p0.x
            };
            let x_right = if p2.y != p1.y {
                p1.x + ((p2.x - p1.x) as f32 * (y - p1.y) as f32 / (p2.y - p1.y) as f32).round()
                    as i32
            } else {
                p1.x
            };

            let x_start = x_left.min(x_right) as u32;
            let x_end = x_left.max(x_right) as u32;

            if y > max_y {
                continue;
            }

            let y = y as usize;

            if y > segments_per_scanline.len() - 1 {
                continue;
            }

            segments_per_scanline[y] += 1; // Increment the count of segments on this scanline
            scanlines[y].push(x_start);
            scanlines[y].push(x_end);
            scanlines[y].push(color);
        }
    }

    // Key: [0..y_max]. Value: how many line segments on a scanline.
    let mut segments_per_scanline: Vec<u32> = Vec::with_capacity(ctx.win.h_usize);
    segments_per_scanline.resize(ctx.win.h_usize, 0);

    // Key: [0..y_max]. Value: a Vec of line segments: [x_start, x_end, color].
    let mut scanlines: Vec<Vec<u32>> = Vec::new();
    scanlines.resize(ctx.win.h_usize, Vec::new());

    let mut fronts: Vec<RectArea<i32>> = Vec::with_capacity(props.len());

    //=======================================

    // Keys: no meaning
    // Values: line segments as `x_start`, `x_end`, `color` triples, one triplet after another, no scanline `y` value saved here.
    let mut flat_data: Vec<u32> = Vec::new();

    // Keys: no meaning
    // Values: indices of scanlines that have at least one line segment
    let mut occupied_scanline_indices: Vec<u32> = Vec::new();

    // Key: scanline index in range [0..y_max].
    // Value: pointer to the start of a scanline in `flat_data` or `0`
    let mut flat_data_ptrs: Vec<u32> = Vec::with_capacity(ctx.win.h_usize);

    // Key: scanline index in range [0..y_max].
    // Value: number of individual elements of all the line segment triplets constituting this scanline.
    let mut scanline_sizes: Vec<u32> = Vec::with_capacity(ctx.win.h_usize);

    flat_data_ptrs.resize(ctx.win.h_usize, 0);
    scanline_sizes.resize(ctx.win.h_usize, 0);
    //=======================================

    for bar in props {
        let Bar3DProps {
            x,
            y,
            width,
            height,
            depth,
            color_front,
            color_top,
            color_side,
            slant,
            project_to_right,
            ..
        } = *bar;
        let slant_x = slant.x as i32;
        let slant_y = slant.y as i32;

        // --- Side face as parallelogram ---
        let (s0, s1, s2, s3) = if project_to_right {
            (
                Point::new(x + width, y - height),
                Point::new(x + width + depth * slant_x, y - height - depth * slant_y),
                Point::new(x + width + depth * slant_x, y - depth * slant_y),
                Point::new(x + width, y),
            )
        } else {
            (
                Point::new(x, y - height),
                Point::new(x - depth * slant_x, y - height - depth * slant_y),
                Point::new(x - depth * slant_x, y - depth * slant_y),
                Point::new(x, y),
            )
        };

        parallelogram_horizontal_scanlines(
            s0,
            s1,
            s2,
            s3,
            color_side,
            &mut segments_per_scanline,
            &mut scanlines,
        );

        /*
                for line in &side_lines {
                    let scanline_ptr = all_scanlines.len();
                    scanline_ptrs.push(scanline_ptr);
                    all_scanlines.extend(line);
                    let y = line[0] as usize;
                    scanline_dict[y][0] += 1; // Increment count for this y
                    scanline_dict[y].push(scanline_ptr); // save a copy of the pointer for this y
                }
        */

        // --- Top face as parallelogram ---
        let (p0, p1, p2, p3) = if project_to_right {
            (
                Point::new(x, y - height),
                Point::new(x + width, y - height),
                Point::new(x + width + depth * slant_x, y - height - depth * slant_y),
                Point::new(x + depth * slant_x, y - height - depth * slant_y),
            )
        } else {
            (
                Point::new(x, y - height),
                Point::new(x + width, y - height),
                Point::new(x + width - depth * slant_x, y - height - depth * slant_y),
                Point::new(x - depth * slant_x, y - height - depth * slant_y),
            )
        };
        parallelogram_horizontal_scanlines(
            p0, p1, p2, p3,
            color_top,
            &mut segments_per_scanline,
            &mut scanlines,
        );

        let mut i = 0;

        for scanline in &scanlines {
            if scanline.is_empty() {
                i += 1;
                continue;
            }

            let scanline_size = scanline.len() as u32;
            scanline_sizes[i] = scanline_size;
            flat_data.extend(scanline);
            flat_data_ptrs[i] = flat_data.len() as u32 - scanline_size;
            occupied_scanline_indices.push(i as u32);
            i += 1;
        }

        // --- Front face batch collect (rectangles) ---
        let front = RectArea::new(x, y - height, width, height + 1, Some(color_front));
        fronts.push(front);
    }

    // println!("\n\nall_scanlines:\n{:#?}\n\n", all_scanlines);
    // println!("scanline_ptrs:\n{:#?}\n\n", scanline_ptrs);
    // println!("scanline_dict:\n{:#?}\n\n", scanline_dict);

    //**********************************************************************************************
    //**********************************************************************************************
/*

// TODO: this is just a CPU version. Remove once both GPU and CPU versions are implemented.

    horizontal_batch_threaded::scanlines_threaded(
        &mut ctx.frame_buf,
        &ctx.win.dimensions,
        flat_data,
        occupied_scanline_indices,
        flat_data_ptrs,
        scanline_sizes,
        ctx.num_threads,
        &mut ctx.gpu_context,
    );

    let front_refs: Vec<&RectArea<i32>> = fronts.iter().collect();
    draw::rectangle::filled_multiple(ctx, &front_refs);
*/
    let front_refs: Vec<&RectArea<i32>> = fronts.iter().collect();

 //##########################################################################################################################
 //##########################################################################################################################
 //##########################################################################################################################
 //##########################################################################################################################
 //##########################################################################################################################
 //##########################################################################################################################


      // TODO: The GPU support is commented out for now.
      // TODO: Don't forget to re-enabled it once the time is right.

      // Build all kernels operating on the same frame buffer
      let kernel1_res =
          buffer_op::gpu::scanlines::scanlines_get_kernel(
              &mut ctx.frame_buf,
              &ctx.win.dimensions,
              // &occupied_scanline_indices,
              &flat_data,

              &flat_data_ptrs,
              &scanline_sizes,

              &mut ctx.gpu_context,
          );

      let kernel2_res = fill_rects_get_kernel(&mut ctx.frame_buf, &ctx.win.dimensions, &front_refs, ctx.win.foreground_color, &mut ctx.gpu_context);

      // If any kernel creation fails, fallback to CPU rendering
      if kernel1_res.is_err() || kernel2_res.is_err()  {
          // println!("%%%%%%%% FALLBACK ON THE CPU: k1{:?} k2{:?}", kernel1_res.unwrap_err(), kernel2_res.unwrap_err());


          horizontal_batch_threaded::scanlines_threaded(
              &mut ctx.frame_buf,
              &ctx.win.dimensions,
              flat_data,
              occupied_scanline_indices,
              flat_data_ptrs,
              scanline_sizes,
              ctx.num_threads,
              &mut ctx.gpu_context,
          );


          let front_refs: Vec<&RectArea<i32>> = fronts.iter().collect();
          draw::rectangle::filled_multiple(ctx, &front_refs);
          return;
      }

      // If all kernels are created successfully, execute them
      let KernelBundle{kernel: kernel1, buffers,..} = kernel1_res.expect("kernel1 failed");
      let buffer = buffers.get(0).expect("No frame buffer in kernel bundle");

      let KernelBundle{kernel:kernel2,..} = kernel2_res.expect("kernel2 failed");

    // println!("%%%%%%%% TRYING THE GPU!!!!");

      let kernels = vec![kernel1, kernel2];
      let gpu_result = execute_kernels_and_read(&kernels,  &buffer, &mut ctx.frame_buf);

      if gpu_result.is_err() {
          eprintln!("GPU execution failed :( {}", gpu_result.unwrap_err());
      }


    //**********************************************************************************************
    //**********************************************************************************************
}
