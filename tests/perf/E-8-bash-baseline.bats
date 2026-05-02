#!/usr/bin/env bats
# E-8-bash-baseline.bats — Perf baseline timing harness for S-8.00
#
# Purpose: Measure warm-invocation wall-clock latency for 3 representative
# bash hooks (one per tier) using hyperfine --warmup 3 --runs 10.
#
# Results per hook are written to /tmp/E-8-timing-<hook>.json by each test;
# the implementer (Task A.4) assembles them into:
#   .factory/measurements/E-8-bash-baseline.json
#
# Prerequisites:
#   - hyperfine >= 1.18  (Task A.0: apt-get install hyperfine  OR  brew install hyperfine)
#   - bats-core >= 1.10
#   - jq >= 1.6
#
# RED GATE: These tests MUST FAIL until:
#   (a) hyperfine is installed (Task A.0)
#   (b) Fixture files exist under tests/perf/fixtures/
#   (c) The hooks are reachable at their expected paths
#
# Run: bats tests/perf/E-8-bash-baseline.bats
#
# Story: S-8.00 (Wave 15 entry-point pre-work)
# See: .factory/stories/S-8.00-perf-baseline-bc-anchor-verification.md Tasks §A.2

# ---------------------------------------------------------------------------
# require_hyperfine: per-test guard that fails loud (no skip) if hyperfine absent
# This is called inside each @test body so all 3 tests fail individually.
# ---------------------------------------------------------------------------
require_hyperfine() {
  if ! command -v hyperfine &>/dev/null; then
    echo "FAIL: hyperfine is not installed." >&2
    echo "Task A.0 required: sudo apt-get install hyperfine  OR  brew install hyperfine" >&2
    return 1
  fi
}

# ---------------------------------------------------------------------------
# Test 1 (AC-1): handoff-validator.sh — Tier 1 / SubagentStop / block-mode
# Representative complexity: JSON parse + threshold logic
# hyperfine target: plugins/vsdd-factory/hooks/handoff-validator.sh
# ---------------------------------------------------------------------------
@test "BC-perf-baseline AC-1: handoff-validator.sh Tier 1 warm-invocation latency" {
  # Task A.0 guard: fail with actionable message if hyperfine absent
  require_hyperfine

  FIXTURE="$(git rev-parse --show-toplevel)/tests/perf/fixtures/handoff-stop.json"
  HOOK="$(git rev-parse --show-toplevel)/plugins/vsdd-factory/hooks/handoff-validator.sh"
  EXPORT_JSON="/tmp/E-8-timing-handoff-validator.json"

  # Fixture must exist
  [ -f "$FIXTURE" ]
  # Hook must exist and be executable
  [ -x "$HOOK" ]

  # AC-1: measure warm-invocation latency with hyperfine
  # SubagentStop — hook reads from stdin; exit 0 expected for valid non-empty result
  run hyperfine \
    --warmup 3 \
    --runs 10 \
    --export-json "$EXPORT_JSON" \
    --input "$FIXTURE" \
    "$HOOK"
  [ "$status" -eq 0 ]

  # Verify export JSON exists and parses
  [ -f "$EXPORT_JSON" ]
  run jq 'empty' "$EXPORT_JSON"
  [ "$status" -eq 0 ]

  # Assert median > 0 (measurement produced a real value)
  median=$(jq '.results[0].median' "$EXPORT_JSON")
  [ -n "$median" ]
  [ "$median" != "null" ]
  # median is in seconds; any positive float passes (> 0)
  run bash -c "echo '$median > 0' | bc -l"
  [ "$output" = "1" ]

  median_ms=$(jq '(.results[0].median * 1000) | floor' "$EXPORT_JSON")
  echo "handoff-validator.sh Tier 1 median: ${median_ms}ms"
}

