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

[CmdletBinding()]
param(
  [switch]$Help,
  [Parameter(ValueFromRemainingArguments=$true)]
  [string[]]$Rest
)

$ErrorActionPreference = 'Stop'

if ($Help -or ($Rest -and ($Rest[0] -eq '--help' -or $Rest[0] -eq '-h'))) {
  # Print the leading comment block (lines starting with '# ') as help —
  # mirrors the bash sibling's `sed -n '2,/^$/p'` pattern.
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
  [Console]::Error.WriteLine("usage: detect-platform.ps1 [-Help]")
  exit 2
}

# --- Detection ---------------------------------------------------------

if ($env:MOCK_UNAME_S) {
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

if ($env:MOCK_UNAME_M) {
  $archString = $env:MOCK_UNAME_M
} else {
  $arch = [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture
  switch ($arch) {
    'X64'   { $archString = 'x86_64' }
    'Arm64' { $archString = 'arm64' }
    'X86'   { $archString = 'i386' }
    'Arm'   { $archString = 'arm' }
    default { $archString = $arch.ToString() }
  }
}

$rawUname = "$osString $archString"
$platform = ''

# Mapping table is intentionally identical to detect-platform.sh — the
# MINGW*/MSYS*/CYGWIN* branches exist purely so the shared test matrix
# (which injects those tuples via MOCK_UNAME_S) passes against both
# implementations. Real PowerShell never sees those OS strings.
switch -Wildcard ($osString) {
  'Darwin' {
    switch ($archString) {
      'arm64'  { $platform = 'darwin-arm64' }
      'x86_64' { $platform = 'darwin-x64' }
    }
  }
  'Linux' {
    switch ($archString) {
      'x86_64'  { $platform = 'linux-x64' }
      'aarch64' { $platform = 'linux-arm64' }
      'arm64'   { $platform = 'linux-arm64' }
    }
  }
  'Windows' {
    switch ($archString) {
      'x86_64' { $platform = 'windows-x64' }
      'amd64'  { $platform = 'windows-x64' }
      'AMD64'  { $platform = 'windows-x64' }
    }
  }
  'MINGW*' {
    switch ($archString) {
      'x86_64' { $platform = 'windows-x64' }
      'amd64'  { $platform = 'windows-x64' }
    }
  }
  'MSYS*' {
    switch ($archString) {
      'x86_64' { $platform = 'windows-x64' }
      'amd64'  { $platform = 'windows-x64' }
    }
  }
  'CYGWIN*' {
    switch ($archString) {
      'x86_64' { $platform = 'windows-x64' }
      'amd64'  { $platform = 'windows-x64' }
    }
  }
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
