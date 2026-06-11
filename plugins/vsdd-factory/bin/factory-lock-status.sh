#!/usr/bin/env bash
# factory-lock-status.sh — shared three-state factory lock display helper (S-17.03)
#
# Implements BC-6.23.001 PC7 + PC8: reads local STATE.md + current git email and
# prints one of four display strings to stdout.
#
# Usage:
#   factory-lock-status.sh <state_md_path> <current_git_email>
#
# Arguments:
#   <state_md_path>       — path to STATE.md to inspect (required)
#   <current_git_email>   — caller-supplied git config user.email (required)
#
# Output (stdout) — exactly one of the four canonical display strings:
#
#   "Factory lock: FREE"
#       factory_lock block is absent, OR expires_at is in the past (expired).
#
#   "Factory lock: HELD by this session (expires <expires_at>)"
#       factory_lock.holder == <current_git_email> AND now <= expires_at.
#
#   "Factory lock: HELD by <holder_email> since <locked_at> (expires <expires_at>)"
#       factory_lock.holder != <current_git_email> AND now <= expires_at.
#
#   "Factory lock: FREE (malformed block — treated as unlocked)"
#       factory_lock key is present but the block is missing required sub-fields
#       (holder, locked_at, or expires_at cannot be parsed).
#
# Three-state decision logic:
#   1. If factory_lock key is absent in frontmatter  → FREE
#   2. If factory_lock key is present but malformed  → FREE (malformed)
#   3. If expires_at is parseable and now > expires_at → FREE (expired)
#   4. If holder == current_git_email (unexpired)    → HELD by this session
#   5. If holder != current_git_email (unexpired)    → HELD by <holder> since <locked_at>
#
# No git operations are performed — caller is responsible for passing the current email.
# Invoked by both factory-health/SKILL.md and factory-worktree-health/SKILL.md so that
# the two skills cannot diverge on display format (BC-6.23.001 PC8 shared-helper mandate).
#
# Exit codes:
#   0 — always exits 0 (display-only; malformed block renders as FREE)

set -euo pipefail

# ---------------------------------------------------------------------------
# Input validation
# ---------------------------------------------------------------------------

usage() {
  printf 'usage: factory-lock-status.sh <state_md_path> <current_git_email>\n' >&2
  exit 1
}

if [[ $# -ne 2 ]]; then
  usage
fi

STATE_MD="$1"
CURRENT_EMAIL="$2"

if [[ -z "$STATE_MD" ]]; then
  printf 'factory-lock-status: <state_md_path> is required\n' >&2
  exit 1
fi

if [[ ! -f "$STATE_MD" ]]; then
  printf 'factory-lock-status: STATE.md path not found: %s\n' "$STATE_MD" >&2
  exit 1
fi

# ---------------------------------------------------------------------------
# TODO(S-17.03): factory-lock-status not implemented
# ---------------------------------------------------------------------------
printf 'TODO(S-17.03): factory-lock-status not implemented\n' >&2
exit 1
