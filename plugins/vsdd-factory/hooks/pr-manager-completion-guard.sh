#!/bin/bash
# pr-manager-completion-guard.sh — SubagentStop hook
#
# Detects FM4: pr-manager trying to stop before emitting STEP_COMPLETE
# for all 9 steps of the lifecycle. Blocks the stop and injects a
# continuation reminder telling pr-manager which step to execute next.
#
# v0.50.0 adds "STEP_COMPLETE: step=N ..." telemetry emissions after
# each step. This hook uses the absence/count of those emissions as
# the FM4 detection signal.
#
# Trigger: SubagentStop on pr-manager subagents.
# Exit 0 on pass (all steps complete, or not pr-manager).
# Exit 2 blocks the stop and injects continuation via stderr.
#
# Deterministic, <100ms, no LLM.

set -euo pipefail

if ! command -v jq &>/dev/null; then
  exit 0  # graceful degradation if jq missing
fi

INPUT=$(cat)
AGENT=$(echo "$INPUT" | jq -r '.agent_type // .subagent_name // "unknown"')
RESULT=$(echo "$INPUT" | jq -r '.last_assistant_message // .result // empty')

# Scope: only pr-manager subagents
case "$AGENT" in
  *pr-manager*|*pr_manager*) ;;
  *) exit 0 ;;
esac

# Count STEP_COMPLETE emissions. The 9-step lifecycle may have legitimate
# skips (step 2 for chore PRs = N/A), so we accept 8 or more as complete.
STEP_COUNT=$(echo "$RESULT" | grep -c "STEP_COMPLETE:" || true)

if (( STEP_COUNT >= 8 )); then
  # Looks complete
  exit 0
fi

# Check if the agent reported BLOCKED — that's a legitimate early exit
if echo "$RESULT" | grep -qE "^(Status:|##?\s*)?\s*BLOCKED" ; then
  exit 0
fi

# Identify highest step number emitted so far
LAST_STEP=$(echo "$RESULT" \
  | grep -oE "STEP_COMPLETE: step=[0-9]+" \
  | grep -oE "[0-9]+$" \
  | sort -n | tail -1 \
  || true)
LAST_STEP="${LAST_STEP:-0}"
NEXT_STEP=$((LAST_STEP + 1))

# Determine next-step hint based on position
case $NEXT_STEP in
  1) HINT="populate PR description from template" ;;
  2) HINT="verify demo evidence (or emit status=na for chore PRs)" ;;
  3) HINT="create PR via github-ops" ;;
  4) HINT="spawn security-reviewer via Agent tool" ;;
  5) HINT="spawn pr-reviewer/pr-review-triage via Agent tool; handle findings; converge" ;;
  6) HINT="spawn github-ops: gh pr checks --watch" ;;
  7) HINT="verify all dependency PRs merged" ;;
  8) HINT="spawn github-ops: gh pr merge --squash --delete-branch (AUTHORIZE_MERGE=yes mode)" ;;
  9) HINT="confirm branch deletion; write review-findings.md; emit final STEP_COMPLETE" ;;
  *) HINT="continue the 9-step lifecycle" ;;
esac

# Block the stop and inject continuation via stderr
echo "" >&2
echo "pr-manager-completion-guard: FM4 guard fired." >&2
echo "" >&2
echo "You emitted STEP_COMPLETE for $STEP_COUNT step(s) and are attempting to stop." >&2
echo "The 9-step lifecycle is MANDATORY. Your dispatch is pre-authorized for the" >&2
echo "full cycle including merge (AUTHORIZE_MERGE=yes per dispatch convention)." >&2
echo "" >&2
echo "CONTINUE TO STEP $NEXT_STEP NOW: $HINT" >&2
echo "" >&2
echo "Emit STEP_COMPLETE: step=$NEXT_STEP name=<name> status=<ok|na|failed> note=<>" >&2
echo "after completing the step. Do NOT exit until step 9 emits STEP_COMPLETE." >&2
echo "" >&2
echo "If you genuinely cannot continue (dependency PR not merged, unexpected CI" >&2
echo "failure, review blocker after 10 cycles), report BLOCKED with the specific" >&2
echo "reason instead of silently exiting mid-flow." >&2

exit 2
