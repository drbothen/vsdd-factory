#!/usr/bin/env bash
# detect-platform.sh — detect host platform for vsdd-factory v1.0 activation.
#
# Resolves `uname -s` / `uname -m` to one of the 5 canonical platform
# strings the v1.0 dispatcher binaries are built for:
#
#   darwin-arm64   darwin-x64   linux-x64   linux-arm64   windows-x64
#
# Output is JSON on stdout, suitable for jq-piping into the activate
# skill's persistence step:
#
#   {
#     "platform": "darwin-arm64" | null,
#     "detected_from": { "os": "Darwin", "arch": "arm64", "raw_uname": "Darwin arm64" },
#     "error": null | "unsupported-platform"
#   }
#
# Exit codes:
#   0  supported platform (platform is non-null)
#   1  unsupported platform (platform is null, error is "unsupported-platform")
#   2  usage error (unknown flag, etc.)
#
# Test override: set `MOCK_UNAME_S` and `MOCK_UNAME_M` to bypass `uname` —
# the test suite in plugins/vsdd-factory/tests/activate.bats uses this to
# exercise every supported and unsupported tuple deterministically.

set -euo pipefail

if [ $# -gt 0 ]; then
  case "$1" in
    --help|-h)
      sed -n '2,/^$/p' "$0"
      exit 0
      ;;
    *)
      echo "error: unknown argument: $1" >&2
      echo "usage: $0 [--help]" >&2
      exit 2
      ;;
  esac
fi

if [ -n "${MOCK_UNAME_S:-}" ]; then
  uname_s="$MOCK_UNAME_S"
else
  uname_s="$(uname -s 2>/dev/null || echo unknown)"
fi
if [ -n "${MOCK_UNAME_M:-}" ]; then
  uname_m="$MOCK_UNAME_M"
else
  uname_m="$(uname -m 2>/dev/null || echo unknown)"
fi

raw_uname="$uname_s $uname_m"
platform=""

case "$uname_s" in
  Darwin)
    case "$uname_m" in
      arm64)  platform="darwin-arm64" ;;
      x86_64) platform="darwin-x64" ;;
    esac
    ;;
  Linux)
    case "$uname_m" in
      x86_64)        platform="linux-x64" ;;
      aarch64|arm64) platform="linux-arm64" ;;
    esac
    ;;
  MINGW*|MSYS*|CYGWIN*)
    # Git Bash / MSYS2 / Cygwin all emit a non-Windows-looking `uname -s`
    # but report the underlying CPU correctly.
    case "$uname_m" in
      x86_64|amd64) platform="windows-x64" ;;
    esac
    ;;
esac

error=""
if [ -z "$platform" ]; then
  error="unsupported-platform"
fi

# Emit JSON. Prefer jq for correct escaping; fall back to a minimal printf
# path only if jq is unavailable, which keeps this script usable on bare
# Windows shells where the activation skill bootstraps before the user has
# installed everything.
if command -v jq >/dev/null 2>&1; then
  jq -n \
    --arg platform "$platform" \
    --arg os "$uname_s" \
    --arg arch "$uname_m" \
    --arg raw "$raw_uname" \
    --arg err "$error" \
    '{
       platform: (if $platform == "" then null else $platform end),
       detected_from: { os: $os, arch: $arch, raw_uname: $raw },
       error: (if $err == "" then null else $err end)
     }'
else
  if [ -n "$platform" ]; then p="\"$platform\""; else p="null"; fi
  if [ -n "$error" ];    then e="\"$error\"";    else e="null"; fi
  printf '{"platform":%s,"detected_from":{"os":"%s","arch":"%s","raw_uname":"%s"},"error":%s}\n' \
    "$p" "$uname_s" "$uname_m" "$raw_uname" "$e"
fi

if [ -n "$error" ]; then
  exit 1
fi
