#!/bin/bash
# validate-pr-description-completeness.sh — PostToolUse hook for PR descriptions
#
# When pr-description.md is written under .factory/code-delivery/, validates
# that all required sections from the PR template are present and no
# placeholder tokens remain (e.g., {story_id}, {pass_count}).
#
# Trigger: PostToolUse on Write/Edit to .factory/code-delivery/*/pr-description.md.
# Exit 0 on pass (or if file is not a pr-description).
# Exit 2 on missing sections or unresolved placeholders.
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

# Only trigger for pr-description.md under code-delivery
case "$FILE_PATH" in
  *code-delivery/*/pr-description.md) ;;
  *) exit 0 ;;
esac

ERRORS=""

# Required sections from the PR template
REQUIRED_SECTIONS=(
  "Architecture Changes"
  "Story Dependencies"
  "Spec Traceability"
  "Test Evidence"
  "Demo Evidence"
  "Pre-Merge Checklist"
)

for section in "${REQUIRED_SECTIONS[@]}"; do
  if ! grep -qi "## $section\|## .*$section" "$FILE_PATH"; then
    ERRORS="${ERRORS:+$ERRORS\n}Missing required section: ## $section"
  fi
done

# Check for unresolved placeholder tokens from the template
PLACEHOLDERS=$(grep -oE '\{[a-z_]+\}' "$FILE_PATH" | sort -u | head -10 || true)
if [[ -n "$PLACEHOLDERS" ]]; then
  PLACEHOLDER_LIST=$(echo "$PLACEHOLDERS" | tr '\n' ', ' | sed 's/,$//')
  ERRORS="${ERRORS:+$ERRORS\n}Unresolved template placeholders: $PLACEHOLDER_LIST"
fi

if [[ -n "$ERRORS" ]]; then
  _emit type=hook.block hook=validate-pr-description-completeness matcher=PostToolUse \
        reason=pr_description_incomplete file_path="$FILE_PATH"
  echo "PR DESCRIPTION INCOMPLETE in $(basename "$(dirname "$FILE_PATH")")/pr-description.md:" >&2
  echo -e "$ERRORS" | while IFS= read -r line; do
    echo "  - $line" >&2
  done
  echo "  Populate all sections from templates/pr-description-template.md before creating the PR." >&2
  exit 2
fi

exit 0
