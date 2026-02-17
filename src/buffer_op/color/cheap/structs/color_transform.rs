use crate::buffer_op::color::cheap::{Brightness, Contrast, Hue, Tint};

/// A type of color transformation that can be applied to image buffers.
#[derive(Clone, Copy, Debug)]
pub enum ColorTransformType {
    Brightness(Brightness),
    Contrast(Contrast),
    Hue(Hue),
    Tint(Tint),
}

/// A collection of color transformations to be applied in sequence.
///
/// This struct holds an ordered list of transformations that will be applied
/// to an image buffer. The order matters, as different sequences of transformations
/// can produce different results (e.g., applying brightness before contrast vs. after).
#[derive(Clone, Debug)]
pub struct ColorTransform {
    /// The ordered list of transformations to apply.
    pub transforms: Vec<ColorTransformType>,
}

impl ColorTransform {
    /// Creates a new `ColorTransform` with the specified transformations.
    /// The transformations will be applied in the order they appear in the vector.
    /// An empty vector is valid and results in no transformations being applied.
    pub fn new(transforms: Vec<ColorTransformType>) -> Self {
        Self { transforms }
    }

    /// Creates an empty `ColorTransform` with no transformations.
    /// This is equivalent to `ColorTransform::new(vec![])`
    pub fn empty() -> Self {
        Self {
            transforms: Vec::new(),
        }
    }

    /// Returns true if this transform contains no transformations.
    pub fn is_empty(&self) -> bool {
        self.transforms.is_empty()
    }

    /// Returns the number of transformations in this collection.
    pub fn len(&self) -> usize {
        self.transforms.len()
    }
}
