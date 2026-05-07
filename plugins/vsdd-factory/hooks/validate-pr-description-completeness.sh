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
  _ERRORS_SUMMARY=$(echo -e "$ERRORS" | tr '\n' '; ' | sed 's/; $//')
  block_pre "validate-pr-description-completeness" \
    "Required PR sections missing: $_ERRORS_SUMMARY" \
    "Populate all sections from templates/pr-description-template.md" \
    "pr_description_incomplete"
fi

exit 0
