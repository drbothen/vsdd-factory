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
#   0  success: variant copied + binary present (and executable on Unix)
#   1  variant missing (hooks.json.<platform> not committed)
#   2  binary missing (dispatcher/bin/<platform> not yet committed)
#   3  binary present but not executable (Unix only — Windows has no
#      executable bit, so this fires only on PowerShell Core 7+ Unix hosts)
#   4  usage error
#
# Both PowerShell-style flags (-Check, -Help) and bash-style aliases
# (--check, --help, -h) are accepted to keep cross-shell muscle memory
# from producing exit-4 surprises.
#
# Side effect: writes `hooks/hooks.json` (overwrites if present).
# Test override: set $env:VSDD_PLUGIN_ROOT_OVERRIDE to use a synthetic
# plugin root instead of the script's own location.

param(
  [switch]$Check,
  [switch]$Help,
  [Parameter(ValueFromRemainingArguments=$true)]
  [string[]]$RestArgs
)

$ErrorActionPreference = 'Stop'
# Set-StrictMode is the PowerShell equivalent of bash's `set -u` — surfaces
# typos and uninitialized variables instead of silently coercing to $null.
Set-StrictMode -Version Latest

# Defensive null-init so subsequent .Count / indexing works without guards.
if ($null -eq $RestArgs) { $RestArgs = @() }

# Accept bash-style aliases for cross-shell muscle memory. We strip them
# from $RestArgs before the positional-arg validation below.
$wantHelp = $Help
if ($RestArgs.Count -gt 0 -and ($RestArgs[0] -eq '--help' -or $RestArgs[0] -eq '-h')) {
  $wantHelp = $true
  $RestArgs = @()
}
$checkMode = $Check
if ($RestArgs.Count -gt 0 -and $RestArgs[0] -eq '--check') {
  $checkMode = $true
  if ($RestArgs.Count -gt 1) {
    $RestArgs = $RestArgs[1..($RestArgs.Count - 1)]
  } else {
    $RestArgs = @()
  }
}

if ($wantHelp) {
  # Print the leading comment block (lines starting with '# ') as help —
  # mirrors the bash sibling's `sed -n '2,/^$/p'` pattern (we stop at the
  # first blank line; bash includes it, the only observable difference).
  $lines = Get-Content -Path $PSCommandPath
  foreach ($line in $lines) {
    if ($line -match '^\s*$') { break }
    if ($line -match '^# ?') { Write-Output ($line -replace '^# ?', '') }
    else { break }
  }
  exit 0
}

if ($RestArgs.Count -ne 1) {
  [Console]::Error.WriteLine('usage: apply-platform.ps1 [-Check|--check] <platform>')
  [Console]::Error.WriteLine('  platform must be one of: darwin-arm64 darwin-x64 linux-x64 linux-arm64 windows-x64')
  exit 4
}

$platform = $RestArgs[0]
$validPlatforms = @('darwin-arm64','darwin-x64','linux-x64','linux-arm64','windows-x64')
# PowerShell's `-contains` operator is case-insensitive by default; the
# bash sibling matches case-sensitively via `case "$platform" in
# darwin-arm64|...) :;; esac`. Fold an explicit Ordinal comparison so
# `apply-platform.ps1 DARWIN-ARM64` is rejected the same way in both.
$platformMatched = $false
foreach ($valid in $validPlatforms) {
  if ([string]::Equals($valid, $platform, [StringComparison]::Ordinal)) {
    $platformMatched = $true
    break
  }
}
if (-not $platformMatched) {
  [Console]::Error.WriteLine("error: unsupported platform: $platform")
  [Console]::Error.WriteLine('supported: darwin-arm64 darwin-x64 linux-x64 linux-arm64 windows-x64')
  exit 4
}

# --- Resolve plugin root ----------------------------------------------

if (-not [string]::IsNullOrEmpty($env:VSDD_PLUGIN_ROOT_OVERRIDE)) {
  $pluginRoot = $env:VSDD_PLUGIN_ROOT_OVERRIDE
} else {
  # The script lives at <plugin_root>/skills/activate/apply-platform.ps1.
  # $PSScriptRoot is empty when the script is invoked via `pwsh -Command`
  # or piped from stdin (rather than `pwsh -File`). Surface a clean
  # diagnostic for that case instead of letting Join-Path throw a
  # confusing CLR null-binding error.
  if ([string]::IsNullOrEmpty($PSScriptRoot)) {
    [Console]::Error.WriteLine('error: $PSScriptRoot is empty — script must be invoked via')
    [Console]::Error.WriteLine('       `pwsh -File apply-platform.ps1 <platform>`, not via')
    [Console]::Error.WriteLine('       `pwsh -Command` or stdin. Alternatively, set')
    [Console]::Error.WriteLine('       $env:VSDD_PLUGIN_ROOT_OVERRIDE to the plugin root.')
    exit 4
  }
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
# PowerShell 7+ (.NET 7+) for cross-platform inspection; we use it when
# present to preserve the bash sibling's exit-code-3 contract on Unix
# hosts running PowerShell Core (e.g., a CI matrix job that mounts the
# .ps1 path on Linux). On native Windows this branch is skipped and
# existence alone is sufficient.
#
# We feature-gate by PowerShell major version (7+) AND by reflection
# property check, so PS 5.1 and older never reach the UnixFileMode access
# (which would be a parser-time TypeNotFound error). Exception handling
# is narrowed to PlatformNotSupportedException only — genuine I/O
# failures (UnauthorizedAccessException, IOException, SecurityException)
# propagate per $ErrorActionPreference = 'Stop' and surface to the user
# instead of silently downgrading exit-3 to "ok".

if ($platform -ne 'windows-x64' -and $PSVersionTable.PSVersion.Major -ge 7) {
  $fileInfo = [System.IO.FileInfo]::new($binary)
  if ($fileInfo.PSObject.Properties.Name -contains 'UnixFileMode') {
    try {
      $mode = $fileInfo.UnixFileMode
      $execMask = [System.IO.UnixFileMode]::UserExecute -bor `
                  [System.IO.UnixFileMode]::GroupExecute -bor `
                  [System.IO.UnixFileMode]::OtherExecute
      if (($mode -band $execMask) -eq [System.IO.UnixFileMode]::None) {
        [Console]::Error.WriteLine("error: dispatcher binary is not executable: $binary")
        [Console]::Error.WriteLine("       fix: chmod +x `"$binary`"")
        exit 3
      }
    } catch [System.PlatformNotSupportedException] {
      # UnixFileMode is the property is present but the API rejects this
      # platform at runtime (e.g., a hypothetical PS7 build without
      # Unix-mode support). Silent fall-through is correct here — the
      # property check above already gated us, and existence is the
      # native-Windows-equivalent signal.
    }
  }
}

# --- Apply or check ----------------------------------------------------

if ($checkMode) {
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
