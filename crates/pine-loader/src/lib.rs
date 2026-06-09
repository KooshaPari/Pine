//! pine-loader — ELF/PE loader adapter.

use pine_core::traits::Loader;

pub struct ElfLoader;

impl Loader for ElfLoader {
    fn load(&self, _path: &str) -> Result<Vec<u8>, String> {
        Ok(vec![])
    }
}

// ========== PE Loader ==========

use std::fmt;

/// Error type for loader operations.
#[derive(Debug)]
pub enum LoaderError {
    /// Error from the goblin parser.
    Goblin(goblin::error::Error),
    /// Invalid PE file.
    InvalidPE(String),
}

impl fmt::Display for LoaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoaderError::Goblin(e) => write!(f, "goblin error: {e}"),
            LoaderError::InvalidPE(msg) => write!(f, "invalid PE: {msg}"),
        }
    }
}

impl std::error::Error for LoaderError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            LoaderError::Goblin(e) => Some(e),
            LoaderError::InvalidPE(_) => None,
        }
    }
}

impl From<goblin::error::Error> for LoaderError {
    fn from(e: goblin::error::Error) -> Self {
        LoaderError::Goblin(e)
    }
}

/// Parsed representation of a PE section.
#[derive(Debug, Clone, PartialEq)]
pub struct PeSection {
    /// Section name.
    pub name: String,
    /// Virtual address.
    pub virtual_address: u64,
    /// Virtual size.
    pub virtual_size: u64,
    /// Size in file.
    pub raw_size: u64,
    /// Offset in file.
    pub raw_offset: u64,
    /// Section characteristics flags.
    pub characteristics: u32,
}

/// Parsed representation of a PE binary.
#[derive(Debug, Clone, PartialEq)]
pub struct PeBinary {
    /// Entry point RVA.
    pub entry_point: u64,
    /// Parsed sections.
    pub sections: Vec<PeSection>,
    /// Preferred image base address.
    pub image_base: u64,
    /// Whether the PE is 64-bit.
    pub is_64_bit: bool,
}

