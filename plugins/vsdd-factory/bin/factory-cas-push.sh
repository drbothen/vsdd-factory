#!/usr/bin/env bash
# factory-cas-push.sh — fetch-then-force-with-lease CAS push for factory-artifacts branch
#
# Implements BC-5.40.001 PC5 (state-burst CAS push) and EC-003 (fetch failure aborts push).
# Replaces the blind `git push origin factory-artifacts` in state-burst/SKILL.md (D6).
#
# Usage:
#   factory-cas-push.sh
#
# No arguments. Operates on the .factory/ worktree relative to the caller's working
# directory (consistent with state-manager burst discipline). Requires git >= 2.6 for
# the --force-with-lease=<refname>:<sha> explicit form (ADR-025 Decision 8).
#
# Sequence:
#   Step 1: git -C .factory fetch origin factory-artifacts
#           On non-zero exit → print AC-010 fetch-failure error to stderr; exit non-zero.
#           The push MUST NOT proceed with a potentially stale EXPECTED_SHA.
#
#   Step 2: EXPECTED_SHA=$(git -C .factory rev-parse origin/factory-artifacts)
#           Capture the expected remote tip SHA immediately after fetch.
#
#   Step 3: git -C .factory push --force-with-lease=factory-artifacts:"${EXPECTED_SHA}" \
#               origin factory-artifacts
#           On non-zero exit (--force-with-lease check failed = concurrent write detected) →
#           print AC-005 CASPushRejected error to stderr; exit non-zero.
#           Remote state MUST NOT be silently clobbered.
#
#   On success: exit 0.
#
# Error messages (exact strings required by ACs):
#   AC-005 / CASPushRejected:
#     "state-burst CAS push failed — concurrent write detected."
#     "Fetch origin/factory-artifacts and retry."
#   AC-010 / fetch failure:
#     "state-burst CAS push failed — fetch error before push. Retry after resolving network."
#
# Exit codes:
#   0 — CAS push succeeded
#   1 — fetch failure (AC-010) or push rejection (AC-005 / CASPushRejected)

set -euo pipefail

# ---------------------------------------------------------------------------
# Step 1: Fetch to synchronize origin/factory-artifacts before capturing SHA.
# On failure, abort immediately — do NOT proceed with a stale EXPECTED_SHA.
# ---------------------------------------------------------------------------

if ! git -C .factory fetch origin factory-artifacts 2>&1; then
  printf 'state-burst CAS push failed — fetch error before push. Retry after resolving network.\n' >&2
  exit 1
fi

# ---------------------------------------------------------------------------
# Step 2: Capture expected SHA immediately after fetch.
# Guard: if rev-parse fails (ref pruned/absent after fetch), emit a
# CASPushRejected-class message and abort — do NOT push with an invalid SHA.
# ---------------------------------------------------------------------------

if ! EXPECTED_SHA="$(git -C .factory rev-parse origin/factory-artifacts 2>&1)"; then
  printf 'state-burst CAS push failed — stale SHA after fetch: origin/factory-artifacts ref could not be resolved. Re-fetch and retry.\n' >&2
  exit 1
fi

# ---------------------------------------------------------------------------
# Step 3: CAS push with explicit --force-with-lease=<refname>:<sha> form.
# If the remote has advanced past EXPECTED_SHA, this push will be rejected
# (non-zero exit), which is the desired CASPushRejected behavior.
# ---------------------------------------------------------------------------

if ! git -C .factory push \
    "--force-with-lease=factory-artifacts:${EXPECTED_SHA}" \
    origin factory-artifacts 2>&1; then
  printf 'state-burst CAS push failed — concurrent write detected.\n' >&2
  printf 'Fetch origin/factory-artifacts and retry.\n' >&2
  exit 1
fi

printf 'state-burst CAS push succeeded (SHA: %s)\n' "$EXPECTED_SHA"
