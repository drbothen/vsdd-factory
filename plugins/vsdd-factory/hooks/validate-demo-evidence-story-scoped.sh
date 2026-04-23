#!/bin/bash
# validate-demo-evidence-story-scoped.sh — PostToolUse hook for POL-010
#
# Enforces that all demo evidence files live under docs/demo-evidence/<STORY-ID>/
# and never directly at docs/demo-evidence/*.md. Prevents evidence-report.md
# collisions when multiple stories' evidence lands in the same target repo.
#
# Trigger: PostToolUse on Write/Edit to files under docs/demo-evidence/.
# Exit 0 on pass (file is in a story subdirectory, or not a demo-evidence file).
# Exit 2 on flat-level file (blocks commit).
#
# Deterministic, <100ms, no LLM.

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

# Only trigger for files under docs/demo-evidence/
case "$FILE_PATH" in
  */docs/demo-evidence/*) ;;
  *) exit 0 ;;
esac

# Extract the path after docs/demo-evidence/
RELATIVE="${FILE_PATH##*/docs/demo-evidence/}"

# Check if the file is directly at the top level (no / in relative path)
# Valid: S-0.02/evidence-report.md (has a /)
# Invalid: evidence-report.md (no /)
if [[ "$RELATIVE" != */* ]]; then
  _emit type=hook.block hook=validate-demo-evidence-story-scoped matcher=PostToolUse \
        reason=demo_evidence_not_story_scoped file_path="$FILE_PATH"
  echo "POL-010 VIOLATION: demo evidence must live under docs/demo-evidence/<STORY-ID>/ — got $FILE_PATH" >&2
  echo "  Move to docs/demo-evidence/<STORY-ID>/$(basename "$FILE_PATH") where <STORY-ID> matches the story's frontmatter story_id field." >&2
  echo "  See policies.yaml POL-010." >&2
  exit 2
fi

exit 0