/// Parse a PE binary from raw bytes.
///
/// Uses `goblin::pe::PE` to parse headers, sections, and the entry point.
pub fn parse_pe(bytes: &[u8]) -> Result<PeBinary, LoaderError> {
    let pe = goblin::pe::PE::parse(bytes)?;

    let entry_point = pe.entry as u64;
    let image_base = pe.image_base;
    let is_64_bit = pe.is_64;

    let sections = pe
        .sections
        .iter()
        .map(|s| {
            let name = String::from_utf8_lossy(&s.name)
                .trim_end_matches('\0')
                .to_string();
            PeSection {
                name,
                virtual_address: s.virtual_address as u64,
                virtual_size: s.virtual_size as u64,
                raw_size: s.size_of_raw_data as u64,
                raw_offset: s.pointer_to_raw_data as u64,
                characteristics: s.characteristics,
            }
        })
        .collect();

    Ok(PeBinary {
        entry_point,
        sections,
        image_base,
        is_64_bit,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_u16(bytes: &mut [u8], offset: usize, value: u16) {
        bytes[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
    }

    fn write_u32(bytes: &mut [u8], offset: usize, value: u32) {
        bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    }

    /// Build a minimal valid PE32 file in memory.
    fn minimal_pe_bytes() -> Vec<u8> {
        let mut b = vec![0u8; 0x400];

        // DOS Header (64 bytes)
        b[0x00] = 0x4D; // 'M'
        b[0x01] = 0x5A; // 'Z'
        write_u32(&mut b, 0x3C, 0x40); // e_lfanew = 0x40

        // PE Signature at 0x40
        b[0x40] = 0x50; // 'P'
        b[0x41] = 0x45; // 'E'
        b[0x42] = 0x00;
        b[0x43] = 0x00;

        // COFF Header (20 bytes) at 0x44
        write_u16(&mut b, 0x44, 0x14C); // Machine: i386
        write_u16(&mut b, 0x46, 0x0001); // NumberOfSections: 1
        // TimeDateStamp (0x48), PointerToSymbolTable (0x4C), NumberOfSymbols (0x50) = 0
        write_u16(&mut b, 0x54, 0x00E0); // SizeOfOptionalHeader: 224
        write_u16(&mut b, 0x56, 0x0102); // Characteristics: EXECUTABLE_IMAGE | 32BIT_MACHINE

        // Optional Header (PE32, 224 bytes) at 0x58
        write_u16(&mut b, 0x58, 0x010B); // Magic: PE32
        b[0x5A] = 0x01; // MajorLinkerVersion
        // SizeOfCode: 0x200
        write_u32(&mut b, 0x5C, 0x200);
        // SizeOfInitializedData: 0x200
        write_u32(&mut b, 0x60, 0x200);
        // SizeOfUninitializedData: 0
        // AddressOfEntryPoint: 0x1000
        write_u32(&mut b, 0x68, 0x1000);
        // BaseOfCode: 0x1000
        write_u32(&mut b, 0x6C, 0x1000);
        // BaseOfData: 0x2000
        write_u32(&mut b, 0x70, 0x2000);
        // ImageBase: 0x10000
        write_u32(&mut b, 0x74, 0x10000);
        // SectionAlignment: 0x1000
        write_u32(&mut b, 0x78, 0x1000);
        // FileAlignment: 0x200
        write_u32(&mut b, 0x7C, 0x200);
        // MajorOperatingSystemVersion: 4
        write_u16(&mut b, 0x80, 0x4);
        // MajorSubsystemVersion: 4
        write_u16(&mut b, 0x88, 0x4);
        // SizeOfImage: 0x2000
        write_u32(&mut b, 0x90, 0x2000);
        // SizeOfHeaders: 0x200
        write_u32(&mut b, 0x94, 0x200);
        // CheckSum: 0
        // Subsystem: 1 (NATIVE)
        write_u16(&mut b, 0x9C, 0x1);
        // DllCharacteristics: 0
        // SizeOfStackReserve: 0x100000
        write_u32(&mut b, 0xA0, 0x100000);
        // SizeOfStackCommit: 0x1000
        write_u32(&mut b, 0xA4, 0x1000);
        // SizeOfHeapReserve: 0x100000
        write_u32(&mut b, 0xA8, 0x100000);
        // SizeOfHeapCommit: 0x1000
        write_u32(&mut b, 0xAC, 0x1000);
        // LoaderFlags: 0
        // NumberOfRvaAndSizes: 16
        write_u32(&mut b, 0xB4, 0x10);
        // Data directories: 16 * 8 = 128 bytes of zeros at 0xB8 (already zeroed)

        // Section Header (40 bytes) at 0x58 + 224 = 0x138
        // Name: ".text\0\0\0" (8 bytes)
        b[0x138] = 0x2E;
        b[0x139] = 0x74;
        b[0x13A] = 0x65;
        b[0x13B] = 0x78;
        b[0x13C] = 0x74;
        // VirtualSize: 0x200
        write_u32(&mut b, 0x140, 0x200);
        // VirtualAddress: 0x1000
        write_u32(&mut b, 0x144, 0x1000);
        // SizeOfRawData: 0x200
        write_u32(&mut b, 0x148, 0x200);
        // PointerToRawData: 0x200
        write_u32(&mut b, 0x14C, 0x200);
        // PointerToRelocations: 0
        // PointerToLinenumbers: 0
        // NumberOfRelocations: 0
        // NumberOfLinenumbers: 0
        // Characteristics: 0x60000020 (CODE | EXECUTE | READ)
        write_u32(&mut b, 0x15C, 0x60000020);

        b
    }

    #[test]
    fn parse_pe_minimal_fixture() {
        let bytes = minimal_pe_bytes();
        let pe = parse_pe(&bytes).expect("should parse minimal PE fixture");
        assert_eq!(pe.entry_point, 0x1000);
        assert_eq!(pe.image_base, 0x10000);
        assert!(!pe.is_64_bit);
        assert_eq!(pe.sections.len(), 1);
        assert_eq!(pe.sections[0].name, ".text");
        assert_eq!(pe.sections[0].virtual_address, 0x1000);
        assert_eq!(pe.sections[0].virtual_size, 0x200);
        assert_eq!(pe.sections[0].raw_size, 0x200);
        assert_eq!(pe.sections[0].raw_offset, 0x200);
        assert_eq!(pe.sections[0].characteristics, 0x60000020);
    }

    #[test]
    fn parse_pe_invalid_data() {
        let result = parse_pe(b"not a pe");
        assert!(result.is_err());
    }

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
