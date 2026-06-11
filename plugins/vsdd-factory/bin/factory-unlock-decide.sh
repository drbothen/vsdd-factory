#!/usr/bin/env bash
# factory-unlock-decide.sh — unlock decision helper for /factory-unlock (S-17.03)
#
# Implements BC-6.23.001 PC4/PC5/PC6 + EC-003/EC-005/EC-008/EC-010 decision tree.
# Pure-core: inspects the factory_lock block + caller identity + --force flag.
# No STATE.md writes — decision and field construction only.
#
# Usage:
#   factory-unlock-decide.sh <state_md_path> <current_git_email> [--force]
#
# Arguments:
#   <state_md_path>       — path to STATE.md to inspect (required)
#   <current_git_email>   — caller-supplied git config user.email (required)
#   --force               — optional; triggers force-steal path (BC-6.23.001 PC6)
#
# Decision tokens (stdout):
#
#   "NOOP_ABSENT"
#       factory_lock block is absent (EC-003/EC-005). Skill exits 0 silently.
#       No event emitted (no holder to name in stolen_from).
#
#   "PROCEED_RELEASE"
#       Plain unlock, holder == current email (PC4). Skill delegates to state-manager
#       to run factory-lock-write.sh clear + factory-cas-push.sh.
#       stdout also contains the 3 release event fields:
#         holder=<email>
#         locked_at=<original_locked_at>
#         released_at=<now_iso8601>
#       Skill emits: factory.lock.released
#
#   "PROCEED_RELEASE_SELF_FORCE"
#       --force run against a self-held lock (EC-010). Treated identically to
#       PROCEED_RELEASE — emits factory.lock.released NOT factory.lock.stolen
#       (stolen_by == stolen_from is not a meaningful audit event).
#       stdout also contains the 3 release event fields (same as PROCEED_RELEASE).
#
#   "PROCEED_FORCE_STEAL"
#       --force run against a foreign lock (PC6). Skill delegates write + push,
#       then emits factory.lock.stolen with the 4-field audit event block:
#         stolen_by=<current_git_email>
#         stolen_from=<factory_lock.holder>
#         holder_locked_at=<factory_lock.locked_at>
#         stolen_at=<now_iso8601>
#       EC-008: emit-event is failure-tolerant (always exit 0); SS-03 unavailability
#       does NOT abort the force-release. Emission failure logged as log_warn.
#
# Error exits (stderr):
#
#   "REFUSED_NOT_HOLDER" (exit 1)
#       Plain unlock, holder != current email (PC5). Stderr message:
#         "Cannot unlock — factory is held by <holder_email>. Use /factory-unlock --force to force-release."
#       STATE.md MUST NOT be modified. No event emitted.
#
# Exit codes:
#   0 — NOOP_ABSENT, PROCEED_RELEASE, PROCEED_RELEASE_SELF_FORCE, PROCEED_FORCE_STEAL
#   1 — REFUSED_NOT_HOLDER

set -euo pipefail

# ---------------------------------------------------------------------------
# Input validation
# ---------------------------------------------------------------------------

usage() {
  printf 'usage: factory-unlock-decide.sh <state_md_path> <current_git_email> [--force]\n' >&2
  exit 1
}

if [[ $# -lt 2 || $# -gt 3 ]]; then
  usage
fi

STATE_MD="$1"
CURRENT_EMAIL="$2"
FORCE_FLAG="${3:-}"

if [[ -z "$STATE_MD" ]]; then
  printf 'factory-unlock-decide: <state_md_path> is required\n' >&2
  exit 1
fi

if [[ ! -f "$STATE_MD" ]]; then
  printf 'factory-unlock-decide: STATE.md path not found: %s\n' "$STATE_MD" >&2
  exit 1
fi

if [[ -n "$FORCE_FLAG" && "$FORCE_FLAG" != "--force" ]]; then
  printf 'factory-unlock-decide: unknown flag: %s (expected --force or absent)\n' "$FORCE_FLAG" >&2
  exit 1
fi

# ---------------------------------------------------------------------------
# TODO(S-17.03): factory-unlock-decide not implemented
# ---------------------------------------------------------------------------
printf 'TODO(S-17.03): factory-unlock-decide not implemented\n' >&2
exit 1
