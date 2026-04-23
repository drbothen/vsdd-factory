#!/bin/bash
# verify-git-push.sh — PreToolUse hook for git push commands
#
# Guards against two classes of dangerous push:
# 1. Force push (--force / -f) — overwrites remote history
# 2. Push to protected branches (main, master, develop) — bypasses PR/review gates
#
# In VSDD, code flows through: feature branch worktree → PR → code review →
# pr-manager merge. Direct pushes to protected branches skip the entire
# quality gate chain (adversarial review, holdout evaluation, demo evidence).
#
# Allowed pushes:
#   git push origin feature/STORY-NNN    — normal per-story workflow
#   git push origin fix/FIX-NNN          — fix branch pushes
#   git push origin factory-artifacts    — state-manager pushing .factory/ artifacts
#   git push -u origin <any-branch>      — setting upstream on a new branch
#   git push origin <non-protected>      — any branch not in the protected list
#
# Exit 0 = allow, Exit 2 = block with diagnostic on stderr.
# Deterministic, <50ms, no LLM.

set -euo pipefail

if ! command -v jq &>/dev/null; then
  exit 0
fi

INPUT=$(cat)
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command // empty')

# Only process git push commands
if [[ "$COMMAND" != *"git push"* ]]; then
  exit 0
fi

_emit() {
  if [ -n "${CLAUDE_PLUGIN_ROOT:-}" ] && [ -x "${CLAUDE_PLUGIN_ROOT}/bin/emit-event" ]; then
    "${CLAUDE_PLUGIN_ROOT}/bin/emit-event" "$@" 2>/dev/null || true
  fi
  return 0
}

block() {
  local reason="$1"
  local suggestion="${2:-}"
  local code="${3:-unknown}"
  _emit type=hook.block hook=verify-git-push matcher=Bash reason="$code" command="$COMMAND"
  echo "BLOCKED by verify-git-push:" >&2
  echo "  $reason" >&2
  if [[ -n "$suggestion" ]]; then
    echo "  Suggestion: $suggestion" >&2
  fi
  exit 2
}

# --- Block force push ---
# Allow --force-with-lease (safe force push — only overwrites if remote matches local expectation)
# Block --force and -f (unconditional force push — overwrites regardless)
if [[ "$COMMAND" == *"--force-with-lease"* ]]; then
  : # Allowed — safe force push
elif [[ "$COMMAND" == *"--force"* ]] || [[ "$COMMAND" == *" -f "* ]] || [[ "$COMMAND" == *" -f"$'\n'* ]] || [[ "$COMMAND" =~ " -f"$ ]]; then
  block \
    "Force push (--force / -f) overwrites remote history irreversibly." \
    "Use 'git push --force-with-lease' for safe force push, or push to a new branch." \
    "git_push_force"
fi

# --- Block push to protected branches ---
# Protected branches: main, master, develop
# Extract the branch being pushed to (last argument after 'origin' or remote name)
PROTECTED_BRANCHES="main master develop"

for branch in $PROTECTED_BRANCHES; do
  # Match patterns: "git push origin main", "git push origin main:main",
  # "git push upstream main", etc.
  if [[ "$COMMAND" =~ git\ push\ [a-zA-Z_-]+\ ${branch}($|\ |:) ]]; then
    block \
      "Direct push to protected branch '$branch' bypasses PR and review gates." \
      "Push to a feature branch and create a PR: git push origin feature/STORY-NNN && gh pr create --base $branch" \
      "git_push_protected"
  fi
done

# Remind about verification for allowed pushes
echo '{"additionalContext": "Push requested. Ensure tests pass and code is reviewed before pushing."}'
exit 0
