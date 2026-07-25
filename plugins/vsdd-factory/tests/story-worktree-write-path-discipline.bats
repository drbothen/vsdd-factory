#!/usr/bin/env bats
# story-worktree-write-path-discipline.bats — S-21.04 gate harness suite.
#
# Two load-bearing layers:
#   DOC-PARITY:         grep/awk assertions against step-g-cleanup.md §G.1 and
#                       _shared-context.md §Spec-Path Discipline to verify the
#                       teardown preflight mandate and write-discipline clause are
#                       present. Pre-implementation: mandate text absent → fail.
#   EXECUTABLE-HARNESS: _run_teardown_preflight() implements BC-6.26.001 PC2
#                       preflight logic, GATED on extracting the 'find .factory'
#                       pattern from step-g-cleanup.md §G.1 (anti-tautology per
#                       TD-VSDD-059). Pre-implementation: gate fires → harness output
#                       lacks 'PREFLIGHT BLOCKED' / 'worktree-remove-invoked' →
#                       assertion-level failures. REMOVE_LOG sentinel makes the
#                       mutant-proving vector load-bearing (POLICY 15 v1.4.10).
#
# Mechanism: filesystem-fixture harness (S-21.04 variant of S-21.03 two-layer pattern).
#   Unlike S-21.03 (CLI stubs for gh/git), S-21.04 uses a tmpfs fixture because
#   the preflight is find-based (filesystem-direct, no CLI tool to stub).
#   Fixture shape documented in tests/fixtures/story-worktree/README.md.
#
# Gate targets:
#   plugins/vsdd-factory/skills/deliver-story/steps/step-g-cleanup.md §G.1
#   plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md §Spec-Path Discipline
# BC: BC-6.26.001 (PC1, PC2a, PC2b, Invariants 1-5)
# Story: S-21.04
#
# Test plan:
#   T-001  AC-003  stray-file-blocks:       stray .factory/ file → PREFLIGHT BLOCKED + git worktree remove NOT called
#   T-002  AC-004  empty-tree-proceeds:     no .factory/ content → teardown proceeds + git worktree remove IS called
#   T-003  AC-005  relocate-retry-proceeds: stray file relocated to canonical mount → retry teardown proceeds

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  STEP_G_CLEANUP="$PLUGIN_ROOT/skills/deliver-story/steps/step-g-cleanup.md"
  SHARED_CONTEXT_MD="$PLUGIN_ROOT/skills/deliver-story/steps/_shared-context.md"
  FIXTURE_DIR="$PLUGIN_ROOT/tests/fixtures/story-worktree"

  # Fixture worktree lifecycle: fresh tmpfs workspace per test run.
  # MOCK_WORKTREE    — simulates .worktrees/S-021/ (story worktree path).
  # CANONICAL_FACTORY — simulates the main-checkout .factory/ mount.
  # REMOVE_LOG       — sentinel: 'worktree-remove-invoked' appended when PC2a proceeds.
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
# DOC-PARITY helpers
# ===========================================================================

# Extracts the Sub-step G.1 block from step-g-cleanup.md.
# Start anchor: /^## Sub-step G\.1/ — exits on next /^## / heading.
_extract_g1_section() {
  awk '
    /^## Sub-step G\.1/ { found=1; next }
    found && /^## / { exit }
    found { print }
  ' "$STEP_G_CLEANUP"
}

# Extracts the Spec-Path Discipline section from _shared-context.md.
# Start anchor: /^### Spec-Path Discipline/ — exits on next /^## / heading.
_extract_spec_path_discipline_section() {
  awk '
    /^### Spec-Path Discipline/ { found=1; next }
    found && /^## / { exit }
    found { print }
  ' "$SHARED_CONTEXT_MD"
}

_assert_doc_marker() {
  # $1=regex  $2=label  $3=section_text
  printf '%s\n' "$3" | grep -qE "$1" || {
    echo "DOC-PARITY FAIL [must contain: $2]"
    false
  }
}

# ===========================================================================
# EXECUTABLE-HARNESS helper
# ===========================================================================

