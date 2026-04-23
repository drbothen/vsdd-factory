#!/bin/bash
# destructive-command-guard.sh — PreToolUse hook for Bash commands
#
# Blocks destructive shell commands that could cause irreversible data loss
# to pipeline state (.factory/), source code (src/), test suites (tests/),
# git history, and shared GitHub state.
#
# Protected paths:
#   .factory/           — pipeline state, specs, stories, convergence history
#   src/                — source code (outside worktree cleanup)
#   tests/              — test suites
#
# Source-of-truth files (no rm, no truncate, no clobbering redirect):
#   STATE.md, BC-INDEX.md, VP-INDEX.md, STORY-INDEX.md, ARCH-INDEX.md,
#   HS-INDEX.md, L2-INDEX.md, prd.md
#
# Exit 0 = allow, Exit 2 = block with diagnostic on stderr.
# Deterministic, <50ms, no LLM.

set -euo pipefail

if ! command -v jq &>/dev/null; then
  exit 0
fi

INPUT=$(cat)
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command // empty')

if [[ -z "$COMMAND" ]]; then
  exit 0
fi

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

# Regex fragment matching rm with a recursive flag (-r, -R, -rf, -fr, -Rf,
# -fR, or long forms --recursive / -r combined with --force).
# Used as: echo "$CMD" | grep -qE "$RM_RECURSIVE"
RM_RECURSIVE='\brm\s+(-[a-zA-Z]*[rR][a-zA-Z]*|--recursive\b)'

# ---------------------------------------------------------------------------
# Catastrophic roots — rm -rf /, /*, ~, ~/, $HOME, *, .*
# ---------------------------------------------------------------------------
# Match target as a standalone argument (followed by end-of-command,
# whitespace, or shell separator). Patterns are intentionally single-quoted
# literals that will be consumed by grep as extended regex.
# shellcheck disable=SC2088,SC2016
for target_re in \
  '/' \
  '/\*' \
  '~' \
  '~/' \
  '\$HOME' \
  '\$HOME/' \
  '\*' \
  '\.\*' \
  ; do
  if echo "$COMMAND" | grep -qE "${RM_RECURSIVE}[^|&;]*\s${target_re}(\s|\$|;|&|\|)"; then
    block \
      "Catastrophic rm target detected: $COMMAND" \
      "Never 'rm -rf' system roots, home, or unscoped wildcards. Target a specific directory."
  fi
done

# ---------------------------------------------------------------------------
# rm -rf / rm -r on protected paths (.factory/, src/, tests/)
# ---------------------------------------------------------------------------
if echo "$COMMAND" | grep -qE "$RM_RECURSIVE"; then
  for protected_re in '\.factory/' '\.factory(\s|$|;|&|\|)' '\bsrc/' '\btests/'; do
    if echo "$COMMAND" | grep -qE "$protected_re"; then
      # Allow .worktrees/ cleanup (normal workflow)
      if [[ "$COMMAND" == *".worktrees/"* ]]; then
        continue
      fi
      # Allow build directories
      if echo "$COMMAND" | grep -qE '\brm\b[^|&;]*\b(target|node_modules|dist|build|\.next|__pycache__|\.pytest_cache)/'; then
        continue
      fi
      block \
        "rm -rf on protected path detected: $COMMAND" \
        "Deleting .factory/, src/, or tests/ causes irreversible data loss. Remove specific files instead."
    fi
  done
fi

# ---------------------------------------------------------------------------
# rm (any form) on source-of-truth files
# ---------------------------------------------------------------------------
SOT_FILES=(STATE.md BC-INDEX.md VP-INDEX.md STORY-INDEX.md ARCH-INDEX.md HS-INDEX.md L2-INDEX.md prd.md)

if echo "$COMMAND" | grep -qE '\brm\b'; then
  for sot_file in "${SOT_FILES[@]}"; do
    if [[ "$COMMAND" == *"$sot_file"* ]]; then
      block \
        "Cannot delete source-of-truth file: $sot_file" \
        "$sot_file is an authoritative artifact. Update it in place — do not delete."
    fi
  done
fi

