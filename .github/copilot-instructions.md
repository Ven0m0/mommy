# GitHub Copilot Instructions

> Organization-wide instructions for GitHub Copilot across all Ven0m0 repositories

## Organization Context

**Ven0m0** is a personal GitHub account focused on building practical open source tools that improve developer workflows and platform engineering practices. Projects emphasize automation, developer experience, and AI-assisted development.

## General Coding Principles

### Code Quality Standards

1. **Readability First**: Write code that is self-documenting and easy to understand
2. **Explicit Over Implicit**: Prefer explicit type annotations and clear variable names
3. **Fail Fast**: Validate inputs early and provide clear error messages
4. **Test Coverage**: Aim for 80%+ test coverage on all projects
5. **Security by Default**: Never commit secrets, use environment variables

### Documentation Requirements

- All public APIs must have documentation
- READMEs should include quick start, installation, and usage examples
- Complex logic should have inline comments explaining the "why"
- Architectural decisions should be documented in ADRs when applicable

### Git Practices

- Use conventional commits: `feat:`, `fix:`, `docs:`, `chore:`, `refactor:`, `test:`
- Keep commits atomic and focused on a single change
- Write descriptive commit messages explaining the change
- Reference issues in commit messages when applicable

## Language-Specific Guidelines

### Python Projects

```python
# Always include type annotations
def process_data(items: list[str], limit: int = 10) -> dict[str, int]:
    """Process items and return counts.

    Args:
        items: List of items to process.
        limit: Maximum items to process.

    Returns:
        Dictionary mapping items to their counts.
    """
    ...
```

**Standards:**
- Python 3.12+ with modern syntax
- Use `uv` for package management
- Format with `ruff`, type check with `pyright`
- Google-style docstrings
- Prefer `dataclasses` or `pydantic` for data structures

### TypeScript/JavaScript Projects

```typescript
// Use explicit types, avoid any
interface UserConfig {
  name: string;
  timeout?: number;
}

export function createClient(config: UserConfig): Client {
  // Implementation
}
```

**Standards:**
- Node.js 22+ with ESM modules
- Use `bun` when `bun.lockb` or `bunfig.toml` is present; otherwise use `pnpm`
- ESLint 9+ flat config with strict rules
- Prefer `interface` over `type` for object shapes
- Use `vitest` for testing

### Go Projects

```go
// Package users provides user management functionality.
package users

// User represents a user in the system.
type User struct {
    ID    string
    Name  string
    Email string
}

// GetByID retrieves a user by their ID.
func (s *Service) GetByID(ctx context.Context, id string) (*User, error) {
    if id == "" {
        return nil, fmt.Errorf("id cannot be empty")
    }
    // Implementation
}
```

**Standards:**
- Go 1.23+ with modules
- Use `golangci-lint` for linting
- Standard project layout: cmd/, internal/, pkg/
- Table-driven tests with subtests
- Context propagation for cancellation

### Rust Projects

```rust
/// Processes the input data and returns results.
///
/// # Arguments
///
/// * `input` - The data to process
///
/// # Returns
///
/// A Result containing the processed data or an error.
///
/// # Examples
///
/// ```
/// let result = process("input")?;
/// ```
pub fn process(input: &str) -> Result<Output, Error> {
    // Implementation
}
```

**Standards:**
- Latest stable Rust
- Use `clippy` with pedantic lints
- Use `cargo-deny` for dependency auditing
- Prefer `thiserror` for error types
- Document all public items with examples

### Java Projects

```java
/**
 * Service for managing resources.
 *
 * <p>Handles business logic for resource operations including
 * creation, retrieval, and updates.
 */
@Service
@Transactional(readOnly = true)
public class ResourceService {

    private final ResourceRepository repository;

    public ResourceService(ResourceRepository repository) {
        this.repository = repository;
    }

    /**
     * Finds a resource by its unique identifier.
     *
     * @param id the resource identifier
     * @return the resource if found
     * @throws ResourceNotFoundException if not found
     */
    public Resource findById(Long id) {
        return repository.findById(id)
            .orElseThrow(() -> new ResourceNotFoundException(id));
    }
}
```

**Standards:**
- Java 21 LTS with Spring Boot 3.3+
- Gradle with Kotlin DSL
- Constructor injection (no field injection)
- Use Java Records for DTOs
- Checkstyle with Google style (modified)

## Project Structure Patterns

### Standard Layouts

**Python:**
```
src/package_name/
tests/
  unit/
  integration/
