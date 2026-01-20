# CLAUDE.md

Project guidance for Claude Code when working with the mommy CLI tool.

## Quick Reference

**Current Version:** 0.1.6 (Cargo.toml) / 0.1.5 (hardcoded in CI workflows)
**Test Suite:** 24 unit tests across 6 modules
**Binary Size:** ~400KB stripped release build
**Entry Point:** `src/main.rs` (414 bytes)

## Development Workflow

### Essential Commands
```bash
# Development cycle
cargo test                         # Run 24 unit tests
cargo clippy -- -D warnings        # Lint (CI enforced)
cargo fmt --check                  # Format check
cargo build -r                     # Release build

# Local testing
cargo run -- ls -la                # Shell wrapper mode
cargo run --bin cargo-mommy build  # Cargo subcommand mode

# Cross-platform builds
cargo build -r --target x86_64-unknown-linux-musl  # Static Linux
cargo build -r --target x86_64-pc-windows-msvc     # Windows
```

### Before Committing
1. Run `cargo test` - all 24 tests must pass
2. Run `cargo clippy -- -D warnings` - zero warnings
3. Run `cargo fmt` - ensure formatting
4. Test both execution modes (shell wrapper + cargo subcommand)

## Architecture

### Binary Design
Single Rust executable with dual entry points:
- **Shell mode:** `mommy <command>` wraps any shell command
- **Cargo mode:** `cargo mommy <command>` integrates with cargo

### Module Structure
| Module | Purpose | Size | Tests |
|--------|---------|------|-------|
| `src/main.rs` | Entry point | 414B | - |
| `src/mommy.rs` | Core logic, command execution | ~8KB | 2 |
| `src/config.rs` | Environment variable handling | ~4KB | 5 |
| `src/affirmations.rs` | Mood system, JSON loading | ~6KB | 6 |
| `src/color.rs` | Terminal styling | ~3KB | 5 |
| `src/utils.rs` | Template processing | ~2KB | 6 |

### Core Design Patterns
- **Stateless execution** - No persistent data or state files
- **Embedded assets** - `assets/affirmations.json` (212 lines) compiled into binary
- **Dual prefix system** - Separate env vars for shell vs cargo modes
- **Template engine** - Supports `{roles}`, `{pronouns}`, `{little}`, `{emotes}` placeholders

## Configuration

### Environment Variables
All variables are optional with sensible defaults:

| Variable | Shell Prefix | Cargo Prefix | Default | Purpose |
|----------|--------------|--------------|---------|---------|
| `ROLES` | `SHELL_MOMMYS_` | `CARGO_MOMMYS_` | "mommy" | Role name(s) |
| `MOODS` | `SHELL_MOMMYS_` | `CARGO_MOMMYS_` | "chill" | Mood selection |
| `AFFIRMATIONS` | `SHELL_MOMMYS_` | `CARGO_MOMMYS_` | - | Custom JSON path |
| `COLOR` | `SHELL_MOMMYS_` | `CARGO_MOMMYS_` | random | Terminal colors |
| `STYLE` | `SHELL_MOMMYS_` | `CARGO_MOMMYS_` | - | Text styling |

### Binary Detection Logic
Located in `src/mommy.rs:47-109`:
- Detects role from executable name (e.g., "daddy" from `cargo-daddy` binary)
- Transforms cargo subcommand names for role-based customization
- Critical for proper dual-mode operation

## Testing

### Test Structure
- **Co-located tests** - `#[cfg(test)]` modules in source files
- **24 total tests** - All must pass for CI
- **Thread safety** - Uses `LazyLock<Mutex<()>>` for env var isolation
- **Deterministic random** - Tests use `fastrand::seed(42)`

### Test Distribution
```
src/config.rs       →  5 tests (env var handling)
src/affirmations.rs →  6 tests (mood system, JSON)
src/color.rs        →  5 tests (color parsing)
src/utils.rs        →  6 tests (template processing)
src/mommy.rs        →  2 tests (integration)
```

### Running Tests
```bash
cargo test                    # All tests
cargo test --lib              # Library tests only
cargo test config::           # Specific module
cargo test -- --nocapture     # Show output
```

## Security Considerations

### Known Security Boundaries

