Repo Improvement Plan — mommy
0) Principles

Small, single-purpose PRs.

Conventional commits: feat:, fix:, ci:, chore:, docs:, test:.

Keep CLI fast, minimal, and scriptable.

Prefer defaults that are quiet in CI.

Avoid breaking changes unless tagged.

1) CI, format, lint (first PR)
Add CI workflow

Create: .github/workflows/ci.yml

Goals

Block on formatting issues.

Block on clippy warnings.

Run full test suite.

Checks

cargo fmt -- --check

cargo clippy --all-targets -- -D warnings

cargo test --workspace --release

Add formatting baseline

Create: rustfmt.toml

max_width = 100

Use defaults otherwise.

Ensure rust-toolchain.toml pins a stable toolchain.

PR title: ci: add fmt, clippy, and test checks

2) Releases & artifacts
Automated release builds

Create: .github/workflows/release.yml

Trigger: push on tags v*.*.*

Build targets:

x86_64-unknown-linux-gnu

aarch64-unknown-linux-gnu

Upload artifacts:

mommy-${target} binary per target.

Local build script

Create: build.sh

Builds both targets.

Idempotent.

Suppresses non-critical errors.

PR title: ci: add release workflow and build script

3) Documentation
Update README

Add clear sections:

What this fork is (and what differs from upstream).

Install methods:

Prebuilt binaries (Releases).

cargo install.

Minimal usage examples (≤5).

Table of environment variables with defaults.

Add USAGE

Create: USAGE.md

Example invocations.

Sample affirmations.json.

Explanation of precedence (flags > env > defaults).

Add CONTRIBUTING

Create: CONTRIBUTING.md

How to run tests locally.

Commit style.

Branch naming.

PR title: docs: expand README + add USAGE and CONTRIBUTING

4) CLI UX & features
Required flags

Implement in main.rs (clap/argh):

-q, --quiet
Suppress normal output; still fail on errors.

-j, --json
Machine-readable output.

-c, --config <file>
Load custom JSON/TOML.

-v, --version
Standardized version output.

Exit code contract

0 — success (even if output suppressed).

Non-zero — internal error or invalid config.

Logging

Add tracing behind RUST_LOG.

Respect --quiet.

PR title: feat: add quiet/json/config flags + stable exit codes

5) Packaging & distribution
Cargo metadata

Update Cargo.toml:

description

homepage

repository

license

keywords

categories

License

Add LICENSE (MIT or Apache-2.0).

Packaging notes

Create: PACKAGING.md

Example Arch PKGBUILD (minimal stub).

Debian build notes (cargo-deb or manual).

Optional: publish forked crate if semantics diverge.

PR title: chore: add metadata, license, and packaging notes

6) Dependency & security hygiene

Create:

.github/dependabot.yml (weekly for cargo + actions)

SECURITY.md

CODE_OF_CONDUCT.md

Add to CI (separate job):

cargo-audit

PR title: chore: add dependabot + security docs + cargo-audit

7) Tests & quality
Integration tests (tests/)

Add tests using assert_cmd + predicates:

mommy --version → exit 0.

mommy --help → exit 0.

mommy --json → valid JSON schema.

mommy --config tests/fixtures/affirmations.json → loads successfully.

Create:

tests/fixtures/affirmations.json (minimal valid file).

Optional

Property test for config parsing (quickcheck).

Basic fuzz target for JSON parsing.

PR title: test: add CLI integration tests

8) Help output & defaults

Ensure --help includes:

List of env vars.

Default values.

Example invocation.

If missing, update clap metadata accordingly.

9) Changelog & templates

Create:

CHANGELOG.md (Keep a “Unreleased” section).

.github/pull_request_template.md

.github/issue_template/bug_report.md

Suggested PR order

ci: add fmt, clippy, and test checks

test: add CLI integration tests

feat: add quiet/json/config flags + stable exit codes

docs: expand README + add USAGE and CONTRIBUTING

ci: add release workflow and build script

chore: add metadata, license, and packaging notes

chore: add dependabot + security docs + cargo-audit

Templates + changelog

Minimal checklist (copy for claude-code)

 .github/workflows/ci.yml

 .github/workflows/release.yml

 rustfmt.toml, rust-toolchain.toml

 Integration tests + fixtures

 CLI flags (--quiet, --json, --config)

 README, USAGE, CONTRIBUTING

 LICENSE, SECURITY, CODE_OF_CONDUCT, Dependabot

 PACKAGING.md + build.sh

 CHANGELOG + GitHub templates
