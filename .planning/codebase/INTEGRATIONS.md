# External Integrations

**Analysis Date:** 2026-01-16

## APIs & External Services

**Not Detected** - This application operates entirely offline with no external API dependencies.

## Data Storage

**File System:**
- Local files only - Custom affirmations JSON file via environment variable
  - Configuration: `SHELL_MOMMYS_AFFIRMATIONS` / `CARGO_MOMMYS_AFFIRMATIONS`
  - Format: JSON file matching `assets/affirmations.json` schema
  - Fallback: Embedded defaults in binary

**Embedded Assets:**
- Affirmations data - Built into binary at compile time
  - Source: `assets/affirmations.json`
  - Access: Direct memory access (no file I/O)
  - Content: Mood-based affirmation templates with 3 moods (chill, ominous, thirsty)

## Authentication & Identity

**Not Applicable** - No user authentication or identity management required.

## System Integration

**Shell Integration:**
- Command execution via bash - `src/mommy.rs` line 218
  - Method: `std::process::Command::new("bash").arg("-c")`
  - Purpose: Execute user commands with aliases support
  - Security: User commands passed through bash -c (potential shell injection risk)

**Cargo Integration:**
- Cargo subcommand - Binary detection and command delegation
  - Implementation: `src/mommy.rs` lines 47-109 (role detection)
  - Integration method: Direct cargo command execution via `std::process::Command`
  - Binary naming: `cargo-mommy` enables `cargo mommy` usage

**Bash Aliases Integration:**
- Aliases file sourcing - `src/mommy.rs` lines 208-216
  - Configuration: `SHELL_MOMMYS_ALIASES` / `CARGO_MOMMYS_ALIASES`
  - Method: `shopt -s expand_aliases; source "${aliases_path}"; ${command}`
  - Purpose: Support user-defined bash aliases in executed commands

## CI/CD & Deployment

**GitHub Actions:**
- Build and Release Pipeline - `.github/workflows/build.yml`
  - Triggers: Push to main/master, tags v*, pull requests, manual dispatch
  - Platforms: Linux GNU, Linux musl, Windows MSVC
  - Artifacts: Pre-compiled binaries and Debian packages

**Security Analysis:**
- Clippy Security Scan - `.github/workflows/rust-clippy.yml`
  - Schedule: Weekly (Wednesday 13:45 UTC)
  - Method: SARIF upload to GitHub Security tab
  - Tools: clippy-sarif, sarif-fmt

**Dependency Management:**
- Dependabot - `.github/dependabot.yml`
  - Scope: Cargo dependencies and GitHub Actions
  - Frequency: Weekly updates

## Package Distribution

**GitHub Releases:**
- Automated release creation on version tags
  - Binaries: mommy-linux-amd64, mommy-linux-amd64-musl, mommy-windows-amd64.exe
  - Packages: shell-mommy_0.1.5_amd64.deb

**Crates.io:**
- Rust package registry - Inferred from `Cargo.toml` metadata
  - Package name: shell-mommy
  - Keywords: "mommy", "silly", "shell", "cli"
  - License: Specified in Cargo.toml

**Debian Packaging:**
- Native .deb package generation in CI
  - Package: shell-mommy
  - Architecture: amd64
  - Binaries: /usr/bin/mommy, /usr/bin/cargo-mommy

## Environment Configuration

**Development:**
- No required external services
- Optional: Custom affirmations JSON file
- Optional: Bash aliases file for enhanced shell integration

**Production:**
- No production-specific configuration
- Same environment variables apply across all environments
- No secrets or credentials required

## Security Considerations

**Shell Command Execution:**
- Risk: Potential shell injection in bash -c execution (`src/mommy.rs` line 218)
- Mitigation: Commands come from user CLI arguments (already trusted context)
- Recommendation: Consider shell escaping for additional safety

**File Access:**
- Risk: Arbitrary file reading for custom affirmations and aliases
- Mitigation: User-controlled file paths (intentional feature)
- Current: No validation beyond JSON parsing for affirmations

---

*Integration audit: 2026-01-16*
*Update when adding/removing external services*