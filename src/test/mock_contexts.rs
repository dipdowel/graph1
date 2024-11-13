use crate::core::context::{GraphContext, WindowContext};

pub struct MockUserData {
    test_number: u32,
}

impl Default for MockUserData {
    fn default() -> Self {
        Self {
            test_number: 0,
        }
    }
}


/// Instantiates a mock graph context
/// with a window size of 800x600 pixels
pub fn get_mock_graph_context() -> GraphContext<MockUserData> {
    GraphContext::new(WindowContext::new(800, 600, None, None), false, false, None)
}