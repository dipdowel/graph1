use std::collections::HashMap;
use crate::graph1::text::font::PixelChar;

/// A map of a regular char (`'A'`, `'b'`, `'3'`, `'!'`, etc.)
/// to a `PixelCharacter` (which contains information for rendering a character).
pub type HashMapCharDescriptions<'a> = HashMap<char, &'a PixelChar>;