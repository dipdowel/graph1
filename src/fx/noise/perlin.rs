use crate::primitives::plane::Dimensions2d;
use crate::primitives::point::Point;
use crate::utils::math::rng;

/// Properties that control the generation of Perlin noise.
///
/// Perlin noise is a gradient noise function that produces natural-looking textures and patterns.
/// This struct allows fine-grained control over its parameters, making it suitable for
/// procedural texture generation, terrain modeling, and animation effects.
pub struct PerlinNoiseProps {
    /// Controls the size of noise patterns (frequency).
    ///
    /// A higher value results in smaller, more frequent patterns,
    /// while a lower value produces larger, smoother patterns.
    pub scale: f64,

    /// The number of layers (octaves) of noise to be combined.
    ///
    /// Each octave adds finer detail to the noise, making it more complex.
    /// More octaves increase computation cost but yield richer textures.
    pub octaves: u32,

    /// Controls how much influence each additional octave has.
    ///
    /// A higher value retains more details from higher octaves,
    /// making the noise rougher. A lower value smooths out details.
    pub persistence: f64,

    /// Determines how much the frequency increases with each octave.
    ///
    /// A higher value makes each subsequent octave introduce finer details
    /// more rapidly, leading to more turbulent patterns.
    pub lacunarity: f64,

    /// A seed value for generating random variations in the noise.
    ///
    /// Changing this value alters the noise output while keeping all
    /// other parameters the same, making it useful for procedural content.
    pub seed: u32,

    /// A fixed random offset applied to the noise calculation.
    ///
    /// This prevents identical outputs when calling `perlin_noise()` multiple times
    /// with the same `seed`. Without this offset, noise patterns would be completely
    /// deterministic and could become repetitive in procedural generation.
    ///
    /// Instead of generating it inside `perlin_noise()`, this value is passed externally,
    /// allowing greater control and ensuring reproducibility if needed.
    ///
    /// **Example use case**:
    /// - If `seed_offset` is set to a fixed value, the output will be completely deterministic.
    /// - If it is randomized per instance, different calls will produce slight variations.
    pub seed_offset: f64,

    /// A 2D offset to shift the noise pattern.
    ///
    /// This is useful for animation effects, scrolling textures, or
    /// shifting noise placement within a larger procedural generation system.
    pub offset: Point<f64>,

    /// Defines a repeating tile size for seamless textures.
    ///
    /// When set, the noise is adjusted to loop smoothly within the given
    /// width and height, making it suitable for textures that wrap.
    pub tile_size: Option<Dimensions2d<f64>>,
}


/// Generates Perlin noise and writes the result into an RGBA buffer.
///
/// # Arguments
/// - `buf`: A mutable slice of `u32` representing an RGBA pixel buffer.
/// - `buf_dimensions`: The width and height of the buffer.
/// - `props`: The properties controlling the Perlin noise generation.
///
/// The function applies multi-octave Perlin noise based on the given properties and fills
/// the buffer with grayscale intensity values.
///
/// # Notes
/// - The output is written as a grayscale image where noise values are mapped to `[0, 255]`.
/// - Uses a basic Perlin noise function with frequency scaling, persistence, and octaves.
/// - If `tile_size` is set, ensures seamless looping by wrapping noise coordinates.
pub fn perlin(buf: &mut [u32], buf_dimensions: &Dimensions2d<usize>, props: &PerlinNoiseProps) {
    
    let Dimensions2d{ w, h } = *buf_dimensions;

    let width = w as f64;
    let height = h as f64;

    for y in 0..buf_dimensions.h{
        for x in 0..buf_dimensions.w {
            let mut amplitude = 1.0;
            let mut frequency = props.scale;
            let mut noise_value = 0.0;

            let mut px = x as f64 / width;
            let mut py = y as f64 / height;

            if let Some(tile_size) = props.tile_size {
                px = (px * tile_size.w) % tile_size.w;
                py = (py * tile_size.h) % tile_size.h;
            }

            px += props.offset.x + props.seed_offset;
            py += props.offset.y + props.seed_offset;

            for _ in 0..props.octaves {
                let perlin = simplex_noise(px * frequency, py * frequency);
                noise_value += perlin * amplitude;
                amplitude *= props.persistence;
                frequency *= props.lacunarity;
            }

            let color = ((noise_value + 1.0) * 127.5) as u32;
            let index = y * w + x;
            buf[index] = (color << 16) | (color << 8) | color | (255 << 24);
        }
    }
}

