# AGENTS.md

Guide for AI agents working in the **mommy** CLI repo. `CLAUDE.md` and
`GEMINI.md` are symlinks to this file — edit `AGENTS.md` only.

## Project Overview

**mommy** is a terminal affirmation tool (Rust, edition 2021) that wraps
shell commands and cargo subcommands with positive/negative feedback.
Reimplementation of shell-mommy + cargo-mommy in one binary.

- Version source of truth: `Cargo.toml` (currently 0.1.6)
- Single Cargo bin target: `mommy` (see "Dual-mode detection" below — there
  is no separate `cargo-mommy` bin target)
- Stateless by default; the only exception is the opt-in `beg` feature
  (see below)
- License: Unlicense

## Repo Structure

```
src/main.rs          # Entry point: calls mommy::mommy(), exits with its code
src/mommy.rs         # Command execution, role transformation, output
src/config.rs        # Env var parsing, dual-prefix support, binary detection
src/affirmations.rs  # Mood system, JSON loading, template instantiation
src/color.rs         # ANSI color parsing and styling
src/utils.rs         # Template substitution engine
src/state.rs         # Mood persistence for the `beg` feature (cfg-gated)
assets/affirmations.json  # Default messages, embedded into the binary
.cargo/config.toml    # Pins target = x86_64-unknown-linux-gnu, custom rustflags
.github/workflows/build.yml  # Only CI workflow: test, build, package, release
PKGBUILD              # Arch Linux packaging
```

No `PLAN.md`, `rust-toolchain.toml`, or `rustfmt.toml` exist in this repo —
don't reference them. `cargo fmt` runs with default settings.

## Dual-mode detection (non-obvious — read before touching mode logic)

There is only **one** compiled binary (`mommy`). Shell vs. cargo mode and
the "mommy"/"daddy" role are both determined by `BinaryInfo::detect()` in
`src/config.rs`, which inspects `env::current_exe()`'s filename:

- Filename starts with `cargo-` → cargo-subcommand mode, env var prefix
  becomes `CARGO_MOMMYS_*` (falls back to `SHELL_MOMMYS_*` then defaults).
- Otherwise → shell mode, prefix `SHELL_MOMMYS_*`.
- `daddy` anywhere in the (prefix-stripped) filename → role `"daddy"`,
  else `"mommy"`.

To exercise cargo-mode locally, put a `cargo-mommy` (or `cargo-daddy`) file
on `PATH` that points at the built `mommy` binary — `cargo run --bin
cargo-mommy` does **not** work, there is no such target. (Note: `Cargo.toml`
only declares one `[[bin]]`, so despite what `README.md` says, `cargo
install shell-mommy` does not currently install a second `cargo-mommy`
binary either — this is a known gap, not something to "fix" via docs.)

Separately, `mommy i mean daddy` is an unrelated easter egg
(`check_role_transformation`/`perform_role_transformation` in
`src/mommy.rs`): it detects that phrase in argv and copies the running
binary to a new filename.

## Build & Test

```bash
cargo build                              # Debug
cargo build -r                           # Release (see .cargo/config.toml: fixed
                                          # target triple, so output lands at
                                          # target/x86_64-unknown-linux-gnu/release/mommy,
                                          # not target/release/mommy)
cargo test                               # 34 tests across 6 modules
cargo test -- --test-threads=1           # Avoid env var races between tests
cargo build -r --target x86_64-unknown-linux-musl   # Static Linux
cargo build -r --target x86_64-pc-windows-msvc      # Windows
```

Tests use `LazyLock<Mutex<()>>` to serialize env-var-mutating tests and
`fastrand::seed(42)` for deterministic randomness.

## Quality Checks (required before committing)

```bash
cargo test
cargo clippy -- -D warnings     # CI-enforced gate, must be clean
cargo fmt --check
```

