/// Defines whether scaling should enlarge or reduce a region.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScaleDirection {
    /// Scale up (enlarge pixels into bigger blocks).
    Up,
    /// Scale down (shrink blocks into single pixels).
    Down,
}
