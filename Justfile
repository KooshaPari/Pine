# Pine Justfile
set shell := ["bash", "-cu"]

# Show available commands
default:
    @just --list

# Build docs
build-docs:
    cd docs && mdbook build 2>/dev/null || echo "mdbook not installed; run: cargo install mdbook"

# Serve docs locally
serve-docs:
    cd docs && mdbook serve 2>/dev/null || echo "mdbook not installed"

# Lint markdown
lint-md:
    npx markdownlint-cli "docs/**/*.md" 2>/dev/null || echo "markdownlint not installed"

# Check dead links
links:
    npx markdown-link-check "docs/**/*.md" 2>/dev/null || echo "markdown-link-check not installed"

# CI-like run
ci: lint-md

# Scaffold Cargo workspace (run once)
scaffold-rust:
    mkdir -p crates/pine-core/src
    mkdir -p crates/pine-loader/src
    mkdir -p crates/pine-syscall/src
    mkdir -p crates/pine-compat/src
    mkdir -p crates/pine-nvms/src
    echo "Scaffolded hexagonal crates. Add Cargo.toml to each."
