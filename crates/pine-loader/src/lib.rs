//! pine-loader — ELF/PE loader adapter.

use goblin::elf::Elf;
use goblin::error::Error as GoblinError;
use std::fmt;

/// Error type returned by the loader.
#[derive(Debug, Clone, PartialEq)]
pub enum LoaderError {
    ParseError(String),
}

impl fmt::Display for LoaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoaderError::ParseError(msg) => write!(f, "parse error: {}", msg),
        }
    }
}

impl std::error::Error for LoaderError {}

impl From<GoblinError> for LoaderError {
    fn from(err: GoblinError) -> Self {
        LoaderError::ParseError(err.to_string())
    }
}

/// Information about a parsed ELF section.
#[derive(Debug, Clone, PartialEq)]
pub struct ElfSection {
    pub name: String,
    pub addr: u64,
    pub size: u64,
    pub flags: u64,
}

/// Information about a parsed ELF symbol.
#[derive(Debug, Clone, PartialEq)]
pub struct ElfSymbol {
    pub name: String,
    pub value: u64,
    pub size: u64,
    pub section_index: u16,
    pub binding: u8,
    pub symbol_type: u8,
}

/// Parsed ELF binary with headers, sections, and symbols.
#[derive(Debug, Clone, PartialEq)]
pub struct ElfBinary {
    pub entry: u64,
    pub arch: u16,
    pub endianness: u8,
    pub sections: Vec<ElfSection>,
    pub symbols: Vec<ElfSymbol>,
}

/// Parse raw bytes as an ELF binary.
///
/// Returns `ElfBinary` populated with headers, sections, and symbols.
pub fn parse_elf(bytes: &[u8]) -> Result<ElfBinary, LoaderError> {
    let elf = Elf::parse(bytes)?;

    let sections = elf
        .section_headers
        .iter()
        .enumerate()
        .map(|(_idx, shdr)| {
            let name = elf
                .shdr_strtab
                .get_at(shdr.sh_name)
                .unwrap_or("")
                .to_string();
            ElfSection {
                name,
                addr: shdr.sh_addr,
                size: shdr.sh_size,
                flags: shdr.sh_flags,
            }
        })
        .collect();

    let symbols: Vec<ElfSymbol> = elf
        .syms
        .iter()
        .filter_map(|sym| {
            let name = elf.strtab.get_at(sym.st_name).unwrap_or("").to_string();
            Some(ElfSymbol {
                name,
                value: sym.st_value,
                size: sym.st_size,
                section_index: sym.st_shndx as u16,
                binding: sym.st_bind(),
                symbol_type: sym.st_type(),
            })
        })
        .collect();

    Ok(ElfBinary {
        entry: elf.header.e_entry,
        arch: elf.header.e_machine,
        endianness: elf.header.e_ident[5],
        sections,
        symbols,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Minimal 64-bit ELF header (128 bytes) with a single null section header.
    const MINIMAL_ELF64: [u8; 128] = [
        // e_ident
        0x7f, 0x45, 0x4c, 0x46, // magic
        0x02, // ELFCLASS64
        0x01, // ELFDATA2LSB
        0x01, // EV_CURRENT
        0x00, // ELFOSABI_NONE
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // padding
        // e_type = ET_EXEC (2)
        0x02, 0x00,
        // e_machine = EM_X86_64 (0x3e)
        0x3e, 0x00,
        // e_version = 1
        0x01, 0x00, 0x00, 0x00,
        // e_entry = 0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        // e_phoff = 0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        // e_shoff = 64
        0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        // e_flags = 0
        0x00, 0x00, 0x00, 0x00,
        // e_ehsize = 64
        0x40, 0x00,
        // e_phentsize = 56
        0x38, 0x00,
        // e_phnum = 0
        0x00, 0x00,
        // e_shentsize = 64
        0x40, 0x00,
        // e_shnum = 1
        0x01, 0x00,
        // e_shstrndx = 0
        0x00, 0x00,
        // Null section header (64 bytes)
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    #[test]
    fn parse_minimal_elf64() {
        let result = parse_elf(&MINIMAL_ELF64);
        assert!(result.is_ok(), "parse_elf failed: {:?}", result.err());
        let elf = result.unwrap();
        assert_eq!(elf.entry, 0);
        assert_eq!(elf.arch, 0x3e); // EM_X86_64
        assert_eq!(elf.endianness, 1); // ELFDATA2LSB
        assert_eq!(elf.sections.len(), 1);
        assert_eq!(elf.sections[0].name, "");
        assert_eq!(elf.sections[0].addr, 0);
        assert_eq!(elf.sections[0].size, 0);
        assert_eq!(elf.sections[0].flags, 0);
        assert!(elf.symbols.is_empty());
    }

    #[test]
    fn parse_elf_with_invalid_magic() {
        let mut bytes = MINIMAL_ELF64.to_vec();
        bytes[0] = 0x00;
        let result = parse_elf(&bytes);
        assert!(result.is_err());
        match result.unwrap_err() {
            LoaderError::ParseError(_) => {}
        }
    }

    #[test]
    fn parse_empty_bytes() {
        let result = parse_elf(&[]);
        assert!(result.is_err());
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn parse_elf_fixture_from_system() {
        // Attempt to read /bin/echo (or any ELF binary) if it exists.
        let path = "/bin/echo";
        if let Ok(bytes) = std::fs::read(path) {
            let result = parse_elf(&bytes);
            assert!(result.is_ok(), "failed to parse {}: {:?}", path, result.err());
            let elf = result.unwrap();
            assert!(!elf.sections.is_empty());
            // Some systems strip symbols, so we don't assert symbols are non-empty.
        }
    }
}
