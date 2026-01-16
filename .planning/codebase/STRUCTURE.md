# Codebase Structure

**Analysis Date:** 2026-01-16

## Directory Layout

```
mommy/
├── .cargo/           # Cargo configuration
├── .github/          # CI/CD workflows and project management
│   ├── agents/      # GitHub agent configs
│   ├── instructions/# GitHub instructions
│   └── workflows/   # Build and security workflows
├── assets/          # Embedded resources
├── src/             # Rust source code
├── Cargo.toml       # Project manifest
├── Cargo.lock       # Dependency lockfile
├── README.md        # User documentation
├── LICENSE          # MIT license
└── PLAN.md          # Development roadmap
```

## Directory Purposes

**src/**
- Purpose: All Rust source code
- Contains: *.rs files with module structure
- Key files: main.rs (entry), mommy.rs (core), config.rs (settings)
- Subdirectories: None (flat structure)

**.cargo/**
- Purpose: Cargo build configuration
- Contains: config.toml with target-specific compiler settings
- Key files: config.toml - Optimization flags and ARM64 cross-compilation setup
- Subdirectories: None

**.github/**
- Purpose: GitHub automation and project management
- Contains: Workflows for CI/CD, dependabot configuration
- Key files: workflows/build.yml (main CI), workflows/rust-clippy.yml (security)
- Subdirectories: agents/, instructions/, workflows/

**assets/**
- Purpose: Embedded JSON data for affirmations
- Contains: affirmations.json with mood-based message templates
- Key files: affirmations.json - Default affirmations in JSON format
- Subdirectories: None

## Key File Locations

**Entry Points:**
- `src/main.rs` - CLI entry point (414 bytes)
- `src/mommy.rs` - Core application logic (7,691 bytes)

**Configuration:**
- `Cargo.toml` - Project manifest and dependencies
- `.cargo/config.toml` - Build optimization and cross-compilation
- `rust-toolchain.toml` - Rust version pinning
- `rustfmt.toml` - Code formatting rules

**Core Logic:**
- `src/config.rs` - Environment variable handling and configuration (10,667 bytes)
- `src/affirmations.rs` - Affirmation loading and selection (7,289 bytes)
- `src/color.rs` - Terminal styling and color management (5,660 bytes)
- `src/utils.rs` - Template processing and utilities (3,489 bytes)

**Testing:**
- Tests co-located in source files using #[cfg(test)] modules
- 19 total unit tests across all modules

**Documentation:**
- `README.md` - User installation and usage guide
- `PLAN.md` - Development roadmap and improvement plan
- `LICENSE` - MIT license

## Naming Conventions

**Files:**
- snake_case.rs: All Rust source files
- kebab-case.toml: Configuration files
- UPPERCASE.md: Important documentation files

**Directories:**
- lowercase: All directories use lowercase names
- No special naming patterns

**Special Patterns:**
- main.rs: Application entry point (Rust convention)
- mod.rs: Not used (flat module structure)

## Where to Add New Code

**New Core Feature:**
- Primary code: Add to existing modules in `src/` or create new module
- Tests: Add #[cfg(test)] module in same file
- Config if needed: Extend `src/config.rs`

**New Configuration Option:**
- Implementation: Add to `src/config.rs` ConfigMommy struct
- Environment variables: Add to dual-prefix system
- Tests: Add to `src/config.rs` test module

**New Affirmation Mood:**
- Implementation: Update `assets/affirmations.json` structure
- Processing: Extend `src/affirmations.rs` mood handling
- Tests: Add mood-specific test cases

**Build/CI Changes:**
- Workflows: Modify `.github/workflows/build.yml`
- Dependencies: Update `Cargo.toml` and run cargo update
- Packaging: Modify Debian package generation in build.yml

## Special Directories

**assets/**
- Purpose: Embedded resources compiled into binary
- Source: Static JSON data for default affirmations
- Committed: Yes (part of distribution)

**.planning/**
- Purpose: Generated codebase documentation (this directory)
- Source: Created by codebase mapping analysis
- Committed: No (development tool output)

**.github/**
- Purpose: GitHub platform integration
- Source: CI/CD workflows, security scanning, dependabot
- Committed: Yes (part of repository configuration)

---

*Structure analysis: 2026-01-16*
*Update when directory structure changes*