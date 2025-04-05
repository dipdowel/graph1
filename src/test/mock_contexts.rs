use crate::core::context::{GraphContext, WindowContext};

#[derive(Debug)]
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
/// with a window size specified by `w` and `h` (width and height)
pub fn get_mock_graph_context(w:u32, h:u32) -> GraphContext<MockUserData> {
    GraphContext::new(WindowContext::new(w, h, None, None), false, false, None, 1, None)
}