# Run the BC-6.26.001 PC2 teardown preflight against a fixture worktree path.
# Args:
#   $1  worktree_path  — simulated story-worktree path (tmpfs fixture)
#   $2  remove_log     — sentinel file; 'worktree-remove-invoked' appended on PC2a proceed
#
# Anti-tautology gate (TD-VSDD-059): first extracts the 'find .factory' pattern
# from step-g-cleanup.md §G.1. Pre-implementation: nothing to extract → gate fires
# and outputs "preflight mandate absent" message. Since that message does not contain
# 'PREFLIGHT BLOCKED' (T-001, T-003) and does not write to REMOVE_LOG (T-002, T-003),
# all three test assertions fail. A harness that implemented the preflight directly
# without this gate would pass T-002 tautologically — violating the Red Gate.
#
# Always returns 0; test assertions check output content and REMOVE_LOG state.
_run_teardown_preflight() {
  local worktree_path="$1" remove_log="$2"
  local g1_section
  g1_section="$(_extract_g1_section)"

  # Anti-tautology gate: the preflight mandate must be extractable from §G.1.
  # Pre-implementation: G.1 has no 'find .factory' command → gate fires.
  # Post-implementation: 'find .factory' present in G.1 → gate passes → proceed.
  if ! printf '%s\n' "$g1_section" | grep -qE 'find.*\.factory'; then
    printf 'HARNESS FAIL: preflight mandate absent from step-g-cleanup.md §G.1 — '\''find .factory'\'' pattern not extractable from G.1 block (BC-6.26.001 PC2 not yet implemented)\n'
    return 0
  fi

  # Execute the find preflight against the fixture worktree path.
  # '|| true' ensures non-zero find exit (absent .factory/) does not abort the harness.
  local stray_files
  stray_files="$(find "${worktree_path}/.factory" -type f 2>/dev/null)" || true

  if [ -n "$stray_files" ]; then
    # PC2b: stray files found — PREFLIGHT BLOCKED; git worktree remove NOT called.
    # REMOVE_LOG intentionally NOT written to (mutant-proving sentinel for T-001).
    printf 'PREFLIGHT BLOCKED: Found factory artifact(s) in story worktree shadow .factory/:\n'
    printf '%s\n' "$stray_files"
    printf 'These files were written to the wrong worktree (issue #523 class) and would be\n'
    printf 'permanently destroyed by git worktree remove. Manual intervention required.\n'
  else
    # PC2a: shadow tree empty — proceed with teardown; record invocation in sentinel.
    printf 'worktree-remove-invoked\n' >> "$remove_log"
  fi
  return 0
}

# ===========================================================================
# T-001 / AC-003 / PC2b error: stray .factory/ file → PREFLIGHT BLOCKED; git worktree remove NOT called
# BC-6.26.001 PC2b, Invariant 2, Invariant 5
# RG-001: no preflight in step-g-cleanup.md → doc-parity fails pre-implementation
# ===========================================================================

