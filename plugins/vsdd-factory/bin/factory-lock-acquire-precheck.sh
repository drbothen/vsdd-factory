#!/usr/bin/env bash
# factory-lock-acquire-precheck.sh — acquire decision helper for /factory-lock (S-17.03)
#
# Implements BC-6.23.001 Preconditions 2+3 and the acquire-path decision tree:
#   EC-006: git fetch failure → hard abort (exit 2)
#   EC-007: git user.email empty → hard abort (exit 2)
#   EC-001: self-held unexpired lock → NOOP_SELF_HELD (exit 0)
#   PC3/ForeignLockHeld: foreign unexpired lock → REFUSED_FOREIGN_LOCK (exit 1)
#   PC1/EC-002: absent or expired lock → PROCEED_ACQUIRE (exit 0)
#
# Usage:
#   factory-lock-acquire-precheck.sh <state_md_path>
#
# Arguments:
#   <state_md_path>  — path to STATE.md (required; read AFTER git fetch)
#
# Decision tokens (stdout on exit 0):
#
#   "PROCEED_ACQUIRE"
#       Lock is absent or expired. Skill should proceed with acquire CAS push.
#
#   "NOOP_SELF_HELD"
#       Lock is held by this session (same email, unexpired). Skill exits 0 with
#       "Factory lock already held by this session." No write, no event.
#
# Decision tokens (stderr on exit 1):
#
#   "REFUSED_FOREIGN_LOCK"
#       Lock is held by a different developer and not expired. Skill exits 1.
#       The 5-field refusal message is also printed to stderr:
#         - Holder email
#         - locked_at timestamp
#         - expires_at timestamp
#         - time_remaining (human-readable)
#         - "/factory-unlock --force" command
#       Message format matches BC-4.13.001 PC1 format exactly (AC-003).
#
# Error exits (stderr, exit 2):
#
#   EC-006: "Fetch failed before lock check. Cannot acquire safely."
#       git fetch origin factory-artifacts returned non-zero.
#
#   EC-007: "git user.email not configured — cannot acquire factory lock."
#       git config user.email returned empty or failed.
#
# Step sequence (implementer target):
#   Step 1: git fetch origin factory-artifacts (EC-006 guard)
#   Step 2: git config user.email (EC-007 guard)
#   Step 3: read local STATE.md factory_lock block
#   Step 4: if self-held (holder == email, unexpired): NOOP_SELF_HELD (exit 0)
#   Step 5: if foreign unexpired (holder != email): REFUSED_FOREIGN_LOCK (exit 1) + 5-field msg
#   Step 6: otherwise (absent or expired): PROCEED_ACQUIRE (exit 0)
#
# Exit codes:
#   0 — PROCEED_ACQUIRE or NOOP_SELF_HELD (see stdout for decision token)
#   1 — REFUSED_FOREIGN_LOCK (see stderr for 5-field refusal message)
#   2 — EC-006 or EC-007 hard abort (see stderr for error message)

set -euo pipefail

# ---------------------------------------------------------------------------
# Input validation
# ---------------------------------------------------------------------------

usage() {
  printf 'usage: factory-lock-acquire-precheck.sh <state_md_path>\n' >&2
  exit 1
}

if [[ $# -ne 1 ]]; then
  usage
fi

STATE_MD="$1"

if [[ -z "$STATE_MD" ]]; then
  printf 'factory-lock-acquire-precheck: <state_md_path> is required\n' >&2
  exit 1
fi

if [[ ! -f "$STATE_MD" ]]; then
  printf 'factory-lock-acquire-precheck: STATE.md path not found: %s\n' "$STATE_MD" >&2
  exit 1
fi

# ---------------------------------------------------------------------------
# TODO(S-17.03): factory-lock-acquire-precheck not implemented
# ---------------------------------------------------------------------------
printf 'TODO(S-17.03): factory-lock-acquire-precheck not implemented\n' >&2
exit 1
