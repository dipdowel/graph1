use crate::buffer_op::gpu::kernel_bundle::KernelBundle;
use crate::core::context::gpu::GpuContext;
use crate::primitives::numeric::Numeric;
use crate::primitives::plane::{Dimensions2d, RectArea};
#[cfg(feature = "gpu")]
use ocl::{Buffer, Kernel};
/// Prepare a 2D tile grid of rectangles for GPU rasterization.
/// Returns (flat_rects, tile_offsets, tile_counts, tiles_x, tiles_y).

pub fn fill_rects_spatial_tiles<T: Numeric + Copy + 'static>(
    cpu_frame_buf: &mut [u32],
    buf_dimensions: &Dimensions2d<u32>,
    rects: &Vec<&RectArea<T>>,
    default_color: u32,
    tiles_x: Option<u32>,
    tiles_y: Option<u32>,
    gpu_context: &mut GpuContext,
) -> Result<(), String> {
    #[cfg(feature = "gpu")]
    {
        let tiles_x = tiles_x.unwrap_or(8).clamp(2, 32);
        let tiles_y = tiles_y.unwrap_or(8).clamp(2, 32);

        let bundle = fill_rects_spatial_tiles_get_kernel(
            cpu_frame_buf,
            buf_dimensions,
            rects,
            default_color,
            tiles_x,
            tiles_y,
            gpu_context,
        )?;
        let kernel = bundle.kernel;
        let gpu_frame_buf = bundle
            .buffers
            .get(0)
            .ok_or("No frame buffer in kernel bundle")?;

        gpu_frame_buf
            .write(cpu_frame_buf.as_ref())
            .enq()
            .map_err(|e| format!("Failed to upload frame buffer: {e}"))?;
        unsafe {
            kernel
                .enq()
                .map_err(|e| format!("Failed to enqueue kernel: {e}"))?;
        }
        gpu_frame_buf
            .read(cpu_frame_buf)
            .enq()
            .map_err(|e| format!("Failed to read GPU buffer: {e}"))?;
        Ok(())
    }
    #[cfg(not(feature = "gpu"))]
    {
        let _ = cpu_frame_buf;
        let _ = buf_dimensions;
        let _ = rects;
        let _ = default_color;
        let _ = tiles_x;
        let _ = tiles_y;
        let _ = gpu_context;
        Err("GPU support is not enabled at compile time.".to_string())
    }
}

