#!/bin/bash
# validate-input-hash.sh — PostToolUse hook for input-hash drift detection
#
# After every Write to .factory/**/*.md, checks the input-hash field:
# 1. If input-hash is missing/placeholder/null and inputs: exists → warn
# 2. If input-hash is present and input files exist → recompute and compare
#
# Non-blocking (advisory) — warns on stale hashes but doesn't block.
#
# Exit 0 always (advisory hook). Diagnostics on stderr.
#
# Deterministic, <500ms, no LLM.

set -euo pipefail

if ! command -v jq &>/dev/null; then
  exit 0
fi

INPUT=$(cat)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

if [[ -z "$FILE_PATH" ]] || [[ ! -f "$FILE_PATH" ]]; then
  exit 0
fi

# Only trigger for .factory/ markdown files
case "$FILE_PATH" in
  *.factory/*.md) ;;
  *) exit 0 ;;
esac

# Skip INDEX files and config files
case "$FILE_PATH" in
  *INDEX.md|*.yaml|*.json|*current-cycle) exit 0 ;;
  *) ;;
esac

# --- Check if file has inputs: field ---
HAS_INPUTS=$(awk '
  /^---$/{ fm++; next }
  fm==1 && /^inputs:/ { print "yes"; exit }
' "$FILE_PATH")

if [[ "$HAS_INPUTS" != "yes" ]]; then
  exit 0  # No inputs field — nothing to hash
fi

# --- Check input-hash value ---
STORED_HASH=$(awk '
  /^---$/{ fm++; next }
  fm==1 && /^input-hash:/ {
    sub(/^input-hash:[ \t]*/, "")
    gsub(/["'"'"']/, "")
    gsub(/^[ \t]+|[ \t]+$/, "")
    print
    exit
  }
' "$FILE_PATH")

# Resolve plugin root for bin helper
PLUGIN_ROOT="${CLAUDE_PLUGIN_ROOT:-}"
if [[ -z "$PLUGIN_ROOT" ]]; then
  PLUGIN_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
fi
HASH_TOOL="$PLUGIN_ROOT/bin/compute-input-hash"

# --- Case 1: No hash or placeholder → remind ---
if [[ -z "$STORED_HASH" ]] || [[ "$STORED_HASH" == "[md5]" ]] || [[ "$STORED_HASH" == "null" ]]; then
  echo "input-hash: artifact $(basename "$FILE_PATH") has inputs: but no computed input-hash." >&2
  echo "  Run: compute-input-hash $(basename "$FILE_PATH") --update" >&2
  exit 0
fi

# --- Case 2: Hash exists → verify if possible ---
if [[ -x "$HASH_TOOL" ]]; then
  COMPUTED=$("$HASH_TOOL" "$FILE_PATH" 2>/dev/null || true)
  if [[ -n "$COMPUTED" ]] && [[ "$COMPUTED" != "$STORED_HASH" ]]; then
    echo "input-hash: DRIFT — $(basename "$FILE_PATH") stored hash ($STORED_HASH) ≠ computed ($COMPUTED)." >&2
    echo "  Inputs may have changed since this artifact was produced." >&2
    echo "  Run: compute-input-hash $(basename "$FILE_PATH") --update" >&2
  fi
fi

exit 0
