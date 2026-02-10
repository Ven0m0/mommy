# AGENTS.md

Comprehensive guide for AI agents working with the **mommy** CLI tool.

## Project Overview

**mommy** is a terminal affirmation tool written in Rust that provides positive feedback for shell commands. It's a reimplementation of shell-mommy and cargo-mommy with dual-mode operation support.

**Key Facts:**
- **Language:** Rust (edition 2021)
- **Version:** 0.1.6 (source of truth: Cargo.toml)
- **Binary Size:** ~400KB (stripped release build)
- **Test Coverage:** 24 unit tests across 6 modules
- **License:** Unlicense (public domain)
- **Repository:** https://github.com/Ven0m0/mommy

### Tech Stack

| Component | Technology | Purpose |
|-----------|-----------|---------|
| Language | Rust 2021 | Core implementation |
| CLI Binary | Dual-mode executable | Shell wrapper + cargo subcommand |
| Terminal Colors | owo-colors 4.2 | ANSI styling |
| Random | fastrand 2.3.0 | Affirmation/color selection |
| Serialization | serde 1.0 + serde_json 1.0 | JSON parsing |
| CI/CD | GitHub Actions | Test, build, release automation |
| Build System | Cargo | Standard Rust toolchain |

### Core Capabilities

1. **Shell Wrapper Mode:** `mommy <command>` - wraps any shell command with affirmations
2. **Cargo Integration:** `cargo mommy <command>` - integrates with cargo workflows
3. **Mood System:** JSON-based affirmations with different emotional tones
4. **Template Engine:** Dynamic text generation with placeholders
5. **Dual Configuration:** Separate env vars for shell vs cargo modes

## Repository Structure

```
mommy/
├── @src/main.rs              # Entry point (414 bytes)
├── @src/mommy.rs             # Core logic, command execution (~8KB)
├── @src/config.rs            # Environment variable handling (~4KB)
├── @src/affirmations.rs      # Mood system, JSON loading (~6KB)
├── @src/color.rs             # Terminal styling (~3KB)
├── @src/utils.rs             # Template processing (~2KB)
├── @assets/affirmations.json # Default messages (212 lines, embedded)
├── @Cargo.toml               # Package manifest (v0.1.6)
├── @.github/workflows/build.yml      # Main CI/CD pipeline
├── @.github/workflows/rust-clippy.yml # Security analysis
├── .cargo/config.toml        # Cross-compilation settings
├── rustfmt.toml              # Code formatting rules
├── rust-toolchain.toml       # Rust version pinning
├── PLAN.md                   # Development roadmap (47 features)
├── README.md                 # User documentation
└── PKGBUILD                  # Arch Linux packaging
```

### Module Responsibilities

| Module | Lines | Tests | Purpose |
|--------|-------|-------|---------|
| `main.rs` | 414B | 0 | Binary entry point routing |
| `mommy.rs` | ~8KB | 2 | Command execution, role detection, output formatting |
| `config.rs` | ~4KB | 5 | Environment variable parsing with dual-prefix support |
| `affirmations.rs` | ~6KB | 6 | JSON loading, mood selection, template instantiation |
| `color.rs` | ~3KB | 5 | ANSI color parsing and styling |
| `utils.rs` | ~2KB | 6 | Template substitution engine |

## Development Workflows

### Initial Setup

```bash
# Prerequisites
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh  # Install Rust

# Clone and setup
git clone https://github.com/Ven0m0/mommy
cd mommy

# Verify environment
cargo --version  # Should be 1.70+ (Rust 2021 edition)
cargo test       # Run all 24 tests
```

### Build Commands

```bash
# Development
cargo build                    # Debug build (~2MB)
cargo build -r                 # Release build (~400KB)

# Testing execution modes
cargo run -- ls -la            # Shell wrapper mode
cargo run --bin cargo-mommy build  # Cargo subcommand mode

# Cross-compilation
cargo build -r --target x86_64-unknown-linux-musl   # Static Linux
cargo build -r --target x86_64-pc-windows-msvc      # Windows
cargo build -r --target x86_64-unknown-linux-gnu    # Linux GNU libc

# Install locally
cargo install --path .
```