# ---------------------------------------------------------------------------
# Output redirection / truncation that wipes source-of-truth files
# Covers:  > FILE   :> FILE   truncate -s 0 FILE   cp /dev/null FILE
# Allows: >> FILE (append), sed -i (in-place edit)
# ---------------------------------------------------------------------------
for sot_file in "${SOT_FILES[@]}"; do
  # Clobbering redirect: `> FILE` or `> path/FILE` (but NOT `>> FILE`)
  # The [^>] lookbehind prevents matching >>.
  if echo "$COMMAND" | grep -qE "(^|[^>])>\s*[^ ]*${sot_file}(\s|$|;|&|\|)"; then
    block \
      "Clobbering redirect to source-of-truth file: $sot_file" \
      "Use '>>' to append, or edit the file with sed/Edit instead of overwriting."
  fi
  # `: > FILE` truncation (idiom)
  if echo "$COMMAND" | grep -qE ":\s*>\s*[^ ]*${sot_file}(\s|$|;|&|\|)"; then
    block \
      "Truncating source-of-truth file via ': >': $sot_file" \
      "$sot_file must not be emptied. Edit in place instead."
  fi
  # truncate -s 0 FILE
  if echo "$COMMAND" | grep -qE "\btruncate\b[^|&;]*${sot_file}"; then
    block \
      "Truncating source-of-truth file: $sot_file" \
      "$sot_file must not be emptied. Edit in place instead."
  fi
  # cp /dev/null FILE
  if echo "$COMMAND" | grep -qE "\bcp\b[^|&;]*/dev/null[^|&;]*${sot_file}"; then
    block \
      "Wiping source-of-truth file via cp /dev/null: $sot_file" \
      "$sot_file must not be emptied. Edit in place instead."
  fi
done

# ---------------------------------------------------------------------------
# find ... -delete / find ... -exec rm  on protected paths
# ---------------------------------------------------------------------------
if echo "$COMMAND" | grep -qE '\bfind\b[^|&;]*(-delete|-exec\s+rm\b)'; then
  # \b doesn't work cleanly around a leading dot, so use a two-pattern match:
  # - .factory followed by word-boundary (end, slash, or space)
  # - src/tests as full words
  if echo "$COMMAND" | grep -qE '\.factory\b|\bsrc\b|\btests\b'; then
    block \
      "find with -delete or -exec rm on protected path: $COMMAND" \
      "find -delete bypasses rm safety checks. Remove specific files explicitly."
  fi
fi

# ---------------------------------------------------------------------------
# git reset --hard (discards uncommitted work)
# ---------------------------------------------------------------------------
if [[ "$COMMAND" == *"git reset --hard"* ]] || [[ "$COMMAND" == *"git reset -hard"* ]]; then
  block \
    "git reset --hard discards all uncommitted changes irreversibly." \
    "Use 'git stash' to preserve changes, or 'git reset --soft' to unstage without losing work."
fi

# ---------------------------------------------------------------------------
# git clean -f (removes untracked files); allow -n / --dry-run
# ---------------------------------------------------------------------------
if [[ "$COMMAND" == *"git clean"* ]]; then
  if [[ "$COMMAND" == *"-n"* ]] || [[ "$COMMAND" == *"--dry-run"* ]]; then
    : # dry-run is fine
  elif [[ "$COMMAND" == *"-f"* ]] || [[ "$COMMAND" == *"--force"* ]]; then
    block \
      "git clean -f removes untracked files irreversibly. This can delete specs, stories, and pipeline state." \
      "Use 'git clean -n' for a dry-run first, then selectively remove specific files."
  fi
fi

# ---------------------------------------------------------------------------
# git checkout -- . / git restore . (discard all working tree changes)
# ---------------------------------------------------------------------------
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

# ---------------------------------------------------------------------------
# git stash drop / git stash clear (discards stashed work)
# ---------------------------------------------------------------------------
if echo "$COMMAND" | grep -qE '\bgit\s+stash\s+(drop|clear)\b'; then
  block \
    "git stash drop/clear discards stashed work irreversibly." \
    "If the stash is genuinely unneeded, use 'git stash show -p' to verify first, then run outside this hook context."
fi

# ---------------------------------------------------------------------------
# git branch -D on protected branches (force-delete local main/master/develop)
# ---------------------------------------------------------------------------
if echo "$COMMAND" | grep -qE '\bgit\s+branch\s+-D\s+(main|master|develop)\b'; then
  block \
    "git branch -D on a protected local branch: $COMMAND" \
    "Protected branches should not be force-deleted locally. Check out another branch instead."
fi

# ---------------------------------------------------------------------------
# git filter-branch / git filter-repo (rewrites history across all refs)
# ---------------------------------------------------------------------------
if echo "$COMMAND" | grep -qE '\bgit\s+(filter-branch|filter-repo)\b'; then
  block \
    "git filter-branch/filter-repo rewrites history across all refs — effectively unrecoverable." \
    "If you truly need to rewrite history, do it in a fresh clone with explicit user confirmation."
fi

