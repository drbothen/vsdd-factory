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

# Source canonical block-message helper (provides block_pre).
if [ -n "${CLAUDE_PLUGIN_ROOT:-}" ] && [ -f "${CLAUDE_PLUGIN_ROOT}/hooks/lib/block.sh" ]; then
  # shellcheck source=lib/block.sh
  source "${CLAUDE_PLUGIN_ROOT}/hooks/lib/block.sh"
fi

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

# --- Case 1: Intentional placeholders → skip silently ---
if [[ "$STORED_HASH" == "[live-state]" ]] || [[ "$STORED_HASH" == "[pending-recompute]" ]]; then
  exit 0
fi

# --- Case 1a: Missing, template placeholder, or null → block ---
if [[ -z "$STORED_HASH" ]] || [[ "$STORED_HASH" == "[md5]" ]] || [[ "$STORED_HASH" == "null" ]]; then
  block_pre "validate-input-hash" \
    "$(basename "$FILE_PATH") has inputs: field but no computed input-hash" \
    "compute-input-hash $(basename "$FILE_PATH") --update" \
    "input_hash_missing"
fi

# --- Case 1b: Hash format validation — must be 7-char lowercase hex ---
HASH_LEN=${#STORED_HASH}
if [[ "$HASH_LEN" -ne 7 ]]; then
  block_pre "validate-input-hash" \
    "input-hash \"$STORED_HASH\" in $(basename "$FILE_PATH") is $HASH_LEN chars; canonical is 7-char truncated MD5" \
    "compute-input-hash $(basename "$FILE_PATH") --update" \
    "input_hash_format"
fi
if ! echo "$STORED_HASH" | grep -qE '^[0-9a-f]{7}$'; then
  block_pre "validate-input-hash" \
    "input-hash \"$STORED_HASH\" in $(basename "$FILE_PATH") contains invalid chars; must be lowercase hex [0-9a-f]" \
    "compute-input-hash $(basename "$FILE_PATH") --update" \
    "input_hash_format"
fi

# --- Case 2: Hash exists → verify if possible ---
if [[ -x "$HASH_TOOL" ]]; then
  COMPUTED=$("$HASH_TOOL" "$FILE_PATH" 2>/dev/null || true)
  if [[ -n "$COMPUTED" ]] && [[ "$COMPUTED" != "$STORED_HASH" ]]; then
    block_pre "validate-input-hash" \
      "input-hash drift in $(basename "$FILE_PATH"): stored $STORED_HASH != computed $COMPUTED. Inputs may have changed since this artifact was produced" \
      "compute-input-hash $(basename "$FILE_PATH") --update" \
      "input_hash_drift"
  fi
fi

exit 0