### Test Workflow

```bash
# Run all tests
cargo test                     # All 24 tests with output capture
cargo test -- --nocapture      # Show println! output
cargo test --lib               # Library tests only

# Specific modules
cargo test config::            # Config module (5 tests)
cargo test affirmations::      # Affirmations module (6 tests)
cargo test color::             # Color module (5 tests)
cargo test utils::             # Utils module (6 tests)
cargo test mommy::             # Integration tests (2 tests)

# Thread safety note: Tests use LazyLock<Mutex<()>> for env var isolation
# Deterministic random: Tests use fastrand::seed(42) for reproducibility
```

### Quality Checks (Pre-commit Requirements)

```bash
# All of these MUST pass before committing
cargo test                     # All tests must pass
cargo clippy -- -D warnings    # Zero warnings (CI enforced)
cargo fmt --check              # Formatting check
cargo fmt                      # Auto-format code

# Security analysis
cargo clippy -- -D clippy::all -D clippy::pedantic
```

### CI/CD Pipeline

**Workflow: `.github/workflows/build.yml`**
- **Triggers:** push to main/master, PRs, tags (v*)
- **Jobs:**
  1. `test` - Run tests, clippy, format check
  2. `build` - Parallel builds for 3 targets (Linux GNU, Linux musl, Windows)
  3. `build-packages` - Create Debian package
  4. `release` - Upload artifacts to GitHub releases

**Build Targets:**
```bash
x86_64-unknown-linux-gnu      → mommy-linux-amd64
x86_64-unknown-linux-musl     → mommy-linux-amd64-musl (static)
x86_64-pc-windows-msvc        → mommy-windows-amd64.exe
# Debian package               → shell-mommy_0.1.5_amd64.deb
```

### Release Process

```bash
# 1. Update version in Cargo.toml
sed -i 's/version = "0.1.6"/version = "0.1.7"/' Cargo.toml

# 2. Create and push tag
git tag -a v0.1.7 -m "Release v0.1.7: <description>"
git push origin v0.1.7

# 3. CI automatically:
#    - Runs all tests
#    - Builds all platform targets
#    - Creates GitHub release
#    - Uploads artifacts
```

## Conventions

### Code Style

**Rust Formatting (`rustfmt.toml`):**
```toml
edition             = "2021"
format_macro_bodies = true
format_strings      = true
imports_granularity = "Crate"    # Group imports by crate
reorder_impl_items  = true       # Alphabetize impl blocks
reorder_imports     = true       # Sort import statements
reorder_modules     = true       # Sort module declarations
wrap_comments       = true       # Wrap long comments
```

**Design Principles:**
1. **Stateless Execution** - No persistent state or configuration files
2. **Embedded Assets** - All data compiled into binary (zero runtime dependencies)
3. **Minimal Error Handling** - Only validate at system boundaries (user input, file I/O)
4. **No Premature Optimization** - Three similar lines > unnecessary abstraction
5. **Zero Backwards Compatibility Hacks** - Delete unused code completely

### Naming Conventions

**Environment Variables:**
- Shell mode: `SHELL_MOMMYS_<VARIABLE>` (e.g., `SHELL_MOMMYS_ROLES`)
- Cargo mode: `CARGO_MOMMYS_<VARIABLE>` (e.g., `CARGO_MOMMYS_MOODS`)
- Fallback hierarchy: Specific prefix → Generic `MOMMYS_*` → Defaults

**Binary Detection:**
- Executable name determines role (e.g., "daddy" from `cargo-daddy`)
- Logic located in `src/mommy.rs:47-109`
- Critical for dual-mode operation

### Commit Conventions

