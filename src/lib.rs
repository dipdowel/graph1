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
    /// Draw rectangles
    pub mod rectangle;

    /// Drawing tools
    pub mod tools {
        /// Fill a shape or a buffer with a color
        pub mod fill;
    }
}

//-=[ F }=------------------------------------------------------------------------------------------

pub mod fx {
    pub mod scanline;
}

//-=[ P }=------------------------------------------------------------------------------------------

/// Building blocks: essential structs, traits, types, constants, etc.
pub mod primitives {
    /// Aliases for complex or peculiar types
    pub mod helper_types;
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

    pub mod bit_operations;
    mod common;
    pub use common::clear_screen;

    /// Utils for processing colors
    pub mod color {
        /// Color adapters for converting between different color models.
        pub mod adapters;
        /// Functions to blend colors taking into account the alpha channel
        pub mod alpha;
        /// Math operations on colors
        pub mod math {

            /// Addition and subtraction of RGBA colors
            mod rgba_operation;
            pub use rgba_operation::ColorOperation;
            pub use rgba_operation::rgba_operation;
        }

        pub mod palettes;

        /// Color properties calculations, color analysis
        pub mod desaturate {
            pub mod intensity;
            pub mod luminance;
        }
    }
    pub mod math;
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