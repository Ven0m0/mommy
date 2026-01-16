# Architecture

**Analysis Date:** 2026-01-16

## Pattern Overview

**Overall:** Single-Binary CLI Application with Dual Entry Points

**Key Characteristics:**
- Single executable with two modes: `mommy` (shell wrapper) and `cargo-mommy` (Cargo subcommand)
- Stateless execution model (no persistent state)
- File-based configuration via environment variables
- Embedded asset system for default affirmations

## Layers

**Entry Layer:**
- Purpose: Parse command-line arguments and detect execution mode
- Contains: Binary name detection, argument routing
- Location: `src/main.rs` (414 bytes - minimal entry point)
- Depends on: Core application layer
- Used by: Shell users and Cargo integration

**Core Application Layer:**
- Purpose: Main business logic and command execution
- Contains: Command wrapping, affirmation selection, output formatting
- Location: `src/mommy.rs` (7,691 bytes)
- Depends on: Configuration, Utilities, Color, Affirmations modules
- Used by: Entry layer

**Configuration Layer:**
- Purpose: Environment variable processing and settings management
- Contains: Dual-prefix env var handling, defaults, role transformation
- Location: `src/config.rs` (10,667 bytes)
- Depends on: Standard library only
- Used by: Core application layer

**Utility Layer:**
- Purpose: Shared helpers and string processing
- Contains: Template filling, random selection, output utilities
- Location: `src/utils.rs` (3,489 bytes), `src/color.rs` (5,660 bytes), `src/affirmations.rs` (7,289 bytes)
- Depends on: External crates (fastrand, owo-colors, serde_json)
- Used by: Core application layer

## Data Flow

**Command Execution Flow:**

1. User runs: `mommy <command>` or `cargo mommy <command>`
2. Binary detects mode via executable name (`src/main.rs`)
3. Configuration loaded from environment variables (`src/config.rs` lines 69-92)
4. Role transformation applied if binary name contains role (`src/mommy.rs` lines 47-109)
5. Command filtered and prepared (`src/mommy.rs` lines 160-180)
6. Shell command executed via `bash -c` or cargo command directly
7. Affirmation selected based on exit code and mood (`src/affirmations.rs`)
8. Output styled and printed to stderr (`src/color.rs`, `src/utils.rs`)

**State Management:**
- Stateless - No persistent state between executions
- Configuration loaded fresh each time
- All data derived from environment variables and command arguments

## Key Abstractions

**ConfigMommy:**
- Purpose: Centralized configuration with parsed environment variables
- Location: `src/config.rs` lines 26-48
- Pattern: Struct with validation and defaults

**Affirmation System:**
- Purpose: Mood-based message selection with template interpolation
- Examples: `src/affirmations.rs` (embedded JSON), `src/utils.rs` (template filling)
- Pattern: JSON-based data with placeholder replacement

**Color/Style System:**
- Purpose: Terminal output styling with random selection
- Location: `src/color.rs`
- Pattern: Style composition with RGB and named color support

## Entry Points

**CLI Entry:**
- Location: `src/main.rs`
- Triggers: Direct execution as `mommy` or `cargo-mommy`
- Responsibilities: Detect mode, delegate to core application

**Core Application:**
- Location: `src/mommy.rs` main function
- Triggers: Called from main entry point
- Responsibilities: Load config, execute command, show affirmation

## Error Handling

**Strategy:** Propagate errors up to main, graceful degradation where possible

**Patterns:**
- Use Result<> types throughout (`src/mommy.rs` returns `Result<()>`)
- Graceful printing utility handles stderr write failures (`src/utils.rs` line 82-86)
- Default values provided for missing configuration
- Custom affirmations fall back to embedded defaults

## Cross-Cutting Concerns

**Logging:**
- Print to stderr via `graceful_print` utility
- No structured logging framework
- Output suppression via quiet mode detection

**Validation:**
- Environment variable parsing with fallbacks
- JSON schema validation for custom affirmations
- Command filtering for shell execution

**Performance:**
- Single-pass string template replacement (`src/utils.rs` lines 25-80)
- Pre-parsed configuration vectors to avoid repeated parsing
- Embedded assets to avoid file I/O for defaults

---

*Architecture analysis: 2026-01-16*
*Update when major patterns change*