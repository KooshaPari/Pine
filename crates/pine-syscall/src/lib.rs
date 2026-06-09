//! pine-syscall — Syscall translation layer.

use pine_core::traits::SyscallHandler;

pub struct LinuxSyscallHandler;

impl SyscallHandler for LinuxSyscallHandler {
    fn handle(&self, _number: u32, _args: &[u64]) -> Result<u64, String> {
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use pine_core::traits::SyscallHandler;
    use crate::LinuxSyscallHandler;

    #[test]
    fn linux_handler_returns_zero_for_unknown_syscall() {
        let handler = LinuxSyscallHandler;
        let result = handler.handle(999, &[1, 2, 3]);
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn linux_handler_implements_syscall_handler() {
        let handler = LinuxSyscallHandler;
        let result = handler.handle(0, &[]);
        assert!(result.is_ok());
    }
}
