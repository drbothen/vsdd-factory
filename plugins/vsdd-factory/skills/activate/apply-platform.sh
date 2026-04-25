#!/usr/bin/env bash
# apply-platform.sh — copy hooks.json.<platform> into place + verify the
# v1.0 dispatcher binary is present.
#
# Companion to detect-platform.sh (S-0.3). detect-platform reports which
# of the 5 canonical platform tuples this host is; apply-platform takes
# that tuple and:
#
#   1. Copies `hooks/hooks.json.<platform>` to `hooks/hooks.json`.
#      (The canonical file is .gitignore'd per S-0.4 since it's
#      generated per-machine at activation time.)
#   2. Verifies the dispatcher binary at
#      `hooks/dispatcher/bin/<platform>/factory-dispatcher[.exe]` exists
#      and is executable. The release workflow commits these binaries
#      (S-2.4); during early v1.0-beta development they may be missing.
#   3. Reports a clean diagnostic on missing inputs without leaving
#      the workspace half-activated.
#
# Usage:
#   apply-platform.sh <platform>
#   apply-platform.sh --check <platform>   # verify-only, no copy
#
# Exit:
#   0  success: variant copied + binary present + executable
#   1  variant missing (hooks.json.<platform> not committed)
#   2  binary missing (dispatcher/bin/<platform> not yet committed —
#      common during early v1.0-beta until S-2.4 wires the
#      release-workflow binary commit)
#   3  binary present but not executable
#   4  usage error
#
# Side effect: writes `hooks/hooks.json` (overwrites if present).
# Test override: set `VSDD_PLUGIN_ROOT_OVERRIDE` to use a synthetic
# plugin root instead of the script's own location.

set -euo pipefail

mode="apply"
case "${1:-}" in
  --check) mode="check"; shift ;;
  --help|-h)
    sed -n '2,/^$/p' "$0"
    exit 0
    ;;
esac

if [ $# -ne 1 ]; then
  echo "usage: $0 [--check] <platform>" >&2
  echo "  platform must be one of: darwin-arm64 darwin-x64 linux-x64 linux-arm64 windows-x64" >&2
  exit 4
fi

platform="$1"
case "$platform" in
  darwin-arm64|darwin-x64|linux-x64|linux-arm64|windows-x64) : ;;
  *)
    echo "error: unsupported platform: $platform" >&2
    echo "supported: darwin-arm64 darwin-x64 linux-x64 linux-arm64 windows-x64" >&2
    exit 4
    ;;
esac

if [ -n "${VSDD_PLUGIN_ROOT_OVERRIDE:-}" ]; then
  plugin_root="$VSDD_PLUGIN_ROOT_OVERRIDE"
else
  # The script lives at <plugin_root>/skills/activate/apply-platform.sh.
  plugin_root="$(cd "$(dirname "$0")/../.." && pwd)"
fi

variant="$plugin_root/hooks/hooks.json.$platform"
canonical="$plugin_root/hooks/hooks.json"

# Windows binaries get a .exe suffix; everything else is bare.
case "$platform" in
  windows-x64) exe_suffix=".exe" ;;
  *)           exe_suffix="" ;;
esac
binary="$plugin_root/hooks/dispatcher/bin/$platform/factory-dispatcher$exe_suffix"

if [ ! -f "$variant" ]; then
  echo "error: missing variant: $variant" >&2
  echo "       The hooks.json.<platform> files are CI-generated from" >&2
  echo "       hooks.json.template by scripts/generate-hooks-json.sh" >&2
  echo "       and should ship with the plugin. Reinstall or run the" >&2
  echo "       generator to regenerate." >&2
  exit 1
fi

if [ ! -f "$binary" ]; then
  echo "error: dispatcher binary missing for $platform" >&2
  echo "       expected: $binary" >&2
  echo "" >&2
  echo "       The release workflow (S-2.4) is responsible for committing" >&2
  echo "       per-platform dispatcher binaries on every tag. During" >&2
  echo "       v1.0-beta development this may not yet be wired up." >&2
  echo "" >&2
  echo "       Workarounds:" >&2
  echo "         - Pin to vsdd-factory v0.79.4 until v1.0.0-beta.1 ships" >&2
  echo "           (no dispatcher binaries needed; bash hooks via legacy" >&2
  echo "           paths)" >&2
  echo "         - Build the dispatcher locally:" >&2
  echo "             cargo build --release -p factory-dispatcher" >&2
  echo "           then copy the binary to:" >&2
  echo "             $binary" >&2
  exit 2
fi

if [ ! -x "$binary" ]; then
  echo "error: dispatcher binary is not executable: $binary" >&2
  echo "       fix: chmod +x \"$binary\"" >&2
  exit 3
fi

if [ "$mode" = "check" ]; then
  echo "ok: variant + binary present for $platform"
  echo "    variant=$variant"
  echo "    binary=$binary"
  exit 0
fi

# Apply: copy the variant into place.
cp "$variant" "$canonical"
echo "ok: applied $platform"
echo "    hooks.json <- $variant"
echo "    binary verified: $binary"
