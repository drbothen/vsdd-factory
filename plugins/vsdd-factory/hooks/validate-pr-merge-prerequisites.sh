#!/bin/bash
# validate-pr-merge-prerequisites.sh — PreToolUse hook on Agent dispatches
#
# Before allowing a github-ops merge dispatch (gh pr merge), checks that
# the evidence trail exists in .factory/code-delivery/STORY-NNN/:
#   - pr-description.md (PR was properly described)
#   - pr-review.md (review was conducted)
#   - security-review.md OR "no findings" in result (security was checked)
#
# Trigger: PreToolUse on Agent tool invocations targeting github-ops with merge.
# Exit 0 on pass (prerequisites met, or not a merge dispatch).
# Exit 2 blocks the merge with instructions.
#
# Deterministic, <500ms, no LLM.

set -euo pipefail

if ! command -v jq &>/dev/null; then
  exit 0
fi

INPUT=$(cat)
TOOL=$(echo "$INPUT" | jq -r '.tool_name // ""')

if [[ "$TOOL" != "Agent" ]]; then
  exit 0
fi

SUBAGENT=$(echo "$INPUT" | jq -r '.tool_input.subagent_type // ""')
PROMPT=$(echo "$INPUT" | jq -r '.tool_input.prompt // ""')

_emit() {
  if [ -n "${CLAUDE_PLUGIN_ROOT:-}" ] && [ -x "${CLAUDE_PLUGIN_ROOT}/bin/emit-event" ]; then
    "${CLAUDE_PLUGIN_ROOT}/bin/emit-event" "$@" 2>/dev/null || true
  fi
  return 0
}

# Scope: only github-ops dispatches that contain "merge"
case "$SUBAGENT" in
  *github-ops*) ;;
  *) exit 0 ;;
esac

if ! echo "$PROMPT" | grep -qiE "gh pr merge|pr merge|merge.*PR"; then
  exit 0  # not a merge dispatch
fi

# Extract story ID from prompt
STORY_ID=$(echo "$PROMPT" | grep -oE 'STORY-[0-9]+' | head -1 || true)
if [[ -z "$STORY_ID" ]]; then
  # Try S-N.NN format
  STORY_ID=$(echo "$PROMPT" | grep -oE 'S-[0-9]+\.[0-9]+' | head -1 || true)
fi
if [[ -z "$STORY_ID" ]]; then
  exit 0  # can't determine story; skip
fi

# Find the code-delivery directory
DELIVERY_DIR=""
# Try to extract project path from prompt
PROJECT_PATH=$(echo "$PROMPT" | grep -oE 'cd [^ ]+' | head -1 | sed 's/^cd //' || true)

if [[ -n "$PROJECT_PATH" ]] && [[ -d "$PROJECT_PATH/.factory/code-delivery/$STORY_ID" ]]; then
  DELIVERY_DIR="$PROJECT_PATH/.factory/code-delivery/$STORY_ID"
elif [[ -d ".factory/code-delivery/$STORY_ID" ]]; then
  DELIVERY_DIR=".factory/code-delivery/$STORY_ID"
fi

if [[ -z "$DELIVERY_DIR" ]]; then
  echo "" >&2
  echo "pr-merge-prerequisites: WARNING — cannot find .factory/code-delivery/$STORY_ID/" >&2
  echo "  Evidence trail directory missing. PR may not have been properly processed." >&2
  # Warn but don't block — directory might not exist yet in early pipeline
  exit 0
fi

ERRORS=""
MISSING_FILES=""  # comma-separated list of basenames for structured emission

# Check 1: pr-description.md exists
if [[ ! -f "$DELIVERY_DIR/pr-description.md" ]]; then
  ERRORS="${ERRORS:+$ERRORS\n}Missing: $DELIVERY_DIR/pr-description.md (PR description not populated)"
  MISSING_FILES="${MISSING_FILES:+$MISSING_FILES,}pr-description.md"
fi

# Check 2: pr-review.md exists
if [[ ! -f "$DELIVERY_DIR/pr-review.md" ]]; then
  ERRORS="${ERRORS:+$ERRORS\n}Missing: $DELIVERY_DIR/pr-review.md (PR review not conducted)"
  MISSING_FILES="${MISSING_FILES:+$MISSING_FILES,}pr-review.md"
fi

# Check 3: security-review.md exists (or security noted as clean elsewhere)
if [[ ! -f "$DELIVERY_DIR/security-review.md" ]]; then
  # Check if pr-description mentions security review was clean
  if [[ -f "$DELIVERY_DIR/pr-description.md" ]]; then
    if ! grep -qiE "security.*clean|security.*no finding|security.*pass|no security" "$DELIVERY_DIR/pr-description.md"; then
      ERRORS="${ERRORS:+$ERRORS\n}Missing: $DELIVERY_DIR/security-review.md (security review not conducted)"
      MISSING_FILES="${MISSING_FILES:+$MISSING_FILES,}security-review.md"
    fi
  else
    ERRORS="${ERRORS:+$ERRORS\n}Missing: $DELIVERY_DIR/security-review.md (security review not conducted)"
    MISSING_FILES="${MISSING_FILES:+$MISSING_FILES,}security-review.md"
  fi
fi

if [[ -n "$ERRORS" ]]; then
  _emit type=hook.block hook=validate-pr-merge-prerequisites matcher=Agent \
        reason=pr_merge_evidence_missing \
        story_id="$STORY_ID" delivery_dir="$DELIVERY_DIR" missing="$MISSING_FILES"
  echo "" >&2
  echo "PR MERGE PREREQUISITES NOT MET for $STORY_ID:" >&2
  echo -e "$ERRORS" | while IFS= read -r line; do
    echo "  - $line" >&2
  done
  echo "" >&2
  echo "  Complete the pr-manager 9-step lifecycle before merging." >&2
  echo "  Steps 1 (description), 4 (security), and 5 (review) must produce" >&2
  echo "  their evidence files before step 8 (merge) can proceed." >&2
  exit 2
fi

exit 0
