# detect-platform.ps1 — PowerShell sibling of detect-platform.sh.
#
# Same JSON contract, same exit codes, same MOCK_UNAME_S / MOCK_UNAME_M
# test-override envvars. Used on native Windows hosts where Claude Code
# falls back to PowerShell because Git Bash is absent (Claude Code
# v2.1.84+).
#
# Resolves the host to one of the 5 canonical platform strings the v1.0
# dispatcher binaries are built for:
#
#   darwin-arm64   darwin-x64   linux-x64   linux-arm64   windows-x64
#
# Output is JSON on stdout, matching detect-platform.sh:
#
#   {
#     "platform": "windows-x64" | null,
#     "detected_from": { "os": "Windows", "arch": "x86_64", "raw_uname": "Windows x86_64" },
#     "error": null | "unsupported-platform"
#   }
#
# Exit codes:
#   0  supported platform (platform is non-null)
#   1  unsupported platform (platform is null, error is "unsupported-platform")
#   2  usage error (unknown flag, etc.)
#
# Test override: set $env:MOCK_UNAME_S and $env:MOCK_UNAME_M to bypass
# real platform detection. The bash sibling honors the same envvars so
# the test matrix is shared across the two implementations.

param(
  [switch]$Help,
  [Parameter(ValueFromRemainingArguments=$true)]
  [string[]]$Rest
)

$ErrorActionPreference = 'Stop'
# Set-StrictMode is the PowerShell equivalent of bash's `set -u` — surfaces
# typos and uninitialized variables instead of silently coercing to $null.
Set-StrictMode -Version Latest