# ---------------------------------------------------------------------------
# git reflog expire + git gc --prune=now (removes the undo safety net)
# ---------------------------------------------------------------------------
if echo "$COMMAND" | grep -qE '\bgit\s+reflog\s+expire\b.*--expire=now'; then
  block \
    "git reflog expire --expire=now removes the recovery safety net for dangling commits." \
    "Let reflog expire naturally (default 90 days)."
fi
if echo "$COMMAND" | grep -qE '\bgit\s+gc\b.*--prune=now'; then
  block \
    "git gc --prune=now removes dangling objects immediately — you lose recovery options." \
    "Use 'git gc' (with default 2-week grace) or skip pruning."
fi

# ---------------------------------------------------------------------------
# git worktree remove --force outside .worktrees/
# ---------------------------------------------------------------------------
if echo "$COMMAND" | grep -qE '\bgit\s+worktree\s+remove\s+.*--force\b'; then
  if [[ "$COMMAND" != *".worktrees/"* ]]; then
    block \
      "git worktree remove --force outside .worktrees/: $COMMAND" \
      "--force discards uncommitted changes in the worktree. Commit or stash first, or target a .worktrees/ path."
  fi
fi

# ---------------------------------------------------------------------------
# --no-verify / --no-gpg-sign on git commit (bypasses hooks we built)
# ---------------------------------------------------------------------------
if echo "$COMMAND" | grep -qE '\bgit\s+(commit|merge|rebase|cherry-pick|am)\b[^|&;]*--no-verify\b'; then
  block \
    "--no-verify bypasses the hooks that enforce attribution, SoT protection, and wave gates." \
    "Fix the underlying issue that triggered the hook. If a hook is wrong, update the hook."
fi
if echo "$COMMAND" | grep -qE '\bgit\s+commit\b[^|&;]*--no-gpg-sign\b'; then
  block \
    "--no-gpg-sign bypasses commit signing." \
    "If a signing key is missing, fix the config rather than skipping the signature."
fi

# ---------------------------------------------------------------------------
# git rm on protected paths
# ---------------------------------------------------------------------------
if [[ "$COMMAND" == *"git rm"* ]]; then
  for protected in ".factory/specs/" ".factory/stories/" ".factory/STATE.md"; do
    if [[ "$COMMAND" == *"$protected"* ]]; then
      block \
        "git rm on protected path: $COMMAND" \
        "Living spec and story files should not be removed from version control. Use lifecycle status fields (retired, deprecated) instead."
    fi
  done
fi

# ---------------------------------------------------------------------------
# gh CLI destructive operations
# ---------------------------------------------------------------------------
if echo "$COMMAND" | grep -qE '\bgh\s+repo\s+delete\b'; then
  block \
    "gh repo delete is irreversible and affects shared GitHub state." \
    "If a repo truly needs deletion, the user should do it from the GitHub UI with confirmation."
fi
if echo "$COMMAND" | grep -qE '\bgh\s+release\s+delete\b'; then
  block \
    "gh release delete removes a published release (may have downstream consumers)." \
    "Releases should only be deleted by the user with explicit intent."
fi
if echo "$COMMAND" | grep -qE '\bgh\s+pr\s+close\b'; then
  block \
    "gh pr close discards PR work and breaks the review audit trail." \
    "If the PR should be abandoned, add a comment explaining why and let the user close it."
fi
if echo "$COMMAND" | grep -qE '\bgh\s+issue\s+delete\b'; then
  block \
    "gh issue delete removes issue history irreversibly." \
    "Close the issue instead ('gh issue close') to preserve the record."
fi

# ---------------------------------------------------------------------------
# curl|bash / wget|sh (remote code execution pattern)
# ---------------------------------------------------------------------------
if echo "$COMMAND" | grep -qE '\b(curl|wget|fetch)\b[^|]*\|\s*(bash|sh|zsh|python|perl|ruby)\b'; then
  block \
    "Piping remote content directly into a shell/interpreter is a supply-chain risk." \
    "Download to a file, inspect it, then run it explicitly."
fi

# ---------------------------------------------------------------------------
# chmod -R / chown -R on protected paths (subtle corruption risk)
# ---------------------------------------------------------------------------
if echo "$COMMAND" | grep -qE '\b(chmod|chown)\s+(-R|--recursive)\b'; then
  for protected in ".factory" "src/" "tests/" ".git/"; do
    if [[ "$COMMAND" == *"$protected"* ]]; then
      block \
        "Recursive chmod/chown on protected path: $COMMAND" \
        "Recursive permission changes can break git metadata and hook executability. Target specific files."
    fi
  done
fi

# All checks passed — allow the command
exit 0
