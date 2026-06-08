//! pine-loader — ELF/PE loader adapter.

use pine_core::traits::Loader;

pub struct ElfLoader;

impl Loader for ElfLoader {
    fn load(&self, _path: &str) -> Result<Vec<u8>, String> {
        Ok(vec![])
    }
}
