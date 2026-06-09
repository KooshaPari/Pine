//! pine-compat — Compatibility shim (Wine-like translation).

pub struct CompatibilityLayer;

impl CompatibilityLayer {
    pub fn new() -> Self {
        Self
    }
}

impl Default for CompatibilityLayer {
    fn default() -> Self {
        Self::new()
    }
}
