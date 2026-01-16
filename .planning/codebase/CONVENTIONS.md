# Coding Conventions

**Analysis Date:** 2026-01-16

## Naming Patterns

**Files:**
- snake_case for all Rust files (main.rs, config.rs, affirmations.rs)
- kebab-case for configuration files (rust-toolchain.toml, but not consistently applied)
- UPPERCASE for documentation (README.md, LICENSE, PLAN.md)

**Functions:**
- snake_case for all functions (load_config, random_vec_pick, fill_template)
- No special prefix for async functions (none used in this codebase)
- Helper functions follow same pattern (graceful_print, apply_style_attr)

**Variables:**
- snake_case for variables (config, raw_command, aliases_path)
- UPPER_SNAKE_CASE for constants (none defined, uses literals)
- No underscore prefix for private members (Rust privacy via modules)

**Types:**
- PascalCase for structs (ConfigMommy, Affirmations)
- snake_case for modules (affirmations, config, color, utils)
- PascalCase for trait implementations (not extensively used)

## Code Style

**Formatting:**
- Rustfmt with default configuration in `rustfmt.toml`
- 4-space indentation (configured in `.editorconfig`)
- No line length specified (rustfmt defaults)
- Consistent spacing around operators

**Linting:**
- Clippy enforced with -D warnings in CI (`.github/workflows/build.yml` line 27)
- Standard Rust linting rules applied
- No custom clippy configuration

## Import Organization

**Order:**
1. Standard library imports (std::env, std::process::Command)
2. External crate imports (fastrand, owo_colors, serde_json)
3. Internal module imports (crate::config, crate::utils)
4. Conditional imports (#[cfg(test)] use statements)

**Grouping:**
- Imports grouped by source with blank lines between groups
- Multiple items from same module: use module::{item1, item2}
- Example in `src/color.rs`: `use owo_colors::{DynColors, Style, OwoColorize}`

**Path Aliases:**
- Use crate:: for internal modules
- No path aliases configured

## Error Handling

**Patterns:**
- Use Result<T, E> types throughout (`src/mommy.rs` returns Result<()>)
- Propagate errors with ? operator when appropriate
- Custom error handling for non-critical failures (graceful_print)

**Error Types:**
- Use std::error::Error for simple cases
- Box<dyn std::error::Error> for main function return
- Custom error handling in `src/utils.rs` graceful_print function

## Logging

**Framework:**
- No logging framework used
- Direct stderr output via eprintln! and custom graceful_print
- No structured logging or log levels

**Patterns:**
- Error output to stderr via `src/utils.rs` graceful_print function
- Regular output (affirmations) also to stderr
- No stdout output (tool is a wrapper)

## Comments

**When to Comment:**
- Module-level documentation with /// comments
- Complex logic explanation (template replacement in `src/utils.rs`)
- Configuration explanations in environment variable handling

**Documentation Style:**
- /// for public API documentation
- // for inline comments explaining non-obvious logic
- No JSDoc-style @param/@returns (Rust doesn't use this pattern)

**TODO Comments:**
- None found in current codebase
- PLAN.md contains development roadmap instead

## Function Design

**Size:**
- Most functions under 50 lines
- Larger functions broken into logical sections with comments
- Single responsibility principle followed

**Parameters:**
- Use references (&str, &[String]) when possible to avoid cloning
- Config passed by reference (&ConfigMommy)
- Options pattern not extensively used (simple parameter lists)

**Return Values:**
- Explicit Result<T, E> for fallible operations
- Option<T> for nullable returns (random_vec_pick)
- Unit type () for side-effect functions

## Module Design

**Exports:**
- Public functions use pub keyword
- Private functions have no visibility modifier
- No public re-exports from modules

**Organization:**
- Flat module structure (no nested modules)
- Each module has focused responsibility
- Tests co-located with #[cfg(test)] modules

---

*Convention analysis: 2026-01-16*
*Update when patterns change*