#!/usr/bin/env bash
# factory-cas-push.sh — fetch-then-force-with-lease CAS push for factory-artifacts branch
#
# Implements BC-5.40.001 PC5 (state-burst CAS push) and EC-003 (fetch failure aborts push).
# Replaces the blind `git push origin factory-artifacts` in state-burst/SKILL.md (D6).
#
# Usage:
#   factory-cas-push.sh
#
# No arguments. Operates on the .factory/ worktree. The worktree is located
# cwd-independently (see FACTORY_DIR resolution below): it works from the repo
# root, from inside the .factory worktree itself (the state-manager's natural
# cwd — issue #631), and from a sibling story worktree. Requires git >= 2.6 for
# the --force-with-lease=<refname>:<sha> explicit form (ADR-025 Decision 8).
#
# Sequence:
#   Step 1: git -C "$FACTORY_DIR" fetch origin factory-artifacts
#           On non-zero exit → print AC-010 fetch-failure error to stderr; exit non-zero.
#           The push MUST NOT proceed with a potentially stale EXPECTED_SHA.
#
#   Step 2: EXPECTED_SHA=$(git -C "$FACTORY_DIR" rev-parse origin/factory-artifacts)
#           Capture the expected remote tip SHA immediately after fetch.
#
#   Step 3: git -C "$FACTORY_DIR" push --force-with-lease=factory-artifacts:"${EXPECTED_SHA}" \
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
# Resolve the .factory worktree cwd-independently (issue #631).
#
# The former relative `git -C .factory` assumed the caller was standing in the
# repo root. When the caller's cwd IS the .factory worktree — the natural cwd
# for a state-manager-role agent whose whole job lives there — the relative
# `.factory` resolved to `.factory/.factory` and every git step died with
# "cannot change to '.factory': No such file or directory", which then got
# reported as a fetch error, sending the operator down a network rabbit-hole.
#
# The .factory worktree is always `<main-worktree>/.factory` (created via
# `git worktree add .factory factory-artifacts`). The main worktree is the
# first entry of `git worktree list --porcelain` regardless of which worktree
# we invoke from — this is the same idiom emit-event, factory-query,
# factory-sla, and factory-dashboard already use to locate the factory dir.
#
# Resolution order:
#   1. cwd/.factory exists → repo-root invocation. Use it directly (relative,
#      byte-identical to the historical behavior; no git call needed).
#   2. else derive <main-worktree>/.factory from `git worktree list`. Covers
#      the caller standing INSIDE .factory (where cwd/.factory would be
#      .factory/.factory — absent) and callers in sibling story worktrees.
#   3. else fail with an actionable message (not the raw `cd` error).
# ---------------------------------------------------------------------------

if [ -d ".factory" ]; then
  FACTORY_DIR=".factory"
else
  MAIN_WT="$(git worktree list --porcelain 2>/dev/null | awk '/^worktree /{print $2; exit}')" || MAIN_WT=""
  if [ -n "$MAIN_WT" ] && [ -d "$MAIN_WT/.factory" ]; then
    FACTORY_DIR="$MAIN_WT/.factory"
  else
    printf 'state-burst CAS push failed — could not locate the .factory worktree from %s. Run from the repo root or the .factory worktree.\n' "$(pwd)" >&2
    exit 1
  fi
fi

# ---------------------------------------------------------------------------
# Step 1: Fetch to synchronize origin/factory-artifacts before capturing SHA.
# On failure, abort immediately — do NOT proceed with a stale EXPECTED_SHA.
# ---------------------------------------------------------------------------

if ! git -C "$FACTORY_DIR" fetch origin factory-artifacts 2>&1; then
  printf 'state-burst CAS push failed — fetch error before push. Retry after resolving network.\n' >&2
  exit 1
fi

# ---------------------------------------------------------------------------
# Step 2: Capture expected SHA immediately after fetch.
# Guard: if rev-parse fails (ref pruned/absent after fetch), emit a
# CASPushRejected-class message and abort — do NOT push with an invalid SHA.
# ---------------------------------------------------------------------------

if ! EXPECTED_SHA="$(git -C "$FACTORY_DIR" rev-parse origin/factory-artifacts 2>&1)"; then
  printf 'state-burst CAS push failed — stale SHA after fetch: origin/factory-artifacts ref could not be resolved. Re-fetch and retry.\n' >&2
  exit 1
fi

# ---------------------------------------------------------------------------
# Object-existence check: rev-parse may succeed (ref resolves) while the
# object is absent from the local store (e.g., partial fetch, GC'd object).
# Pushing a ghost SHA is unsafe — abort with CASPushRejected (EC-008 / F-R1-003).
# ---------------------------------------------------------------------------
if ! git -C "$FACTORY_DIR" cat-file -e "${EXPECTED_SHA}^{commit}" 2>/dev/null; then
  printf 'state-burst CAS push failed — stale SHA after fetch: object %s is absent from local store. Re-fetch and retry.\n' "$EXPECTED_SHA" >&2
  exit 1
fi

# ---------------------------------------------------------------------------
# Step 3: CAS push with explicit --force-with-lease=<refname>:<sha> form.
# If the remote has advanced past EXPECTED_SHA, this push will be rejected
# (non-zero exit), which is the desired CASPushRejected behavior.
# ---------------------------------------------------------------------------

if ! git -C "$FACTORY_DIR" push \
    "--force-with-lease=factory-artifacts:${EXPECTED_SHA}" \
    origin factory-artifacts 2>&1; then
  printf 'state-burst CAS push failed — concurrent write detected.\n' >&2
  printf 'Fetch origin/factory-artifacts and retry.\n' >&2
  exit 1
fi

printf 'state-burst CAS push succeeded (SHA: %s)\n' "$EXPECTED_SHA"
