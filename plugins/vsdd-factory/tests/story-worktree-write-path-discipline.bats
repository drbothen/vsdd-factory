#!/usr/bin/env bats
# story-worktree-write-path-discipline.bats — S-21.04 gate harness suite.
#
# Two load-bearing layers (to be activated in Step 3 — Red Gate tests):
#   DOC-PARITY:         grep/awk assertions on step-g-cleanup.md to verify the teardown
#                       preflight mandate text is present (find command, PREFLIGHT BLOCKED
#                       message, PC2a/PC2b logic). If the mandate text is absent or removed,
#                       these fail — the pre-implementation Red Gate must show failures here.
#   EXECUTABLE-HARNESS: inline bash helper _run_teardown_preflight() implementing the
#                       BC-6.26.001 PC2 assertion procedure, executed against a filesystem
#                       fixture built in $WORK by setup(). Observable outcome file ($REMOVE_LOG)
#                       allows literal assertions on NOT-removed / removed.
#
# Mechanism: test-double harness (W1 S-21.03 precedent).
#   Unlike S-21.03 (CLI stubs for gh/git), S-21.04 uses a filesystem fixture because the
#   preflight is `find`-based (filesystem-direct, no CLI tool to stub). The harness helper
#   is defined inline in this .bats file; the fixture shape is documented in
#   tests/fixtures/story-worktree/README.md.
#
# Gate target: plugins/vsdd-factory/skills/deliver-story/steps/step-g-cleanup.md §G.1
# BC:          BC-6.26.001 (PC2a, PC2b, Invariant 2, Invariant 5)
# Story:       S-21.04
#
# Test plan:
#   T-001  AC-003  stray-file-blocks:       stray .factory/ file → PREFLIGHT BLOCKED; git worktree remove NOT called
#   T-002  AC-004  empty-tree-proceeds:     no .factory/ content → teardown proceeds; git worktree remove IS called
#   T-003  AC-005  relocate-retry-proceeds: stray file relocated to canonical mount → retry teardown proceeds

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  STEP_G_CLEANUP="$PLUGIN_ROOT/skills/deliver-story/steps/step-g-cleanup.md"
  FIXTURE_DIR="$PLUGIN_ROOT/tests/fixtures/story-worktree"

  # Fixture worktree lifecycle: create temp workspace for this test run.
  # MOCK_WORKTREE simulates .worktrees/S-021/ (the story worktree path).
  # CANONICAL_FACTORY simulates the main-checkout .factory/ mount.
  # REMOVE_LOG records whether the mock git worktree remove was invoked.
  WORK="$(mktemp -d)"
  MOCK_WORKTREE="$WORK/story-worktree"
  CANONICAL_FACTORY="$WORK/canonical-factory"
  REMOVE_LOG="$WORK/worktree-remove.log"
  touch "$REMOVE_LOG"
  mkdir -p "$MOCK_WORKTREE" "$CANONICAL_FACTORY"
}

teardown() {
  [ -n "${WORK:-}" ] && rm -rf "$WORK"
}

# ===========================================================================
# T-001 / AC-003 / PC2b error: stray .factory/ file → PREFLIGHT BLOCKED; git worktree remove NOT called
# BC-6.26.001 PC2b, Invariant 2, Invariant 5
# RG-001: no preflight in step-g-cleanup.md → doc-parity fails pre-implementation
# ===========================================================================

@test "T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED; git worktree remove NOT called" {
  skip "stub — Red Gate tests land in Step 3"
}

# ===========================================================================
# T-002 / AC-004 / PC2a happy-path: empty shadow .factory/ → teardown proceeds; git worktree remove IS called
# BC-6.26.001 PC2a
# RG-002: preflight not present (trivially passes but test verifies preflight IS invoked)
# ===========================================================================

@test "T-002 S-21.04 AC-004: empty-tree-proceeds — teardown proceeds; git worktree remove IS called" {
  skip "stub — Red Gate tests land in Step 3"
}

# ===========================================================================
# T-003 / AC-005 / PC2b → PC2a retry: stray file relocated to canonical mount → retry teardown proceeds
# BC-6.26.001 PC2b → PC2a retry path
# RG-003: preflight not present
# ===========================================================================

@test "T-003 S-21.04 AC-005: relocate-retry-proceeds — stray file relocated; retry teardown proceeds" {
  skip "stub — Red Gate tests land in Step 3"
}
