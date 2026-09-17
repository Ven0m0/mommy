# mommy - affirmations in your terminal 💞

![screenshot](https://github.com/sleepymincy/sleepymincy/blob/main/.gitfiles/repos/images/mommy.png)

Clearly inspired by by [Gankra/cargo-mommy](https://github.com/Gankra/cargo-mommy) and
original (in Bash) [sudofox/shell-mommy](https://github.com/sudofox/shell-mommy).

After using Bash implementation for a bit, I've decided to try writing my own
implementation in Rust for the sake of learning new things. ~~In the process I think I
got too far lost in the cult of Rust.~~

**NEW:** Full Windows/PowerShell support, alongside cargo-mommy integration! Use as
`mommy` for shell commands and (with one extra copy step, see below) as `cargo-mommy`
for a Cargo subcommand.

## Quick Links

- [How to build](#how-to-build)
- [Easy install](#easy-install)
- [Windows install](#windows-install)
- [Configuration](#configuration)
- [The `beg` feature](#the-beg-feature)
- [Known bugs / limitations](#known-bugs--limitations)
- [License information](#license-information)

## How to build

- Get [Rust](https://rustup.rs/)
- `git clone https://github.com/Ven0m0/mommy`
- `cd mommy`
- `cargo build` or `cargo build -r` for release version (recommended)
- Compiled binary will be in `./target/release/`

## Easy install

There is only one compiled binary, `mommy` - it detects shell-vs-cargo mode and the
mommy/daddy role from its own filename (see `BinaryInfo::detect` in `src/config.rs`).
`cargo install` cannot produce a second `cargo-mommy` binary on its own, so if you want
to use it as a Cargo subcommand you need one extra copy step:

```sh
cargo install --git https://github.com/Ven0m0/mommy
cp ~/.cargo/bin/mommy ~/.cargo/bin/cargo-mommy   # enables `cargo mommy ...`
```

(The upstream `shell-mommy` crate on crates.io is a different, unrelated fork - this
repo isn't published there.)

## Windows install

Run the installer from PowerShell:

```powershell
.\install.ps1
```

This builds and installs `mommy` via `cargo install`, copies it to `cargo-mommy.exe` so
`cargo mommy <cmd>` works too, and appends a marker-guarded block to
`$env:USERPROFILE\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` that wraps your
prompt so mommy reacts to every command's exit code (sets `SHELL_MOMMYS_NEEDY=1`
internally - see [Configuration](#configuration) below for what that does). Re-running
the script updates the block in place instead of duplicating it.

- `.\install.ps1 -SkipProfile` - install the binaries only, skip the profile hook
- `.\install.ps1 -Uninstall` - remove the profile block and uninstall the crate

For `SHELL_MOMMYS_ALIASES` on Windows, point it at a `.ps1` file that defines functions
or `Set-Alias` entries - mommy dot-sources it before running your command.

## Configuration

### Environment Variables

This tool supports **dual prefixes** for environment variables:

- `SHELL_MOMMYS_*` - for shell command usage (e.g., `mommy ls`)
- `CARGO_MOMMYS_*` - for cargo subcommand usage (e.g., `cargo mommy build`)

The tool automatically detects which binary you're using and prefers the appropriate
prefix, with fallback support for cross-compatibility.

Available environment variables:

- `SHELL_MOMMYS_EMOTES` / `CARGO_MOMMYS_EMOTES` - to set the emotes to anything u want
- `SHELL_MOMMYS_LITTLE` / `CARGO_MOMMYS_LITTLE` - to set the petnames mommy is using
  towards u
- `SHELL_MOMMYS_ROLES` / `CARGO_MOMMYS_ROLES` - to change mommy to daddy or whatever
  else (auto-detected from binary name)
- `SHELL_MOMMYS_PRONOUNS` / `CARGO_MOMMYS_PRONOUNS` - to change mommy's pronouns
- `SHELL_MOMMYS_MOODS` / `CARGO_MOMMYS_MOODS` - picks the set of possible responses
  (default: "chill", possible values: "chill", "ominous", "thirsty"; an unknown mood
  falls back to "chill")
- `SHELL_MOMMYS_MOOD_MIXING` / `CARGO_MOMMYS_MOOD_MIXING` - `1` or `0` (default);
  when `1` and the picked mood is "ominous", has a 20% chance to blend in a "thirsty"
  line too
- `SHELL_MOMMYS_COLOR` / `CARGO_MOMMYS_COLOR` - to change text color
- `SHELL_MOMMYS_STYLE` / `CARGO_MOMMYS_STYLE` - to change text style
- `SHELL_MOMMYS_COLOR_RGB` / `CARGO_MOMMYS_COLOR_RGB` - to set custom rgb color for the
  text
- `SHELL_MOMMYS_ALIASES` / `CARGO_MOMMYS_ALIASES` - provide path to your aliases file
  for mommy to source. On Linux/macOS this is sourced with `bash` (a `.bashrc`-style
  file); on Windows it's dot-sourced with `powershell` (a `.ps1` file defining
  functions or `Set-Alias`)
- `SHELL_MOMMYS_AFFIRMATIONS` / `CARGO_MOMMYS_AFFIRMATIONS` - provide a path to a valid
  `.json` file, formatted exactly like
  [assets/affirmations.json](https://github.com/Ven0m0/mommy/blob/master/assets/affirmations.json)
  (a top-level `moods` object with `positive`/`negative` arrays per mood - see
  [Moods System](#moods-system-advanced-responses) below), otherwise the code will fall
  back to built-in default affirmations
- `SHELL_MOMMYS_NEEDY` / `CARGO_MOMMYS_NEEDY` - can be `1`, or `0` (default), decides if
  mommy is accepting exit code as an argument, or a command
- `SHELL_MOMMY_ONLY_NEGATIVE` / `CARGO_MOMMY_ONLY_NEGATIVE` - can be `1` or `0`
  (default), decides if mommy only talks when exit code is not 0
- `SHELL_MOMMY_RECURSION_LIMIT` / `CARGO_MOMMY_RECURSION_LIMIT` - internal recursion
  counter passed to child invocations, not meant to be set by hand; mommy refuses to run
  once it reaches 100 (see [Advanced Features](#advanced-features))

You can either specify environment variables every time you run mommy:

```ansi
you@archbtw:~$ SHELL_MOMMYS_COLOR="blue" SHELL_MOMMYS_STYLE="bold" mommy ls -l
drwxr-xr-x - you 20 April 04:20 📁 dir1
drwxr-xr-x - you 20 April 04:20 📁 dir2
drwxr-xr-x - you 20 April 04:20 📁 dir3
you're doing so well~! 💓 <- will be blue and bold
```

Or all add this to your `.bashrc` (or any other rc file) to customize it user wide, for
example:

```sh
export SHELL_MOMMYS_PRONOUNS="his"
export SHELL_MOMMYS_ROLES="daddy"
export SHELL_MOMMYS_LITTLE="discord kitten/kitty"
export SHELL_MOMMYS_EMOTES="🤤/💕/🥺/💋"
export SHELL_MOMMYS_COLOR="blue/red" # Will be randomly rotated between blue and red colors.
export SHELL_MOMMYS_STYLE="bold,italic/bold" # Will be randomly rotated between bold italic style and just bold style.
export SHELL_MOMMYS_COLOR_RGB="255,164,243/255,50,50" # Will be randomly rotated between lilac and red colors in this example. Note, that this setting will overwrite SHELL_MOMMYS_COLOR !!!
export SHELL_MOMMYS_ALIASES="$HOME/.config/aliases"
export SHELL_MOMMYS_AFFIRMATIONS="$HOME/.config/affirmations.json"
export SHELL_MOMMYS_NEEDY=1 # Will make mommy take error code instead of a command, which can allow you to run mommy at all times
export SHELL_MOMMY_ONLY_NEGATIVE=1 # Will make mommy only print affirmations if exit code is not 0
```

When you set `SHELL_MOMMYS_NEEDY` variable to `1`, mommy will accept exit codes instead
of commands as an argument. Examples:

- `sjdfhsdjkfhsdf; mommy $?` <- returns exit code `127`, which will result in negative
  response from mommy
- `ls; mommy $?` <- returns exit code `0`, which will make mommy give a positive
  response

To make this behavior consistent, you can add these to your relevant rc files:

```bash
# ~/.bashrc
export PROMPT_COMMAND="mommy \$?; $PROMPT_COMMAND"
```

...or, a more safe option, if your shell gives you issues:

```bash
# ~/.bashrc
# Double check that mommy is not already present in PROMPT_COMMAND, to avoid duplicate affirmations
if [[ $(echo $PROMPT_COMMAND) != *"mommy"* ]]; then
    export PROMPT_COMMAND="mommy \$?; $PROMPT_COMMAND"
fi
```

```bash
# ~/.zshrc
precmd() { mommy $? }
```

```bash
# Others (not tested)
export PS1="\$(mommy \$?)$PS1"
```

On Windows, `.\install.ps1` sets this up for you by wrapping your PowerShell `prompt`
function - see [Windows install](#windows-install).

You can also change `affirmations.json` before building, or load your own with
`SHELL_MOMMYS_AFFIRMATIONS` during runtime, to un-degenerate this piece of software or
make it worse. I'm not the one to judge.

## Cargo-Mommy Features

Once you've copied the binary to `cargo-mommy` (see [Easy install](#easy-install) /
[Windows install](#windows-install)), you can use it as a Cargo subcommand:

### Basic Usage

```bash
# Use as a cargo subcommand
cargo mommy build
cargo mommy test
cargo mommy run

# Quiet mode - suppresses mommy's output
cargo mommy --quiet build
cargo mommy -q test
```

### Role Transformation

Transform the binary into different roles without reinstalling:

```bash
# Create a cargo-daddy binary
mommy i mean daddy
# or
cargo mommy i mean daddy

# Now you can use:
cargo daddy build
```

This copies the currently running binary to a new file next to itself (with `.exe`
appended on Windows), named `cargo-<role>` or `<role>` depending on which mode you're
running in.

### Advanced Features

- **Recursion Protection**: Each invocation increments a counter passed to the child
  process via `SHELL_MOMMY_RECURSION_LIMIT` / `CARGO_MOMMY_RECURSION_LIMIT`. If it
  reaches 100, mommy refuses to run and exits with code `2` instead of looping forever.
- **Binary Name Detection**: Automatically detects if you're using `cargo-mommy` vs
  `mommy` and adjusts behavior
- **Dual Environment Variable Support**: Works with both `CARGO_MOMMYS_*` and
  `SHELL_MOMMYS_*` prefixes
- **Quiet Mode**: Use `--quiet` or `-q` flags to suppress affirmations while still
  running commands

### Example Cargo Usage

```bash
# Set cargo-specific variables
export CARGO_MOMMYS_ROLES="daddy"
export CARGO_MOMMYS_PRONOUNS="his"
export CARGO_MOMMYS_LITTLE="kiddo"

# Run cargo commands with affirmations
cargo mommy build --release
# daddy is so proud of his little kiddo~ 💞

# Use quiet mode for CI/CD
cargo mommy -q test
# (runs tests silently without affirmations)
```

### Moods System (Advanced Responses)

Mommy now supports different moods that change the style and tone of her responses! You
can set this with the `SHELL_MOMMYS_MOODS` environment variable.

Available moods:

- **chill** (default): The classic mommy experience with sweet and supportive messages
- **ominous**: A darker, more foreboding tone with references to cosmic horror and
  ancient powers
- **thirsty**: More flirtatious and playful responses (slightly spicy! 🌶️)

You can specify multiple moods separated by `/`, and mommy will randomly pick one each
time:

```bash
export SHELL_MOMMYS_MOODS="chill"           # Always use chill mood
export SHELL_MOMMYS_MOODS="chill/ominous"   # Randomly alternate between chill and ominous
export SHELL_MOMMYS_MOODS="ominous"         # Always use ominous mood
```

Example usage:

```bash
# Try the ominous mood
SHELL_MOMMYS_MOODS="ominous" mommy ls
# Might output: "What you have set in motion today will be remembered for aeons to come! 💞"

# Mix multiple moods
SHELL_MOMMYS_MOODS="chill/ominous/thirsty" mommy echo "hello"
# Output will randomly be from any of the three moods

# Occasionally blend ominous with thirsty
SHELL_MOMMYS_MOODS="ominous" SHELL_MOMMYS_MOOD_MIXING=1 mommy echo "hello"
```

**Custom Affirmations with Moods:**

If you're using a custom affirmations file via `SHELL_MOMMYS_AFFIRMATIONS`, structure it
with moods! See
[assets/affirmations.json](https://github.com/Ven0m0/mommy/blob/master/assets/affirmations.json)
for the format: a top-level `moods` object with mood names as keys, each containing
`positive` and `negative` arrays. There is no supported top-level fallback -
`assets/affirmations.json` itself only defines `moods`, so an unrecognized mood name
just falls back to the `chill` entry inside `moods`.

## The `beg` feature

An opt-in, stateful mood that isn't built by default:

```sh
cargo build --features beg
```

When enabled, mommy remembers whether it's "angry" in `~/.mommy.state` (or
`%USERPROFILE%\.mommy.state` on Windows, falling back to the OS temp dir if neither
`HOME` nor `USERPROFILE` is set - see `src/state.rs`). After a failing command mommy
gets angry and keeps refusing (exit code `1`) until you run it again with `please` in
the command:

```sh
mommy false        # fails, mommy gets angry
mommy ls           # still angry, exits 1 until you say please
mommy ls please    # forgiven, mood resets to chill
```

This is the only part of the codebase that persists anything to disk - everything else
is stateless.

## Known bugs / limitations

- Only one binary (`mommy`) is built by Cargo - there is no separate `cargo-mommy`
  target, so `cargo install` alone won't give you the Cargo-subcommand entry point. You
  need the manual copy step in [Easy install](#easy-install) (`install.ps1` does this
  for you on Windows).
- `.github/workflows/build.yml` currently hardcodes version `0.1.5` in its Debian
  packaging job while `Cargo.toml` is at a newer version - a known drift, not something
  end users need to worry about.
- `SHELL_MOMMYS_ALIASES` / `CARGO_MOMMYS_ALIASES` must point at a `bash`-compatible file
  on Linux/macOS and a `.ps1` file on Windows - the two are not interchangeable.
- Setting `SHELL_MOMMYS_NEEDY=1` and wiring mommy into your prompt (as `install.ps1`
  does) means it runs on *every* prompt render, not just after commands you care about.
- Open up an [issue](https://github.com/Ven0m0/mommy/issues/new) if you find something
  else.

## License information

This is free and unencumbered software released into the public domain.

Anyone is free to copy, modify, publish, use, compile, sell, or distribute this
software, either in source code form or as a compiled binary, for any purpose,
commercial or non-commercial, and by any means.

In jurisdictions that recognize copyright laws, the author or authors of this software
dedicate any and all copyright interest in the software to the public domain. We make
this dedication for the benefit of the public at large and to the detriment of our heirs
and successors. We intend this dedication to be an overt act of relinquishment in
perpetuity of all present and future rights to this software under copyright law.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR
PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM,
DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

For more information, please refer to <https://unlicense.org>
