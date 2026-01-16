# Testing Patterns

**Analysis Date:** 2026-01-16

## Test Framework

**Runner:**
- Cargo test - Built-in Rust test framework
- Config: No separate config file (uses Cargo.toml defaults)

**Assertion Library:**
- Built-in Rust assert! and assert_eq! macros
- Custom assertions for specific patterns (color testing)

**Run Commands:**
```bash
cargo test                              # Run all tests
cargo test --verbose                    # Verbose output
cargo test config::tests::test_cargo   # Run specific test
cargo clippy                           # Linting (enforced in CI)
```

## Test File Organization

**Location:**
- Tests co-located with source files using #[cfg(test)] modules
- No separate tests/ directory

**Naming:**
- test_* for all test functions
- Tests grouped by functionality within modules

**Structure:**
```
src/
  config.rs
    #[cfg(test)] mod tests { ... }    # 5 tests
  affirmations.rs
    #[cfg(test)] mod tests { ... }    # 6 tests
  color.rs
    #[cfg(test)] mod tests { ... }    # 5 tests
  utils.rs
    #[cfg(test)] mod tests { ... }    # 1 test
  mommy.rs                            # 2 tests (integration-style)
```

## Test Structure

**Suite Organization:**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::load_config;

    #[test]
    fn test_specific_behavior() {
        // Arrange
        let input = setup_test_data();

        // Act
        let result = function_under_test(input);

        // Assert
        assert_eq!(result, expected_value);
    }
}
```

**Patterns:**
- Use fastrand::seed(42) for deterministic random testing
- Environment variable mocking with std::sync::LazyLock and Mutex
- Test isolation using scoped env var changes
- Clear arrange/act/assert structure in complex tests

## Mocking

**Framework:**
- No external mocking framework
- Manual mocking via test helper functions

**Patterns:**
```rust
// Environment variable testing (src/config.rs)
static ENV_MUTEX: std::sync::LazyLock<std::sync::Mutex<()>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(()));

#[test]
fn test_with_env_var() {
    let _lock = ENV_MUTEX.lock().unwrap();
    std::env::set_var("TEST_VAR", "value");
    // test code
    std::env::remove_var("TEST_VAR");
}
```

**What to Mock:**
- Environment variables (using std::env::set_var in tests)
- Random behavior (using fastrand::seed for deterministic results)

**What NOT to Mock:**
- File system operations (not extensively used)
- Network calls (none in current codebase)

## Fixtures and Factories

**Test Data:**
```rust
// Factory pattern (src/utils.rs tests)
#[test]
fn test_fill_template() {
    fastrand::seed(42);  // Deterministic randomness
    let mut config = load_config();
    config.roles = vec!["daddy".to_string(), "mommy".to_string()];
    config.pronouns = vec!["his".to_string(), "her".to_string()];
    // test with known config
}
```

**Location:**
- Factory functions defined inline in test functions
- No separate fixtures directory
- Test data created programmatically

## Coverage

**Requirements:**
- No enforced coverage target
- All tests pass in CI (`.github/workflows/build.yml` line 25)
- 19 tests total across all modules

**Configuration:**
- No coverage collection configured
- Tests run via cargo test in CI

**View Coverage:**
```bash
cargo test  # Basic test execution only
```

## Test Types

**Unit Tests:**
- Test individual functions in isolation
- Examples: color name parsing, template filling, config loading
- Fast execution (all complete in 0.00s)

**Integration Tests:**
- Tests in main.rs and mommy.rs test full workflows
- Example: `test_cargo_prefix_vars` tests full environment variable processing
- Still run as unit tests (no separate integration directory)

**End-to-End Tests:**
- Not currently implemented
- Planned in PLAN.md (integration tests with assert_cmd)

## Common Patterns

**Environment Variable Testing:**
```rust
#[test]
fn test_env_config() {
    let _lock = ENV_MUTEX.lock().unwrap();
    std::env::set_var("SHELL_MOMMYS_ROLES", "daddy");
    let config = load_config();
    assert_eq!(config.roles, vec!["daddy"]);
    std::env::remove_var("SHELL_MOMMYS_ROLES");
}
```

**Random Behavior Testing:**
```rust
#[test]
fn test_random_selection() {
    fastrand::seed(42);  // Deterministic seed
    let result = random_function();
    assert_eq!(result, "expected_with_seed_42");
}
```

**Color/Style Testing:**
```rust
#[test]
fn test_color_output() {
    let output = styled_function();
    assert!(output.contains("\\x1b["), "expected ANSI escape codes");
    assert!(output.contains("expected_text"));
}
```

---

*Testing analysis: 2026-01-16*
*Update when test patterns change*