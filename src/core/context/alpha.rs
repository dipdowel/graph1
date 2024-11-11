#[derive(Debug, Default)]
/// Available methods for alpha blending
pub enum AlphaMethod {
    /// Use integer-based alpha blending, faster but less precise
    #[default]
    Int,
    /// Use floating-point-based alpha blending, slower but more precise
    Float,
}

#[derive(Debug)]
pub struct AlphaContext {
    /// If false, the alpha channel will be ignored when performing image/color operations and rendering
    pub enabled: bool,
    /// Method for alpha blending: `Float` for precision, `Int` for speed
    pub method: AlphaMethod,
}

impl Default for AlphaContext {
    fn default() -> Self {
        Self {
            enabled: false,
            method: AlphaMethod::Int,
        }
    }
}
