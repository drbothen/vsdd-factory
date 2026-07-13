#!/usr/bin/env bats
# bundle-orphan-detection.bats — AC-006 dual-registry orphan detection tests (S-19.04).
#
# Guards the dual-registry invariant from EC-003: a WASM present under
# hook-plugins/ is non-orphan if referenced by EITHER hooks-registry.toml OR
# resolvers-registry.toml.  Checking only one registry produces a false-positive
# orphan classification (root cause of v1.0 story defect for vsdd-context-resolvers.wasm).
#
# Tests:
#   T-006  AC-006  resolvers-registry-only WASM → non-orphan (dual-registry regression gate)
#   T-007  AC-006  neither-registry WASM → orphan
#   T-008  AC-006  negative-control (F-P2-010): resolvers-only WASM classified
#                  orphan when resolvers-registry check is omitted — confirms the
#                  dual-registry check is load-bearing, not advisory.
#
# All three tests FAIL at Red Gate because the implementation script
# (plugins/vsdd-factory/bin/bundle-orphan-check.sh) does not exist yet.
# The implementer creates the script; the tests pass once it is present and
# correct.
#
# Script interface (implemented by S-19.04 implementer):
#   bundle-orphan-check.sh <hook-plugins-dir> <hooks-registry> [<resolvers-registry>]
#
#   Exit 0  : zero orphan WASMs found.
#   Exit >0 : at least one orphan WASM found; prints "ORPHAN: <name>" per orphan.
#
# VP Trace: —  (AC-006 does not map to a VP; wires EAC-005 as load-bearing leg)
# Story: S-19.04

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../.." && pwd)"
  ORPHAN_CHECK="$REPO_ROOT/plugins/vsdd-factory/bin/bundle-orphan-check.sh"

  WORK="$(mktemp -d)"
  mkdir -p "$WORK/hook-plugins"

  # Minimal hooks-registry.toml: references hooks-only.wasm only.
  cat > "$WORK/hooks-registry.toml" << 'TOML'
schema_version = 2

[[hooks]]
name = "hooks-only-hook"
event = "PostToolUse"
tool = "^(Edit|Write)$"
plugin = "hook-plugins/hooks-only.wasm"
timeout_ms = 5000
on_error = "continue"
TOML

  # Minimal resolvers-registry.toml: references resolvers-only.wasm only.
  cat > "$WORK/resolvers-registry.toml" << 'TOML'
schema_version = 1

[[resolvers]]
name = "resolvers_only"
plugin = "hook-plugins/resolvers-only.wasm"
context_key = "resolvers_only"
path_allow = [".factory/wave-state.yaml"]
TOML

  # Populate hook-plugins/ with three WASMs:
  #   hooks-only.wasm       — referenced by hooks-registry.toml only
  #   resolvers-only.wasm   — referenced by resolvers-registry.toml only
  #   neither-registry.wasm — referenced by neither registry (orphan)
  touch "$WORK/hook-plugins/hooks-only.wasm"
  touch "$WORK/hook-plugins/resolvers-only.wasm"
  touch "$WORK/hook-plugins/neither-registry.wasm"
}

teardown() {
  rm -rf "$WORK"
}

# ---------------------------------------------------------------------------
# T-006  AC-006  [FAILS AT RED GATE]
# A WASM referenced only by resolvers-registry.toml must be classified as
# non-orphan when the dual-registry check is used.  This is the regression
# gate for the v1.0 story defect (EC-003): vsdd-context-resolvers.wasm (hyphen)
# was falsely classified as an orphan because only hooks-registry.toml was checked.
#
# Red Gate failure: bundle-orphan-check.sh does not exist → status=127 ≠ 0 → fails.
# ---------------------------------------------------------------------------
@test "T-006 AC-006: resolvers-registry-only WASM is non-orphan (dual-registry check)" {
  # resolvers-only.wasm is in resolvers-registry.toml but NOT in hooks-registry.toml.
  # With dual-registry check the script must exit 0 (no orphans).
  run "$ORPHAN_CHECK" \
    "$WORK/hook-plugins" \
    "$WORK/hooks-registry.toml" \
    "$WORK/resolvers-registry.toml"

  if [ "$status" -ne 0 ]; then
    echo "FAIL: script exited $status (expected 0 — no orphans)"
    echo "output: $output"
    false
  fi

  # resolvers-only.wasm must NOT appear in orphan output.
  if [[ "$output" == *"resolvers-only.wasm"* ]]; then
    echo "FAIL: resolvers-only.wasm incorrectly classified as orphan: $output"
    false
  fi
}

# ---------------------------------------------------------------------------
# T-007  AC-006  [FAILS AT RED GATE]
# A WASM present in hook-plugins/ but referenced by neither registry must be
# classified as an orphan (script exits non-zero and prints "ORPHAN: <name>").
#
# Red Gate failure: bundle-orphan-check.sh does not exist → output is empty
# (no "ORPHAN: neither-registry.wasm" line) → output assertion fails.
# ---------------------------------------------------------------------------
@test "T-007 AC-006: neither-registry WASM classified as orphan" {
  # neither-registry.wasm is in hook-plugins/ but not in either registry.
  run "$ORPHAN_CHECK" \
    "$WORK/hook-plugins" \
    "$WORK/hooks-registry.toml" \
    "$WORK/resolvers-registry.toml"

  # Script must exit non-zero (orphan found).
  [ "$status" -ne 0 ]

  # Script must emit a diagnostic line for the orphan.
  if [[ "$output" != *"ORPHAN: neither-registry.wasm"* ]]; then
    echo "FAIL: expected 'ORPHAN: neither-registry.wasm' in output; got: $output"
    false
  fi
}

# ---------------------------------------------------------------------------
# T-008  AC-006 negative-control (F-P2-010)  [FAILS AT RED GATE]
# When the script is invoked with ONLY hooks-registry (no resolvers-registry
# argument), a WASM referenced only by resolvers-registry.toml MUST be
# classified as an orphan.  This confirms the dual-registry check is
# load-bearing: if the check were advisory, a single-registry invocation
# would incorrectly classify the WASM as non-orphan.
#
# Red Gate failure: bundle-orphan-check.sh does not exist → output is empty
# (no "ORPHAN: resolvers-only.wasm" line) → output assertion fails.
# ---------------------------------------------------------------------------
@test "T-008 AC-006 negative-control: resolvers-only WASM is orphan when resolvers-registry check omitted" {
  # Invoke with only hooks-registry — no resolvers-registry argument.
  run "$ORPHAN_CHECK" \
    "$WORK/hook-plugins" \
    "$WORK/hooks-registry.toml"

  # Single-registry mode must exit non-zero (resolvers-only.wasm is orphan).
  [ "$status" -ne 0 ]

  # Script must emit a diagnostic line for the false-orphan in single-registry mode.
  if [[ "$output" != *"ORPHAN: resolvers-only.wasm"* ]]; then
    echo "FAIL: expected 'ORPHAN: resolvers-only.wasm' in single-registry output; got: $output"
    echo "NOTE: this confirms the dual-registry check is load-bearing, not advisory."
    false
  fi
}
