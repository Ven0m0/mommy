# Dependency Audit Report

Generated: 2025-12-18

## Executive Summary

This audit analyzed the dependencies of `shell-mommy` v0.1.6 for security vulnerabilities, outdated packages, and unnecessary bloat. One **critical security issue** was identified requiring immediate action.

### Key Findings

- **1 Security Vulnerability** (unmaintained dependency)
- **1 Configuration Issue** (invalid Rust edition)
- **Production dependencies: Lean** (4 direct, ~19 total)
- **Dev dependencies: Heavy but acceptable** (1 direct, ~44 total)

---

## Security Vulnerabilities

### 🔴 CRITICAL: ansi_term is Unmaintained

**Package:** `ansi_term v0.12.1`
**Advisory:** RUSTSEC-2021-0139
**Date:** 2021-08-18
**URL:** https://rustsec.org/advisories/RUSTSEC-2021-0139

**Impact:**
- The `ansi_term` crate has been unmaintained since 2021
- No security patches or bug fixes will be released
- Used in: `src/color.rs` for terminal color styling

**Recommendation:** **REPLACE IMMEDIATELY**

**Suggested Alternatives:**

1. **owo-colors (v4.2.3)** - Recommended
   - Zero-allocation terminal colors
   - Actively maintained
   - Similar API to ansi_term
   - Excellent performance

2. **yansi (v1.0.1)** - Alternative
   - Dead simple ANSI terminal color painting
   - Actively maintained
   - Minimal dependencies

---

## Outdated Packages

All other dependencies are up-to-date:

| Package | Current | Latest | Status |
|---------|---------|--------|--------|
| fastrand | 2.3.0 | 2.3.0 | ✅ Current |
| serde | 1.0.228 | 1.0.228 | ✅ Current |
| serde_json | 1.0.145 | 1.0.145 | ✅ Current |
| serial_test | 3.2.0 | 3.2.0 | ✅ Current |

---

## Dependency Bloat Analysis

### Production Dependencies (✅ LEAN)

**Direct dependencies:** 4
- `ansi_term`: Terminal styling (needs replacement)
- `fastrand`: Random number generation (appropriate, lightweight)
- `serde`: Serialization framework (required for JSON)
- `serde_json`: JSON parsing (required for affirmations)

**Total dependency tree:** ~19 crates

**Assessment:** Production dependencies are minimal and appropriate for the use case. Each serves a clear purpose:
- `fastrand`: Used for random selection of affirmations, colors, and styles
- `serde/serde_json`: Essential for parsing `assets/affirmations.json`
- `ansi_term`: Terminal styling (needs security fix)

### Dev Dependencies (⚠️ HEAVY BUT ACCEPTABLE)

**Direct dependencies:** 1
- `serial_test v3.2.0`: Test serialization

**Transitive dependencies:** ~43 crates including:
- futures (async runtime - 8 crates)
- parking_lot (synchronization - 5 crates)
- scc, sdd (concurrent data structures)
- Various proc-macros

**Assessment:** The `serial_test` dependency pulls in significant async/concurrency infrastructure that seems excessive for this simple CLI tool's testing needs. However, this only affects development builds, not production binaries.

**Recommendation:** OPTIONAL - Consider if serial test execution is truly needed, or if tests can be restructured to run in parallel without conflicts.

---

## Other Issues

### 🟡 Invalid Rust Edition

**File:** `Cargo.toml:5`
**Current:** `edition = "2024"`
**Issue:** Rust edition "2024" does not exist

**Valid editions:**
- 2015 (default)
- 2018
- 2021 (latest stable)

**Recommendation:** Change to `edition = "2021"`

---

## Dependency Usage Analysis

### ansi_term Usage
**File:** `src/color.rs`
**Usage:**
- `Color` enum for named colors and RGB
- `Style` struct for text styling (bold, italic, underline, etc.)
- Methods: `bold()`, `italic()`, `dimmed()`, `underline()`, `blink()`, `reverse()`, `hidden()`

### fastrand Usage
**Files:** `src/color.rs`, `src/mommy.rs`, `src/utils.rs`
**Usage:**
- `fastrand::usize(..range)` for random index selection
- `fastrand::seed(n)` in tests for deterministic output

### serde/serde_json Usage
**File:** `src/affirmations.rs`
**Usage:**
- `#[derive(Deserialize)]` for parsing affirmations JSON
- `serde_json::from_str()` for JSON parsing

---

## Recommended Actions

### Priority 1: CRITICAL (Do Immediately)

1. **Replace ansi_term with owo-colors**
   ```toml
   # In Cargo.toml, replace:
   ansi_term = "0.12"
   # With:
   owo-colors = "4.2"
   ```

2. **Update src/color.rs** to use owo-colors API

3. **Fix Rust edition**
   ```toml
   # In Cargo.toml, change:
   edition = "2024"
   # To:
   edition = "2021"
   ```

### Priority 2: OPTIONAL (Consider for future)

4. **Review serial_test necessity**
   - Evaluate if test serialization is truly needed
   - Consider restructuring tests to avoid shared state
   - Potential removal could reduce dev dependency count from 44 to ~20

---

## Detailed Migration Guide: ansi_term → owo-colors

### API Differences

**ansi_term:**
```rust
use ansi_term::{Color, Style};

let mut style = Style::new();
style = style.fg(Color::Red);
style = style.bold();
let output = style.paint("text");
```

**owo-colors:**
```rust
use owo_colors::{OwoColorize, Style};

let output = "text".red().bold();
// or for more complex styling:
let output = "text".style(Style::new().red().bold());
```

### Changes Required in src/color.rs

The migration will require:
1. Replacing `use ansi_term::{Color, Style}` with owo-colors imports
2. Updating color enum matching (similar API)
3. Refactoring style chaining (builder pattern to method chaining)
4. Testing all color outputs remain identical

---

## Conclusion

The project has a lean production dependency footprint, which is excellent. The critical issue is the unmaintained `ansi_term` dependency, which should be replaced immediately. The invalid Rust edition should also be corrected. Dev dependencies are heavy but acceptable since they don't affect production builds.

**Overall Health:** 🟡 Good with critical fixes needed

**Action Required:** Replace ansi_term and fix edition immediately