**Shell Injection Risk** (`src/mommy.rs:218`):
- Uses `bash -c` without shell escaping for command execution
- Accepts user CLI args as trusted input (appropriate for CLI context)
- Aliases file sourcing also uses shell execution
- **Mitigation:** Commands originate from authenticated user, not external input

**File System Access**:
- Custom JSON files loaded from user-specified paths
- Bash aliases sourced from user-controlled files
- **Context:** Intentional features; users control input sources

### When Making Changes
- Never introduce SQL injection, XSS, or command injection vulnerabilities
- Validate external input at system boundaries
- Don't add unnecessary error handling for impossible scenarios
- Trust internal code and framework guarantees

## CI/CD

### GitHub Actions Workflows
- **`.github/workflows/build.yml`** - Test, build, release
  - Runs on: push to main/master, PRs, tags (v*)
  - Jobs: test → build (3 targets) → build-packages → release
  - Targets: Linux GNU, Linux musl, Windows MSVC
- **`.github/workflows/rust-clippy.yml`** - Security analysis

### Build Artifacts
- `mommy-linux-amd64` - GNU libc binary
- `mommy-linux-amd64-musl` - Static musl binary
- `mommy-windows-amd64.exe` - Windows binary
- `shell-mommy_0.1.5_amd64.deb` - Debian package

### Release Process
```bash
# Create release
git tag -a v0.1.7 -m "Release v0.1.7"
git push origin v0.1.7

# CI automatically:
# 1. Runs tests
# 2. Builds all targets
# 3. Creates GitHub release
# 4. Uploads artifacts
```

## Known Issues & TODOs

### Critical Issue: Version Mismatch
**Problem:** Version hardcoded in CI workflows differs from `Cargo.toml`
- `Cargo.toml`: `0.1.6`
- `.github/workflows/build.yml` lines 99, 108, 112: `0.1.5`

**Solution:** Extract version from `Cargo.toml` dynamically:
```bash
VERSION=$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version')
```

### Missing Features (see PLAN.md)
Production features not yet implemented:
- CLI flags: `--quiet`, `--json`, `--config`
- Integration tests with actual command execution
- Specific error types beyond generic `Result<()>`
- Automated version extraction in CI

## Dependencies

### Runtime (4 crates)
```toml
fastrand = "2.3.0"      # RNG for affirmations/colors
owo-colors = "4.2"      # Terminal styling/ANSI colors
serde_json = "1.0"      # Custom JSON parsing
serde = "1.0"           # Serialization (with derive feature)
```

### Build Configuration
- **Aggressive optimization** - LTO, single codegen unit, strip symbols
- **Panic abort** - Smaller binaries, faster compilation
- **Cross-compilation** - ARM64 support with custom pre-build steps

## Key Files

```
mommy/
├── src/
│   ├── main.rs           # 414 byte entry point
│   ├── mommy.rs          # Core logic
│   ├── config.rs         # Env var handling
│   ├── affirmations.rs   # Mood system
│   ├── color.rs          # Terminal styling
│   └── utils.rs          # Template engine
├── assets/
│   └── affirmations.json # 212 lines of default messages
├── .github/workflows/
│   ├── build.yml         # Main CI/CD pipeline
│   └── rust-clippy.yml   # Security analysis
├── .cargo/config.toml    # Cross-compilation settings
├── Cargo.toml            # v0.1.6 (source of truth)
├── PLAN.md               # Roadmap (47 planned features)
└── CLAUDE.md             # This file
```

## Development Guidelines

### Code Style
- **Avoid over-engineering** - Only implement requested features
- **No premature abstraction** - Three similar lines > unnecessary helper
- **Minimal error handling** - Only validate at system boundaries
- **No backwards compatibility hacks** - Delete unused code completely
- **Keep it simple** - Bug fixes don't need refactoring

### When Adding Features
1. Check PLAN.md for alignment with roadmap
2. Add tests first (TDD approach)
3. Update this file if architecture changes
4. Run full test suite before committing
5. Consider impact on both shell and cargo modes

### Commit Conventions
Use conventional commits (see PLAN.md principles):
- `feat:` - New features
- `fix:` - Bug fixes
- `test:` - Test additions/changes
- `docs:` - Documentation
- `ci:` - CI/CD changes
- `chore:` - Maintenance tasks