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
# Intended sequence (NOT YET IMPLEMENTED):
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
#     "state-burst CAS push failed — concurrent write detected.\nFetch origin/factory-artifacts and retry."
#   AC-010 / fetch failure:
#     "state-burst CAS push failed — fetch error before push. Retry after resolving network."
#
# Exit codes:
#   0 — CAS push succeeded
#   1 — fetch failure (AC-010) or push rejection (AC-005 / CASPushRejected)

set -euo pipefail

# ---------------------------------------------------------------------------
# Stub body — NOT YET IMPLEMENTED (S-17.01 implementer fills in below)
# ---------------------------------------------------------------------------

# Intended (NOT YET IMPLEMENTED):
#   Step 1 — fetch:
#     if ! git -C .factory fetch origin factory-artifacts; then
#       printf 'state-burst CAS push failed — fetch error before push. Retry after resolving network.\n' >&2
#       exit 1
#     fi
#
#   Step 2 — capture expected SHA:
#     EXPECTED_SHA="$(git -C .factory rev-parse origin/factory-artifacts)"
#
#   Step 3 — CAS push with explicit lease SHA:
#     if ! git -C .factory push \
#         --force-with-lease=factory-artifacts:"${EXPECTED_SHA}" \
#         origin factory-artifacts; then
#       printf 'state-burst CAS push failed — concurrent write detected.\nFetch origin/factory-artifacts and retry.\n' >&2
#       exit 1
#     fi

printf 'TODO(S-17.01): factory-cas-push not implemented\n' >&2
exit 1
