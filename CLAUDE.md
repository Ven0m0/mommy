# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

**Build and Test:**
```bash
cargo build                        # Build debug version
cargo build -r                     # Build release version
cargo test                         # Run all 19 unit tests
cargo clippy -- -D warnings        # Lint (enforced in CI)
cargo fmt --check                  # Check formatting
```

**Local Testing:**
```bash
cargo run -- ls -la                # Test as shell wrapper
cargo run --bin cargo-mommy build  # Test as cargo subcommand
```

**Cross-platform Build:**
```bash
cargo build -r --target x86_64-unknown-linux-musl  # Static Linux binary
cargo build -r --target x86_64-pc-windows-msvc     # Windows binary
```

## Architecture Overview

**Binary Pattern:** Single Rust executable with dual entry points:
- `mommy <command>` - Shell command wrapper mode
- `cargo mommy <command>` - Cargo subcommand mode

**Core Modules:**
- `src/main.rs` - Entry point (414 bytes, minimal)
- `src/mommy.rs` - Main application logic (command execution, affirmations)
- `src/config.rs` - Environment variable handling with dual-prefix system
- `src/affirmations.rs` - JSON-based mood system with embedded defaults
- `src/color.rs` - Terminal styling with random selection
- `src/utils.rs` - Template processing and utilities

**Key Design Patterns:**
- Stateless execution (no persistent data)
- Embedded assets (`assets/affirmations.json` compiled into binary)
- Dual environment variable prefixes: `SHELL_MOMMYS_*` and `CARGO_MOMMYS_*`
- Template-based affirmations with placeholders: `{roles}`, `{pronouns}`, `{little}`, `{emotes}`

## Configuration System

**Environment Variables (all optional):**
- `SHELL_MOMMYS_ROLES` / `CARGO_MOMMYS_ROLES` - Role names (default: "mommy")
- `SHELL_MOMMYS_MOODS` / `CARGO_MOMMYS_MOODS` - Mood selection (default: "chill")
- `SHELL_MOMMYS_AFFIRMATIONS` / `CARGO_MOMMYS_AFFIRMATIONS` - Custom JSON file path
- `SHELL_MOMMYS_COLOR` / `CARGO_MOMMYS_COLOR` - Terminal colors
- `SHELL_MOMMYS_STYLE` / `CARGO_MOMMYS_STYLE` - Text styling

**Binary Detection Logic (`src/mommy.rs:47-109`):**
- Detects role from executable name (e.g., "daddy" from "daddy" or "cargo-daddy")
- Transforms cargo subcommand binary names for role-based customization
- Critical for proper dual-mode operation

## Testing

**Test Organization:**
- Co-located tests using `#[cfg(test)]` modules in each source file
- 19 total unit tests across all modules
- Environment variable testing uses `LazyLock<Mutex<()>>` for isolation
- Deterministic random testing via `fastrand::seed(42)`

**Key Test Files:**
- `src/config.rs` - 5 tests (environment variable handling)
- `src/affirmations.rs` - 6 tests (mood system, JSON loading)
- `src/color.rs` - 5 tests (color parsing, styling)
- `src/utils.rs` - 1 test (template processing)
- `src/mommy.rs` - 2 tests (integration scenarios)

## Security Considerations

**Shell Injection Risk (`src/mommy.rs:218`):**
- Raw command execution via `bash -c` without shell escaping
- Commands come from user CLI args (trusted context) but consider escaping for defense in depth
- Aliases file sourcing also uses shell execution

**File Access:**
- Custom affirmations JSON loaded from user-specified paths
- Bash aliases sourced from user-specified files
- Both are intentional features but represent potential attack surface

## CI/CD Pipeline

**GitHub Actions:**
- `.github/workflows/build.yml` - Main CI (test, build, release)
- `.github/workflows/rust-clippy.yml` - Security analysis
- Multi-platform builds: Linux GNU/musl, Windows MSVC
- Debian package generation included in release workflow

**Release Process:**
- Tag-triggered releases (v*.*.*)
- Pre-compiled binaries attached to GitHub releases
- Version mismatch: Cargo.toml shows v0.1.6 but CI builds v0.1.5 Debian package

## Known Issues

**Version Management:**
- Hardcoded version in `.github/workflows/build.yml` lines 99, 108, 112
- Should extract from `Cargo.toml` to avoid mismatches

**Missing Production Features (see PLAN.md):**
- No `--quiet`, `--json`, `--config` CLI flags
- No integration tests with actual command execution
- Error handling could be more specific than generic `Result<()>`

## File Locations

**Key Files:**
- `Cargo.toml` - Dependencies and metadata
- `assets/affirmations.json` - Default mood-based messages (213 lines)
- `.cargo/config.toml` - Cross-compilation and optimization settings
- `PLAN.md` - Comprehensive improvement roadmap (47 planned features)

**Generated Documentation:**
- `.planning/codebase/` - Comprehensive codebase analysis (7 documents, 886 lines total)

## Dependencies

**Runtime (4 total):**
- `fastrand 2.3.0` - Random selection for affirmations/colors
- `owo-colors 4.2.3` - Terminal styling and ANSI colors
- `serde_json 1.0.149` - Custom affirmations JSON parsing
- `serde 1.0.228` - Serialization framework