pyproject.toml
```

**TypeScript:**
```
src/
tests/
package.json
tsconfig.json
```

**Go:**
```
cmd/app/
internal/
pkg/
go.mod
```

## AI Assistant Integration

### CLAUDE.md Files

Each repository should have a `CLAUDE.md` file at the root containing:
- Project overview and structure
- Build and test commands
- Code style requirements
- Architecture guidelines
- Common patterns and examples

### MCP Server Configuration

Projects using VS Code should include `.vscode/mcp.json` with:
- context7 for documentation lookup
- filesystem for project navigation
- memory for session persistence (when applicable)

## Security Practices

1. **No Hardcoded Secrets**: Use environment variables or secret managers
2. **Dependency Scanning**: Enable Dependabot and review CVEs
3. **SHA-Pinned Actions**: Prefer commit SHA pinning; at minimum pin to a major version tag
4. **Minimal Permissions**: GITHUB_TOKEN with least privilege
5. **Secret Scanning**: Pre-commit hooks with gitleaks

## CI/CD Standards

All projects should include:

```yaml
# Minimum CI workflow structure
name: CI
on: [push, pull_request]

jobs:
  lint:
    # Linting and formatting checks
  test:
    # Unit and integration tests
  build:
    # Build verification
```

For reusable workflows, prefer the templates in `.github/workflows/`:
- `comprehensive-lint.yml`
- `bun.yml`
- `uv-lock.yml`
- `dependabot-automerge.yml`
- `git-maintenance.yml`
- `img-opt.yml`

## Common Patterns

### Error Handling

```python
# Python - explicit error messages
msg = f"Failed to process {item}: {reason}"
raise ValueError(msg)
```

```typescript
// TypeScript - Result pattern or explicit throws
function process(input: string): Result<Output, ProcessError> {
  if (!input) {
    return err(new ProcessError("Input required"));
  }
  return ok(doProcess(input));
}
```

```go
// Go - wrap errors with context
if err != nil {
    return fmt.Errorf("failed to fetch user %s: %w", id, err)
}
```

### Testing Patterns

- Use table-driven tests for multiple cases
- Mock external dependencies
- Test edge cases and error paths
- Include integration tests for API boundaries

### Configuration

- Use environment variables for runtime config
- Use structured config files (YAML/TOML) for complex settings
- Validate configuration at startup
- Provide sensible defaults

## Content and Documentation

For content repositories and documentation:

- Use Markdown with consistent formatting
- Include frontmatter with required metadata
- Validate links and spelling in CI
- Follow SEO best practices for public content

## When Generating Code

1. **Match existing style**: Look at surrounding code for patterns
2. **Include tests**: Generate tests alongside implementation
3. **Add documentation**: Include docstrings/comments for complex logic
4. **Handle errors**: Include proper error handling
5. **Consider edge cases**: Account for null, empty, and boundary conditions

---

## Project-Specific: mommy CLI Tool

### Project Context

**mommy** (v0.1.6) is a Rust terminal affirmation tool with dual-mode operation:
- Shell wrapper: `mommy <command>`
- Cargo subcommand: `cargo mommy <command>`

**Key Constraints:**
- 24 unit tests must always pass
- Binary size target: ~400KB (stripped)
- Zero external runtime dependencies
- Stateless execution (no config files)

### Mommy-Specific Rust Patterns

**Environment Variables (Dual-Prefix System):**
```rust
// Shell mode: SHELL_MOMMYS_*
// Cargo mode: CARGO_MOMMYS_*
let prefix = if is_cargo_subcommand() {
    "CARGO_MOMMYS_"
} else {
    "SHELL_MOMMYS_"
};

let value = env::var(format!("{prefix}VARIABLE"))
    .unwrap_or_else(|_| "default".to_string());
```

**Thread-Safe Environment Variable Tests:**
```rust
#[cfg(test)]
mod tests {
    use std::sync::{LazyLock, Mutex};

    static ENV_LOCK: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

