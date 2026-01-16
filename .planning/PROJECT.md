# Mommy CLI Enhancement Project

## What This Is

A comprehensive enhancement of the mommy CLI tool focused on making it highly customizable while maintaining full backward compatibility. The project adds advanced mood mixing, optimizes dependencies for smaller binary size, improves the CLI interface, introduces new mood types, and enhances configuration capabilities for extensive personalization.

## Core Value

Users can extensively personalize their mommy experience through rich customization options while existing workflows continue working exactly as before.

## Requirements

### Validated

- ✓ Dual-mode CLI operation (`mommy` shell wrapper + `cargo mommy` subcommand) — existing
- ✓ Three-mood affirmation system (chill, ominous, thirsty) with template interpolation — existing
- ✓ Environment variable configuration with dual-prefix system (SHELL_MOMMYS_*/CARGO_MOMMYS_*) — existing
- ✓ Terminal styling with colors and text effects — existing
- ✓ Shell command wrapping with exit code-based affirmation selection — existing
- ✓ Custom affirmations via JSON file support — existing
- ✓ Bash aliases integration — existing
- ✓ Multi-platform binary distribution (Linux GNU/musl, Windows) — existing

### Active

- [ ] Mood mixing system with configurable probability (20% default for thirsty+ominous)
- [ ] Environment variable toggle for mood mixing (off by default)
- [ ] Comprehensive dependency audit and optimization for smaller binary size
- [ ] CLI improvements with proper flags, help system, and argument handling
- [ ] Additional mood types beyond the current three (chill, ominous, thirsty)
- [ ] Enhanced configuration system for extensive user personalization
- [ ] Dynamic response variations (word shuffling/mixing for freshness)

### Out of Scope

- Breaking changes to existing environment variables — maintain compatibility
- Removal of embedded affirmations system — backwards compatibility requirement
- Changes to dual-mode CLI behavior — core functionality must remain identical
- Network-dependent features — keep tool fully offline

## Context

This is a brownfield enhancement of an existing Rust CLI tool with:

- **Current state**: Well-architected 50KB binary with 4 dependencies (fastrand, owo-colors, serde/serde_json)
- **User base**: Existing users rely on current environment variable patterns and behavior
- **Technical foundation**: Clean module structure with co-located tests, comprehensive CI/CD
- **Architecture**: Stateless execution with embedded assets and template-based affirmations
- **Quality**: 19 passing unit tests, documented security considerations, established conventions

## Constraints

- **Compatibility**: All existing environment variables and behavior must continue working unchanged
- **Binary size**: Enhanced features should not significantly increase binary size; ideally reduce it
- **Dependencies**: Minimize dependency count and choose lightweight alternatives where possible
- **Performance**: Maintain or improve current execution speed
- **Platform support**: Continue supporting Linux GNU/musl and Windows compilation targets

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Maintain full backward compatibility | Existing users should not experience any breaking changes | — Pending |
| Focus on extensive customization | User feedback indicates desire for more personalization options | — Pending |
| Optimize dependencies comprehensively | Current 4 deps might have lighter alternatives or could be eliminated | — Pending |

---
*Last updated: 2026-01-16 after initialization*