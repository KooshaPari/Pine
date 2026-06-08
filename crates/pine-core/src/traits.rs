//! Core domain traits (ports).

pub trait Loader {
    fn load(&self, path: &str) -> Result<Vec<u8>, String>;
}

pub trait SyscallHandler {
    fn handle(&self, number: u32, args: &[u64]) -> Result<u64, String>;
}