// /// Computes a basic Perlin-like noise function using a Simplex noise approximation.
// ///
// /// This function serves as a placeholder for a more complex noise algorithm.
// fn simplex_noise(x: f64, y: f64) -> f64 {
//     let n = (x.sin() * y.cos()).fract();
//     2.0 * (n - 0.5)  // Normalize to range [-1,1]
// }


fn simplex_noise(x: f64, y: f64) -> f64 {


    let random_numbers = rng::normal_pseudo(512, 1, 100, 2, 395);
    

    // Gradient hash function (Perlin's improved noise uses a permutation table)
    let hash = |x: i32, y: i32| -> usize {
        // let perm: [usize; 512] =
        random_numbers[ (x as usize + random_numbers[y as usize % 256]) % 256 ]
    };



    // Skewing/Unskewing factors for 2D Simplex
    const F2: f64 = 0.5 * (std::f64::consts::SQRT_2 - 1.0);
    const G2: f64 = (3.0 - std::f64::consts::SQRT_2) / 6.0;

    // Skew input space to determine simplex grid cell
    let s = (x + y) * F2;
    let i = (x + s).floor() as i32;
    let j = (y + s).floor() as i32;

    // Unskew cell back to original space
    let t = (i + j) as f64 * G2;
    let x0 = x - (i as f64 - t);
    let y0 = y - (j as f64 - t);

    // Determine simplex triangle offsets
    let (i1, j1) = if x0 > y0 { (1, 0) } else { (0, 1) };

    // Offsets for other triangle corners
    let x1 = x0 - i1 as f64 + G2;
    let y1 = y0 - j1 as f64 + G2;
    let x2 = x0 - 1.0 + 2.0 * G2;
    let y2 = y0 - 1.0 + 2.0 * G2;

    // Compute hashed gradient indices for corners
    let gi0 = hash(i, j) % 12;
    let gi1 = hash(i + i1, j + j1) % 12;
    let gi2 = hash(i + 1, j + 1) % 12;

    // Compute gradient contributions
    let t0 = 0.5 - x0 * x0 - y0 * y0;
    let n0 = if t0 > 0.0 {
        t0 * t0 * gradient_dot(gi0, x0, y0)
    } else {
        0.0
    };

    let t1 = 0.5 - x1 * x1 - y1 * y1;
    let n1 = if t1 > 0.0 {
        t1 * t1 * gradient_dot(gi1, x1, y1)
    } else {
        0.0
    };

    let t2 = 0.5 - x2 * x2 - y2 * y2;
    let n2 = if t2 > 0.0 {
        t2 * t2 * gradient_dot(gi2, x2, y2)
    } else {
        0.0
    };

    // Sum contributions and normalize
    70.0 * (n0 + n1 + n2)
}

 

// Computes dot product of gradient vector and input vector
fn gradient_dot(hash: usize, x: f64, y: f64) -> f64 {
    let gradients: [(f64, f64); 12] = [
        (1.0, 1.0), (-1.0, 1.0), (1.0, -1.0), (-1.0, -1.0),
        (1.0, 0.0), (-1.0, 0.0), (0.0, 1.0), (0.0, -1.0),
        (0.707, 0.707), (-0.707, 0.707), (0.707, -0.707), (-0.707, -0.707),
    ];
    let g = gradients[hash % 12];
    g.0 * x + g.1 * y
}
