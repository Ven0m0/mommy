# Technology Stack

**Analysis Date:** 2026-01-16

## Languages

**Primary:**
- Rust 2021 Edition - All application code

**Secondary:**
- Shell scripting - GitHub Actions workflows
- Markdown - Documentation files

## Runtime

**Environment:**
- Rust Nightly - Configured in `rust-toolchain.toml`
- Multi-platform compilation targets: Linux GNU/musl, Windows MSVC

**Package Manager:**
- Cargo - Rust package manager
- Lockfile: `Cargo.lock` present (version 4)

## Frameworks

**Core:**
- None - Vanilla Rust CLI application

**Testing:**
- Cargo test - Built-in Rust test framework with #[test] annotations
- 19 unit tests across modules (`src/config.rs`, `src/affirmations.rs`, `src/color.rs`, `src/utils.rs`)

**Build/Dev:**
- Cargo - Build system and dependency management
- Rustfmt - Code formatting (configured in `rustfmt.toml`)
- Clippy - Linting (enforced in CI with -D warnings)

## Key Dependencies

**Critical:**
- fastrand 2.3.0 - Random selection for affirmations, colors, emotes, moods
- owo-colors 4.2.3 - Terminal color styling and text effects (bold, italic, etc.)
- serde_json 1.0.149 - JSON parsing for custom affirmations file
- serde 1.0.228 - Serialization framework with derive macros

**Infrastructure:**
- None - Uses only Rust standard library for core functionality

## Configuration

**Environment:**
- Dual-prefix environment variables: SHELL_MOMMYS_* and CARGO_MOMMYS_*
- Custom affirmations via JSON file path: `SHELL_MOMMYS_AFFIRMATIONS`
- Aliases file sourcing: `SHELL_MOMMYS_ALIASES`

**Build:**
- `.cargo/config.toml` - Target-specific compiler flags and optimization
- `rustfmt.toml` - Code formatting configuration
- `rust-toolchain.toml` - Rust version pinning

## Platform Requirements

**Development:**
- Any platform with Rust toolchain
- No external dependencies or services required

**Production:**
- Distributed as pre-compiled binaries via GitHub Releases
- Debian packages available (.deb format)
- Can be installed via cargo install from crates.io

---

*Stack analysis: 2026-01-16*
*Update after major dependency changes*