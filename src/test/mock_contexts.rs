use crate::core::context::{GraphContext, WindowContext};

#[derive(Debug)]
pub struct MockUserData {}

impl Default for MockUserData {
    fn default() -> Self {
        Self {}
    }
}

/// Instantiates a mock graph context
/// with a window size specified by `w` and `h` (width and height)
pub fn get_mock_graph_context(w: u32, h: u32) -> GraphContext<MockUserData> {
    GraphContext::new(
        WindowContext::new(w, h, None, None),
        false,
        1,
        None,
        1,
        None,
    )
}
