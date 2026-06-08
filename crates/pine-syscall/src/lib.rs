//! pine-syscall — Syscall translation layer.

use pine_core::traits::SyscallHandler;

pub struct LinuxSyscallHandler;

impl SyscallHandler for LinuxSyscallHandler {
    fn handle(&self, _number: u32, _args: &[u64]) -> Result<u64, String> {
        Ok(0)
    }
}
