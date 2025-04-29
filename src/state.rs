use crate::{TormResult, buffer::Buffer};

/// The state that will be used throughout torm.
pub struct State {
    pub test_buffer: Buffer,

    /// Exit state.
    pub exit: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            test_buffer: Buffer::new(),
            exit: false,
        }
    }
}

impl State {
    pub fn new() -> TormResult<Self> {
        Ok(Self {
            test_buffer: Buffer::from_file("src/torm.rs")?,
            exit: false,
        })
    }
}
