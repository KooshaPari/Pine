# Pine — Wine-Equivalent for Phenotype

**Pine** is a compatibility layer enabling Windows, macOS, and Linux applications to run on Phenotype-native infrastructure. Like Wine translates Windows syscalls to POSIX, Pine translates applications into Phenotype execution environments.

## PINE where PINE is an enhancemnt over WINE similar to Proton and Re-eng of Crossover  + Adaptation to Phenotype OS targets
## Status

**PRE-ALPHA** — Foundation bootstrapped 2026-04-30.

## Architecture

See `docs/ARCHITECTURE.md` for layer design.

## Competitor Analysis

| Project | What it does | Pine relevance |
|---------|-------------|----------------|
| **Wine** | Windows→Linux syscall translation | Canonical reference |
| **Proton** | Wine + DXVK/GE-Proton for Steam | DirectX→Vulkan translation |
| **Box86/Box64** | x86/ARM binary translation | Interpreter-level emulation |
| **Darling** | macOS apps on Linux | Cocoa→GTK translation |
| **QEMU** | Full hardware emulation | Emulation reference |
| **OrbStack** | Docker Desktop replacement | Docker-less containerization |
| **nvms/nanovms** | Phenotype microVM runtime | Isolation layer to build on |
| **Firecracker** | Lightweight microVMs | MicroVM isolation reference |
| **Docker Desktop** | Container runtime | Docker API compatibility |

## Why Pine

Phenotype needs application compatibility. Users will want to run Windows apps (legacy LOB, games), macOS apps (native tooling), Linux GUI apps, Android apps, and other cross-platform binaries. Pine is the translation and compatibility layer.

## Key Research Questions

1. **Translation scope**: Syscall-only (Wine) vs. binary translation (Box86)?
2. **Isolation**: Use nvms microVMs for untrusted code, native execution for trusted?
3. **Performance target**: Wine ~5-15% overhead vs QEMU 10-1000%?
4. **Win32 surface**: Which APIs first? Start with filesystem + process + networking?
5. **DXVK**: Can we leverage Proton's DXVK/vkd3d-proton?
6. **nvms integration**: Build isolation layer on existing Firecracker runtime?

## Stack

- Core: Rust (translation layer, syscall emulation)
- Runtime: Go (orchestration, lifecycle management)
- Agents: Python/TypeScript (testing harness)
