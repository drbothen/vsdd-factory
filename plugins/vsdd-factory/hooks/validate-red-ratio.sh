#!/bin/bash
# validate-red-ratio.sh — PostToolUse hook for Red Gate density check enforcement
#
# Validates that red-gate-log files or story files with red_ratio: frontmatter
# meet the RED_RATIO >= 0.5 threshold before Step 4 implementer dispatch.
#
# Trigger: PostToolUse on Write/Edit to:
#   .factory/logs/red-gate-log-*.md
#   .factory/stories/S-*.md (only when red_ratio: field is present in file)
#
# Exit 0 on pass (RED_RATIO >= 0.5), full_exception_path: true, or remediation: option_b.
# Exit 2 on RED_RATIO < 0.5 with no exception path.
#
# Deterministic, <100ms, no LLM. Read-only — does NOT write to any .factory/ files.
#
# AC-007 / BC-8.29.001, BC-8.29.002, BC-8.29.003

set -euo pipefail

if ! command -v jq &>/dev/null; then
  exit 0
fi

INPUT=$(cat)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

_emit() {
  if [ -n "${CLAUDE_PLUGIN_ROOT:-}" ] && [ -x "${CLAUDE_PLUGIN_ROOT}/bin/emit-event" ]; then
    "${CLAUDE_PLUGIN_ROOT}/bin/emit-event" "$@" 2>/dev/null || true
  fi
  return 0
}

if [[ -z "$FILE_PATH" ]] || [[ ! -f "$FILE_PATH" ]]; then
  exit 0
fi

# Only trigger for red-gate-log files or story files
# Match: .factory/logs/red-gate-log-*.md (canonical path) or bare red-gate-log-*.md (test/temp paths)
# Match: .factory/stories/S-*.md (only when red_ratio: field is present)
case "$FILE_PATH" in
  */.factory/logs/red-gate-log-*.md) ;;
  */red-gate-log-*.md) ;;
  */.factory/stories/S-*.md)
    # For story files, only proceed if red_ratio: field is present
    if ! grep -q '^red_ratio:' "$FILE_PATH" 2>/dev/null; then
      exit 0
    fi
    ;;
  *) exit 0 ;;
esac

# Extract story ID from file path
STORY_ID=$(basename "$FILE_PATH" .md)

# Extract fields from the file using grep on key: value lines
# Support both YAML frontmatter format and table format (| field: value |)
_extract_field() {
  local field="$1"
  local file="$2"
  # Try YAML-style: "field: value" (possibly in a table cell "| field: value |")
  grep -m1 "^\s*${field}:\||\s*${field}:" "$file" 2>/dev/null \
    | sed 's/.*'"${field}"':[[:space:]]*//' \
    | sed 's/[[:space:]]*|.*//' \
    | tr -d '[:space:]' \
    || true
}

red_count=$(_extract_field 'red_count' "$FILE_PATH")
total_new_tests=$(_extract_field 'total_new_tests' "$FILE_PATH")
exempt_count=$(_extract_field 'exempt_count' "$FILE_PATH")
remediation=$(_extract_field 'remediation' "$FILE_PATH")
full_exception_path=$(_extract_field 'full_exception_path' "$FILE_PATH")

# Option B election: mutation testing is the compensating control — pass
if [[ "$remediation" == "option_b" ]]; then
  exit 0
fi

# Full exception path explicitly acknowledged — pass
if [[ "$full_exception_path" == "true" ]]; then
  exit 0
fi

# Validate we have the numeric fields we need
if [[ -z "$red_count" ]] || [[ -z "$total_new_tests" ]] || [[ -z "$exempt_count" ]]; then
  # Missing fields means this file doesn't have the expected structure — skip
  exit 0
fi

# Ensure fields are integers
if ! [[ "$red_count" =~ ^[0-9]+$ ]] || ! [[ "$total_new_tests" =~ ^[0-9]+$ ]] || ! [[ "$exempt_count" =~ ^[0-9]+$ ]]; then
  exit 0
fi

# Compute total_effective = total_new_tests - exempt_count
total_effective=$(( total_new_tests - exempt_count ))

# If total_effective <= 0 and no full_exception_path acknowledgment, block
if (( total_effective <= 0 )); then
  _emit type=hook.block hook=validate-red-ratio matcher=PostToolUse \
        reason=red_ratio_below_threshold file_path="$FILE_PATH"
  echo "RED_RATIO BLOCK: total_effective=0 with no full_exception_path acknowledgment story=${STORY_ID}" >&2
  exit 2
fi

# Integer-precise RED_RATIO check: red_count * 2 >= total_effective
# This is equivalent to red_count / total_effective >= 0.5 without float arithmetic.
# AC-007 spec-mandated form: red_count * 2 >= (total_new_tests - exempt_count)
if (( red_count * 2 >= total_effective )); then
  # RED_RATIO >= 0.5 — gate passes
  exit 0
else
  # RED_RATIO < 0.5 — block
  _emit type=hook.block hook=validate-red-ratio matcher=PostToolUse \
        reason=red_ratio_below_threshold file_path="$FILE_PATH"
  echo "RED_RATIO BLOCK: ratio=${red_count}/${total_effective} threshold=0.5 story=${STORY_ID} path=${FILE_PATH}" >&2
  exit 2
fi
