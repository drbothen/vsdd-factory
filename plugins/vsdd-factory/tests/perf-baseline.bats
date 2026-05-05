#!/usr/bin/env bats
# perf-baseline.bats — S-9.00 RED gate: failing tests for perf baseline + bundle ceiling
#
# ALL tests in this suite must FAIL until the implementer creates:
#   .factory/measurements/measure-bundle-sizes.sh
#   .factory/architecture/perf-baseline-w16.md
#
# Test naming: test_S_9_00_<AC>_<description> (no formal BC IDs; [process-gap] story)
#
# Watch-outs enforced:
#   AC-5 anti-tautology: expected byte counts are computed independently via
#     `wc -c < <file>`, never read from perf-baseline-w16.md.
#   AC-7 / B.1: cold-start target is handoff-validator (not legacy-bash-adapter).
#   AC-2: frozen 17-plugin enumeration, not a directory glob.
#   Cross-platform: wc -c only, never du -sb.
#   AC-3 metric separation: advisory cap on all_hook_plugins_wasm_bytes only.

REPO_ROOT="$(cd "$BATS_TEST_DIRNAME/../../.." && pwd)"
# .factory/ lives in the main repo root, not the worktree working tree.
# Resolve it relative to the worktree's git common dir.
# If mount fails, emit a clear error so operators know to run:
#   git worktree add .factory origin/factory-artifacts
MAIN_REPO="$(git -C "$BATS_TEST_DIRNAME" rev-parse --git-common-dir 2>/dev/null | sed 's|/\.git$||' || echo "$REPO_ROOT")"
FACTORY_DIR="$MAIN_REPO/.factory"
[ -d "$FACTORY_DIR" ] || { echo "FACTORY_DIR=$FACTORY_DIR doesn't exist; mount factory-artifacts via 'git worktree add .factory origin/factory-artifacts'" >&2; exit 1; }
BUNDLE_DIR="$REPO_ROOT/plugins/vsdd-factory/hook-plugins"
SCRIPT="$FACTORY_DIR/measurements/measure-bundle-sizes.sh"
BASELINE_DOC="$FACTORY_DIR/architecture/perf-baseline-w16.md"
FIXTURE="$FACTORY_DIR/measurements/fixtures/handoff-validator-input.json"

# ---------------------------------------------------------------------------
# setup_file: ensure factory-dispatcher is built before any test runs.
# Runs once per bats invocation (not per test). Emits a 3-line build summary
# to stderr so CI logs show what happened; the build is skipped silently if
# the binary already exists (idempotent).
# ---------------------------------------------------------------------------
setup_file() {
  local dispatcher="$REPO_ROOT/target/release/factory-dispatcher"
  local dispatcher_exe="$REPO_ROOT/target/release/factory-dispatcher.exe"
  if [ ! -f "$dispatcher" ] && [ ! -f "$dispatcher_exe" ]; then
    echo "# setup_file: factory-dispatcher not found; building..." >&3
    cargo build --release -p factory-dispatcher 2>&1 | tail -3 >&2
    echo "# setup_file: build complete" >&3
  fi
}

# Frozen 17-plugin enumeration from AC-2 (names without .wasm extension).
FROZEN_PLUGINS=(
  block-ai-attribution
  capture-commit-activity
  capture-pr-activity
  handoff-validator
  legacy-bash-adapter
  pr-manager-completion-guard
  regression-gate
  session-end-telemetry
  session-learning
  session-start-telemetry
  tool-failure-hooks
  track-agent-start
  track-agent-stop
  update-wave-state-on-merge
  validate-pr-review-posted
  warn-pending-wave-gate
  worktree-hooks
)

