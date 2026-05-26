#!/bin/bash
# validate-state-size.sh — PostToolUse hook for STATE.md and lessons.md size enforcement
#
# Checks line count of STATE.md after every Write/Edit.
# STATE.md: WARN at >200 lines, BLOCK at >500 lines (unless the write reduced size).
# lessons.md: WARN at >3500 lines, BLOCK at >4000 lines (unless the write reduced size).
#
# Trigger: PostToolUse on Write/Edit to STATE.md or .factory/cycles/*/lessons.md.
# Exit 0 on pass (or if file is not a target, or if write reduced size).
# Exit 2 on bloat detected with diagnostic on stderr.
#
# Deterministic, <100ms, no LLM.

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

if [[ -z "$FILE_PATH" ]]; then
  exit 0
fi

# Determine target arm
case "$FILE_PATH" in
  */.factory/STATE.md|*.factory/STATE.md)
    TARGET="state"
    ;;
  */.factory/cycles/*/lessons.md|*.factory/cycles/*/lessons.md)
    TARGET="lessons"
    ;;
  *)
    exit 0
    ;;
esac

# ─── STATE.md arm ────────────────────────────────────────────────────────────
if [[ "$TARGET" == "state" ]]; then
  if [[ ! -f "$FILE_PATH" ]]; then
    exit 0
  fi

  LINE_COUNT=$(wc -l < "$FILE_PATH" | tr -d ' \t\n')

  # Check if this write reduced the file size (compaction).
  # Use git to compare against the committed version.
  PARENT_DIR=$(dirname "$FILE_PATH")
  PRIOR_COUNT=0
  if command -v git &>/dev/null; then
    _git_out=$(git -C "$PARENT_DIR" show HEAD:STATE.md 2>/dev/null | wc -l | tr -d ' ' 2>/dev/null) || _git_out=0
    # Ensure PRIOR_COUNT is a clean integer (strip trailing newlines/spaces)
    PRIOR_COUNT="${_git_out//[[:space:]]/}"
    PRIOR_COUNT="${PRIOR_COUNT:-0}"
    # If still not numeric, default to 0
    [[ "$PRIOR_COUNT" =~ ^[0-9]+$ ]] || PRIOR_COUNT=0
  fi

  # If the write REDUCED lines, always allow (compaction in progress)
  if [[ "$LINE_COUNT" -lt "$PRIOR_COUNT" ]]; then
    exit 0
  fi

  if [[ "$LINE_COUNT" -gt 500 ]]; then
    block_pre "validate-state-size" \
      "STATE.md exceeds 500-line limit ($LINE_COUNT lines). STATE.md should be a quick status check, not a history log" \
      "Run /vsdd-factory:compact-state to extract historical content to cycle files" \
      "state_md_bloat"
  elif [[ "$LINE_COUNT" -gt 200 ]]; then
    echo "STATE.md SIZE WARNING:" >&2
    echo "  STATE.md has $LINE_COUNT lines (recommended: <200, limit: 500)." >&2
    echo "  Consider running /vsdd-factory:compact-state to extract" >&2
    echo "  historical content to cycle files before it grows further." >&2
    exit 0
  fi
  exit 0
fi

# ─── lessons.md arm ──────────────────────────────────────────────────────────
# D-442(e) lessons.md size gate: advisory >3500, block >4000.
if [[ "$TARGET" == "lessons" ]]; then
  if [[ ! -f "$FILE_PATH" ]]; then
    exit 0
  fi

  LESSONS_LINE_COUNT=$(wc -l < "$FILE_PATH" | tr -d ' \t\n')

  # Compaction-in-progress: always allow if write REDUCED lines.
  # Use git to compare against the committed version.
  # Compute canonical paths to handle macOS /private symlink differences.
  LESSONS_PARENT_DIR=$(dirname "$FILE_PATH")
  LESSONS_PRIOR_COUNT=0
  if command -v git &>/dev/null; then
    _lessons_git_root=$(git -C "$LESSONS_PARENT_DIR" rev-parse --show-toplevel 2>/dev/null) || _lessons_git_root=""
    if [[ -n "$_lessons_git_root" ]]; then
      # Resolve both paths to canonical form (handles macOS /var -> /private/var symlinks)
      _file_canonical=$(cd "$(dirname "$FILE_PATH")" && pwd -P)/$(basename "$FILE_PATH")
      _root_canonical=$(cd "$_lessons_git_root" && pwd -P)
      _lessons_git_rel="${_file_canonical#"${_root_canonical}"/}"
      _lessons_git_out=$(git -C "$LESSONS_PARENT_DIR" show HEAD:"$_lessons_git_rel" 2>/dev/null | wc -l | tr -d ' ' 2>/dev/null) || _lessons_git_out=0
    else
      _lessons_git_out=0
    fi
    LESSONS_PRIOR_COUNT="${_lessons_git_out//[[:space:]]/}"
    LESSONS_PRIOR_COUNT="${LESSONS_PRIOR_COUNT:-0}"
    [[ "$LESSONS_PRIOR_COUNT" =~ ^[0-9]+$ ]] || LESSONS_PRIOR_COUNT=0
  fi

  if [[ "$LESSONS_LINE_COUNT" -lt "$LESSONS_PRIOR_COUNT" ]]; then
    exit 0
  fi

  if [[ "$LESSONS_LINE_COUNT" -gt 4000 ]]; then
    block_pre "validate-state-size" \
      "lessons.md exceeds 4000-line hard limit ($LESSONS_LINE_COUNT lines). WASM fuel exhaustion risk per D-442(e)" \
      "Run /vsdd-factory:compact-lessons to archive historical content to lessons-archive-*.md" \
      "lessons_md_fuel_risk"
  elif [[ "$LESSONS_LINE_COUNT" -gt 3500 ]]; then
    echo "LESSONS.MD SIZE WARNING:" >&2
    echo "  lessons.md has $LESSONS_LINE_COUNT lines (recommended: ≤3500, hard limit: 4000)." >&2
    echo "  Consider running /vsdd-factory:compact-lessons before it grows further (D-442(e))." >&2
    exit 0
  fi
  exit 0
fi
