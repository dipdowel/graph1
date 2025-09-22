/// Represents a mutually exclusive value: either `Primary(P)` or `Secondary(S)`.
#[derive(Debug, Clone)]
pub enum Variant<P, S> {
    /// Contains a value of type `P` (primary).
    Primary(P),

    /// Contains a value of type `S` (secondary).
    Secondary(S),
}

impl<P, S> Variant<P, S> {
    /// Creates a `Primary` variant.
    pub fn primary(value: P) -> Self {
        Variant::Primary(value)
    }

    /// Creates a `Secondary` variant.
    pub fn secondary(value: S) -> Self {
        Variant::Secondary(value)
    }

    /// Returns `true` if the enum is `Primary`.
    pub fn is_primary(&self) -> bool {
        matches!(self, Variant::Primary(_))
    }

    /// Returns `true` if the enum is `Secondary`.
    pub fn is_secondary(&self) -> bool {
        matches!(self, Variant::Secondary(_))
    }

    /// Returns a reference to the inner value if `Primary`, else `None`.
    pub fn as_primary(&self) -> Option<&P> {
        if let Variant::Primary(ref p) = self {
            Some(p)
        } else {
            None
        }
    }

    /// Returns a reference to the inner value if `Secondary`, else `None`.
    pub fn as_secondary(&self) -> Option<&S> {
        if let Variant::Secondary(ref s) = self {
            Some(s)
        } else {
            None
        }
    }
}
