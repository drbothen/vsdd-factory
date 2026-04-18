#!/bin/bash
# destructive-command-guard.sh — PreToolUse hook for Bash commands
#
# Blocks destructive shell commands that could cause irreversible data loss
# to pipeline state (.factory/), source code (src/), or test suites (tests/).
#
# Protected paths:
#   .factory/           — pipeline state, specs, stories, convergence history
#   src/                — source code (outside worktree cleanup)
#   tests/              — test suites
#
# Protected operations:
#   rm -rf / rm -r      — recursive delete on protected paths
#   rm on INDEX/STATE   — delete of authoritative source-of-truth files
#   git reset --hard    — discards all uncommitted changes
#   git clean -f[d]     — removes untracked files (not dry-run)
#   git checkout -- .   — discards all working tree changes
#   git restore .       — discards all working tree changes
#
# Allowed operations:
#   rm of individual non-critical files (temp, logs, build artifacts)
#   rm -rf target/ / node_modules/ / dist/ / build/ (build directories)
#   rm -rf .worktrees/STORY-NNN/ (worktree cleanup is normal workflow)
#   git reset --soft (preserves changes in staging)
#   git clean -n (dry-run only)
#   git stash (preserves changes)
#
# Exit 0 = allow, Exit 2 = block with diagnostic on stderr.
# Deterministic, <50ms, no LLM.

set -euo pipefail

if ! command -v jq &>/dev/null; then
  # If jq is missing, allow — other hooks will catch issues
  exit 0
fi

INPUT=$(cat)
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command // empty')

if [[ -z "$COMMAND" ]]; then
  exit 0
fi

# --- Helper: emit block message and exit ---
block() {
  local reason="$1"
  local suggestion="${2:-}"
  echo "BLOCKED by destructive-command-guard:" >&2
  echo "  $reason" >&2
  if [[ -n "$suggestion" ]]; then
    echo "  Suggestion: $suggestion" >&2
  fi
  exit 2
}

# --- git reset --hard (discards uncommitted work) ---
if [[ "$COMMAND" == *"git reset --hard"* ]] || [[ "$COMMAND" == *"git reset -hard"* ]]; then
  block \
    "git reset --hard discards all uncommitted changes irreversibly." \
    "Use 'git stash' to preserve changes, or 'git reset --soft' to unstage without losing work."
fi

# --- git clean -f (removes untracked files) ---
# Allow git clean -n (dry-run) but block -f (force)
if [[ "$COMMAND" == *"git clean"* ]]; then
  # Allow dry-run
  if [[ "$COMMAND" == *"-n"* ]] || [[ "$COMMAND" == *"--dry-run"* ]]; then
    exit 0
  fi
  # Block force clean
  if [[ "$COMMAND" == *"-f"* ]] || [[ "$COMMAND" == *"--force"* ]]; then
    block \
      "git clean -f removes untracked files irreversibly. This can delete specs, stories, and pipeline state." \
      "Use 'git clean -n' for a dry-run first, then selectively remove specific files."
  fi
fi

# --- git checkout -- . or git restore . (discard all working tree changes) ---
if [[ "$COMMAND" == *"git checkout -- ."* ]] || [[ "$COMMAND" == *"git checkout -- '*'"* ]]; then
  block \
    "git checkout -- . discards all working tree changes irreversibly." \
    "Use 'git stash' to preserve changes, or target specific files: 'git checkout -- <file>'."
fi
if [[ "$COMMAND" == *"git restore ."* ]] || [[ "$COMMAND" == *"git restore --staged ."* ]]; then
  block \
    "git restore . discards all working tree changes irreversibly." \
    "Target specific files: 'git restore <file>'."
fi

# --- rm -rf / rm -r on protected paths ---
# Match: rm -rf, rm -r, rm -Rf, rm -fr (flag order varies)
if echo "$COMMAND" | grep -qE '\brm\b.*-[rRf]*[rR][rRf]*\b'; then
  # Check if targeting a protected path
  for protected in ".factory/" ".factory " "src/" "tests/"; do
    if [[ "$COMMAND" == *"$protected"* ]]; then
      # Allow .worktrees/ cleanup (normal workflow)
      if [[ "$COMMAND" == *".worktrees/"* ]]; then
        exit 0
      fi
      # Allow build directories
      if echo "$COMMAND" | grep -qE '\brm\b.*\b(target|node_modules|dist|build|\.next|__pycache__|\.pytest_cache)/'; then
        exit 0
      fi
      block \
        "rm -rf on protected path detected: $COMMAND" \
        "Deleting .factory/, src/, or tests/ causes irreversible data loss. Remove specific files instead."
    fi
  done
fi

# --- rm (non-recursive) on source-of-truth files ---
if echo "$COMMAND" | grep -qE '\brm\b'; then
  # Block deletion of INDEX files and STATE.md
  for sot_file in "STATE.md" "BC-INDEX.md" "VP-INDEX.md" "STORY-INDEX.md" "ARCH-INDEX.md" "HS-INDEX.md" "L2-INDEX.md" "prd.md"; do
    if [[ "$COMMAND" == *"$sot_file"* ]]; then
      block \
        "Cannot delete source-of-truth file: $sot_file" \
        "$sot_file is an authoritative artifact. If it needs to be replaced, update it in place — do not delete."
    fi
  done
fi

# --- git rm on protected paths ---
if [[ "$COMMAND" == *"git rm"* ]]; then
  for protected in ".factory/specs/" ".factory/stories/" ".factory/STATE.md"; do
    if [[ "$COMMAND" == *"$protected"* ]]; then
      block \
        "git rm on protected path: $COMMAND" \
        "Living spec and story files should not be removed from version control. Use lifecycle status fields (retired, deprecated) instead."
    fi
  done
fi

# All checks passed — allow the command
exit 0