`cargo clippy -- -D clippy::all -D clippy::pedantic` surfaces additional
style opinions beyond the CI gate. Mechanical ones (`uninlined_format_args`,
`redundant_closure_for_method_calls`) are safe to apply via `cargo clippy
--fix`. Treat `unnecessary_wraps` / `struct_excessive_bools` suggestions as
informational only — they push toward API reshaping that conflicts with
this project's minimal-abstraction style (see Design Principles below).

## The `beg` feature (opt-in, `--features beg`)

Adds a stateful "angry until you say please" mood, persisted as JSON to
`~/.mommy.state` via `State`/`Mood` in `src/state.rs`. This is the only
part of the codebase that touches disk for persistence — everything else
is stateless. Build/test it explicitly: `cargo test --features beg`.

## Conventions

**Design Principles:**
1. Stateless execution (except `beg`, above) — no config files
2. Embedded assets — all data compiled into the binary
3. Minimal error handling — validate only at system boundaries
4. No premature abstraction — three similar lines over unnecessary generality
5. Delete unused code completely, no compatibility shims

**Env var naming:** `SHELL_MOMMYS_<VAR>` / `CARGO_MOMMYS_<VAR>`, falling
back to generic `MOMMYS_*`, then hardcoded defaults. Exception:
`ONLY_NEGATIVE` uses `SHELL_MOMMY_`/`CARGO_MOMMY_` (no trailing S).

**Template placeholders** (`src/utils.rs`): `{roles}`, `{pronouns}`,
`{little}`, `{emotes}` — randomly resolved from the active config's vectors.

**Commits:** [Conventional Commits](https://www.conventionalcommits.org/)
(`feat:`, `fix:`, `test:`, `docs:`, `ci:`, `chore:`).

## Known Issues

- **CI version mismatch**: `.github/workflows/build.yml` hardcodes `0.1.5`
  in the Debian packaging job (lines ~99, 108, 112) while `Cargo.toml` is at
  `0.1.6`. Keep these in sync manually, or extract via `cargo metadata
  --no-deps --format-version 1 | jq -r '.packages[0].version'`.
- **`bash -c` in `src/mommy.rs`** (alias expansion path): intentional, not
  a vulnerability — this is a CLI tool executing the invoking user's own
  command line.
- **Windows**: the core tool (shell wrapper, cargo subcommand, moods,
  colors, `beg` state) is cross-platform — `cargo check`/`clippy` are clean
  against `x86_64-pc-windows-gnu` for both feature sets, and `perform_role_transformation`
  already appends `.exe` via `std::env::consts::EXE_SUFFIX`. The one real
  gap is the `SHELL_MOMMYS_ALIASES`/`CARGO_MOMMYS_ALIASES` feature, which
  shells out to `bash -c` with `shopt -s expand_aliases` — a bash builtin,
  not available without Git Bash/WSL. Everything else (including the
  no-aliases command path) uses `Command::new(filtered_args[0])` directly
  and needs no shell at all. `src/state.rs` reads `HOME` with a
  `USERPROFILE` fallback for this reason — Windows doesn't set `HOME` by
  default outside Git Bash/MSYS.
- **`.gitignore` had a bare `src/` entry** (inherited from a `makepkg`
  template, meant for the packaging tool's scratch `src/`/`pkg/` dirs) that
  silently shadowed the real source tree and dropped new files from `git
  add`. Removed. If a new file under `src/` mysteriously doesn't appear in
  `git status`, check `.gitignore` first.

## Release Process

```bash
sed -i 's/version = "0.1.6"/version = "0.1.7"/' Cargo.toml
git tag -a v0.1.7 -m "Release v0.1.7: <description>"
git push origin v0.1.7   # CI builds all targets and publishes the release
```

## Resources

- `README.md` — end-user docs
- Upstream: [Gankra/cargo-mommy](https://github.com/Gankra/cargo-mommy),
  [sudofox/shell-mommy](https://github.com/sudofox/shell-mommy)
