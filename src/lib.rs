/// Core functionality for the library, e.g. contexts, default values, etc.
pub mod graph1_core {
    /// `GraphContext` and its sub-contexts
    pub mod context;
    /// Default colors used in the library if no custom colors specified
    pub mod default_colors;
    /// Alpha blending configurations
    pub mod alpha;
}
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

/// Utilities for working with colors, color-specific math, pixel model conversions, etc.
pub mod utils {
    /// Utils for processing colors
    pub mod color {
        /// Functions to blend colors taking into account the alpha channel
        pub mod alpha;
        /// Color adapters for converting between different color models.
        pub mod adapters;
        /// Math operations on colors
        pub mod math {

            /// Addition and subtraction of RGBA colors
            pub mod rgba_operation;
        }
        pub mod palettes;
        /// Color properties calculations, color analysis
        pub mod properties {
            pub mod intensity;
            pub mod luminance;

        }
    }
}

/// Drawing tools and operations
pub mod draw{
    /// Draw rectangles
    pub mod rectangle;
    /// Draw lines
    pub mod line;
    /// Draw circles
    pub mod circle;

    /// Drawing tools
    pub mod tools {
        /// Fill a shape or a buffer with a color
        pub mod fill;
    }
}

// pub mod draw;
// pub mod primitives;
// pub mod text;
// pub mod tools;
// pub mod utils;
// pub mod filters;
