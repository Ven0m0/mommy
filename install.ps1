#Requires -Version 5.1
<#
.SYNOPSIS
    Install mommy via cargo and wire it into your PowerShell profile.
.DESCRIPTION
    Builds/installs the `mommy` binary with `cargo install`, copies it to
    `cargo-mommy.exe` so `cargo mommy <cmd>` works too, and appends a
    marker-guarded block to the given profile that wraps `prompt` so mommy
    reacts to every command's exit code (the block sets SHELL_MOMMYS_NEEDY=1
    for the session). Creates ~/.config/mommy/config.json with defaults
    ("needy": false) if it doesn't exist; an existing file is left alone.
    Works on Windows PowerShell 5.1 and PowerShell 7+, from a checkout or
    piped straight from the web.
.PARAMETER ProfilePath
    Profile file to modify. Defaults to $PROFILE of the shell running the script.
.PARAMETER SkipProfile
    Install the binaries only; don't touch the profile.
.PARAMETER Uninstall
    Remove the profile block and uninstall the crate/binaries.
.EXAMPLE
    irm https://raw.githubusercontent.com/Ven0m0/mommy/master/install.ps1 | iex
    Install without a checkout.
.EXAMPLE
    & ([scriptblock]::Create((irm https://raw.githubusercontent.com/Ven0m0/mommy/master/install.ps1))) -Uninstall
    Pass parameters when running from the web.
#>
[CmdletBinding()]
param(
    [string]$ProfilePath = $PROFILE,
    [switch]$SkipProfile,
    [switch]$Uninstall
)

$ErrorActionPreference = 'Stop'

$MarkerStart = '# >>> mommy >>>'
$MarkerEnd = '# <<< mommy <<<'
$ProfileBlock = @"
$MarkerStart
# The prompt hook passes a bare exit code, which mommy only accepts in needy
# mode. Enabling it here (not in config.json) keeps `mommy <command>` elsewhere
# unaffected; needy mode still runs anything that isn't a lone number.
`$env:SHELL_MOMMYS_NEEDY = '1'
`$__mommyWrap = {
    if (-not `$global:__mommyPromptWrapped) {
        `$global:__mommyInnerPrompt = (Get-Command prompt).ScriptBlock
        function global:prompt {
            `$ok = `$?
            # Other prompt integrations can leave a non-int (or a bool) in
            # LASTEXITCODE; -as [int] yields `$null for those instead of passing
            # junk like 'True' to mommy, which only accepts an exit code.
            `$last = `$global:LASTEXITCODE -as [int]
            `$code = if (`$ok) { 0 } elseif (`$last) { `$last } else { 1 }
            # The host drops native stderr while it evaluates prompt, so the
            # affirmation must be captured and re-emitted through the host.
            # Windows PowerShell makes redirected native stderr a terminating
            # error under 'Stop', so relax it for this function only.
            `$ErrorActionPreference = 'Continue'
            mommy `$code 2>&1 | ForEach-Object { Write-Host "`$_" }
            `$global:LASTEXITCODE = `$last
            & `$global:__mommyInnerPrompt
        }
        `$global:__mommyPromptWrapped = `$true
    }
}
# Some profiles (e.g. dotfile setups with async oh-my-posh/zoxide init) redefine
# global:prompt on a deferred PowerShell.OnIdle queue *after* the profile finishes
# loading - wrapping prompt immediately here would just get overwritten later.
# Enqueue onto that queue instead so mommy wraps whatever prompt wins last.
if (`$global:__initQueue -is [System.Collections.Queue]) {
    `$global:__initQueue.Enqueue(`$__mommyWrap)
} else {
    & `$__mommyWrap
}
Remove-Variable -Name __mommyWrap
$MarkerEnd
"@

function Remove-MommyBlock([string[]]$Lines) {
    $out = [System.Collections.Generic.List[string]]::new()
    $skipping = $false
    foreach ($line in $Lines) {
        if ($line.Trim() -eq $MarkerStart) { $skipping = $true; continue }
        if ($line.Trim() -eq $MarkerEnd) { $skipping = $false; continue }
        if (-not $skipping) { $out.Add($line) }
    }
    return $out
}

# $IsWindows doesn't exist on Windows PowerShell 5.1, which only runs on Windows.
$exeSuffix = if ($PSVersionTable.PSEdition -eq 'Desktop' -or $IsWindows) { '.exe' } else { '' }
$cargoHome = if ($env:CARGO_HOME) { $env:CARGO_HOME } else { Join-Path $HOME '.cargo' }
$cargoBin = Join-Path $cargoHome 'bin'
$cargoMommyExe = Join-Path $cargoBin "cargo-mommy$exeSuffix"

if ($Uninstall) {
    if (Test-Path -LiteralPath $ProfilePath) {
        $lines = Get-Content -LiteralPath $ProfilePath
        Remove-MommyBlock $lines | Set-Content -LiteralPath $ProfilePath
        Write-Host "Removed mommy block from $ProfilePath"
    }
    # Windows PowerShell turns native stderr into a terminating error under
    # 'Stop'; a missing crate is fine here.
    try { cargo uninstall shell-mommy 2>$null } catch { $null = $_ }
    Remove-Item -LiteralPath $cargoMommyExe -ErrorAction SilentlyContinue
    Write-Host "Uninstalled mommy."
    return
}

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    throw "cargo not found. Install Rust first: https://rustup.rs/"
}

# $PSScriptRoot is empty when the script is piped into iex; install from git then.
if ($PSScriptRoot -and (Test-Path -LiteralPath (Join-Path $PSScriptRoot 'Cargo.toml'))) {
    cargo install --locked --force --path $PSScriptRoot
} else {
    cargo install --locked --force --git https://github.com/Ven0m0/mommy
}
if ($LASTEXITCODE) { throw "cargo install failed with exit code $LASTEXITCODE." }

$mommyExe = Join-Path $cargoBin "mommy$exeSuffix"
if (-not (Test-Path -LiteralPath $mommyExe)) {
    throw "cargo install succeeded but $mommyExe not found."
}
Copy-Item -LiteralPath $mommyExe -Destination $cargoMommyExe -Force
Write-Host "Installed mommy and cargo-mommy to $cargoBin"

if ($SkipProfile) { return }

$configRoot = if ($env:XDG_CONFIG_HOME) { $env:XDG_CONFIG_HOME } else { Join-Path $HOME '.config' }
$configDir = Join-Path $configRoot 'mommy'
$configPath = Join-Path $configDir 'config.json'
if (-not (Test-Path -LiteralPath $configPath)) {
    # Keep in sync with examples/config.json and install.sh.
    $config = [ordered]@{
        _comment = @(
            'moods: mommy picks one at random per message. Available moods:'
            '  chill   - wholesome, supportive encouragement (default; unknown names fall back to it)'
            '  ominous - eldritch, cult-like praise and disapproval'
            '  thirsty - flirty/NSFW, opt-in only'
            'needy: true makes a lone number argument an exit code instead of a command.'
            '  The install.sh/install.ps1 prompt hooks enable it on their own via SHELL_MOMMYS_NEEDY.'
        )
        moods    = @('chill')
        needy    = $false
    }
    $null = New-Item -ItemType Directory -Path $configDir -Force
    $config | ConvertTo-Json | Set-Content -LiteralPath $configPath
    Write-Host "Created default config at $configPath"
}

if (-not $ProfilePath) { throw 'No $PROFILE in this host; pass -ProfilePath or -SkipProfile.' }
$profileDir = Split-Path -Parent $ProfilePath
if (-not (Test-Path $profileDir)) { New-Item -ItemType Directory -Path $profileDir -Force | Out-Null }
if (-not (Test-Path $ProfilePath)) { New-Item -ItemType File -Path $ProfilePath -Force | Out-Null }

$lines = Get-Content -LiteralPath $ProfilePath
$withoutOld = Remove-MommyBlock $lines
$newContent = ($withoutOld -join "`n").TrimEnd() + "`n`n" + $ProfileBlock
Set-Content -LiteralPath $ProfilePath -Value $newContent
Write-Host "Wired mommy into $ProfilePath - restart PowerShell or run '. `$PROFILE' to activate."
