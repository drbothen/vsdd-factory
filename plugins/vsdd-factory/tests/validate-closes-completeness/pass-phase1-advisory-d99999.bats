#!/usr/bin/env bats
# pass-phase1-advisory-d99999.bats — ADR-041 sentinel: D-99999 is the canonical "correctly
#                                     formatted but nonexistent D-NNN" test fixture value.
#                                     Phase 1 must emit advisory only (not block).
#
# Traces to:
#   BC-5.39.007 postcondition 10 (cross-site staleness => advisory log only, NOT block)
#   BC-5.39.007 EC-010 (Phase 1/2 boundary: staleness detection deferred to Phase 2)
#   ADR-041 §Decision 1 (D-99999 is the canonical sentinel; allocator-range-adjacent predecessor retired)
#
# CRITICAL BOUNDARY: D-99999 is a correctly-formatted cite ID matching the D-\d+ pattern.
# Phase 1 does NOT block on correctly-formatted IDs that reference nonexistent decisions.
# Cross-site staleness detection is Phase 2 scope per BC-5.39.007 Phase 1/2 boundary table.
#
# Two scenarios (mutant proof — proves the gate actually fires):
#   A. Fixture with "**Closes:** D-99999" (correct format, nonexistent D-NNN)
#      => exits 0 (Continue, advisory only — Phase 1/2 boundary)
#   B. MUTANT: same fixture but "**Closes:** fixed-the-thing" (malformed, no structured ID)
#      => exits 2 (block, D-419(c)) — proves the exit-0 assertion in scenario A is real
#
# RED GATE PHASE: test skips because validate-closes-completeness.wasm is not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-closes-completeness.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-closes-completeness/pass-phase1-advisory-d99999"
  WORK="$(mktemp -d)"
  mkdir -p "$WORK/hook-plugins"
  mkdir -p "$WORK/.factory/logs"
}

teardown() {
  [ -n "${WORK:-}" ] && [ -d "$WORK" ] && find "$WORK" -type f -delete && find "$WORK" -type d -mindepth 1 | sort -r | xargs rmdir 2>/dev/null && rmdir "$WORK" 2>/dev/null || true
}

_setup_fixture() {
  cp -r "$FIXTURE_SRC/factory/." "$WORK/.factory/"
}

_write_registry() {
  cat > "$WORK/hooks-registry.toml" << 'TOML'
schema_version = 2

[[hooks]]
name = "validate-closes-completeness"
event = "PostToolUse"
tool = "Edit|Write"
plugin = "hook-plugins/validate-closes-completeness.wasm"
priority = 156
timeout_ms = 5000
on_error = "continue"

[hooks.capabilities.read_file]
path_allow = [
  ".factory/cycles",
  ".factory/STATE.md",
  ".factory/cycles/v1.0-brownfield-backfill/INDEX.md",
  ".factory/cycles/v1.0-brownfield-backfill/lessons.md",
]
TOML
}

_require_artifacts() {
  if [ ! -x "$DISPATCHER" ]; then
    skip "dispatcher binary not built -- run: cargo build --release -p factory-dispatcher"
  fi
  if [ ! -f "$WASM_PLUGIN" ]; then
    skip "validate-closes-completeness.wasm not built -- implement T-4 through T-7 of S-15.12"
  fi
}

_lessons_md_envelope() {
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"pass-phase1-advisory-d99999","tool_input":{"file_path":".factory/cycles/v1.0-brownfield-backfill/lessons.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# Scenario A: D-99999 (canonical sentinel, nonexistent) => exits 0 (advisory only, NOT block)
# Traces to BC-5.39.007 postcondition 10 / EC-010; ADR-041 §Decision 1
# CRITICAL: if this test fails (exits 2), the Phase 1/2 boundary is broken.
# ---------------------------------------------------------------------------

@test "ADR-041 sentinel PASS: hook emits Continue for D-99999 cite (correctly formatted, nonexistent, Phase 1 advisory only)" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_lessons_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 0: correctly-formatted ID must NOT block in Phase 1
  [ "$status" -eq 0 ]

  # No blocking_plugins= — advisory only, not a block
  [[ "$output" != *"blocking_plugins="* ]]
}

# ---------------------------------------------------------------------------
# Scenario B (MUTANT PROOF): malformed cite => exits 2 (block, D-419(c))
# This proves the exit-0 assertion in Scenario A is real — the hook CAN block.
# If this test fails (exits 0), the check_cite_id_format gate is inert (false-clean).
# ---------------------------------------------------------------------------

@test "ADR-041 sentinel MUTANT PROOF: hook blocks on malformed cite (proves Scenario A gate is live)" {
  _require_artifacts
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  # Write mutant fixture inline: "**Closes:** fixed-the-thing" — freeform prose, no structured ID.
  # This is the malformed case that MUST block (D-419(c)). If the hook exits 0 here,
  # the cite-format gate is inert and Scenario A provides no meaningful coverage.
  mkdir -p "$WORK/.factory/cycles/v1.0-brownfield-backfill"
  cat > "$WORK/.factory/cycles/v1.0-brownfield-backfill/lessons.md" << 'MD'
# Lessons Log — v1.0-brownfield-backfill

## L-EDP1-055 — Mutant: malformed cite for sentinel proof

Category: process
Date: 2026-04-22

Lesson: this fixture uses a malformed Closes cite to prove the format gate fires.

**Closes:** fixed-the-thing
MD

  local envelope
  envelope="$(_lessons_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 2: malformed cite must block
  [ "$status" -eq 2 ]

  # blocking_plugins= names this hook — gate is live, not inert
  [[ "$output" == *"blocking_plugins=validate-closes-completeness"* ]]
}
