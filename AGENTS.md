# AGENTS.md

Guide for AI agents working in the **mommy** CLI repo. `CLAUDE.md` and
`GEMINI.md` are symlinks to this file — edit `AGENTS.md` only.

## Project Overview

**mommy** is a terminal affirmation tool (Rust, edition 2024) that wraps
shell commands and cargo subcommands with positive/negative feedback.
Reimplementation of shell-mommy + cargo-mommy in one binary.

- Version source of truth: `Cargo.toml` (currently 0.1.6)
- Single Cargo bin target: `mommy` (see "Dual-mode detection" below — there
  is no separate `cargo-mommy` bin target)
- Stateless by default: mommy writes no state except in the opt-in `beg` feature
  (see below). It reads one optional config file, `~/.config/mommy/config.json`
  (`moods`, `needy`; env vars override it; see `load_file_config` in `src/config.rs`)
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
examples/config.json  # Default config.json; "_comment" documents moods (kept in
                      # sync with install.sh/install.ps1); a test keeps it parseable
install.sh            # Linux/bash installer: cargo install + PROMPT_COMMAND hook
install.ps1           # Windows/PowerShell installer: cargo install + prompt hook
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
cargo build -r                           # Release, output at target/release/mommy
                                          # (.cargo/config.toml pins x86-64-v3
                                          # rustflags per-target — gnu-linux and
                                          # windows-msvc get it explicitly since a
                                          # target.<triple>.rustflags key REPLACES
                                          # [build]'s, it doesn't merge; not a
                                          # forced --target)
cargo test                               # 40 tests
cargo test -- --test-threads=1           # Avoid env var races between tests
cargo build -r --target x86_64-unknown-linux-musl   # Static Linux
cargo build -r --target x86_64-pc-windows-msvc      # Windows
```

Tests use `LazyLock<Mutex<()>>` to serialize env-var-mutating tests and
`fastrand::seed(42)` for deterministic randomness.

`[profile.dev]` and `[profile.release.build-override]` in `Cargo.toml` keep
local iteration fast: dev builds use `codegen-units = 256` and
`incremental = true` (mirrored by the removed `[build] incremental = false`
that used to force non-incremental everywhere), and proc-macro/build-script
deps (`serde_derive`, `syn`, ...) compile at `opt-level = 0` even in a
release build — only the final `shell-mommy` binary gets the full
`lto = "fat"` / `codegen-units = 1` treatment from `[profile.release]`.

## Quality Checks (required before committing)

```bash
cargo test-ci    # = cargo test --locked
cargo clippy-ci   # = cargo clippy --all-targets --locked -- -D warnings (CI-enforced gate)
cargo fmt-ci      # = cargo fmt --check
```

These are `.cargo/config.toml` `[alias]` shortcuts that mirror
`.github/workflows/build.yml`'s `test` job exactly; `cargo build-ci` mirrors
its release build (`build --release --locked`).

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
1. Stateless execution (except `beg`, above). The only config file is the optional
   read-only `config.json`. Env vars stay the primary interface
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

- **PowerShell prompt hook swallows stderr**: mommy prints to stderr
  (`graceful_print`). The PowerShell host discards native stderr while it
  evaluates `prompt`, so a bare `mommy $code` in `prompt` prints nothing, with
  no error. `install.ps1` uses `mommy $code 2>&1 | ForEach-Object { Write-Host "$_" }`.
  Calling `prompt` from a script does not reproduce this. Only the host's own
  prompt render does (verified by reading the conhost buffer).

- **CI version mismatch**: `.github/workflows/build.yml` hardcodes `0.1.5`
  in the Debian packaging job (lines ~99, 108, 112) while `Cargo.toml` is at
  `0.1.6`. Keep these in sync manually, or extract via `cargo metadata
  --no-deps --format-version 1 | jq -r '.packages[0].version'`.
- **`bash -c` / `powershell -Command` in `src/mommy.rs`** (alias expansion
  path): intentional, not a vulnerability — this is a CLI tool executing the
  invoking user's own command line.
- **Windows**: fully supported and verified natively (`cargo build`, `cargo
  test`, `cargo clippy -- -D warnings`, `cargo fmt --check` all clean on
  `x86_64-pc-windows-msvc`, including `--features beg`). `.cargo/config.toml`
  no longer force-pins `build.target` to `x86_64-unknown-linux-gnu` — that
  pin only broke native (non-`--target`) builds on non-Linux hosts, and CI's
  `build` job already passes `--target` explicitly per matrix entry so it was
  never load-bearing there; the x86-64-v3 `rustflags` stay scoped to the
  `[target.x86_64-unknown-linux-gnu]` table. `perform_role_transformation`
  appends `.exe` via `std::env::consts::EXE_SUFFIX` and skips the Unix
  `chmod` step (`#[cfg(unix)]`). `src/state.rs` reads `HOME` with a
  `USERPROFILE` fallback since Windows doesn't set `HOME` by default outside
  Git Bash/MSYS. The `SHELL_MOMMYS_ALIASES`/`CARGO_MOMMYS_ALIASES` feature
  branches on `#[cfg(unix)]` vs `#[cfg(windows)]` in `execute_command`: Unix
  sources the file with `bash -c` (needs `eval` since bash expands aliases
  at read time); Windows dot-sources a `.ps1` file with `powershell -Command`
  (no `eval` needed — PowerShell resolves functions/`Set-Alias` from an
  earlier statement in the same script). `shell_quote`/`powershell_quote` in
  `src/utils.rs` are each `#[cfg(unix)]`/`#[cfg(windows)]`-gated to match.
  Everything else (including the no-aliases command path) uses
  `Command::new(filtered_args[0])` directly and needs no shell at all.
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
# PKGBUILD: set pkgver=0.1.7, pkgrel=1, then `updpkgsums` (needs the pushed tag)
```

`PKGBUILD` notes: `cargo-mommy` is a hard link (a symlink would resolve back to
`mommy` via `current_exe()` and lose cargo mode), and `RUSTFLAGS` is exported so
`.cargo/config.toml`'s x86-64-v3 flags don't leak into the distributed binary.

## Resources

- `README.md` — end-user docs
- Upstream: [Gankra/cargo-mommy](https://github.com/Gankra/cargo-mommy),
  [sudofox/shell-mommy](https://github.com/sudofox/shell-mommy)
