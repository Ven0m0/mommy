---
name: release
description: Cut a new mommy release — bumps the version everywhere it's hardcoded, tags, and pushes. Use only when the user explicitly asks to release/tag/publish a new version.
disable-model-invocation: true
---

# Release mommy

This repo has **three separate files** that hardcode the version number, and
none of them are checked against each other automatically. Missing one is
the known recurring bug documented in `AGENTS.md`. Update all of them
together:

| File | What to change |
|------|-----------------|
| `Cargo.toml` | `version = "X.Y.Z"` |
| `.github/workflows/build.yml` | Three occurrences: `Version: X.Y.Z`, and two `shell-mommy_X.Y.Z_amd64.deb` strings (Debian packaging job) |
| `PKGBUILD` | `pkgver=X.Y.Z` |

## Steps

1. Confirm the new version number with the user if not already given.
   Read the current version from `Cargo.toml`'s `[package] version`.

2. Update all three files above to the new version. Use exact string
   replacement — do not touch unrelated version-looking strings (e.g.
   dependency version pins in `Cargo.toml`).

3. Refresh the lockfile so `Cargo.lock`'s own package entry matches:

   ```bash
   cargo check
   ```

4. Run the same checks CI and the Arch package build both require —
   don't skip `--all-features`, since `PKGBUILD`'s `check()` step runs
   `cargo test --all-features` and the default feature set alone won't
   catch a feature-gated regression (this happened before with the `beg`
   feature):

   ```bash
   cargo fmt --check
   cargo clippy --all-targets --all-features -- -D warnings
   cargo test --all-features
   ```

   If anything fails, stop and fix it before proceeding — do not tag a
   broken commit.

5. `PKGBUILD`'s `sha256sums` is tied to the *tagged* release tarball, so it
   can't be computed until after the tag is pushed. Leave it as `SKIP` (or
   whatever it currently is) unless the user asks you to also update the
   Arch package checksum after the tag exists — that requires downloading
   `https://github.com/Ven0m0/mommy/archive/refs/tags/vX.Y.Z.tar.gz` and
   running `sha256sum` on it, which only works after step 6.

6. Commit the version bump, then tag and push — confirm with the user
   before pushing, since this triggers the release CI job:

   ```bash
   git add Cargo.toml Cargo.lock .github/workflows/build.yml PKGBUILD
   git commit -m "chore: bump version to X.Y.Z"
   git tag -a vX.Y.Z -m "Release vX.Y.Z: <short description>"
   git push origin master
   git push origin vX.Y.Z
   ```

CI then builds all targets, packages, and publishes the GitHub release.
