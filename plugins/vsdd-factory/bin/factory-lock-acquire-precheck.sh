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
# Helper: extract a sub-field value from factory_lock block in frontmatter.
# Argument: field name (e.g. "holder", "locked_at", "expires_at")
# Prints the value without surrounding quotes, or empty string if not found.
# ---------------------------------------------------------------------------
_extract_lock_field() {
  local field="$1"
  awk -v field="$field" '
    /^---$/ { fence++; next }
    fence == 1 && /^factory_lock:/ { in_lock=1; next }
    fence == 1 && in_lock && /^  / {
      if ($0 ~ "^  " field ":") {
        val = $0
        sub("^  " field ": *", "", val)
        gsub(/^"/, "", val)
        gsub(/"$/, "", val)
        print val
        exit
      }
    }
    fence == 1 && in_lock && !/^  / { in_lock=0 }
    fence >= 2 { exit }
  ' "$STATE_MD" 2>/dev/null
}

# ---------------------------------------------------------------------------
# Helper: convert ISO-8601 UTC timestamp to Unix epoch seconds.
# Supports BSD date (macOS) and GNU date (Linux).
# ---------------------------------------------------------------------------
_iso_to_epoch() {
  local ts="$1"
  if date --version >/dev/null 2>&1; then
    # GNU date
    date -u -d "${ts}" +%s 2>/dev/null || true
  else
    # BSD date (macOS)
    date -u -j -f "%Y-%m-%dT%H:%M:%SZ" "${ts}" +%s 2>/dev/null || true
  fi
}

# ---------------------------------------------------------------------------
# Helper: compute human-readable time remaining from seconds.
# Format: "<N> min remaining" — ALWAYS minutes, no "hr" form.
# Mirrors the guard's format_time_remaining (BC-4.13.001 AC-003 parity).
# ---------------------------------------------------------------------------
_seconds_to_human() {
  local secs="$1"
  if [[ "$secs" -le 0 ]]; then
    printf '0 min remaining'
    return
  fi
  local mins=$(( secs / 60 ))
  printf '%d min remaining' "$mins"
}

# ---------------------------------------------------------------------------
# Step 1: git fetch origin factory-artifacts (EC-006 guard)
# Must complete before any lock state is read.
# ---------------------------------------------------------------------------
if ! git fetch origin factory-artifacts 2>/dev/null; then
  printf 'Fetch failed before lock check. Cannot acquire safely.\n' >&2
  exit 2
fi

# ---------------------------------------------------------------------------
# Step 2: git config user.email (EC-007 guard)
# ---------------------------------------------------------------------------
CURRENT_EMAIL=""
if ! CURRENT_EMAIL="$(git config user.email 2>/dev/null | tr -d '\n')" || [[ -z "$CURRENT_EMAIL" ]]; then
  printf 'git user.email not configured — cannot acquire factory lock.\n' >&2
  exit 2
fi

# ---------------------------------------------------------------------------
# Step 3: Read local STATE.md factory_lock block
# Check if factory_lock key is present in frontmatter
# ---------------------------------------------------------------------------
HAS_LOCK=false
if awk '/^---$/{f++} f==1 && /^factory_lock:/{found=1} f>=2{exit} END{exit !found}' "$STATE_MD" 2>/dev/null; then
  HAS_LOCK=true
fi

# ---------------------------------------------------------------------------
# Step 4-6: Decision tree
# ---------------------------------------------------------------------------

if [[ "$HAS_LOCK" == "false" ]]; then
  # Absent lock — proceed
  printf 'PROCEED_ACQUIRE\n'
  exit 0
fi

# Lock block is present — extract fields
HOLDER="$(_extract_lock_field "holder")"
LOCKED_AT="$(_extract_lock_field "locked_at")"
EXPIRES_AT="$(_extract_lock_field "expires_at")"

# Malformed block (missing required fields) — treat as absent, proceed
if [[ -z "$HOLDER" || -z "$LOCKED_AT" || -z "$EXPIRES_AT" ]]; then
  printf 'PROCEED_ACQUIRE\n'
  exit 0
fi

# Check expiry (BC-4.13.001 PC2: now >= expires_at is expired)
NOW_EPOCH="$(date -u +%s 2>/dev/null || true)"
EXPIRES_EPOCH="$(_iso_to_epoch "$EXPIRES_AT")"

if [[ -z "$EXPIRES_EPOCH" || -z "$NOW_EPOCH" ]]; then
  # Cannot parse — treat as absent, proceed
  printf 'PROCEED_ACQUIRE\n'
  exit 0
fi

if [[ "$NOW_EPOCH" -ge "$EXPIRES_EPOCH" ]]; then
  # Expired lock — treat as absent (EC-002)
  printf 'PROCEED_ACQUIRE\n'
  exit 0
fi

# Lock is unexpired. Compare holder to current email.
if [[ "$HOLDER" == "$CURRENT_EMAIL" ]]; then
  # Step 4: Self-held, unexpired — NOOP_SELF_HELD (EC-001)
  printf 'NOOP_SELF_HELD\n'
  printf 'Already held by this session.\n'
  exit 0
fi

# Step 5: Foreign unexpired lock — REFUSED_FOREIGN_LOCK (PC3)
# Compute time remaining — mirrors guard's format_time_remaining (AC-003 parity)
REMAINING_SECS=$(( EXPIRES_EPOCH - NOW_EPOCH ))
TIME_REMAINING="$(_seconds_to_human "$REMAINING_SECS")"

# Emit message to stderr in the SAME format as the guard's build_block_message (AC-003).
# Guard format (build_block_message in verify-factory-lock/src/lib.rs):
#   BLOCKED by verify-factory-lock: factory-artifacts branch is locked by <holder>.
#   locked_at: <locked_at>
#   expires_at: <expires_at> (<N> min remaining)
#   To break the lock: /factory-unlock --force
{
  printf 'REFUSED_FOREIGN_LOCK\n'
  printf 'BLOCKED by verify-factory-lock: factory-artifacts branch is locked by %s.\n' "$HOLDER"
  printf 'locked_at: %s\n' "$LOCKED_AT"
  printf 'expires_at: %s (%s)\n' "$EXPIRES_AT" "$TIME_REMAINING"
  printf 'To break the lock: /factory-unlock --force\n'
} >&2

exit 1
