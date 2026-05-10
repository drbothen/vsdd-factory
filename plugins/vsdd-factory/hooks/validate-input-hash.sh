#!/bin/bash
# validate-input-hash.sh — PostToolUse hook for input-hash drift detection
#
# After every Write to .factory/**/*.md, checks the input-hash field:
# 1. If input-hash is missing/placeholder/null and inputs: exists → block (exit 2)
# 2. If hash format is invalid (not 7-char lowercase hex) → block (exit 2)
# 3. If hash tool is available and computed hash drifts from stored → block (exit 2)
#
# Blocking — enforces that input-hash is always present and current.
# Use the input-hash bypass markers ([live-state], [pending-recompute]) to
# exempt files that are intentionally unhashed during active pipeline runs.
#
# Exit 2 on violations. Exit 0 when no inputs: field or hash is current.
#
# Deterministic, <500ms, no LLM.

set -euo pipefail

# Source canonical block-message helper (provides block_pre).
_SELF_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
_BLOCK_SH="${CLAUDE_PLUGIN_ROOT:+${CLAUDE_PLUGIN_ROOT}/hooks/lib/block.sh}"
_BLOCK_SH="${_BLOCK_SH:-${_SELF_DIR}/lib/block.sh}"
# shellcheck source=lib/block.sh disable=SC1091
if [ -f "$_BLOCK_SH" ]; then source "$_BLOCK_SH"; fi

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