@test "T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED; git worktree remove NOT called" {
  # Fixture: stray .factory/stories/S-021-DELIVERY.md inside mock worktree.
  # Represents a DELIVERY ledger written via CWD-relative path from story worktree —
  # primary issue #523 failure mode (gitignored-shadow mechanism: BC-6.26.001 Invariant 5).
  #
  # Pre-implementation red gate:
  #   DOC-PARITY (§G.1): 'find .factory' absent from G.1 → first assertion fails with
  #     "DOC-PARITY FAIL [must contain: step-g-cleanup.md §G.1: find .factory teardown ...]"
  #   DOC-PARITY (§Spec-Path Discipline): 'Write Discipline', 'CANONICAL_FACTORY_ROOT',
  #     'DELIVERY' absent → assertions fail (AC-001 coverage; BC-6.26.001 PC1, Invariants 1, 4).
  #   HARNESS: extraction gate fires → 'PREFLIGHT BLOCKED' absent from output → assertion fails.
  #   SENTINEL: REMOVE_LOG empty → [ ! -s REMOVE_LOG ] trivially passes (no removal invoked).
  # Post-implementation: DOC-PARITY passes; PREFLIGHT BLOCKED emitted; REMOVE_LOG empty (RG-001 closed).

  # --- Fixture setup: stray factory artifact in shadow .factory/ ---
  mkdir -p "$MOCK_WORKTREE/.factory/stories"
  printf 'stray DELIVERY ledger — written via CWD-relative path from story worktree CWD\n' \
    > "$MOCK_WORKTREE/.factory/stories/S-021-DELIVERY.md"

  # --- DOC-PARITY: step-g-cleanup.md §G.1 teardown preflight mandate ---
  # (BC-6.26.001 PC2, Invariant 2, Invariant 5)
  local g1_section
  g1_section="$(_extract_g1_section)"

  _assert_doc_marker 'find.*\.factory' \
    "step-g-cleanup.md §G.1: find .factory teardown preflight command (BC-6.26.001 PC2, Invariant 5)" \
    "$g1_section"
  _assert_doc_marker 'PREFLIGHT BLOCKED' \
    "step-g-cleanup.md §G.1: PREFLIGHT BLOCKED halt message on stray files (BC-6.26.001 PC2b, Invariant 2)" \
    "$g1_section"
  _assert_doc_marker 'git worktree remove' \
    "step-g-cleanup.md §G.1: git worktree remove command present for PC2a proceed path (BC-6.26.001 PC2a)" \
    "$g1_section"
  # Negative assertion: plain 'git worktree remove' only — no --force flag (BC-6.26.001 PC2a)
  if printf '%s\n' "$g1_section" | grep -qE 'git worktree remove[[:space:]]+--force'; then
    echo "DOC-PARITY FAIL: step-g-cleanup.md §G.1 uses 'git worktree remove --force' — must be plain 'git worktree remove' without --force (BC-6.26.001 PC2a)"
    false
  fi

  # --- DOC-PARITY: _shared-context.md §Spec-Path Discipline Write Discipline clause (AC-001 coverage) ---
  # (BC-6.26.001 PC1, Invariant 1, Invariant 4)
  local spec_path_section
  spec_path_section="$(_extract_spec_path_discipline_section)"

  _assert_doc_marker 'Write Discipline' \
    "_shared-context.md §Spec-Path Discipline: explicit Write Discipline clause required (BC-6.26.001 PC1, Invariant 1)" \
    "$spec_path_section"
  _assert_doc_marker 'CANONICAL_FACTORY_ROOT' \
    "_shared-context.md §Spec-Path Discipline: CANONICAL_FACTORY_ROOT variable mandate in Write Discipline clause (BC-6.26.001 PC1)" \
    "$spec_path_section"
  _assert_doc_marker 'DELIVERY' \
    "_shared-context.md §Spec-Path Discipline: DELIVERY ledger named as load-bearing case (BC-6.26.001 Invariant 4)" \
    "$spec_path_section"

  # --- HARNESS: stray file → PREFLIGHT BLOCKED; REMOVE_LOG must remain empty ---
  # Mutant-proving vector (POLICY 15 v1.4.10): REMOVE_LOG sentinel is load-bearing.
  # If the preflight guard is absent or bypassed, the harness writes to REMOVE_LOG →
  # [ ! -s REMOVE_LOG ] fails, proving the guard is actually enforced.
  local preflight_out
  preflight_out="$(_run_teardown_preflight "$MOCK_WORKTREE" "$REMOVE_LOG")"

  printf '%s\n' "$preflight_out" | grep -q 'PREFLIGHT BLOCKED' || {
    echo "HARNESS FAIL: PREFLIGHT BLOCKED not in preflight output — got: $preflight_out"
    false
  }
  # Mutant-proving sentinel: git worktree remove must NOT be invoked on PREFLIGHT BLOCKED path
  [ ! -s "$REMOVE_LOG" ] || {
    echo "HARNESS FAIL: REMOVE_LOG non-empty — git worktree remove was invoked but MUST NOT be on PREFLIGHT BLOCKED path — log: $(cat "$REMOVE_LOG")"
    false
  }
}

# ===========================================================================
# T-002 / AC-004 / PC2a happy-path: empty shadow .factory/ → teardown proceeds; git worktree remove IS called
# BC-6.26.001 PC2a
# RG-002: preflight not present — test verifies preflight IS invoked (not just that fixture trivially passes)
# ===========================================================================

@test "T-002 S-21.04 AC-004: empty-tree-proceeds — teardown proceeds; git worktree remove IS called" {
  # Fixture: no .factory/ directory in mock worktree (EC-003 / EC-005 scenario).
  #
  # Pre-implementation red gate:
  #   DOC-PARITY (§G.1): 'find .factory' absent → first assertion fails with
  #     "DOC-PARITY FAIL [must contain: step-g-cleanup.md §G.1: find .factory preflight command ...]"
  #   HARNESS: extraction gate fires → 'worktree-remove-invoked' never written to REMOVE_LOG →
  #     grep on REMOVE_LOG fails with "HARNESS FAIL: worktree-remove-invoked not in REMOVE_LOG ..."
  # Post-implementation: DOC-PARITY passes; harness writes 'worktree-remove-invoked' to REMOVE_LOG.

  # No fixture setup: MOCK_WORKTREE exists but has no .factory/ directory (clean worktree state).

  # --- DOC-PARITY: step-g-cleanup.md §G.1 preflight mandate ---
  # Both PC2a and PC2b logic live in the same G.1 mandate block.
  local g1_section
  g1_section="$(_extract_g1_section)"

  _assert_doc_marker 'find.*\.factory' \
    "step-g-cleanup.md §G.1: find .factory preflight command (PC2a happy-path requires mandate present — BC-6.26.001 PC2a)" \
    "$g1_section"
  _assert_doc_marker 'PREFLIGHT BLOCKED' \
    "step-g-cleanup.md §G.1: PREFLIGHT BLOCKED mandate present (PC2a and PC2b defined in same G.1 block — BC-6.26.001 PC2)" \
    "$g1_section"
  _assert_doc_marker 'git worktree remove' \
    "step-g-cleanup.md §G.1: git worktree remove command for PC2a proceed path (BC-6.26.001 PC2a)" \
    "$g1_section"

  # --- HARNESS: empty shadow tree → proceed; REMOVE_LOG written ---
  local preflight_out
  preflight_out="$(_run_teardown_preflight "$MOCK_WORKTREE" "$REMOVE_LOG")"

  grep -q 'worktree-remove-invoked' "$REMOVE_LOG" || {
    echo "HARNESS FAIL: worktree-remove-invoked not in REMOVE_LOG — teardown did not proceed — log: $(cat "$REMOVE_LOG"), preflight output: $preflight_out"
    false
  }
}

