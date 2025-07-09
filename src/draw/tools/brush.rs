use crate::primitives::plane::Dimensions2d;

/// An abstract paint brush
#[derive(Debug)]
pub enum Brush {
    /// A circular brush
    Circle {
        /// The radius of the circle
        radius: f64,
    },
    /// A rectangular brush
    Rectangle { size: Dimensions2d },
}

impl Brush {
    pub fn new_circle(radius: f64) -> Brush {
        Brush::Circle { radius }
    }
    pub fn new_rectangle(w: u32, h: u32) -> Brush {
        Brush::Rectangle {
            size: Dimensions2d::new(w, h),
        }
    }

    pub(crate) fn default() -> Brush {
        Brush::new_rectangle(10, 10)
    }
}
