
/// Defines a common interface for saving and restoring the state of Graph1 contexts.
/// Implementing this trait allows a context to be cloned and replaced with a previously saved state,
/// which is useful for undo/redo functionality or context state management in applications.

#[allow(dead_code)] // Dead code is allowed here to prevent `unused` warnings if the trait is implemented only using the macro.
pub trait ContextSnapshot {
    /// Returns a copy of this context (used for saving current state).
    fn get_context(&self) -> Self;

    /// Replaces this context with another one (used for restoring saved state).
    fn set_context(&mut self, ctx: Self);
}


/*
// This macro is great in theory, but the complier cannot find the implemented methods in a context :-(
#[macro_export]
macro_rules! impl_context_snapshot {
    ($t:ty) => {
        impl ContextSnapshot for $t {
            fn get_context(&self) -> Self {
                self.clone()
            }
            fn set_context(&mut self, ctx: Self) {
                *self = ctx;
            }
        }
    };
}
*/