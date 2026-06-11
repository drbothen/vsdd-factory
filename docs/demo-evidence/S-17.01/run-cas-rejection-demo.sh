#!/usr/bin/env bash
# run-cas-rejection-demo.sh — demonstrate AC-005 CAS push rejection with a real race
#
# This script demonstrates the EXACT force-with-lease CAS collision contract.
# It manually executes the 3-step sequence from factory-cas-push.sh, injecting
# a racer push BETWEEN step 2 (rev-parse) and step 3 (push) to produce the
# real --force-with-lease rejection.
#
# The real script (factory-cas-push.sh) is what ships; this demo script shows
# the internal contract with an observable race window.

set -uo pipefail

WORKTREE=/Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-17.01
FIXTURE_SCRIPT="$WORKTREE/docs/demo-evidence/S-17.01/setup-cas-fixture.sh"

echo "=== Building real git bare+clone fixture ==="
source "$FIXTURE_SCRIPT"
echo ""

echo "=== State before demo ==="
echo "Session-B local log:"
git -C "$CAS_CLONE_B" log --oneline | head -3
echo ""
echo "Remote (bare) log:"
git -C "$CAS_BARE" log --oneline | head -3
echo ""

echo "=== Step 1: Session-B fetches origin/factory-artifacts ==="
git -C "$CAS_CLONE_B" fetch origin factory-artifacts
echo ""

echo "=== Step 2: Capture EXPECTED_SHA from remote tip ==="
EXPECTED_SHA=$(git -C "$CAS_CLONE_B" rev-parse origin/factory-artifacts)
echo "EXPECTED_SHA = $EXPECTED_SHA"
echo ""

echo "=== [RACE] Racer pushes to remote BEFORE session-B can push ==="
git -C "$CAS_RACER" push origin factory-artifacts
echo ""
echo "Remote now at: $(git -C "$CAS_BARE" log --oneline | head -1)"
echo ""

echo "=== Step 3: Session-B pushes with --force-with-lease=factory-artifacts:<EXPECTED_SHA> ==="
echo "Command: git push --force-with-lease=factory-artifacts:${EXPECTED_SHA} origin factory-artifacts"
echo ""
if ! git -C "$CAS_CLONE_B" push \
    "--force-with-lease=factory-artifacts:${EXPECTED_SHA}" \
    origin factory-artifacts 2>&1; then
  echo ""
  printf 'state-burst CAS push failed — concurrent write detected.\n'
  printf 'Fetch origin/factory-artifacts and retry.\n'
  echo ""
  echo "=== CASPushRejected — remote state preserved ==="
  echo "Remote (bare) still at racer's commit:"
  git -C "$CAS_BARE" log --oneline | head -3
  exit 1
fi

echo "ERROR: push should have been rejected but succeeded"
exit 2
