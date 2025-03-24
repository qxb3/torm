/// The state that will be used throughout torm.
pub struct State {
    pub exit: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            exit: false,
        }
    }
}

impl State {
    pub fn new() -> Self {
        Self::default()
    }
}