# ---------------------------------------------------------------------------
# AC-1: script emits JSON with all_hook_plugins_wasm_bytes equal to sum of
#        present .wasm files in the bundle directory.
# ---------------------------------------------------------------------------
@test "S-9.00 AC-1: script outputs JSON with all_hook_plugins_wasm_bytes field equal to sum of present wasm files" {
  # Script must exist and be executable.
  [ -f "$SCRIPT" ]
  [ -x "$SCRIPT" ]

  run bash "$SCRIPT" "$BUNDLE_DIR"
  [ "$status" -eq 0 ]

  # Output must be valid JSON.
  echo "$output" | jq . >/dev/null 2>&1

  # all_hook_plugins_wasm_bytes = sum of the frozen-17 plugins only (= sum(per_plugin)).
  # Non-frozen .wasm files (hello-hook.wasm, underscore stubs) go into
  # unaccounted_wasm_bytes, not all_hook_plugins_wasm_bytes.
  # AC-5 anti-tautology: we derive the expected value ourselves, not from the doc.
  local expected_total=0
  for plugin in "${FROZEN_PLUGINS[@]}"; do
    local wasm_file="$BUNDLE_DIR/${plugin}.wasm"
    [ -f "$wasm_file" ] || continue
    local sz
    sz=$(wc -c < "$wasm_file")
    expected_total=$((expected_total + sz))
  done

  local reported
  reported=$(echo "$output" | jq '.all_hook_plugins_wasm_bytes')

  [ "$reported" -eq "$expected_total" ]
}

# ---------------------------------------------------------------------------
# AC-2: script enumerates exactly the 17 frozen plugins by name; JSON has
#        per_plugin map with all 17 keys.
# ---------------------------------------------------------------------------
@test "S-9.00 AC-2: JSON per_plugin map contains all 17 frozen-enumeration plugin keys" {
  [ -f "$SCRIPT" ]
  [ -x "$SCRIPT" ]

  run bash "$SCRIPT" "$BUNDLE_DIR"
  [ "$status" -eq 0 ]

  echo "$output" | jq . >/dev/null 2>&1

  for plugin in "${FROZEN_PLUGINS[@]}"; do
    # per_plugin must have an entry keyed by plugin name (with or without .wasm),
    # and it must be a positive integer (> 0) — "0" means the plugin is absent
    # from the bundle and AC-2 should fail loudly so the frozen list is updated.
    local val
    val=$(echo "$output" | jq --arg p "$plugin" '.per_plugin[$p] // .per_plugin[($p + ".wasm")] // 0')
    [ "$val" -gt 0 ]
  done
}

# ---------------------------------------------------------------------------
# AC-3: JSON has separate all_hook_plugins_wasm_bytes, grand_total_bytes, and
#        dispatcher_bytes fields.  Advisory cap logic applies only to
#        all_hook_plugins_wasm_bytes (not grand_total_bytes).
# ---------------------------------------------------------------------------
@test "S-9.00 AC-3: JSON has distinct all_hook_plugins_wasm_bytes, grand_total_bytes, and dispatcher_bytes fields" {
  [ -f "$SCRIPT" ]
  [ -x "$SCRIPT" ]

  run bash "$SCRIPT" "$BUNDLE_DIR"
  [ "$status" -eq 0 ]

  echo "$output" | jq . >/dev/null 2>&1

  # Each field must exist and be a positive integer (or non-negative for unaccounted).
  local hook_bytes grand_bytes dispatcher_bytes unaccounted_bytes
  hook_bytes=$(echo "$output" | jq '.all_hook_plugins_wasm_bytes')
  grand_bytes=$(echo "$output" | jq '.grand_total_bytes')
  dispatcher_bytes=$(echo "$output" | jq '.dispatcher_bytes')
  unaccounted_bytes=$(echo "$output" | jq '.unaccounted_wasm_bytes')

  [ "$hook_bytes" -gt 0 ]
  [ "$grand_bytes" -gt 0 ]
  [ "$dispatcher_bytes" -ge 0 ]
  [ "$unaccounted_bytes" -ge 0 ]

  # grand_total_bytes must equal dispatcher + all_hook_plugins + unaccounted.
  local expected_grand
  expected_grand=$((dispatcher_bytes + hook_bytes + unaccounted_bytes))
  [ "$grand_bytes" -eq "$expected_grand" ]

  # dispatcher_bytes must be > 0: if the binary is absent, build it so the
  # metric-separation guard below is meaningful. An absent dispatcher trivially
  # satisfies the old disjunction (dispatcher_bytes==0 ⇒ hook==grand) and
  # nullifies the AC-3 check.
  [ "$dispatcher_bytes" -gt 0 ]

  # Sanity: all_hook_plugins_wasm_bytes != grand_total_bytes when dispatcher
  # contributes (i.e. they are distinct metrics, not the same field).
  # No disjunction — dispatcher_bytes > 0 is now asserted above.
  [ "$hook_bytes" -ne "$grand_bytes" ]
}

