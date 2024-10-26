pub mod graph1_core {
    pub mod context;
    pub mod default_colors;
}
pub mod primitives {
    pub mod helper_types;
    pub mod numeric;
    mod pixel;
    pub use pixel::Pixel;
    pub mod plane;
    pub mod point;
}

pub mod utils {
    pub mod color {
        pub mod adapters;
        pub mod math {
            pub mod operations;
            pub mod rgba_operation;
        }
    }
}

pub mod draw{
    pub mod rectangle;
    pub mod tools {
        pub mod fill;
    }
}

// pub mod draw;
// pub mod primitives;
// pub mod text;
// pub mod tools;
// pub mod utils;
// pub mod filters;