    #[test]
    fn test_env_var() {
        let _guard = ENV_LOCK.lock();
        env::set_var("TEST_VAR", "value");
        // test code
        env::remove_var("TEST_VAR");
    }
}
```

**Deterministic Random Testing:**
```rust
#[test]
fn test_random_selection() {
    fastrand::seed(42);  // Reproducible results
    let result = get_random_affirmation();
    assert_eq!(result, "expected value");
}
```

**Zero-Allocation Terminal Colors:**
```rust
use owo_colors::OwoColorize;
println!("{}", message.bright_magenta().bold());
```

**Embedded Assets:**
```rust
// Compile-time embedding
const AFFIRMATIONS_JSON: &str = include_str!("../assets/affirmations.json");
```

### Module Responsibilities

| Module | Purpose | Key Functions |
|--------|---------|---------------|
| `src/main.rs` | Entry routing | Minimal (414 bytes) |
| `src/mommy.rs` | Core logic | Command execution, role detection |
| `src/config.rs` | Configuration | Env var parsing with dual-prefix |
| `src/affirmations.rs` | Mood system | JSON loading, template selection |
| `src/color.rs` | Styling | ANSI color parsing |
| `src/utils.rs` | Templates | `{roles}`, `{pronouns}`, etc. substitution |

### Design Anti-Patterns to Avoid

**❌ Don't Add:**
- Heavy dependencies (clap, tokio, anyhow)
- Config files (use env vars only)
- Premature abstractions (traits for <3 uses)
- Custom error types (use `Result<T>` with `Box<dyn Error>`)
- Backwards compatibility hacks (delete unused code)

**✅ Do Add:**
- Co-located tests in source files
- Tests with `ENV_LOCK` for env var safety
- Deterministic tests with `fastrand::seed(42)`
- Documentation for public APIs
- Examples in doc comments

### Pre-commit Requirements

```bash
cargo test                  # All 24 tests must pass
cargo clippy -- -D warnings # Zero warnings (CI enforced)
cargo fmt                   # Format code
```

### Template System

Supported placeholders in affirmation strings:
- `{roles}` - Role name(s) from config
- `{pronouns}` - Pronoun set
- `{little}` - Term of endearment
- `{emotes}` - Emoji/emoticons

### Build Configuration

```toml
[profile.release]
codegen-units = 1         # Single codegen unit
strip = true              # Remove debug symbols
lto = "fat"               # Full link-time optimization
opt-level = 3             # Maximum optimization
panic = "abort"           # Smaller binary size
```

### Common Tasks for Code Generation

**Adding a Config Variable:**
1. Add field to `Config` struct in `src/config.rs`
2. Parse from env var in `Config::from_env()`
3. Add test with `ENV_LOCK` in `config::tests`
4. Update `README.md` env vars table

**Adding a New Mood:**
1. Edit `assets/affirmations.json`
2. Add validation test in `affirmations::tests`
3. Rebuild to embed new data

**Adding a Color Style:**
1. Update parsing in `src/color.rs`
2. Add test case in `color::tests`
3. Use `owo_colors` for styling

### Architecture Principles

1. **Stateless Execution** - No persistent state or config files
2. **Embedded Assets** - All data compiled into binary at build time
3. **Minimal Dependencies** - Only 4 runtime crates
4. **Simple Design** - No abstractions for <3 similar uses
5. **Fast Execution** - Zero-allocation where possible
6. **Thread-Safe Tests** - Use `ENV_LOCK` for env var isolation

### Security Context

- Shell commands from CLI args are user-trusted (appropriate for CLI tool)
- File paths from env vars are user-controlled (intentional feature)
- No SQL, no web endpoints, no external network calls
- Commands executed via `bash -c` (user authentication boundary)

### Quick Reference

```bash
# Development
cargo build -r              # Release build (~400KB)
cargo run -- ls -la         # Test shell mode
cargo run --bin cargo-mommy build  # Test cargo mode

# Testing
cargo test                  # Run all 24 tests
cargo test config::         # Specific module
cargo test -- --nocapture   # Show output

# Cross-compilation
cargo build -r --target x86_64-unknown-linux-musl

# Release
git tag -a v0.1.7 -m "Release v0.1.7"
git push origin v0.1.7
```

For comprehensive documentation, see:
- **User Guide:** `README.md`
- **AI Agents:** `AGENTS.md` (full technical reference)
- **Roadmap:** `PLAN.md` (47 planned features)
