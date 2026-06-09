//! pine-loader — ELF/PE loader adapter.

use pine_core::traits::Loader;

pub struct ElfLoader;

impl Loader for ElfLoader {
    fn load(&self, _path: &str) -> Result<Vec<u8>, String> {
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use crate::ElfLoader;
    use pine_core::traits::Loader;

    #[test]
    fn elf_loader_implements_loader() {
        let loader = ElfLoader;
        let result = loader.load("/nonexistent");
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn elf_loader_loads_empty_for_missing_file() {
        let loader = ElfLoader;
        let bytes = loader.load("/tmp/does_not_exist.elf").unwrap();
        assert_eq!(bytes, vec![]);
    }
}