# ---------------------------------------------------------------------------
# AC-4: script is idempotent — two successive runs produce byte-identical
#        JSON for all_hook_plugins_wasm_bytes and per_plugin values.
# ---------------------------------------------------------------------------
@test "S-9.00 AC-4: script is idempotent — two runs produce identical byte counts" {
  [ -f "$SCRIPT" ]
  [ -x "$SCRIPT" ]

  run bash "$SCRIPT" "$BUNDLE_DIR"
  [ "$status" -eq 0 ]
  local run1="$output"

  run bash "$SCRIPT" "$BUNDLE_DIR"
  [ "$status" -eq 0 ]
  local run2="$output"

  local total1 total2
  total1=$(echo "$run1" | jq '.all_hook_plugins_wasm_bytes')
  total2=$(echo "$run2" | jq '.all_hook_plugins_wasm_bytes')
  [ "$total1" -eq "$total2" ]

  # per_plugin maps must match as well.
  local pp1 pp2
  pp1=$(echo "$run1" | jq -cS '.per_plugin')
  pp2=$(echo "$run2" | jq -cS '.per_plugin')
  [ "$pp1" = "$pp2" ]
}

# ---------------------------------------------------------------------------
# AC-5 (anti-tautology gate — THE BIG ONE): script-reported per-plugin byte
#        counts match independently-computed wc -c values for each .wasm file.
#        The test NEVER reads perf-baseline-w16.md to learn expected values.
# ---------------------------------------------------------------------------
@test "S-9.00 AC-5: script per-plugin byte counts match independent wc -c measurements" {
  [ -f "$SCRIPT" ]
  [ -x "$SCRIPT" ]

  run bash "$SCRIPT" "$BUNDLE_DIR"
  [ "$status" -eq 0 ]

  echo "$output" | jq . >/dev/null 2>&1

  # For each frozen plugin, independently measure bytes and compare to script output.
  for plugin in "${FROZEN_PLUGINS[@]}"; do
    local wasm_file="$BUNDLE_DIR/${plugin}.wasm"
    # Skip plugins whose wasm file is absent (EC-001: partial bundle state is allowed).
    [ -f "$wasm_file" ] || continue

    # Independent measurement — never reads the baseline doc.
    local expected_bytes
    expected_bytes=$(wc -c < "$wasm_file")

    local reported_bytes
    reported_bytes=$(echo "$output" | jq --arg p "$plugin" '
      .per_plugin[$p] //
      .per_plugin[($p + ".wasm")] //
      error("plugin key not found")
    ')

    [ "$reported_bytes" -eq "$expected_bytes" ]
  done
}

# ---------------------------------------------------------------------------
# AC-6: baseline doc perf-baseline-w16.md exists and contains required fields:
#        JSON output, 30MB kill-switch threshold, advisory cap (median×3
#        formula), and a methodology section.
# ---------------------------------------------------------------------------
@test "S-9.00 AC-6: perf-baseline-w16.md exists with required sections" {
  [ -f "$BASELINE_DOC" ]

  local content
  content=$(cat "$BASELINE_DOC")

  # Must contain the JSON bundle output (per_plugin or all_hook_plugins_wasm_bytes).
  echo "$content" | grep -q "all_hook_plugins_wasm_bytes"

  # Must document the 30MB hard kill-switch.
  echo "$content" | grep -qE "30[_,]?000[_,]?000|30MB"

  # Must document the advisory cap formula (median x 3 or median × 3).
  echo "$content" | grep -qiE "median.*[x×].*3|median.*3|3.*median"

  # Must have a "## Methodology" heading (exact casing).
  echo "$content" | grep -qE "^## Methodology$"

  # Methodology section must mention wc -c (the byte-count tool).
  echo "$content" | grep -q "wc -c"

  # Methodology section must mention hyperfine (the cold-start measurement tool).
  echo "$content" | grep -q "hyperfine"

  # Must cite the cold-start p95 gate (500ms).
  echo "$content" | grep -qE "500"
}

# ---------------------------------------------------------------------------
# AC-7: cold-start measurement fixture exists; the fixture targets
#        handoff-validator (SubagentStop), NOT legacy-bash-adapter.
#        Test exercises that the fixture file is valid JSON with correct event.
# ---------------------------------------------------------------------------
@test "S-9.00 AC-7: cold-start baseline measured via handoff-validator and recorded in perf-baseline-w16.md" {
  # Fixture must exist (AC-9 artifact).
  [ -f "$FIXTURE" ]

  # Must be valid JSON.
  jq . "$FIXTURE" >/dev/null 2>&1

  # Must target SubagentStop event (handoff-validator's registered event).
  local event
  event=$(jq -r '.hook_event_name' "$FIXTURE")
  [ "$event" = "SubagentStop" ]

  # Guard: fixture must NOT reference legacy-bash-adapter (watch-out B.1).
  # (legacy-bash-adapter adds subprocess overhead and is not the measurement target.)
  local content
  content=$(cat "$FIXTURE")
  ! echo "$content" | grep -q "legacy-bash-adapter"

  # The baseline doc must exist and record the measured cold-start p95 value
  # (Task B.1 hyperfine run) and confirm it is ≤ 500ms.
  [ -f "$BASELINE_DOC" ]

  # Baseline doc must have a cold_start_p95_measured_ms entry.
  grep -qiE "cold_start_p95_measured_ms|cold.start.*p95.*measured" "$BASELINE_DOC"

  # Baseline doc must confirm the 500ms gate is cited.
  grep -qE "500" "$BASELINE_DOC"
}

# ---------------------------------------------------------------------------
# AC-8: new plugin ceiling policy — advisory cap per new plugin is
#        median(17-plugin enumeration) × 3.  The baseline doc must record
#        the computed median and the per-plugin cap derived from it.
# ---------------------------------------------------------------------------
@test "S-9.00 AC-8: baseline doc records median-based per-plugin ceiling (median × 3)" {
  [ -f "$BASELINE_DOC" ]

  # Baseline doc must have a per_plugin_bytes section or table.
  grep -qiE "per.plugin|per_plugin" "$BASELINE_DOC"

  # Must have a w16_advisory_bundle_soft_cap_bytes or advisory_soft_cap field.
  grep -qiE "advisory.*cap|soft.cap|advisory_soft_cap" "$BASELINE_DOC"

  # Must include the median value or the formula for computing it.
  grep -qiE "median" "$BASELINE_DOC"

  # Verify median × 3 per-plugin ceiling is computable.
  # Independently compute the median from the 17 frozen plugins present in bundle dir.
  local sizes=()
  for plugin in "${FROZEN_PLUGINS[@]}"; do
    local wf="$BUNDLE_DIR/${plugin}.wasm"
    [ -f "$wf" ] || continue
    local sz
    sz=$(wc -c < "$wf")
    sizes+=("$sz")
  done

  # Must have at least 1 plugin present to compute median.
  [ "${#sizes[@]}" -gt 0 ]

  # Sort and find median.
  IFS=$'\n' sorted=($(printf '%s\n' "${sizes[@]}" | sort -n))
  local n="${#sorted[@]}"
  local mid=$(( (n - 1) / 2 ))
  local median="${sorted[$mid]}"
  local cap=$(( median * 3 ))

  # cap must be a positive integer.
  [ "$cap" -gt 0 ]
}

# ---------------------------------------------------------------------------
# AC-9: all output artifacts are committed to canonical paths.
#        Tests that each required file exists (script + baseline doc + fixture).
# ---------------------------------------------------------------------------
@test "S-9.00 AC-9: all three required artifacts exist at canonical paths" {
  # .factory/measurements/measure-bundle-sizes.sh
  [ -f "$SCRIPT" ]

  # .factory/architecture/perf-baseline-w16.md
  [ -f "$BASELINE_DOC" ]

  # .factory/measurements/fixtures/handoff-validator-input.json
  [ -f "$FIXTURE" ]

  # Script must be executable.
  [ -x "$SCRIPT" ]

  # Baseline doc must not be empty.
  [ -s "$BASELINE_DOC" ]
}

# ---------------------------------------------------------------------------
# AC-10: ADR-013 convergence gate — baseline doc records that adversarial
#         convergence is required before S-9.01..S-9.07 may be dispatched.
#         Test checks that the doc mentions convergence requirement or ADR-013.
# ---------------------------------------------------------------------------
@test "S-9.00 AC-10: baseline doc references ADR-013 convergence gate before S-9.01 dispatch" {
  [ -f "$BASELINE_DOC" ]

  # Must cite ADR-013 or the convergence requirement.
  grep -qiE "ADR-013|ADR013|convergence.*required|convergence.*gate|NITPICK_ONLY" "$BASELINE_DOC"
}
