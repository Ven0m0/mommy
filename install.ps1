#Requires -Version 5.1
<#
.SYNOPSIS
    Install mommy via cargo and wire it into your PowerShell profile.
.DESCRIPTION
    Builds/installs the `mommy` binary with `cargo install`, copies it to
    `cargo-mommy.exe` so `cargo mommy <cmd>` works too, and appends a
    marker-guarded block to the given profile that wraps `prompt` so mommy
    reacts to every command's exit code (SHELL_MOMMYS_NEEDY=1).
.PARAMETER ProfilePath
    Profile file to modify. Defaults to the current-user PowerShell profile.
.PARAMETER SkipProfile
    Install the binaries only; don't touch the profile.
.PARAMETER Uninstall
    Remove the profile block and uninstall the crate/binaries.
#>
[CmdletBinding()]
param(
    [string]$ProfilePath = "$env:USERPROFILE\Documents\PowerShell\Microsoft.PowerShell_profile.ps1",
    [switch]$SkipProfile,
    [switch]$Uninstall
)

$ErrorActionPreference = 'Stop'

$MarkerStart = '# >>> mommy >>>'
$MarkerEnd = '# <<< mommy <<<'
$ProfileBlock = @"
$MarkerStart
`$env:SHELL_MOMMYS_NEEDY = '1'
`$__mommyWrap = {
    if (-not `$global:__mommyPromptWrapped) {
        `$global:__mommyInnerPrompt = (Get-Command prompt).ScriptBlock
        function global:prompt {
            `$ok = `$?
            `$last = `$global:LASTEXITCODE
            `$code = if (`$ok) { 0 } elseif (`$last) { `$last } else { 1 }
            mommy `$code
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

if ($Uninstall) {
    if (Test-Path $ProfilePath) {
        $lines = Get-Content -LiteralPath $ProfilePath
        Remove-MommyBlock $lines | Set-Content -LiteralPath $ProfilePath
        Write-Host "Removed mommy block from $ProfilePath"
    }
    cargo uninstall shell-mommy 2>$null
    $cargoBin = if ($env:CARGO_HOME) { Join-Path $env:CARGO_HOME 'bin' } else { "$env:USERPROFILE\.cargo\bin" }
    Remove-Item -LiteralPath (Join-Path $cargoBin 'cargo-mommy.exe') -ErrorAction SilentlyContinue
    Write-Host "Uninstalled mommy."
    return
}

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    throw "cargo not found. Install Rust first: https://rustup.rs/"
}

if (Test-Path (Join-Path $PSScriptRoot 'Cargo.toml')) {
    cargo install --path $PSScriptRoot --force
} else {
    cargo install --git https://github.com/Ven0m0/mommy --force
}

$cargoBin = if ($env:CARGO_HOME) { Join-Path $env:CARGO_HOME 'bin' } else { "$env:USERPROFILE\.cargo\bin" }
$mommyExe = Join-Path $cargoBin 'mommy.exe'
if (-not (Test-Path $mommyExe)) {
    throw "cargo install succeeded but $mommyExe not found."
}
Copy-Item -LiteralPath $mommyExe -Destination (Join-Path $cargoBin 'cargo-mommy.exe') -Force
Write-Host "Installed mommy.exe and cargo-mommy.exe to $cargoBin"

if ($SkipProfile) { return }

$profileDir = Split-Path -Parent $ProfilePath
if (-not (Test-Path $profileDir)) { New-Item -ItemType Directory -Path $profileDir -Force | Out-Null }
if (-not (Test-Path $ProfilePath)) { New-Item -ItemType File -Path $ProfilePath -Force | Out-Null }

$lines = Get-Content -LiteralPath $ProfilePath
$withoutOld = Remove-MommyBlock $lines
$newContent = ($withoutOld -join "`n").TrimEnd() + "`n`n" + $ProfileBlock
Set-Content -LiteralPath $ProfilePath -Value $newContent
Write-Host "Wired mommy into $ProfilePath - restart PowerShell or run '. `$PROFILE' to activate."
