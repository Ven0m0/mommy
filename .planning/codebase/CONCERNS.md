# Codebase Concerns

**Analysis Date:** 2026-01-16

## Tech Debt

**Version Mismatch in CI:**
- Issue: Hardcoded version "0.1.5" in Debian package generation doesn't match Cargo.toml version "0.1.6"
- Why: Manual version management without automation
- Files: `.github/workflows/build.yml` lines 99, 108, 112 vs `Cargo.toml` line 3
- Impact: Released packages have incorrect version metadata
- Fix approach: Extract version from Cargo.toml or use automated versioning

**TODO Items in PLAN.md:**
- Issue: 47 planned improvements documented but not implemented
- Why: Active development roadmap with many pending features
- Files: `PLAN.md` lines 115-279 (CLI flags, integration tests, security docs)
- Impact: Missing production-ready features like --quiet, --json, proper error handling
- Fix approach: Prioritize critical items like CLI flags and integration tests

## Known Bugs

**None identified in current implementation** - All 19 tests pass successfully.

## Security Considerations

**Potential Shell Injection:**
- Risk: Raw command execution via `bash -c` without shell escaping
- Files: `src/mommy.rs` line 218 (`Command::new("bash").arg("-c").arg(&run_command)`)
- Current mitigation: Commands come from user CLI arguments (trusted input)
- Recommendations: Add shell escaping for defense in depth, especially for alias integration

**Unvalidated File Access:**
- Risk: Arbitrary file reading for custom affirmations and aliases
- Files: `src/affirmations.rs` (loads custom JSON), `src/mommy.rs` lines 208-216 (sources alias files)
- Current mitigation: User controls file paths (intentional feature)
- Recommendations: Consider file path validation or sandboxing for paranoid security

**Embedded Secrets Potential:**
- Risk: Future addition of hardcoded secrets in affirmations or config
- Files: `assets/affirmations.json`, `src/config.rs` default values
- Current mitigation: No secrets currently present
- Recommendations: Add pre-commit hooks to scan for secrets

## Performance Bottlenecks

**No significant performance issues identified** - Lightweight CLI tool with minimal processing.

## Fragile Areas

**Environment Variable Handling:**
- Why fragile: Complex dual-prefix fallback system with string manipulation
- Files: `src/config.rs` lines 69-92 (env_with_fallback function)
- Common failures: Incorrect prefix generation, case sensitivity issues
- Safe modification: Add more comprehensive tests for edge cases
- Test coverage: 5 tests cover main scenarios, but edge cases untested

**Binary Name Detection:**
- Why fragile: Role transformation based on executable filename parsing
- Files: `src/mommy.rs` lines 47-109 (check_role_transformation)
- Common failures: Unexpected binary names, case variations, symlinks
- Safe modification: Add validation for expected patterns only
- Test coverage: Limited testing of unusual filename patterns

## Scaling Limits

**Not applicable** - Single-user CLI tool with no scaling requirements.

## Dependencies at Risk

**Minimal Risk:**
- All dependencies are stable, well-maintained Rust crates
- fastrand 2.3.0 - Active maintenance
- owo-colors 4.2.3 - Active maintenance
- serde ecosystem 1.0.x - Industry standard, very stable

**GitHub Actions:**
- Risk: Using specific commit SHA for actions-rs/toolchain (security best practice)
- Impact: Manual updates required for security patches
- Files: `.github/workflows/rust-clippy.yml` line 34

## Missing Critical Features

**Production CLI Features:**
- Problem: Missing standard CLI flags (--quiet, --json, --config, --version)
- Current workaround: Users must use environment variables only
- Files: `PLAN.md` lines 120-131 documents required flags
- Implementation complexity: Medium (requires argument parsing library)

**Integration Testing:**
- Problem: No end-to-end tests for actual command execution
- Current workaround: Manual testing only
- Files: `PLAN.md` lines 196-218 documents test plan
- Implementation complexity: Low (assert_cmd crate available)

**Error Handling:**
- Problem: Generic error types and minimal error context
- Current workaround: Basic error propagation
- Files: Functions return `Result<()>` without specific error types
- Implementation complexity: Medium (custom error enum)

## Test Coverage Gaps

**Command Execution Flow:**
- What's not tested: End-to-end command wrapping and affirmation display
- Risk: Integration failures between modules could go unnoticed
- Priority: High
- Files: No integration tests exist
- Difficulty to test: Medium (requires process spawning test setup)

**Environment Variable Edge Cases:**
- What's not tested: Invalid UTF-8, very long values, special characters
- Risk: Parsing failures or security issues with malformed input
- Priority: Medium
- Files: `src/config.rs` tests cover happy path only
- Difficulty to test: Low (unit tests can be added easily)

**Binary Detection Edge Cases:**
- What's not tested: Symlinks, unusual file extensions, Unicode in paths
- Risk: Role detection failures leading to incorrect behavior
- Priority: Low
- Files: `src/mommy.rs` role transformation logic
- Difficulty to test: Medium (filesystem setup required)

---

*Concerns audit: 2026-01-16*
*Update as issues are fixed or new ones discovered*