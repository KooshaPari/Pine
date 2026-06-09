//! pine-nvms — nvms microVM integration adapter.

pub struct NvmsRuntime;

impl NvmsRuntime {
    pub fn new() -> Self {
        Self
    }
}

impl Default for NvmsRuntime {
    fn default() -> Self {
        Self::new()
    }
}
