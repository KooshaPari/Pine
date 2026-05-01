# Pine Architecture — Draft 2026-04-30

## Overview

Pine provides application compatibility through:
1. **Binary parsing** (PE/ELF/Mach-O loaders)
2. **Syscall translation** (Windows/macOS/Linux → Phenotype)
3. **API emulation** (Win32, Cocoa, POSIX compatibility)
4. **Isolation** (nvms microVMs for untrusted code)
5. **Performance** (native where possible, emulation where needed)

## Layer Architecture

```
┌──────────────────────────────────────┐
│  Application Layer                   │
│  (Windows .exe / macOS .app / Linux)│
├──────────────────────────────────────┤
│  Binary Loader                       │
│  (PE / ELF / Mach-O parser)         │
├──────────────────────────────────────┤
│  Syscall Emulation Layer            │
│  (Win32 → Phenotype / POSIX → Phenotype)│
├──────────────────────────────────────┤
│  Compatibility Libraries            │
│  (winelib equivalents)              │
├──────────────────────────────────────┤
│  nvms Isolation Layer               │
│  (Firecracker microVMs)            │
├──────────────────────────────────────┤
│  Phenotype Native Runtime           │
│  (Rust core + Go orchestration)    │
└──────────────────────────────────────┘
```

## nvms Integration Strategy

Pine SHOULD build on `nvms` (nanovms repo) as the isolation layer:
- Trusted code: Direct Phenotype-native execution
- Semi-trusted: gVisor sandbox (~90ms startup)
- Untrusted: Firecracker microVM (~125ms startup)

This mirrors nvms tier architecture: WASM (~1ms) / gVisor (~90ms) / Firecracker (~125ms).

## Phase 1: Foundation

1. Binary parsing: PE/ELF/Mach-O via `goblin` crate
2. Basic syscall: filesystem + process (Rust)
3. nvms integration: microVM-based isolation
4. Test harness: benchmark suite

## Phase 2: Windows Compatibility

1. Win32 API: kernel32, user32, gdi32, ws2_32 core
2. DXVK integration: DirectX→Vulkan translation
3. Wine testsuite: Pass Windows AppCompat tests
4. Performance target: <15% overhead

## Phase 3: Cross-Platform

1. macOS: Darling-inspired Cocoa→Phenotype
2. Linux: Enhanced POSIX compatibility
3. Android: Mobile application compatibility

## Key Dependencies

- `goblin`: Binary parsing (PE/ELF/Mach-O)
- `nvms`: MicroVM isolation layer
- Rust stable toolchain (MSRV TBD)
