## Tool Preferences
Modern tools first, legacy fallbacks allowed:
- **Search**: `rg`>`grep`, `fd`>`find`
- **Data**: `jaq`>`jq` (JSON), `yq` (YAML/XML)
- **File Ops**: `eza`>`ls`, `bat`>`cat`, `sd`>`sed`
- **Archive**: `zstd`>`xz`, `aria2c`>`curl`>`wget`
- **Dev**: `bun`>`npm`, `uv`>`pip`, `ruff`>`black`, `gix clone`>`git clone`
- **Interactive**: `fzf` (fuzzy select)

## Bash Standards
Template:
```bash
#!/usr/bin/env bash
# shellcheck enable=all shell=bash source-path=SCRIPTDIR
set -euo pipefail; shopt -s nullglob globstar
IFS=$'\n\t' LC_ALL=C
has(){ command -v -- "$1" &>/dev/null; }
msg(){ printf '%s\n' "$@"; }
log(){ printf '%s\n' "$@" >&2; }
die(){ printf '%s\n' "$1" >&2; exit "${2:-1}"; }
```

Rules:
- Tests: `[[ ]]`, regex `=~`
- Arrays: `mapfile -t`, `declare -A`
- Strings: `${v//p/r}`, `${v%%p*}` (no sed/awk for simple edits)
- I/O: `<<<"$v"`, `< <(cmd)`, `exec {fd}<file`
- Perf: Min forks, batch ops, `fcat(){ printf '%s\n' "$(<${1})"; }`
- Forbidden: eval, backticks, ls parsing, unquoted exp, expr, remote source
- Normalize: `() {`→`(){`, `> /`→`>/`, `>/dev/null 2>&1`→`&>/dev/null`
- Lint: shellcheck, shellharden, shfmt

## Python Standards
- PEP 8/257, 2-space indent, typed returns, `slots=True` dataclasses
- Perf: O(1) dict/set, precompile regex, `sys.stdin.read()`, `python3 -OO`
- Security: Specific exceptions only, no subprocess in hot paths

## Code Standards
- **KISS**: Simple over clever
- **YAGNI**: Build when needed, not before
- **DRY**: Extract repeated logic
- **Naming**: Descriptive (getUserById not getUsr)
- **Functions**: Small, single task
- **Fail Fast**: Validate early, clear errors
- **Security**: No secrets in logs/commits, validate inputs
- **Imports**: stdlib → third-party → local, alphabetical
- **Comments**: Why, not what
- **Changes**: Minimal, focused

## Communication
- No emojis, no em dashes (-), clear/direct
- Review first, report findings before changes
- Avoid "successfully" without test proof
