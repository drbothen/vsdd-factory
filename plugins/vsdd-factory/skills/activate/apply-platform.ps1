# apply-platform.ps1 — PowerShell sibling of apply-platform.sh.
#
# Same exit-code contract, same VSDD_PLUGIN_ROOT_OVERRIDE test override,
# same diagnostic messages. Used on native Windows hosts where Claude
# Code falls back to PowerShell because Git Bash is absent.
#
# Companion to detect-platform.ps1 (S-0.3 follow-up). detect-platform
# reports which of the 5 canonical platform tuples this host is;
# apply-platform takes that tuple and:
#
#   1. Copies `hooks/hooks.json.<platform>` to `hooks/hooks.json`.
#      (The canonical file is .gitignore'd per S-0.4 since it's
#      generated per-machine at activation time.)
#   2. Verifies the dispatcher binary at
#      `hooks/dispatcher/bin/<platform>/factory-dispatcher[.exe]`
#      exists. On Windows there is no executable bit, so the +x check
#      from the bash sibling is a no-op here — file existence is the
#      executability signal.
#   3. Reports a clean diagnostic on missing inputs without leaving
#      the workspace half-activated.
#
# Usage:
#   apply-platform.ps1 <platform>
#   apply-platform.ps1 -Check <platform>   # verify-only, no copy
#
# Exit:
#   0  success: variant copied + binary present
#   1  variant missing (hooks.json.<platform> not committed)
#   2  binary missing (dispatcher/bin/<platform> not yet committed)
#   3  binary present but not executable (rarely observable on Windows;
#      kept for cross-implementation contract parity)
#   4  usage error
#
# Side effect: writes `hooks/hooks.json` (overwrites if present).
# Test override: set $env:VSDD_PLUGIN_ROOT_OVERRIDE to use a synthetic
# plugin root instead of the script's own location.

[CmdletBinding()]
param(
  [switch]$Check,
  [switch]$Help,
  [Parameter(Position=0, ValueFromRemainingArguments=$true)]
  [string[]]$RestArgs
)

$ErrorActionPreference = 'Stop'

if ($Help) {
  # Print the leading comment block (lines starting with '# ') as help.
  $lines = Get-Content -Path $PSCommandPath
  foreach ($line in $lines) {
    if ($line -match '^\s*$') { break }
    if ($line -match '^# ?') { Write-Output ($line -replace '^# ?', '') }
    else { break }
  }
  exit 0
}

if (-not $RestArgs -or $RestArgs.Count -ne 1) {
  [Console]::Error.WriteLine('usage: apply-platform.ps1 [-Check] <platform>')
  [Console]::Error.WriteLine('  platform must be one of: darwin-arm64 darwin-x64 linux-x64 linux-arm64 windows-x64')
  exit 4
}

$platform = $RestArgs[0]
$validPlatforms = @('darwin-arm64','darwin-x64','linux-x64','linux-arm64','windows-x64')
if ($validPlatforms -notcontains $platform) {
  [Console]::Error.WriteLine("error: unsupported platform: $platform")
  [Console]::Error.WriteLine('supported: darwin-arm64 darwin-x64 linux-x64 linux-arm64 windows-x64')
  exit 4
}

# --- Resolve plugin root ----------------------------------------------

if ($env:VSDD_PLUGIN_ROOT_OVERRIDE) {
  $pluginRoot = $env:VSDD_PLUGIN_ROOT_OVERRIDE
} else {
  # The script lives at <plugin_root>/skills/activate/apply-platform.ps1.
  # Forward slashes are accepted by PowerShell's path APIs on every host
  # OS; backslashes would break on PowerShell Core running on Unix.
  $pluginRoot = (Resolve-Path -Path (Join-Path -Path $PSScriptRoot -ChildPath '../..')).Path
}

$variant   = Join-Path -Path $pluginRoot -ChildPath "hooks/hooks.json.$platform"
$canonical = Join-Path -Path $pluginRoot -ChildPath 'hooks/hooks.json'

