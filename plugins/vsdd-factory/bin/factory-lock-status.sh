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
#   3. If expires_at is parseable and now >= expires_at → FREE (expired)
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
# Parse factory_lock block from YAML frontmatter.
# Frontmatter is between the first and second --- fences.
# We extract the factory_lock section (key + 2-space-indented sub-fields).
# ---------------------------------------------------------------------------

# Check if factory_lock key is present in frontmatter
_has_factory_lock() {
  awk '/^---$/{f++} f==1 && /^factory_lock:/{found=1} f>=2{exit} END{exit !found}' "$STATE_MD" 2>/dev/null
}

# Extract a sub-field value from the factory_lock block in frontmatter.
# Argument: field name (e.g. "holder", "locked_at", "expires_at")
# Prints the value without surrounding quotes, or empty string if not found.
_extract_lock_field() {
  local field="$1"
  awk -v field="$field" '
    /^---$/ { fence++; next }
    fence == 1 && /^factory_lock:/ { in_lock=1; next }
    fence == 1 && in_lock && /^  / {
      # Match "  <field>: <value>" — strip indentation, field name, colon, quotes
      if ($0 ~ "^  " field ":") {
        val = $0
        # Remove leading whitespace and field name + colon
        sub("^  " field ": *", "", val)
        # Strip surrounding double-quotes if present
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
# Convert ISO-8601 UTC timestamp (YYYY-MM-DDTHH:MM:SSZ) to Unix epoch seconds.
# Supports BSD date (macOS) and GNU date (Linux).
# Prints the epoch integer, or empty string on failure.
# ---------------------------------------------------------------------------
_iso_to_epoch() {
  local ts="$1"
  # Strip the trailing Z and replace T separator for date parsing
  # Format: YYYY-MM-DDTHH:MM:SSZ
  if date --version >/dev/null 2>&1; then
    # GNU date
    date -u -d "${ts}" +%s 2>/dev/null || true
  else
    # BSD date (macOS): requires format string -j -f
    # Input format: 2099-01-01T00:00:00Z
    # BSD date -j -f "%Y-%m-%dT%H:%M:%SZ" "$ts" +%s
    date -u -j -f "%Y-%m-%dT%H:%M:%SZ" "${ts}" +%s 2>/dev/null || true
  fi
}

# ---------------------------------------------------------------------------
# Main decision logic
# ---------------------------------------------------------------------------

# Step 1: Check if factory_lock key is present in frontmatter
if ! _has_factory_lock; then
  printf 'Factory lock: FREE\n'
  exit 0
fi

# Step 2: factory_lock key is present — extract sub-fields
HOLDER="$(_extract_lock_field "holder")"
LOCKED_AT="$(_extract_lock_field "locked_at")"
EXPIRES_AT="$(_extract_lock_field "expires_at")"

# Step 3: Validate required sub-fields — if any are empty, treat as malformed
if [[ -z "$HOLDER" || -z "$LOCKED_AT" || -z "$EXPIRES_AT" ]]; then
  printf 'Factory lock: FREE (malformed block — treated as unlocked)\n'
  exit 0
fi

# Step 4: Check expiry. Convert expires_at to epoch and compare with now.
NOW_EPOCH="$(date -u +%s 2>/dev/null || true)"
EXPIRES_EPOCH="$(_iso_to_epoch "$EXPIRES_AT")"

if [[ -z "$EXPIRES_EPOCH" || -z "$NOW_EPOCH" ]]; then
  # Cannot parse expires_at — treat as malformed
  printf 'Factory lock: FREE (malformed block — treated as unlocked)\n'
  exit 0
fi

# BC-4.13.001 PC2 boundary semantics: now >= expires_at is expired (treat as FREE)
if [[ "$NOW_EPOCH" -ge "$EXPIRES_EPOCH" ]]; then
  printf 'Factory lock: FREE\n'
  exit 0
fi

# Step 5: Lock is unexpired. Compare holder to current email.
if [[ "$HOLDER" == "$CURRENT_EMAIL" ]]; then
  printf 'Factory lock: HELD by this session (expires %s)\n' "$EXPIRES_AT"
else
  printf 'Factory lock: HELD by %s since %s (expires %s)\n' "$HOLDER" "$LOCKED_AT" "$EXPIRES_AT"
fi

exit 0