# ===========================================================================
# T-003 / AC-005 / PC2b → PC2a retry: stray file relocated to canonical mount → retry teardown proceeds
# BC-6.26.001 PC2b → PC2a retry path
# RG-003: preflight not present
# ===========================================================================

@test "T-003 S-21.04 AC-005: relocate-retry-proceeds — stray file relocated; retry teardown proceeds" {
  # Fixture: stray .factory/ file; relocate to canonical mount; retry teardown.
  # Exercises BC-6.26.001 PC2b → PC2a retry path (Option A relocation per BC PC2b §3).
  #
  # Pre-implementation red gate:
  #   DOC-PARITY (§G.1): 'find .factory' absent → first assertion fails with
  #     "DOC-PARITY FAIL [must contain: step-g-cleanup.md §G.1: find .factory preflight command ...]"
  #   HARNESS: extraction gate fires on both first-pass and retry → 'worktree-remove-invoked'
  #     never written → grep on REMOVE_LOG fails with "HARNESS FAIL: retry teardown did not proceed ..."
  # Post-implementation: first pass BLOCKED (stray file present); after relocation, retry proceeds.

  # --- Fixture setup: stray factory artifact ---
  mkdir -p "$MOCK_WORKTREE/.factory/stories"
  printf 'stray DELIVERY ledger — to be relocated via Option A retry path\n' \
    > "$MOCK_WORKTREE/.factory/stories/S-021-DELIVERY.md"

  # --- DOC-PARITY: step-g-cleanup.md §G.1 preflight mandate ---
  local g1_section
  g1_section="$(_extract_g1_section)"

  _assert_doc_marker 'find.*\.factory' \
    "step-g-cleanup.md §G.1: find .factory preflight command (PC2b→PC2a retry path requires mandate — BC-6.26.001 PC2b)" \
    "$g1_section"
  _assert_doc_marker 'PREFLIGHT BLOCKED' \
    "step-g-cleanup.md §G.1: PREFLIGHT BLOCKED mandate (first pass blocks; retry path gated by same mandate — BC-6.26.001 PC2b, Invariant 2)" \
    "$g1_section"
  _assert_doc_marker 'git worktree remove' \
    "step-g-cleanup.md §G.1: git worktree remove for PC2a retry proceed path (BC-6.26.001 PC2a)" \
    "$g1_section"

  # --- HARNESS: first pass (stray file present) ---
  # Post-implementation this emits PREFLIGHT BLOCKED. Not asserted here; Red Gate
  # proves correctness by REMOVE_LOG staying empty until after relocation.
  local first_out
  first_out="$(_run_teardown_preflight "$MOCK_WORKTREE" "$REMOVE_LOG")"

  # --- Relocation: move stray file to canonical mount (Option A per BC-6.26.001 PC2b §3) ---
  mkdir -p "$CANONICAL_FACTORY/stories"
  mv "$MOCK_WORKTREE/.factory/stories/S-021-DELIVERY.md" \
     "$CANONICAL_FACTORY/stories/S-021-DELIVERY.md"

  # --- HARNESS: retry pass after relocation (shadow tree now empty → PC2a proceed) ---
  local retry_out
  retry_out="$(_run_teardown_preflight "$MOCK_WORKTREE" "$REMOVE_LOG")"

  grep -q 'worktree-remove-invoked' "$REMOVE_LOG" || {
    echo "HARNESS FAIL: retry teardown did not proceed after stray file relocated — REMOVE_LOG: $(cat "$REMOVE_LOG"), first-pass output: $first_out, retry output: $retry_out"
    false
  }
}
