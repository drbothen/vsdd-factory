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
  block_pre "factory-branch-guard" \
    "Cannot write to $FACTORY_DIR/ — not mounted as a git worktree. .factory/ must be a worktree on the $EXPECTED_BRANCH branch, not a regular directory" \
    "git worktree add $FACTORY_DIR $EXPECTED_BRANCH" \
    "factory_no_worktree"
fi

# Check 2: Is the worktree on the correct branch?
if command -v git &>/dev/null; then
  CURRENT_BRANCH=$(git -C "$FACTORY_DIR" rev-parse --abbrev-ref HEAD 2>/dev/null || echo "unknown")
  if [[ "$CURRENT_BRANCH" != "$EXPECTED_BRANCH" ]] && [[ "$CURRENT_BRANCH" != "unknown" ]]; then
    block_pre "factory-branch-guard" \
      "Cannot write to $FACTORY_DIR/ — worktree is on branch '$CURRENT_BRANCH', expected '$EXPECTED_BRANCH'. Artifacts written on the wrong branch will be lost or misplaced" \
      "cd $FACTORY_DIR && git checkout $EXPECTED_BRANCH" \
      "factory_wrong_branch"
  fi
fi

# All checks passed — worktree exists and is on correct branch
exit 0
