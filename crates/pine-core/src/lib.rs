//! pine-core — Domain types and shared interfaces for Pine.

pub mod traits;
pub mod types;

#[cfg(test)]
mod tests {
    use super::types::*;

    #[test]
    fn process_id_newtype() {
        let pid = ProcessId(42);
        assert_eq!(pid.0, 42);
    }

    #[test]
    fn syscall_number_newtype() {
        let num = SyscallNumber(0x80);
        assert_eq!(num.0, 0x80);
    }
}
