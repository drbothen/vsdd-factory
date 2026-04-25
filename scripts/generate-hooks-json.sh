#!/usr/bin/env bash
# generate-hooks-json.sh — render hooks.json.template into 5 platform-specific
# variants and validate each as JSON.
#
# Why this exists: Claude Code's hooks.json can't reference a different binary
# per OS at runtime — there's no variable expansion for platform. So at install
# time the activation skill copies the right hooks.json.<platform> into place.
# This generator owns the single template that all 5 variants derive from, so
# operators don't hand-edit five files in lockstep. CI runs the generator on
# every push and diffs against the committed variants; drift fails the build.
#
# Usage:
#   scripts/generate-hooks-json.sh           # generate (default)
#   scripts/generate-hooks-json.sh --check   # generate to a temp dir, diff
#                                            #   against committed variants,
#                                            #   exit nonzero on drift
#
# Placeholders in the template:
#   {{PLATFORM}}      → darwin-arm64 / darwin-x64 / linux-x64 / linux-arm64 / windows-x64
#   {{EXE_SUFFIX}}    → ""   for unix
#                       ".exe" for windows-x64

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
TEMPLATE="$REPO_ROOT/plugins/vsdd-factory/hooks/hooks.json.template"
OUT_DIR="$REPO_ROOT/plugins/vsdd-factory/hooks"

# Each entry: "platform:exe-suffix". Add platforms here as the matrix expands.
PLATFORMS=(
  "darwin-arm64:"
  "darwin-x64:"
  "linux-x64:"
  "linux-arm64:"
  "windows-x64:.exe"
)

mode="generate"
if [ $# -ge 1 ]; then
  case "$1" in
    --check) mode="check" ;;
    --help|-h)
      sed -n '2,/^$/p' "$0"
      exit 0
      ;;
    *)
      echo "error: unknown argument: $1" >&2
      echo "usage: $0 [--check]" >&2
      exit 2
      ;;
  esac
fi

if [ ! -f "$TEMPLATE" ]; then
  echo "error: template not found: $TEMPLATE" >&2
  exit 1
fi

work_dir=""
target_dir="$OUT_DIR"
if [ "$mode" = "check" ]; then
  work_dir="$(mktemp -d)"
  target_dir="$work_dir"
  trap 'rm -rf "$work_dir"' EXIT
fi

drift=0
for entry in "${PLATFORMS[@]}"; do
  platform="${entry%%:*}"
  suffix="${entry#*:}"
  out="$target_dir/hooks.json.$platform"

  # sed substitution; both placeholders are alphanumeric/hyphen/dot and don't
  # need regex escaping. Suffix is empty or ".exe".
  sed -e "s/{{PLATFORM}}/$platform/g" \
      -e "s/{{EXE_SUFFIX}}/$suffix/g" \
      "$TEMPLATE" > "$out"

  if command -v jq >/dev/null 2>&1; then
    if ! jq empty "$out" >/dev/null 2>&1; then
      echo "error: generated $out is not valid JSON" >&2
      exit 1
    fi
  fi

  if [ "$mode" = "check" ]; then
    committed="$OUT_DIR/hooks.json.$platform"
    if [ ! -f "$committed" ]; then
      echo "error: missing committed variant: $committed" >&2
      drift=1
    elif ! diff -u "$committed" "$out" >/dev/null 2>&1; then
      echo "drift detected for $platform:" >&2
      diff -u "$committed" "$out" >&2 || true
      drift=1
    fi
  else
    echo "wrote $out"
  fi
done

if [ "$mode" = "check" ] && [ "$drift" -ne 0 ]; then
  echo
  echo "error: committed hooks.json.<platform> variants do not match template." >&2
  echo "       Run scripts/generate-hooks-json.sh and commit the diff." >&2
  exit 1
fi

if [ "$mode" = "check" ]; then
  echo "no drift between hooks.json.template and the 5 committed variants."
fi