if ($Help -or ($Rest -and ($Rest[0] -eq '--help' -or $Rest[0] -eq '-h'))) {
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

if ($Rest -and $Rest.Count -gt 0) {
  [Console]::Error.WriteLine("error: unknown argument: $($Rest[0])")
  [Console]::Error.WriteLine("usage: detect-platform.ps1 [-Help|--help|-h]")
  exit 2
}

# --- Detection ---------------------------------------------------------
#
# MOCK_UNAME_S/M empty-string semantics mirror the bash sibling's
# `[ -n "${VAR:-}" ]` test: empty string falls through to real detection,
# only a non-empty value overrides. `[string]::IsNullOrEmpty` is the
# explicit PowerShell equivalent (avoids the loose-truthiness pitfalls of
# bare `if ($env:VAR)` where strings like "0" or "False" could surprise).

if (-not [string]::IsNullOrEmpty($env:MOCK_UNAME_S)) {
  $osString = $env:MOCK_UNAME_S
} else {
  $rti = [System.Runtime.InteropServices.RuntimeInformation]
  $osPlatform = [System.Runtime.InteropServices.OSPlatform]
  if ($rti::IsOSPlatform($osPlatform::Windows)) {
    $osString = 'Windows'
  } elseif ($rti::IsOSPlatform($osPlatform::OSX)) {
    $osString = 'Darwin'
  } elseif ($rti::IsOSPlatform($osPlatform::Linux)) {
    $osString = 'Linux'
  } else {
    $osString = 'unknown'
  }
}

if (-not [string]::IsNullOrEmpty($env:MOCK_UNAME_M)) {
  $archString = $env:MOCK_UNAME_M
} else {
  $arch = [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture
  switch -CaseSensitive ($arch.ToString()) {
    'X64'   { $archString = 'x86_64'; break }
    'Arm64' { $archString = 'arm64';  break }
    'X86'   { $archString = 'i386';   break }
    'Arm'   { $archString = 'arm';    break }
    default { $archString = $arch.ToString() }
  }
}

$rawUname = "$osString $archString"
$platform = ''

# Mapping table mirrors detect-platform.sh exactly. The MINGW*/MSYS*/CYGWIN*
# branches exist purely so the shared test matrix (which injects those
# tuples via MOCK_UNAME_S) passes against both implementations — real
# PowerShell never sees those OS strings.
#
# Restructured as if/elseif chain (instead of nested switch) for two
# reasons: (1) PowerShell switch fires ALL matching arms by default unless
# explicit `break` is used; bash `case` is first-match-wins. (2) PowerShell
# switch is case-INSENSITIVE by default; bash `case` is case-sensitive.
# The if/elseif form makes both semantics explicit and unambiguous.
function Test-CaseSensitive([string]$value, [string]$pattern) {
  return [string]::Equals($value, $pattern, [StringComparison]::Ordinal)
}
function Test-CaseSensitiveLike([string]$value, [string]$prefix) {
  return $value.StartsWith($prefix, [StringComparison]::Ordinal)
}

if (Test-CaseSensitive $osString 'Darwin') {
  if     (Test-CaseSensitive $archString 'arm64')  { $platform = 'darwin-arm64' }
  elseif (Test-CaseSensitive $archString 'x86_64') { $platform = 'darwin-x64' }
}
elseif (Test-CaseSensitive $osString 'Linux') {
  if     (Test-CaseSensitive $archString 'x86_64')  { $platform = 'linux-x64' }
  elseif (Test-CaseSensitive $archString 'aarch64') { $platform = 'linux-arm64' }
  elseif (Test-CaseSensitive $archString 'arm64')   { $platform = 'linux-arm64' }
}
elseif (Test-CaseSensitive $osString 'Windows') {
  # Bash sibling matches `x86_64|amd64` case-sensitively. We do the same.
  if     (Test-CaseSensitive $archString 'x86_64') { $platform = 'windows-x64' }
  elseif (Test-CaseSensitive $archString 'amd64')  { $platform = 'windows-x64' }
}
elseif ((Test-CaseSensitiveLike $osString 'MINGW') -or
        (Test-CaseSensitiveLike $osString 'MSYS')  -or
        (Test-CaseSensitiveLike $osString 'CYGWIN')) {
  # Git Bash / MSYS2 / Cygwin emit a non-Windows-looking `uname -s` but
  # report the underlying CPU correctly. The bash sibling uses
  # `case "$uname_m" in x86_64|amd64) ...` here.
  if     (Test-CaseSensitive $archString 'x86_64') { $platform = 'windows-x64' }
  elseif (Test-CaseSensitive $archString 'amd64')  { $platform = 'windows-x64' }
}

$err = ''
if ([string]::IsNullOrEmpty($platform)) {
  $err = 'unsupported-platform'
}

# --- Emit JSON ---------------------------------------------------------
#
# We hand-build the JSON so the field ordering and null encoding match
# detect-platform.sh exactly. ConvertTo-Json on PowerShell 5.1 emits
# differently-formatted output than jq, and the .bats matrix asserts
# against jq-style output, so handcrafted is safer than ConvertTo-Json
# for cross-implementation contract parity.

function ConvertTo-JsonString([string]$s) {
  if ($null -eq $s) { return 'null' }
  $escaped = $s.Replace('\', '\\').Replace('"', '\"').Replace("`n", '\n').Replace("`r", '\r').Replace("`t", '\t')
  return "`"$escaped`""
}

$platformJson = if ([string]::IsNullOrEmpty($platform)) { 'null' } else { ConvertTo-JsonString $platform }
$errJson      = if ([string]::IsNullOrEmpty($err))      { 'null' } else { ConvertTo-JsonString $err }
$osJson       = ConvertTo-JsonString $osString
$archJson     = ConvertTo-JsonString $archString
$rawJson      = ConvertTo-JsonString $rawUname

$json = "{`"platform`":$platformJson,`"detected_from`":{`"os`":$osJson,`"arch`":$archJson,`"raw_uname`":$rawJson},`"error`":$errJson}"
[Console]::Out.WriteLine($json)

if (-not [string]::IsNullOrEmpty($err)) {
  exit 1
}
exit 0
