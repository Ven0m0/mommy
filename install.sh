#!/usr/bin/env bash
# Install mommy via cargo and wire it into your bash prompt.
#
# Builds/installs the `mommy` binary with `cargo install`, copies it to
# `cargo-mommy` so `cargo mommy <cmd>` works too, and appends a marker-guarded
# block to ~/.bashrc that hooks PROMPT_COMMAND so mommy reacts to every
# command's exit code (the block sets SHELL_MOMMYS_NEEDY=1 for the session).
# Creates ~/.config/mommy/config.json with defaults ("needy": false) if it
# doesn't exist; an existing file is left alone.
#
# Usage: install.sh [--rc-file PATH] [--skip-profile] [--uninstall]
#   From the web: curl -fsSL https://raw.githubusercontent.com/Ven0m0/mommy/master/install.sh | bash
#   With options: curl -fsSL .../install.sh | bash -s -- --uninstall
set -euo pipefail

readonly usage='Usage: install.sh [--rc-file PATH] [--skip-profile] [--uninstall]'
readonly repo_url='https://github.com/Ven0m0/mommy'
readonly marker_start='# >>> mommy >>>'
readonly marker_end='# <<< mommy <<<'

rc_file="$HOME/.bashrc"
cargo_bin="${CARGO_HOME:-$HOME/.cargo}/bin"

remove_block() {
  [[ -f $rc_file ]] || return 0
  sed -i "/^${marker_start}\$/,/^${marker_end}\$/d" "$rc_file"
}

# Directory of this script's checkout, or nothing when piped from curl
# (BASH_SOURCE is empty then, and dirname would fall back to the cwd).
checkout_dir() {
  local src="${BASH_SOURCE[0]:-}"
  [[ -f $src ]] || return 0
  local dir
  dir="$(cd "$(dirname "$src")" && pwd)"
  [[ -f $dir/Cargo.toml ]] && echo "$dir"
  return 0
}

write_default_config() {
  local config_dir="${XDG_CONFIG_HOME:-$HOME/.config}/mommy"
  local config_path="$config_dir/config.json"
  [[ -e $config_path ]] && return 0
  mkdir -p "$config_dir"
  # Keep in sync with examples/config.json and install.ps1.
  cat >"$config_path" <<'EOF'
{
  "_comment": [
    "moods: mommy picks one at random per message. Available moods:",
    "  chill   - wholesome, supportive encouragement (default; unknown names fall back to it)",
    "  ominous - eldritch, cult-like praise and disapproval",
    "  thirsty - flirty/NSFW, opt-in only",
    "needy: true makes a lone number argument an exit code instead of a command.",
    "  The install.sh/install.ps1 prompt hooks enable it on their own via SHELL_MOMMYS_NEEDY."
  ],
  "moods": ["chill"],
  "needy": false
}
EOF
  echo "Created default config at $config_path"
}

write_rc_block() {
  remove_block
  touch "$rc_file"
  cat >>"$rc_file" <<'EOF'
# >>> mommy >>>
# The prompt hook passes a bare exit code, which mommy only accepts in needy
# mode. Enabling it here (not in config.json) keeps `mommy <command>` elsewhere
# unaffected; needy mode still runs anything that isn't a lone number.
export SHELL_MOMMYS_NEEDY=1
__mommy_prompt() {
  local code=$?
  mommy "$code"
  # Hand the exit code on so later PROMPT_COMMAND entries still see it.
  return "$code"
}
# Must run first in PROMPT_COMMAND, before anything else overwrites $?.
# ponytail: a tool that later prepends itself to PROMPT_COMMAND steals $?; move
# this block to the end of the rc file if that happens.
if [[ ${PROMPT_COMMAND:-} != *__mommy_prompt* ]]; then
  PROMPT_COMMAND="__mommy_prompt${PROMPT_COMMAND:+;$PROMPT_COMMAND}"
fi
# <<< mommy <<<
EOF
  echo "Wired mommy into $rc_file - restart your shell or run 'source $rc_file' to activate."
}

main() {
  local skip_profile=false uninstall=false
  while (($#)); do
    case "$1" in
      --rc-file) rc_file="${2:?--rc-file needs a path}"; shift 2 ;;
      --skip-profile) skip_profile=true; shift ;;
      --uninstall) uninstall=true; shift ;;
      -h | --help) echo "$usage"; return 0 ;;
      *) echo "unknown option: $1" >&2; echo "$usage" >&2; return 1 ;;
    esac
  done

  if $uninstall; then
    remove_block
    echo "Removed mommy block from $rc_file"
    cargo uninstall shell-mommy 2>/dev/null || true
    rm -f "$cargo_bin/cargo-mommy"
    echo "Uninstalled mommy."
    return 0
  fi

  command -v cargo >/dev/null || { echo "cargo not found. Install Rust first: https://rustup.rs/" >&2; return 1; }

  local dir
  dir="$(checkout_dir)"
  if [[ -n $dir ]]; then
    cargo install --locked --force --path "$dir"
  else
    cargo install --locked --force --git "$repo_url"
  fi

  [[ -x $cargo_bin/mommy ]] || { echo "cargo install succeeded but $cargo_bin/mommy not found." >&2; return 1; }
  cp -f "$cargo_bin/mommy" "$cargo_bin/cargo-mommy"
  echo "Installed mommy and cargo-mommy to $cargo_bin"

  $skip_profile && return 0
  write_default_config
  write_rc_block
}

# Everything above only defines functions, so a truncated download from
# `curl | bash` can't run half an install.
main "$@"