Use [Conventional Commits](https://www.conventionalcommits.org/) format:

```
feat: add JSON output mode flag
fix: correct color parsing for hex values
test: add integration tests for template engine
docs: update README with cargo-mommy examples
ci: extract version dynamically from Cargo.toml
chore: update dependencies to latest versions
```

### Architecture Patterns

**Dual Entry Point Design:**
```rust
// src/main.rs - Routes to appropriate mode
fn main() -> Result<()> {
    if is_cargo_subcommand() {
        cargo_mode()
    } else {
        shell_mode()
    }
}
```

**Template System:**
- Placeholders: `{roles}`, `{pronouns}`, `{little}`, `{emotes}`
- Processed by `src/utils.rs`
- Supports random selection from arrays

**Configuration Precedence:**
1. Environment variables (prefix-specific)
2. Environment variables (generic `MOMMYS_*`)
3. Hardcoded defaults

## Dependencies

### Runtime Dependencies

```toml
[dependencies]
fastrand = "2.3.0"              # Fast, non-cryptographic RNG
owo-colors = "4.2"              # Zero-alloc terminal colors
serde_json = "1.0"              # JSON parsing (efficient, battle-tested)
serde = { version = "1.0", features = ["derive"] }
```

**Dependency Rationale:**
- `fastrand`: No need for cryptographic security in affirmation selection
- `owo-colors`: Zero-allocation ANSI coloring (performance critical)
- `serde_json`: De facto standard for JSON in Rust ecosystem

### Build Configuration

```toml
[profile.release]
codegen-units = 1         # Single codegen unit for max optimization
strip = true              # Remove debug symbols
lto = "fat"               # Full link-time optimization
opt-level = 3             # Maximum optimization
debug = false             # No debug info
debug-assertions = false  # Disable runtime checks
overflow-checks = false   # Assume no integer overflow
panic = "abort"           # Smaller binary, faster panic
rpath = false             # No runtime library path
```

**Result:** ~400KB stripped binary (vs ~2MB debug build)

### Cross-Compilation Setup

```toml
[workspace.metadata.cross.target.aarch64-unknown-linux-gnu]
pre-build = [
  "dpkg --add-architecture $CROSS_DEB_ARCH",
  "apt-get update -y && apt-get -y install libssl-dev:$CROSS_DEB_ARCH"
]
```

## Common Tasks

### Adding a New Affirmation Mood

1. **Edit `assets/affirmations.json`:**
```json
{
  "moods": {
    "new_mood": {
      "positive": ["great job, {little}~"],
      "negative": ["it's okay, {little}"]
    }
  }
}
```

2. **Run tests to validate JSON:**
```bash
cargo test affirmations::tests::test_load_affirmations
```

3. **Build and test:**
```bash
cargo build -r
SHELL_MOMMYS_MOODS=new_mood ./target/release/mommy echo "test"
```

### Adding a Configuration Variable

1. **Add to `src/config.rs`:**
```rust
pub struct Config {
    pub new_option: String,
    // ... existing fields
}

impl Config {
    fn from_env(prefix: &str) -> Self {
        let new_option = env::var(format!("{prefix}NEW_OPTION"))
            .unwrap_or_else(|_| "default".to_string());
        // ...
    }
}
```

2. **Add tests in `src/config.rs`:**
```rust
#[test]
fn test_new_option() {
    let _guard = ENV_LOCK.lock();
    env::set_var("SHELL_MOMMYS_NEW_OPTION", "value");
    let config = Config::from_env("SHELL_MOMMYS_");
    assert_eq!(config.new_option, "value");
}
```

3. **Update documentation:**
- `README.md` - Add to environment variables table
- `AGENTS.md` - Update configuration section

### Debugging Test Failures

```bash
# Run specific test with output
cargo test test_name -- --nocapture

# Run tests serially (avoid env var conflicts)
cargo test -- --test-threads=1

# Enable backtraces
RUST_BACKTRACE=1 cargo test

# Debug build with symbols
cargo build && rust-gdb target/debug/mommy
```

### Fixing CI Failures

**Common issues:**
1. **Clippy warnings:** `cargo clippy -- -D warnings` (fix all warnings)
2. **Format check:** `cargo fmt` (auto-fix)
3. **Test failures:** `cargo test` (all must pass)
4. **Version mismatch:** Update hardcoded versions in `.github/workflows/build.yml`

### Performance Profiling

```bash
# Build with profiling
cargo build --release

# Profile with perf (Linux)
perf record -g ./target/release/mommy echo "test"
perf report

# Check binary size
ls -lh target/release/mommy
strip target/release/mommy  # Should be ~400KB
```

## Configuration Reference

### Environment Variables

| Variable | Shell Prefix | Cargo Prefix | Default | Example |
|----------|--------------|--------------|---------|---------|
| `ROLES` | `SHELL_MOMMYS_` | `CARGO_MOMMYS_` | "mommy" | "daddy/mommy" |
| `MOODS` | `SHELL_MOMMYS_` | `CARGO_MOMMYS_` | "chill" | "cute/ominous/yikes" |
| `AFFIRMATIONS` | `SHELL_MOMMYS_` | `CARGO_MOMMYS_` | (embedded) | "/path/to/custom.json" |
| `COLOR` | `SHELL_MOMMYS_` | `CARGO_MOMMYS_` | random | "pink/blue/#FF69B4" |
| `STYLE` | `SHELL_MOMMYS_` | `CARGO_MOMMYS_` | none | "bold/italic/underline" |

### Template Placeholders

| Placeholder | Description | Example Values |
|------------|-------------|----------------|
| `{roles}` | Role name(s) | "mommy", "daddy/mommy" |
| `{pronouns}` | Pronoun set | "she/her", "they/them" |
| `{little}` | Term of endearment | "dear", "sweetheart", "little one" |
| `{emotes}` | Emoji/emoticons | "💖", "uwu", "^_^" |

## Known Issues & Workarounds

### Critical: Version Mismatch in CI

**Problem:** Hardcoded version in `.github/workflows/build.yml` (lines 99, 108, 112) is `0.1.5`, but `Cargo.toml` has `0.1.6`.

**Solution:**
```bash
# Extract version dynamically
VERSION=$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version')
```

### Security Considerations

**Shell Injection Risk (`src/mommy.rs:218`):**
- Uses `bash -c` without escaping
- **Context:** Accepts user CLI args as trusted input (appropriate for CLI tool)
- **Not a vulnerability:** Commands originate from authenticated user

**File System Access:**
- Custom JSON files loaded from user-specified paths
- Bash aliases sourced from user files
- **Context:** Intentional features; users control their input

### Missing Features (See PLAN.md)

Not yet implemented:
- CLI flags: `--quiet`, `--json`, `--config`
- Integration tests with actual command execution
- Specific error types (currently uses generic `Result<()>`)
- Automated version extraction in CI
- Cargo publish workflow

## Quick Reference Card

```bash
# Essential commands
cargo test                              # Run all 24 tests
cargo clippy -- -D warnings             # Lint (must pass)
cargo fmt                               # Format code
cargo build -r                          # Release build

# Testing modes
cargo run -- ls -la                     # Shell wrapper
cargo run --bin cargo-mommy build       # Cargo subcommand

# Cross-platform
cargo build -r --target x86_64-unknown-linux-musl

# Release
git tag -a v0.1.7 -m "Release v0.1.7"
git push origin v0.1.7
```

## Additional Resources

- **Roadmap:** See `PLAN.md` for 47 planned features
- **User Docs:** See `README.md` for end-user guide
- **Upstream Projects:**
  - [Gankra/cargo-mommy](https://github.com/Gankra/cargo-mommy) - Original cargo integration
  - [sudofox/shell-mommy](https://github.com/sudofox/shell-mommy) - Original Bash implementation

---

**Last Updated:** 2026-02-10
**Document Version:** 1.0
**For:** AI Agents (Claude, Copilot, Gemini, etc.)