pub fn fill_rects_spatial_tiles_get_kernel<T: Numeric + Copy + 'static>(
    cpu_frame_buf: &mut [u32],
    buf_dimensions: &Dimensions2d<u32>,
    rects: &Vec<&RectArea<T>>,
    default_color: u32,
    tiles_x: u32,
    tiles_y: u32,
    gpu_context: &mut GpuContext,
) -> Result<KernelBundle, String> {
    #[cfg(feature = "gpu")]
    {
        // let start = Instant::now(); // Start timing

        let (flat_rects, tile_offsets, tile_counts, tiles_x, tiles_y) = tile_rects_grid(
            rects,
            buf_dimensions.w,
            buf_dimensions.h,
            tiles_x,
            tiles_y,
            default_color,
        );

        let kernel_src = include_str!("fill_rects_spatial_tiles.c");
        let kernel_name = "fill_rects_tiles";
        let program_name = "fill_rects_tiles_program";
        let buf_len = cpu_frame_buf.len();

        if gpu_context.get_program(program_name).is_none() {
            gpu_context
                .load_program(&kernel_src, program_name)
                .map_err(|e| format!("Failed to build OpenCL program: {e}"))?;
        }
        let program = gpu_context
            .get_program(program_name)
            .ok_or("Program not loaded (unknown error)")?;
        let queue = gpu_context
            .queue
            .as_ref()
            .ok_or("No OpenCL queue in context")?
            .clone();
        let queue_ref = queue.as_ref();

        let flat_rects_buf = Buffer::<u32>::builder()
            .queue(queue_ref.clone())
            .len(flat_rects.len())
            .copy_host_slice(&flat_rects)
            .build()
            .map_err(|e| format!("Failed to create OpenCL flat_rects buffer: {e}"))?;

        let offsets_buf = Buffer::<u32>::builder()
            .queue(queue_ref.clone())
            .len(tile_offsets.len())
            .copy_host_slice(&tile_offsets)
            .build()
            .map_err(|e| format!("Failed to create OpenCL tile_offsets buffer: {e}"))?;

        let counts_buf = Buffer::<u32>::builder()
            .queue(queue_ref.clone())
            .len(tile_counts.len())
            .copy_host_slice(&tile_counts)
            .build()
            .map_err(|e| format!("Failed to create OpenCL tile_counts buffer: {e}"))?;

        let gpu_frame_buf = gpu_context.get_or_create_buffer(buf_len, queue_ref, cpu_frame_buf)?;

        let tile_w = (buf_dimensions.w + tiles_x - 1) / tiles_x;
        let tile_h = (buf_dimensions.h + tiles_y - 1) / tiles_y;

        let kernel = Kernel::builder()
            .program(&program)
            .name(kernel_name)
            .queue(queue_ref.clone())
            .global_work_size(buf_len)
            .arg(&gpu_frame_buf)
            .arg(buf_dimensions.w)
            .arg(buf_dimensions.h)
            .arg(&flat_rects_buf)
            .arg(&offsets_buf)
            .arg(&counts_buf)
            .arg(tiles_x)
            .arg(tiles_y)
            .arg(tile_w)
            .arg(tile_h)
            .build()
            .map_err(|e| format!("Failed to build kernel: {e}"))?;

        // let duration = start.elapsed(); // Measure elapsed time
        // println!("[fill_rects_tiles_get_kernel] duration:  {:?}", duration.as_micros());

        Ok(KernelBundle {
            kernel,
            buffers: vec![gpu_frame_buf, flat_rects_buf, offsets_buf, counts_buf],
        })
    }
    #[cfg(not(feature = "gpu"))]
    {
        let _ = cpu_frame_buf;
        let _ = buf_dimensions;
        let _ = rects;
        let _ = default_color;
        let _ = tiles_x;
        let _ = tiles_y;
        let _ = gpu_context;
        Err("GPU support is not enabled at compile time.".to_string())
    }
}

#[cfg(feature = "gpu")]
pub fn tile_rects_grid<T: Numeric + Copy>(
    rects: &[&RectArea<T>],
    buf_width: u32,
    buf_height: u32,
    tiles_x: u32,
    tiles_y: u32,
    default_color: u32,
) -> (Vec<u32>, Vec<u32>, Vec<u32>, u32, u32) {
    let tile_w = (buf_width + tiles_x - 1) / tiles_x;
    let tile_h = (buf_height + tiles_y - 1) / tiles_y;
    let n_tiles = tiles_x * tiles_y;
    let mut tiles: Vec<Vec<u32>> = vec![Vec::new(); n_tiles as usize];

    for rect in rects {
        let x = rect.top_left.x.to_u32();
        let y = rect.top_left.y.to_u32();
        let w = rect.dimensions.w.to_u32();
        let h = rect.dimensions.h.to_u32();
        let color = rect.color.unwrap_or(default_color);

        let start_tx = (x / tile_w).min(tiles_x - 1);
        let end_tx = ((x + w - 1) / tile_w).min(tiles_x - 1);
        let start_ty = (y / tile_h).min(tiles_y - 1);
        let end_ty = ((y + h - 1) / tile_h).min(tiles_y - 1);

        for ty in start_ty..=end_ty {
            for tx in start_tx..=end_tx {
                let idx = (ty * tiles_x + tx) as usize;
                tiles[idx].push(x);
                tiles[idx].push(y);
                tiles[idx].push(w);
                tiles[idx].push(h);
                tiles[idx].push(color);
            }
        }
    }

    let mut flat: Vec<u32> = Vec::new();
    let mut offsets = Vec::with_capacity(n_tiles as usize);
    let mut counts = Vec::with_capacity(n_tiles as usize);
    let mut curr = 0;
    for tile in &tiles {
        offsets.push(curr as u32 / 5);
        counts.push((tile.len() / 5) as u32);
        flat.extend(tile);
        curr += tile.len();
    }

    (flat, offsets, counts, tiles_x, tiles_y)
}
