#!/bin/bash
# validate-pr-review-posted.sh — SubagentStop hook on pr-reviewer
#
# Verifies that pr-reviewer actually wrote pr-review.md AND posted a
# formal GitHub review (gh pr review, not gh pr comment). Catches the
# silent failure where findings are written locally but never posted.
#
# Trigger: SubagentStop on pr-reviewer subagents.
# Exit 0 on pass (review written + posted, or not pr-reviewer).
# Exit 2 blocks if review file not written or gh pr review not invoked.
#
# Deterministic, <100ms, no LLM.

set -euo pipefail

if ! command -v jq &>/dev/null; then
  exit 0
fi

INPUT=$(cat)
AGENT=$(echo "$INPUT" | jq -r '.agent_type // .subagent_name // "unknown"')
RESULT=$(echo "$INPUT" | jq -r '.last_assistant_message // .result // empty')

_emit() {
  if [ -n "${CLAUDE_PLUGIN_ROOT:-}" ] && [ -x "${CLAUDE_PLUGIN_ROOT}/bin/emit-event" ]; then
    "${CLAUDE_PLUGIN_ROOT}/bin/emit-event" "$@" 2>/dev/null || true
  fi
  return 0
}

# Scope: only pr-reviewer
case "$AGENT" in
  *pr-reviewer*|*pr_reviewer*|*pr-review-triage*) ;;
  *) exit 0 ;;
esac

ERRORS=""

# Check 1: pr-review.md was written
if ! echo "$RESULT" | grep -qE "pr-review\.md|wrote.*review|review.*written|Write.*pr-review"; then
  ERRORS="${ERRORS:+$ERRORS\n}pr-review.md may not have been written to .factory/code-delivery/"
fi

# Check 2: formal GitHub review was posted (gh pr review, not gh pr comment)
if echo "$RESULT" | grep -qE "gh pr comment"; then
  ERRORS="${ERRORS:+$ERRORS\n}Used 'gh pr comment' instead of 'gh pr review' — findings won't show as a formal review verdict"
fi

if ! echo "$RESULT" | grep -qE "gh pr review|pr review.*posted|review.*posted.*GitHub|APPROVE|REQUEST_CHANGES"; then
  ERRORS="${ERRORS:+$ERRORS\n}No evidence that a formal GitHub review was posted via 'gh pr review'"
fi

# Check 3: review had a verdict (--approve or --request-changes)
if echo "$RESULT" | grep -qE "gh pr review" && ! echo "$RESULT" | grep -qE "approve|request-changes|APPROVE|REQUEST_CHANGES"; then
  ERRORS="${ERRORS:+$ERRORS\n}Review posted but no verdict (--approve or --request-changes) detected"
fi

if [[ -n "$ERRORS" ]]; then
  _emit type=hook.block hook=validate-pr-review-posted matcher=SubagentStop \
        reason=pr_review_not_posted subagent="$AGENT"
  echo "PR REVIEW POSTING INCOMPLETE:" >&2
  echo -e "$ERRORS" | while IFS= read -r line; do
    echo "  - $line" >&2
  done
  echo "  pr-reviewer MUST: (1) write pr-review.md, (2) spawn github-ops with" >&2
  echo "  'gh pr review --approve' or 'gh pr review --request-changes --body-file'." >&2
  echo "  NEVER use 'gh pr comment' for review verdicts." >&2
  exit 2
fi

exit 0
