//-=[ C }=------------------------------------------------------------------------------------------

/// Core functionality for the library, e.g. contexts, default values, etc.
pub mod core {

    /// Heart of the library: `GraphContext` and its sub-contexts
    pub mod context {
        pub mod alpha;
        mod bezier;
        mod graph;
        mod window;

        pub use alpha::AlphaContext;
        // pub use alpha::AlphaMethod;
        pub use bezier::BezierContext;
        pub use graph::GraphContext;
        pub use window::WindowContext;
    }

    /// Default colors used in the library if no custom colors specified
    pub mod default_colors;
}
//-=[ D }=------------------------------------------------------------------------------------------

/// Drawing tools and operations
pub mod draw {
    /// Draw circles
    pub mod circle;
    /// Draw lines
    pub mod line;

    /// Various closed shapes with multiple vertices
    pub mod polygons {
        mod closed_perimeter;
        mod polygon;
        mod star;
        /// Connects a given vector of points with lines
        pub use closed_perimeter::closed_perimeter;

        /// Draws a polygon based on the provided properties
        pub use polygon::polygon;
        /// Properties for drawing a polygon
        pub use polygon::PolygonProperties;

        /// Draws a star based on the provided properties
        pub use star::star;
        /// Properties for drawing a star
        pub use star::StarProperties;
    }

    /// Draw rectangles
    pub mod rectangle;

    /// Drawing tools
    pub mod tools {
        /// Fill a shape or a buffer with a color
        pub mod fill {
            mod buffer;
            mod flood;
            /// Fill a buffer with a color
            pub use buffer::buffer;
            /// Fill a shape with a color
            pub use flood::flood;
        }
    }
}

//-=[ F }=------------------------------------------------------------------------------------------

pub mod fx {
    pub mod scanline;
    
    // FIXME!
    // FIXME!
    // FIXME!
    // FIXME!
    pub mod noise {
    //     mod perlin;
    //     pub use perlin::perlin;
    //     pub use perlin::PerlinNoiseProps;
        mod white_noise;
        pub use white_noise::WhiteNoise;
        pub use white_noise::WhiteNoiseProps;
    }
    
}

//-=[ P }=------------------------------------------------------------------------------------------

/// Building blocks: essential structs, traits, types, constants, etc.
pub mod primitives {
    /// Aliases for complex or peculiar types
    pub mod helper_types;

    /// Math-related primitives
    pub mod math;
    /// `Numeric` - a convenience trait, simplifies conversions between numeric types.
    pub mod numeric;
    mod pixel;
    /// `Pixel` - a simple struct representing a position on a physical screen + a color in RGBA.
    pub use pixel::Pixel;
    /// Various structs to operate on a 2D-plane
    pub mod plane;
    /// `Point` - a struct representing a position on a 2D-plane.
    /// Unlike `Pixel`, it can have negative and fractional coordinates.
    pub mod point;
}

//-=[ T }=------------------------------------------------------------------------------------------

pub mod test {
    pub mod mock_contexts;
}
pub mod text {

    pub mod char_width_map;
    pub mod font;
    pub mod font_constants;
    pub mod font_embedder;
    pub mod printer;

    pub mod utils;
}

//-=[ U }=------------------------------------------------------------------------------------------

/// Utilities for working with colors, color-specific math, pixel model conversions, etc.
pub mod utils {
    mod common;
    pub use common::clear_screen;

    /// Utils for processing colors
    pub mod color {
        /// Color adapters for converting between different color models.
        pub mod adapters {
            mod adapter_statistics;
            mod rgba_to_0rgb;
            mod rgba_to_0rgb_unsafe;
            mod rgba_to_abgr;
            mod rgba_to_abgr_unsafe;
            mod single_pixel;

            pub use adapter_statistics::AdapterStatistics;
            pub use rgba_to_0rgb::rgba_to_0rgb;
            pub use rgba_to_0rgb_unsafe::rgba_to_0rgb_unsafe;
            pub use rgba_to_abgr::rgba_to_abgr;
            pub use rgba_to_abgr_unsafe::rgba_to_abgr_unsafe;
            pub use single_pixel::rgba_color_to_0rgb;
            pub use single_pixel::rgba_color_to_abgr;
        }

        /// Functions to blend colors taking into account the alpha channel
        pub mod alpha;
        /// Conversions between RGBA and 1-bit image
        pub mod bit_operations;
        /// Math operations on colors
        pub mod math {

            /// Addition and subtraction of RGBA colors
            mod rgba_operation;
            pub use rgba_operation::rgba_operation;
            pub use rgba_operation::ColorOperation;
        }

        pub mod gradient;
        pub mod palettes;

        /// Color properties calculations, color analysis
        pub mod desaturate {
            pub mod intensity;
            pub mod luminance;
        }
    }
    pub mod math {
        mod power;

        pub use power::is_power_of_two;
        pub use power::nearest_power_of_two_towards_zero;

        pub mod oscillator {
            mod sine;
            pub use sine::sine;
        }

        pub mod rng {
            pub(crate) mod helpers{
                pub(crate)mod normalize_xor_shift_input;
                pub(crate)mod normalize_min_max;
            }

            // TODO: add random color generator!
            pub mod color;
            pub mod gray;

            mod xor_shift;
            pub use xor_shift::XorShiftRng;

        }
    }

    pub mod pixel_copy {
        pub mod image_data;
    }
    pub mod validators;
}

// pub mod draw;
// pub mod primitives;
// pub mod text;
// pub mod tools;
// pub mod utils;
// pub mod filters;
