#!/usr/bin/env bash
# factory-lock-write.sh — write/renew/clear the factory_lock frontmatter block in STATE.md
#
# Implements BC-5.40.001 PC1 (schema correctness), PC2 (unlock clears block),
# PC4 (mid-burst TTL renewal), and Invariants 2/3 (TTL = 2700s, expires_at = now + 2700s).
#
# Usage:
#   factory-lock-write.sh acquire <state_md_path>
#   factory-lock-write.sh renew  <state_md_path>
#   factory-lock-write.sh clear  <state_md_path>
#
# Modes:
#   acquire — Write a new factory_lock block with:
#               holder    = $(git config user.email | tr -d '\n')
#               locked_at = ISO-8601 UTC timestamp of acquisition (YYYY-MM-DDTHH:MM:SSZ)
#               expires_at = locked_at + TTL_SECONDS (= now + 2700s)
#             Overwrites any existing factory_lock block.
#
#   renew   — Refresh factory_lock.expires_at = now + 2700s while preserving
#             locked_at and holder unchanged. No-op (exit 0) if factory_lock key is absent.
#
#   clear   — Remove the factory_lock key ENTIRELY from STATE.md frontmatter.
#             Key deletion, NOT null assignment (BC-5.40.001 PC2 / Invariant "not null").
#             After clear, grep for 'factory_lock' in the frontmatter MUST return non-zero.
#
# Arguments:
#   <state_md_path>  — absolute or relative path to the STATE.md file to modify (required)
#
# TTL constant (non-configurable per BC-5.40.001 Invariant 2 and AC-007):
#   TTL_SECONDS=2700 (45 minutes). MUST NOT be overridden via environment or arguments.
#
# Outputs:
#   stdout — human-readable status message on success
#   stderr — error message on failure
#
# Exit codes:
#   0 — operation succeeded
#   1 — error (invalid args, missing file, git config failure, write failure)

set -euo pipefail

# ---------------------------------------------------------------------------
# TTL constant — non-configurable (BC-5.40.001 Invariant 2)
# ---------------------------------------------------------------------------
TTL_SECONDS=2700

# ---------------------------------------------------------------------------
# Input validation
# ---------------------------------------------------------------------------

usage() {
  printf 'usage: factory-lock-write.sh <acquire|renew|clear> <state_md_path>\n' >&2
  exit 1
}

if [[ $# -ne 2 ]]; then
  usage
fi

MODE="$1"
STATE_MD="$2"

if [[ "$MODE" != "acquire" && "$MODE" != "renew" && "$MODE" != "clear" ]]; then
  printf 'factory-lock-write: unknown mode %s — expected acquire, renew, or clear\n' "$MODE" >&2
  exit 1
fi

if [[ -z "$STATE_MD" ]]; then
  printf 'factory-lock-write: <state_md_path> is required\n' >&2
  exit 1
fi

if [[ ! -f "$STATE_MD" ]]; then
  printf 'factory-lock-write: STATE.md path not found: %s\n' "$STATE_MD" >&2
  exit 1
fi

# ---------------------------------------------------------------------------
# Mode dispatch
# ---------------------------------------------------------------------------

case "$MODE" in

  acquire)
    # Intended (NOT YET IMPLEMENTED):
    #   1. holder=$(git config user.email | tr -d '\n')
    #   2. locked_at=$(date -u +%Y-%m-%dT%H:%M:%SZ)
    #   3. expires_at = locked_at + TTL_SECONDS (ISO-8601 UTC arithmetic)
    #   4. Write/replace the factory_lock YAML block in STATE.md frontmatter:
    #        factory_lock:
    #          holder: "<holder>"
    #          locked_at: "<locked_at>"
    #          expires_at: "<expires_at>"
    #   All three fields MUST be present (BC-5.40.001 PC1 / SchemaViolation on missing field).
    #   locked_at and expires_at MUST use format YYYY-MM-DDTHH:MM:SSZ.
    #   expires_at MUST equal locked_at + exactly TTL_SECONDS (= ${TTL_SECONDS}).
    printf 'TODO(S-17.01): factory-lock-write acquire not implemented\n' >&2
    exit 1
    ;;

  renew)
    # Intended (NOT YET IMPLEMENTED):
    #   1. If factory_lock key is absent from STATE.md frontmatter → exit 0 (no-op).
    #   2. new_expires_at=$(date -u +%Y-%m-%dT%H:%M:%SZ) + TTL_SECONDS
    #   3. Update factory_lock.expires_at = new_expires_at in STATE.md frontmatter.
    #   4. factory_lock.locked_at and factory_lock.holder MUST NOT change (BC-5.40.001 PC4).
    #   RenewalMissed error variant is detectable post-commit by comparing old vs new expires_at.
    printf 'TODO(S-17.01): factory-lock-write renew not implemented\n' >&2
    exit 1
    ;;

  clear)
    # Intended (NOT YET IMPLEMENTED):
    #   1. Remove the factory_lock key and its sub-fields entirely from STATE.md frontmatter.
    #   2. Key MUST be deleted — NOT set to null (factory_lock: null is a StaleNullBlock
    #      violation per BC-5.40.001 PC2).
    #   3. After clear, grep for 'factory_lock' in the frontmatter region MUST return non-zero
    #      (key entirely absent).
    printf 'TODO(S-17.01): factory-lock-write clear not implemented\n' >&2
    exit 1
    ;;

esac
