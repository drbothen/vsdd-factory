#!/bin/bash
# factory-branch-guard.sh — PreToolUse hook for Edit|Write
#
# Ensures .factory/ writes only happen when .factory/ is a proper git
# worktree on the factory-artifacts branch. Prevents:
#
# 1. Writing to .factory/ when it's a regular directory (not a worktree)
#    — artifacts would end up on the wrong branch (develop/main)
# 2. Writing to .factory/ when the worktree is on the wrong branch
#    — artifacts committed to a non-factory branch
#
# The worktree lifecycle:
#   devops-engineer creates: git worktree add .factory factory-artifacts
#   state-manager writes: Write tool to .factory/STATE.md, etc.
#   state-manager commits: Bash tool for git add/commit in .factory/
#
# This hook guards step 2 — ensuring the worktree from step 1 exists
# and is on the correct branch before any Write/Edit to .factory/ paths.
#
# Exit 0 = allow, Exit 2 = block with diagnostic on stderr.
# Deterministic, <100ms, no LLM.

set -euo pipefail

if ! command -v jq &>/dev/null; then
  exit 0
fi

INPUT=$(cat)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')
TOOL_NAME=$(echo "$INPUT" | jq -r '.tool_name // "Edit|Write"')

_emit() {
  if [ -n "${CLAUDE_PLUGIN_ROOT:-}" ] && [ -x "${CLAUDE_PLUGIN_ROOT}/bin/emit-event" ]; then
    "${CLAUDE_PLUGIN_ROOT}/bin/emit-event" "$@" 2>/dev/null || true
  fi
  return 0
}

if [[ -z "$FILE_PATH" ]]; then
  exit 0
fi

# Only check paths containing .factory/
if [[ "$FILE_PATH" != *".factory/"* ]] && [[ "$FILE_PATH" != *".factory-project/"* ]]; then
  exit 0
fi

# Derive the .factory root from the file path
FACTORY_DIR=""
if [[ "$FILE_PATH" == *".factory/"* ]]; then
  # Extract everything up to and including .factory/
  FACTORY_DIR="${FILE_PATH%%/.factory/*}/.factory"
elif [[ "$FILE_PATH" == *".factory-project/"* ]]; then
  FACTORY_DIR="${FILE_PATH%%/.factory-project/*}/.factory-project"
fi

if [[ -z "$FACTORY_DIR" ]]; then
  exit 0
fi

# Determine expected branch
EXPECTED_BRANCH="factory-artifacts"
if [[ "$FACTORY_DIR" == *".factory-project" ]]; then
  EXPECTED_BRANCH="factory-project-artifacts"
fi

# Check 1: Is .factory/ a git worktree? (has .git marker file)
if [[ ! -e "$FACTORY_DIR/.git" ]]; then
  _emit type=hook.block hook=factory-branch-guard matcher="$TOOL_NAME" reason=factory_not_worktree file_path="$FILE_PATH" factory_dir="$FACTORY_DIR"
  echo "BLOCKED by factory-branch-guard:" >&2
  echo "  Cannot write to $FACTORY_DIR/ — not mounted as a git worktree." >&2
  echo "  .factory/ must be a worktree on the $EXPECTED_BRANCH branch, not a regular directory." >&2
  echo "  Recovery: git worktree add $FACTORY_DIR $EXPECTED_BRANCH" >&2
  exit 2
fi

# Check 2: Is the worktree on the correct branch?
if command -v git &>/dev/null; then
  CURRENT_BRANCH=$(git -C "$FACTORY_DIR" rev-parse --abbrev-ref HEAD 2>/dev/null || echo "unknown")
  if [[ "$CURRENT_BRANCH" != "$EXPECTED_BRANCH" ]] && [[ "$CURRENT_BRANCH" != "unknown" ]]; then
    _emit type=hook.block hook=factory-branch-guard matcher="$TOOL_NAME" reason=factory_wrong_branch file_path="$FILE_PATH" factory_dir="$FACTORY_DIR" current_branch="$CURRENT_BRANCH" expected_branch="$EXPECTED_BRANCH"
    echo "BLOCKED by factory-branch-guard:" >&2
    echo "  Cannot write to $FACTORY_DIR/ — worktree is on branch '$CURRENT_BRANCH', expected '$EXPECTED_BRANCH'." >&2
    echo "  Artifacts written on the wrong branch will be lost or misplaced." >&2
    echo "  Recovery: cd $FACTORY_DIR && git checkout $EXPECTED_BRANCH" >&2
    exit 2
  fi
fi

# All checks passed — worktree exists and is on correct branch
exit 0