# Windows binaries get a .exe suffix; everything else is bare.
$exeSuffix = if ($platform -eq 'windows-x64') { '.exe' } else { '' }
$binary    = Join-Path -Path $pluginRoot -ChildPath "hooks/dispatcher/bin/$platform/factory-dispatcher$exeSuffix"

# --- Variant present ---------------------------------------------------

if (-not (Test-Path -Path $variant -PathType Leaf)) {
  [Console]::Error.WriteLine("error: missing variant: $variant")
  [Console]::Error.WriteLine('       The hooks.json.<platform> files are CI-generated from')
  [Console]::Error.WriteLine('       hooks.json.template by scripts/generate-hooks-json.sh')
  [Console]::Error.WriteLine('       and should ship with the plugin. Reinstall or run the')
  [Console]::Error.WriteLine('       generator to regenerate.')
  exit 1
}

# --- Binary present ----------------------------------------------------

if (-not (Test-Path -Path $binary -PathType Leaf)) {
  [Console]::Error.WriteLine("error: dispatcher binary missing for $platform")
  [Console]::Error.WriteLine("       expected: $binary")
  [Console]::Error.WriteLine('')
  [Console]::Error.WriteLine('       The release workflow (S-2.4) is responsible for committing')
  [Console]::Error.WriteLine('       per-platform dispatcher binaries on every tag. During')
  [Console]::Error.WriteLine('       v1.0-beta development this may not yet be wired up.')
  [Console]::Error.WriteLine('')
  [Console]::Error.WriteLine('       Workarounds:')
  [Console]::Error.WriteLine('         - Pin to vsdd-factory v0.79.4 until v1.0.0-beta.1 ships')
  [Console]::Error.WriteLine('           (no dispatcher binaries needed; bash hooks via legacy')
  [Console]::Error.WriteLine('           paths)')
  [Console]::Error.WriteLine('         - Build the dispatcher locally:')
  [Console]::Error.WriteLine('             cargo build --release -p factory-dispatcher')
  [Console]::Error.WriteLine('           then copy the binary to:')
  [Console]::Error.WriteLine("             $binary")
  exit 2
}

# --- Executability -----------------------------------------------------
#
# Windows has no chmod bit. The CLR's UnixFileMode API is available on
# PowerShell 7+ for cross-platform inspection; we use it when present to
# preserve the bash sibling's exit-code-3 contract on Unix hosts running
# PowerShell Core (e.g., a CI matrix job that mounts the .ps1 path on
# Linux). On native Windows this branch is skipped and existence alone
# is sufficient.

if ($platform -ne 'windows-x64') {
  try {
    $fileInfo = [System.IO.FileInfo]::new($binary)
    if ($fileInfo.PSObject.Properties.Name -contains 'UnixFileMode') {
      $mode = $fileInfo.UnixFileMode
      $execMask = [System.IO.UnixFileMode]::UserExecute -bor `
                  [System.IO.UnixFileMode]::GroupExecute -bor `
                  [System.IO.UnixFileMode]::OtherExecute
      if (($mode -band $execMask) -eq [System.IO.UnixFileMode]::None) {
        [Console]::Error.WriteLine("error: dispatcher binary is not executable: $binary")
        [Console]::Error.WriteLine("       fix: chmod +x `"$binary`"")
        exit 3
      }
    }
  } catch {
    # UnixFileMode not available (PowerShell 5.1 / older CLR) — fall back
    # to existence-only, matching native-Windows semantics.
  }
}

# --- Apply or check ----------------------------------------------------

if ($Check) {
  Write-Output "ok: variant + binary present for $platform"
  Write-Output "    variant=$variant"
  Write-Output "    binary=$binary"
  exit 0
}

Copy-Item -Path $variant -Destination $canonical -Force
Write-Output "ok: applied $platform"
Write-Output "    hooks.json <- $variant"
Write-Output "    binary verified: $binary"
exit 0
