/// Enumeration of supported line clipping algorithms.
///
/// These styles determine how line segments are clipped when they intersect
/// or fall outside of the clipping area (`RectArea`). Each style has different
/// performance characteristics and geometric behavior.
#[derive(Debug)]
pub enum LineClippingStyle {
    /// Clips the line segment using the Liang–Barsky algorithm.
    ///
    /// A mathematically efficient, parametric clipping method using inequalities.
    /// It avoids loops and branching for optimal performance.
    /// Best suited for high-performance, real-time rendering.
    LiangBarsky,

    /// Clips the line segment using the Cohen–Sutherland algorithm.
    ///
    /// Region-code based method that quickly rejects or clips lines using bitwise
    /// comparisons. More verbose but intuitive and easy to debug.
    /// Useful for simple 2D GUIs or editors.
    CohenSutherland,

    /// Uses "Elastic Slide" clipping by clamping endpoints to the clipping rectangle.
    ///
    /// Not geometrically correct, but fast and forgiving.
    /// Ideal for prototyping, animations, or when accuracy is less critical.
    /// Can also be used for to simulate simple elastic deformations.
    ElasticSlide,
}