# ---------------------------------------------------------------------------
# Test 2 (AC-1+AC-2): validate-bc-title.sh — Tier 2 / PostToolUse:Edit|Write
# Representative complexity: grep + awk pattern match (low complexity)
# hyperfine target: plugins/vsdd-factory/hooks/validate-bc-title.sh
# AC-2: Tier 2 aggregate projection = median_ms × 23; assert <= 200ms (or record violation)
# ---------------------------------------------------------------------------
@test "BC-perf-baseline AC-1+AC-2: validate-bc-title.sh Tier 2 warm-invocation latency + aggregate projection" {
  # Task A.0 guard
  require_hyperfine

  FIXTURE="$(git rev-parse --show-toplevel)/tests/perf/fixtures/post-tool-use-edit.json"
  HOOK="$(git rev-parse --show-toplevel)/plugins/vsdd-factory/hooks/validate-bc-title.sh"
  EXPORT_JSON="/tmp/E-8-timing-validate-bc-title.json"

  # Fixture must exist
  [ -f "$FIXTURE" ]
  # Hook must exist and be executable
  [ -x "$HOOK" ]

  # AC-1: measure warm-invocation latency
  # PostToolUse:Edit|Write — hook reads from stdin; exit 0 expected for non-BC file
  run hyperfine \
    --warmup 3 \
    --runs 10 \
    --export-json "$EXPORT_JSON" \
    --input "$FIXTURE" \
    "$HOOK"
  [ "$status" -eq 0 ]

  # Verify export JSON exists and parses
  [ -f "$EXPORT_JSON" ]
  run jq 'empty' "$EXPORT_JSON"
  [ "$status" -eq 0 ]

  # Assert median > 0
  median=$(jq '.results[0].median' "$EXPORT_JSON")
  [ -n "$median" ]
  [ "$median" != "null" ]
  run bash -c "echo '$median > 0' | bc -l"
  [ "$output" = "1" ]

  median_ms=$(jq '(.results[0].median * 1000) | floor' "$EXPORT_JSON")
  echo "validate-bc-title.sh Tier 2 median: ${median_ms}ms"

  # AC-2: Tier 2 aggregate projection — 23 plugins × median_ms
  # If projection > 200ms, the E-8 AC-7b 200ms p95 ceiling assumption is violated.
  # This test records the result; the implementer must update E-8 epic AC-7b if violated.
  projected=$(jq '((.results[0].median * 1000) | floor) * 23' "$EXPORT_JSON")
  echo "AC-2 Tier 2 aggregate projection: ${projected}ms (23 × ${median_ms}ms)"
  # Assert projection is a non-null number (measurement valid)
  [ -n "$projected" ]
  [ "$projected" != "null" ]
}

# ---------------------------------------------------------------------------
# Test 3 (AC-1): protect-bc.sh — Tier 3 / PreToolUse:Edit|Write / block-mode
# Representative complexity: permissionDecision envelope, green-BC guard
# hyperfine target: plugins/vsdd-factory/hooks/protect-bc.sh
# ---------------------------------------------------------------------------
@test "BC-perf-baseline AC-1: protect-bc.sh Tier 3 warm-invocation latency" {
  # Task A.0 guard
  require_hyperfine

  FIXTURE="$(git rev-parse --show-toplevel)/tests/perf/fixtures/pre-tool-use-edit.json"
  HOOK="$(git rev-parse --show-toplevel)/plugins/vsdd-factory/hooks/protect-bc.sh"
  EXPORT_JSON="/tmp/E-8-timing-protect-bc.json"

  # Fixture must exist
  [ -f "$FIXTURE" ]
  # Hook must exist and be executable
  [ -x "$HOOK" ]

  # AC-1: measure warm-invocation latency
  # PreToolUse:Edit|Write — fixture points to non-BC file so hook emits "allow"
  # protect-bc.sh exits 0 (emit_allow) for non-.factory/specs/behavioral-contracts/ paths
  run hyperfine \
    --warmup 3 \
    --runs 10 \
    --export-json "$EXPORT_JSON" \
    --input "$FIXTURE" \
    "$HOOK"
  [ "$status" -eq 0 ]

  # Verify export JSON exists and parses
  [ -f "$EXPORT_JSON" ]
  run jq 'empty' "$EXPORT_JSON"
  [ "$status" -eq 0 ]

  # Assert median > 0
  median=$(jq '.results[0].median' "$EXPORT_JSON")
  [ -n "$median" ]
  [ "$median" != "null" ]
  run bash -c "echo '$median > 0' | bc -l"
  [ "$output" = "1" ]

  median_ms=$(jq '(.results[0].median * 1000) | floor' "$EXPORT_JSON")
  echo "protect-bc.sh Tier 3 median: ${median_ms}ms"
}
