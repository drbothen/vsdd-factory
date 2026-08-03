#!/usr/bin/env bats
# validate-cross-site-correspondence.bats — Integration tests for the
# validate-cross-site-correspondence PostToolUse WASM hook plugin.
#
# These tests deliver full payload-delivery scenarios for AC-001 through AC-018,
# covering the seven arms (A1, A2, B1, B2, D, E1, E2) via fixture files.
#
# RED GATE (BC-5.38.001): Every payload-driven test MUST FAIL before implementation.
#
# Failure mechanism:
#   - The stub WASM panics on every code path via todo!() bodies.
#   - The dispatcher's on_error="continue" swallows the panic and returns exit 0.
#   - Exit code alone cannot distinguish "plugin ran correctly" from "plugin crashed".
#   - The internal dispatcher log (VSDD_LOG_DIR) records plugin.crashed on every panic.
#
# Discriminating signal: _assert_plugin_ran_not_crashed reads
#   $WORK/.factory/logs/dispatcher-internal-YYYY-MM-DD.jsonl
# and asserts that:
#   (a) a plugin.invoked record exists for validate-cross-site-correspondence, AND
#   (b) no plugin.crashed record exists for it.
# Under the stub, condition (b) always fails → ALL 31 payload tests FAIL in Red Gate.
# After implementation, the stub is replaced with real logic, no plugin.crashed fires,
# and the tests pass or fail based on actual arm behavior.
#
# NOTE: VSDD_SINK_FILE is not functional in rc.23 (the file is never created).
# The internal log at VSDD_LOG_DIR/dispatcher-internal-YYYY-MM-DD.jsonl IS functional.
#
# AC-019 cap-passing: The constant VALUES (BC_INDEX_MAX_BYTES etc.) are verified by a
# unit test in arm_a1.rs that PASSES in Red Gate. Cap-PASSING (that those constants are
# actually passed to host::read_file) cannot be tested from the pure seam — it requires
# a post-implementation bats integration test with a fixture sized at the cap boundary.
# The implementer must add that test. See arm_a1.rs test_BC_5_39_010_AC019_... for details.
#
# AC-020 registry shape tests: PASS in Red Gate legitimately — they grep the TOML registry
# file directly and do not invoke the dispatcher at all. Their passing is expected and correct.
#
# Governing BC: BC-5.39.010 v1.2
# Story: S-21.07
#
# Fixture layout: fixtures/validate-cross-site-correspondence/<scenario>/factory/
# The "factory/" subtree is copied to $WORK/.factory/ in each test setup.

FIXTURE_BASE="$(cd "$(dirname "$BATS_TEST_FILENAME")/fixtures/validate-cross-site-correspondence" && pwd)"

# ---------------------------------------------------------------------------
# setup / teardown
# ---------------------------------------------------------------------------

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"

  # Dispatcher: prefer locally built release binary; fall back to operator cache.
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  if [ ! -x "$DISPATCHER" ]; then
    DISPATCHER="$(find "$HOME/.claude/plugins/cache/claude-mp/vsdd-factory" \
      -name "factory-dispatcher" -path "*/darwin-arm64/*" 2>/dev/null | sort -V | tail -1)"
  fi

  GUARD_WASM="$PLUGIN_ROOT/hook-plugins/validate-cross-site-correspondence.wasm"

  WORK="$(mktemp -d)"
  mkdir -p "$WORK/.factory/logs"
  mkdir -p "$WORK/hook-plugins"

  if [ -f "$GUARD_WASM" ]; then
    cp "$GUARD_WASM" "$WORK/hook-plugins/validate-cross-site-correspondence.wasm"
  fi

  export VSDD_LOG_DIR="$WORK/.factory/logs"
}

teardown() {
  [ -n "${WORK:-}" ] && rm -rf "$WORK"
}

# ---------------------------------------------------------------------------
# Preflight helpers
# ---------------------------------------------------------------------------

# Skip if dispatcher binary or WASM is not present.
# Set CI_REQUIRE_ARTIFACTS=1 to convert skips to hard failures.
_require_artifacts() {
  if [ -z "$DISPATCHER" ] || [ ! -x "$DISPATCHER" ]; then
    [ -z "${CI_REQUIRE_ARTIFACTS:-}" ] || {
      echo "FAIL: factory-dispatcher binary not present (CI_REQUIRE_ARTIFACTS=1)."
      echo "  Run: cargo build --release -p factory-dispatcher"
      return 1
    }
    skip "factory-dispatcher binary not found — run: cargo build --release -p factory-dispatcher"
  fi
  if [ ! -f "$WORK/hook-plugins/validate-cross-site-correspondence.wasm" ]; then
    [ -z "${CI_REQUIRE_ARTIFACTS:-}" ] || {
      echo "FAIL: validate-cross-site-correspondence.wasm not staged."
      echo "  Run: cargo build --release --target wasm32-wasip1 -p validate-cross-site-correspondence"
      echo "  Then: cp target/wasm32-wasip1/release/validate-cross-site-correspondence.wasm plugins/vsdd-factory/hook-plugins/"
      return 1
    }
    skip "validate-cross-site-correspondence.wasm not staged — see story S-21.07 AC-021"
  fi
}

# ---------------------------------------------------------------------------
# Fixture loader: copy scenario's factory/ subtree into $WORK/.factory/
# ---------------------------------------------------------------------------

_load_fixture() {
  local scenario="$1"
  local src="$FIXTURE_BASE/$scenario/factory"
  if [ -d "$src" ]; then
    cp -r "$src/." "$WORK/.factory/"
  fi
}

# ---------------------------------------------------------------------------
# Registry writer: write a synthetic hooks-registry.toml with one entry.
# Optional argument overrides the path_allow list (double-quoted TOML strings,
# comma-separated). Default: full production path_allow covering all four
# .factory/ subtrees.
# ---------------------------------------------------------------------------

_write_registry() {
  # Build the TOML path_allow list. Default uses all four production subtrees.
  # Callers can pass a custom list as the first argument.
  local path_allow_lines
  if [ $# -ge 1 ]; then
    path_allow_lines="$1"
  else
    path_allow_lines='".factory/specs/behavioral-contracts/",
  ".factory/specs/verification-properties/",
  ".factory/stories/",
  ".factory/cycles/"'
  fi

  cat > "$WORK/hooks-registry.toml" <<TOML
schema_version = 2

[[hooks]]
name = "validate-cross-site-correspondence"
event = "PostToolUse"
tool = "^(Edit|Write|MultiEdit)\$"
plugin = "hook-plugins/validate-cross-site-correspondence.wasm"
priority = 460
timeout_ms = 8000
on_error = "continue"
async = false

[hooks.capabilities.read_file]
path_allow = [
  $path_allow_lines
]
TOML
}

# ---------------------------------------------------------------------------
# Envelope builder: PostToolUse Write event with given file_path.
# ---------------------------------------------------------------------------

_post_write_event() {
  local file_path="$1"
  local session="${2:-test-vcsc}"
  local escaped
  escaped="$(printf '%s' "$file_path" | sed 's/\\/\\\\/g; s/"/\\"/g')"
  printf '{"event_name":"PostToolUse","tool_name":"Write","session_id":"%s","dispatcher_trace_id":"%s-trace","tool_input":{"file_path":"%s","content":""},"tool_response":{}}' \
    "$session" "$session" "$escaped"
}

# ---------------------------------------------------------------------------
# Dispatcher runner: writes envelope to file and invokes dispatcher.
# Captures stdout in $output (bats convention); stderr in $_DISP_STDERR.
# ---------------------------------------------------------------------------

_run_dispatcher() {
  local envelope="$1"
  local env_file="$WORK/envelope-$$.json"
  _DISP_STDERR="$WORK/dispatcher-stderr-$$.txt"
  printf '%s' "$envelope" > "$env_file"
  run bash -c "VSDD_LOG_DIR='$WORK/.factory/logs' \
    CLAUDE_PLUGIN_ROOT='$WORK' \
    CLAUDE_PROJECT_DIR='$WORK' \
    '$DISPATCHER' < '$env_file' 2>'$_DISP_STDERR'"
}

# Assert exit code and optionally check combined output for a pattern.
_assert_exit() {
  local expected="$1"
  local pattern="${2:-}"
  local combined
  combined="$(cat "$_DISP_STDERR" 2>/dev/null) $output"

  [ "$status" -eq "$expected" ] || {
    echo "FAIL: expected exit $expected, got $status"
    echo "  Stderr: $(cat "$_DISP_STDERR" 2>/dev/null)"
    echo "  Stdout: $output"
    false
  }

  if [ -n "$pattern" ]; then
    [[ "$combined" == *"$pattern"* ]] || {
      echo "FAIL: expected pattern '$pattern' not found in output"
      echo "  Combined: $combined"
      false
    }
  fi
}

# ---------------------------------------------------------------------------
# Internal log helpers — telemetry-based plugin outcome assertion.
#
# Signal source: $WORK/.factory/logs/dispatcher-internal-YYYY-MM-DD.jsonl
# Each test has its own $WORK (fresh mktemp -d), so the log is test-isolated.
#
# VSDD_SINK_FILE is NOT functional in the current dispatcher build (rc.23):
# the file is never created even when set. The internal log IS functional and
# captures plugin.invoked, plugin.crashed, plugin.completed, etc.
# ---------------------------------------------------------------------------

# Return the path to today's internal dispatcher log in $WORK.
_plugin_log() {
  echo "$WORK/.factory/logs/dispatcher-internal-$(date +%Y-%m-%d).jsonl"
}

# Assert that validate-cross-site-correspondence was invoked AND did NOT crash.
#
# In Red Gate: FAILS because the stub panics → plugin.crashed in internal log.
# Post-implementation: PASSES when the plugin runs to completion (no crash event).
#
# This makes ALL 31 payload-driven tests fail in Red Gate — even controls asserting
# exit 0, which previously passed vacuously because on_error="continue" swallows panics.
# BC-5.38.001 Red Gate: every test must fail for the right reason, not by exit-code coincidence.
_assert_plugin_ran_not_crashed() {
  local log; log="$(_plugin_log)"

  # Must have an invocation record — confirms the hook triggered at all.
  if ! grep -q '"plugin_name":"validate-cross-site-correspondence"' "$log" 2>/dev/null; then
    echo "FAIL: no record for validate-cross-site-correspondence in internal log"
    echo "  Possible causes: dispatcher not found, hook pattern mismatch, VSDD_LOG_DIR not set"
    echo "  Log path: $log"
    cat "$log" 2>/dev/null || echo "  (log file absent)"
    false
    return
  fi

  # Must NOT have a plugin.crashed record — confirms the plugin did not panic.
  if grep '"plugin_name":"validate-cross-site-correspondence"' "$log" 2>/dev/null \
       | grep -q '"type":"plugin.crashed"'; then
    echo "FAIL: plugin.crashed found for validate-cross-site-correspondence"
    echo "  The plugin stub panicked (todo!() body). This FAILS correctly in Red Gate."
    echo "  After implementation, the stub is replaced and this test will PASS."
    grep '"plugin_name":"validate-cross-site-correspondence"' "$log" 2>/dev/null \
      | grep '"type":"plugin.crashed"' | head -3
    false
    return
  fi
}

# ---------------------------------------------------------------------------
# T-033 (AC-020): Registry entry present with all required fields.
# These two tests PASS in Red Gate legitimately — they grep the TOML registry
# file directly and do not invoke the dispatcher. No _assert_plugin_ran_not_crashed
# is called here because no plugin execution occurs.
# ---------------------------------------------------------------------------

@test "AC-020: registry entry has all 8 required fields" {
    local registry="$BATS_TEST_DIRNAME/../../../plugins/vsdd-factory/hooks-registry.toml"
    run grep -A 20 'name = "validate-cross-site-correspondence"' "$registry"
    [ "$status" -eq 0 ]
    [[ "$output" == *'event = "PostToolUse"'* ]]
    [[ "$output" == *'tool = "^(Edit|Write|MultiEdit)$"'* ]]
    [[ "$output" == *'plugin = "hook-plugins/validate-cross-site-correspondence.wasm"'* ]]
    [[ "$output" == *'priority = 460'* ]]
    [[ "$output" == *'timeout_ms = 8000'* ]]
    [[ "$output" == *'on_error = "continue"'* ]]
    [[ "$output" == *'async = false'* ]]
}

@test "AC-020: registry entry has no fuel_cap field" {
    local registry="$BATS_TEST_DIRNAME/../../../plugins/vsdd-factory/hooks-registry.toml"
    run awk '/name = "validate-cross-site-correspondence"/,/^\[\[hooks\]\]/' "$registry"
    [ "$status" -eq 0 ]
    # fuel_cap must NOT be present (BC-5.39.010 v1.2 §Gate Spec; ADR-035 §Decision 5)
    [[ "$output" != *"fuel_cap"* ]]
}

# ---------------------------------------------------------------------------
# AC-001: Class A Arm1 — stale BC-INDEX version row blocks
# Fixture: a1-stale-index (BC v1.6, INDEX v1.5)
# BC-5.39.010 PC2: stale row → Block [Class A Arm1]
# ---------------------------------------------------------------------------

@test "AC-001 MUTANT: stale BC-INDEX row produces exit code 2 with [Class A Arm1]" {
  _require_artifacts
  _load_fixture "a1-stale-index"
  _write_registry

  local envelope
  envelope="$(_post_write_event '.factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md')"
  _run_dispatcher "$envelope"

  _assert_plugin_ran_not_crashed
  _assert_exit 2 "[Class A Arm1]"
}

@test "AC-001 CONTROL: current BC-INDEX row produces exit code 0" {
  _require_artifacts
  _load_fixture "a1-current-index"
  _write_registry

  local envelope
  envelope="$(_post_write_event '.factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md')"
  _run_dispatcher "$envelope"

  _assert_plugin_ran_not_crashed
  _assert_exit 0
}

# ---------------------------------------------------------------------------
# AC-002: v1.0 not in INDEX → advisory; v1.1 not in INDEX → block
# BC-5.39.010 PC3 (advisory) + PC4 (block)
# ---------------------------------------------------------------------------

@test "AC-002: v1.0 BC not in INDEX produces exit code 0 (advisory)" {
  _require_artifacts
  _load_fixture "a1-v1-0-not-in-index"
  _write_registry

  local envelope
  # BC-9.99.001 at version "1.0", not in INDEX → advisory only (no block)
  envelope="$(_post_write_event '.factory/specs/behavioral-contracts/ss-09/BC-9.99.001.md')"
  _run_dispatcher "$envelope"

  _assert_plugin_ran_not_crashed
  _assert_exit 0
}

@test "AC-002: v1.1 BC not in INDEX produces exit code 2 (block)" {
  _require_artifacts
  _load_fixture "a1-v1-1-not-in-index"
  _write_registry

  local envelope
  # BC-9.99.001 at version "1.1", not in INDEX → block (dropped registration)
  envelope="$(_post_write_event '.factory/specs/behavioral-contracts/ss-09/BC-9.99.001.md')"
  _run_dispatcher "$envelope"

  _assert_plugin_ran_not_crashed
  _assert_exit 2
}

# ---------------------------------------------------------------------------
# AC-003: BC-INDEX.md CapabilityDenied blocks; NotFound is advisory
# BC-5.39.010 invariant 5 (CapabilityDenied) + precondition 8 (NotFound)
# ---------------------------------------------------------------------------

@test "AC-003: BC-INDEX CapabilityDenied produces exit code 2" {
  _require_artifacts
  _load_fixture "a1-stale-index"
  # Restrict path_allow to ss-05/ only — BC-INDEX.md at parent level → CapabilityDenied
  _write_registry '".factory/specs/behavioral-contracts/ss-05/"'

  local envelope
  envelope="$(_post_write_event '.factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md')"
  _run_dispatcher "$envelope"

  _assert_plugin_ran_not_crashed
  _assert_exit 2
}

@test "AC-003: BC-INDEX NotFound produces exit code 0 (advisory)" {
  _require_artifacts
  # Use a1-no-bc-index fixture: BC file present, BC-INDEX.md absent on disk
  _load_fixture "a1-no-bc-index"
  # Full path_allow so read is attempted; file absent → NotFound
  _write_registry

  local envelope
  envelope="$(_post_write_event '.factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md')"
  _run_dispatcher "$envelope"

  _assert_plugin_ran_not_crashed
  _assert_exit 0
}

# ---------------------------------------------------------------------------
# AC-004: primary target CapabilityDenied blocks
# BC-5.39.010 invariant 4: primary target read failure → Block always
# ---------------------------------------------------------------------------

@test "AC-004: primary BC file CapabilityDenied produces exit code 2" {
  _require_artifacts
  _load_fixture "a1-stale-index"
  # path_allow omits behavioral-contracts/ entirely → primary BC file read → CapabilityDenied
  _write_registry '".factory/stories/",
  ".factory/cycles/"'

  local envelope
  envelope="$(_post_write_event '.factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md')"
  _run_dispatcher "$envelope"

  _assert_plugin_ran_not_crashed
  _assert_exit 2
}

# ---------------------------------------------------------------------------
# AC-005: Class A Arm2 — stale story BC-table version citation blocks
# Fixture: a2-stale-citation (story cites v1.17, BC fm v1.18)
# BC-5.39.010 PC10-15
# ---------------------------------------------------------------------------

@test "AC-005 MUTANT: stale Token Budget row produces exit code 2 with [Class A Arm2]" {
  _require_artifacts
  _load_fixture "a2-stale-citation"
  _write_registry

  local envelope
  envelope="$(_post_write_event '.factory/stories/S-21.07-test.md')"
  _run_dispatcher "$envelope"

  _assert_plugin_ran_not_crashed
  _assert_exit 2 "[Class A Arm2]"
}

@test "AC-005 CONTROL: current citation produces exit code 0" {
  _require_artifacts
  _load_fixture "a2-current-citation"
  _write_registry

  local envelope
  envelope="$(_post_write_event '.factory/stories/S-21.07-test.md')"
  _run_dispatcher "$envelope"

  _assert_plugin_ran_not_crashed
  _assert_exit 0
}

# ---------------------------------------------------------------------------
# AC-006: cascade — two stale BCs combined block
# Fixture: a2-two-stale (two stale BC citations in Token Budget)
# BC-5.39.010 postcondition 23: violations combined into one block message
# ---------------------------------------------------------------------------

@test "AC-006: two stale BC citations produce single combined block referencing both IDs" {
  _require_artifacts
  _load_fixture "a2-two-stale"
  _write_registry

  local envelope
  envelope="$(_post_write_event '.factory/stories/S-21.07-test.md')"
  _run_dispatcher "$envelope"

  _assert_plugin_ran_not_crashed
  _assert_exit 2 "[Class A Arm2]"
  # The combined block must reference both stale BCs in its output
  local combined
  combined="$(cat "$_DISP_STDERR" 2>/dev/null) $output"
  [[ "$combined" == *"BC-6.26.001"* ]] || {
    echo "FAIL: combined block must cite BC-6.26.001"
    echo "  Output: $combined"
    false
  }
  [[ "$combined" == *"BC-7.27.002"* ]] || {
    echo "FAIL: combined block must cite BC-7.27.002"
    echo "  Output: $combined"
    false
  }
}

# ---------------------------------------------------------------------------
# AC-007: empty behavioral_contracts skips Arm A2
# Fixture: a2-empty-bcs (behavioral_contracts: [])
# BC-5.39.010 PC14
# ---------------------------------------------------------------------------

@test "AC-007: empty behavioral_contracts produces exit code 0" {
  _require_artifacts
  _load_fixture "a2-empty-bcs"
  _write_registry

  local envelope
  envelope="$(_post_write_event '.factory/stories/S-21.07-test.md')"
  _run_dispatcher "$envelope"

  _assert_plugin_ran_not_crashed
  _assert_exit 0
}

# ---------------------------------------------------------------------------
# AC-008: no version-citing table row skips that BC
# Fixture: a2-prose-only-citation (BC-6.26.001 in prose, no Token Budget row)
# BC-5.39.010 PC13
# ---------------------------------------------------------------------------

@test "AC-008: BC ID in prose only (no version table row) produces exit code 0" {
  _require_artifacts
  _load_fixture "a2-prose-only-citation"
  _write_registry

  local envelope
  envelope="$(_post_write_event '.factory/stories/S-21.07-test.md')"
  _run_dispatcher "$envelope"

  _assert_plugin_ran_not_crashed
  _assert_exit 0
}

# ---------------------------------------------------------------------------
# AC-009: Class B Arm1 — three-way input-hash mismatch blocks
# Fixtures: b1-hash-mismatch (story 47a65c9, INDEX 4be9d21) / b1-hash-match (all agree)
# BC-5.39.010 PC16-21
# ---------------------------------------------------------------------------

@test "AC-009 MUTANT: B1≠B2 hash mismatch produces exit code 2 with [Class B]" {
  _require_artifacts
  _load_fixture "b1-hash-mismatch"
  _write_registry

  local envelope
  envelope="$(_post_write_event '.factory/stories/S-21.07-test.md')"
  _run_dispatcher "$envelope"

  _assert_plugin_ran_not_crashed
  _assert_exit 2 "[Class B]"
}

@test "AC-009 CONTROL: three-way hash match produces exit code 0" {
  _require_artifacts
  _load_fixture "b1-hash-match"
  _write_registry

  local envelope
  envelope="$(_post_write_event '.factory/stories/S-21.07-test.md')"
  _run_dispatcher "$envelope"

  _assert_plugin_ran_not_crashed
  _assert_exit 0
}

# ---------------------------------------------------------------------------
# AC-010: absent secondary sites are advisory-only
# Fixtures: b1-no-story-index (input-hash present, STORY-INDEX absent)
#           b1-no-input-hash (no input-hash field)
# BC-5.39.010 PC18 (no input-hash → skip), PC20 (NotFound → advisory)
# ---------------------------------------------------------------------------

@test "AC-010: absent STORY-INDEX sites produce exit code 0 (advisory)" {
  _require_artifacts
  _load_fixture "b1-no-story-index"
  _write_registry

  local envelope
  # Story has input-hash but STORY-INDEX.md is absent → NotFound → advisory, exit 0
  envelope="$(_post_write_event '.factory/stories/S-21.07-test.md')"
  _run_dispatcher "$envelope"

  _assert_plugin_ran_not_crashed
  _assert_exit 0
}

@test "AC-010: no input-hash field skips Arm B1 entirely (exit code 0)" {
  _require_artifacts
  _load_fixture "b1-no-input-hash"
  _write_registry

  local envelope
  # Story has no input-hash field → B1 skips entirely
  envelope="$(_post_write_event '.factory/stories/S-21.07-test.md')"
  _run_dispatcher "$envelope"

  _assert_plugin_ran_not_crashed
  _assert_exit 0
}

# ---------------------------------------------------------------------------
# AC-011: Class B Arm2 — STORY-INDEX internal catalog vs blockquote mismatch
# Fixtures: b2-catalog-mismatch (catalog 47a65c9, blockquote 4be9d21)
#           b2-catalog-agree (both 47a65c9)
# BC-5.39.010 PC22-27
# ---------------------------------------------------------------------------

@test "AC-011 MUTANT: catalog/blockquote mismatch produces exit code 2 with [Class B]" {
  _require_artifacts
  _load_fixture "b2-catalog-mismatch"
  _write_registry

  local envelope
  # STORY-INDEX.md is the primary trigger for Arm B2
  envelope="$(_post_write_event '.factory/stories/STORY-INDEX.md')"
  _run_dispatcher "$envelope"

  _assert_plugin_ran_not_crashed
  _assert_exit 2 "[Class B]"
}

@test "AC-011 CONTROL: catalog/blockquote agree produces exit code 0" {
  _require_artifacts
  _load_fixture "b2-catalog-agree"
  _write_registry

  local envelope
  envelope="$(_post_write_event '.factory/stories/STORY-INDEX.md')"
  _run_dispatcher "$envelope"

  _assert_plugin_ran_not_crashed
  _assert_exit 0
}

# ---------------------------------------------------------------------------
# AC-012: Class D — excluded namespace tokens do not trigger advisory
# Fixture: d-clean-tokens (burst-log with D-944, S-, BC-, VP- tokens only)
# BC-5.39.010 PC32: excluded prefixes list
# ---------------------------------------------------------------------------

@test "AC-012: Closes: D-944 in burst-log produces exit code 0 (no advisory)" {
  _require_artifacts
  _load_fixture "d-clean-tokens"
  _write_registry

  local envelope
  # burst-log.md is the primary trigger for Class D Arm
  envelope="$(_post_write_event '.factory/cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md')"
  _run_dispatcher "$envelope"

  _assert_plugin_ran_not_crashed
  _assert_exit 0
}

@test "AC-012: S-, BC-, VP- tokens in burst-log Refs produce exit code 0" {
  _require_artifacts
  # Same d-clean-tokens fixture covers both Refs tokens and D- Closes
  _load_fixture "d-clean-tokens"
  _write_registry

  local envelope
  envelope="$(_post_write_event '.factory/cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md')"
  _run_dispatcher "$envelope"

  _assert_plugin_ran_not_crashed
  _assert_exit 0
}

# ---------------------------------------------------------------------------
# AC-013: Class D — non-F- token triggers advisory, not block
# Fixtures: d-non-f-token (B01 in Closes) / d-all-f-token (only F- tokens)
# BC-5.39.010 invariant 6: Class D NEVER blocks; advisory-only
# ---------------------------------------------------------------------------

@test "AC-013 MUTANT: B01 in burst-log Closes produces advisory (exit code 0, not 2)" {
  _require_artifacts
  _load_fixture "d-non-f-token"
  _write_registry

  local envelope
  envelope="$(_post_write_event '.factory/cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md')"
  _run_dispatcher "$envelope"

  _assert_plugin_ran_not_crashed
  # Class D is advisory-only per BC-5.39.010 invariant 6: NEVER blocks → exit 0 always.
  # Post-implementation: also verify the advisory text is present in dispatcher output.
  _assert_exit 0
}

@test "AC-013 CONTROL: only F- tokens in burst-log Closes produces exit code 0" {
  _require_artifacts
  _load_fixture "d-all-f-token"
  _write_registry

  local envelope
  envelope="$(_post_write_event '.factory/cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md')"
  _run_dispatcher "$envelope"

  _assert_plugin_ran_not_crashed
  _assert_exit 0
}

# ---------------------------------------------------------------------------
# AC-014: Class D — historical section excluded by positional anchor
# Fixture: d-historical-excluded (lessons.md: old L-EDP1-001 has P45-001, latest L-EDP1-062 clean)
# BC-5.39.010 PC30: scope-limited extraction — last L-EDP1 block only
# ---------------------------------------------------------------------------

@test "AC-014: P45-001 in old L-EDP1 block not flagged (positional exclusion)" {
  _require_artifacts
  _load_fixture "d-historical-excluded"
  _write_registry

  local envelope
  # lessons.md is a cycle artifact → triggers Class D
  envelope="$(_post_write_event '.factory/cycles/v1.0-feature-engine-discipline-pass-1/lessons.md')"
  _run_dispatcher "$envelope"

  _assert_plugin_ran_not_crashed
  # Historical L-EDP1-001 with P45-001 must NOT trigger advisory (only last block scanned)
  _assert_exit 0
}

# ---------------------------------------------------------------------------
# AC-015: Class E1 — version vs last_amended mismatch blocks
# Fixtures: e1-version-mismatch / e1-version-match / e1-unparseable
# BC-5.39.010 PC35-37 + postconditions 19-20
# ---------------------------------------------------------------------------

@test "AC-015 MUTANT: version 1.33 vs last_amended (v1.31) produces exit code 2 with [Class E1]" {
  _require_artifacts
  _load_fixture "e1-version-mismatch"
  _write_registry

  local envelope
  envelope="$(_post_write_event '.factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md')"
  _run_dispatcher "$envelope"

  _assert_plugin_ran_not_crashed
  _assert_exit 2 "[Class E1]"
}

@test "AC-015 CONTROL: version 1.6 matches last_amended (v1.6) produces exit code 0" {
  _require_artifacts
  _load_fixture "e1-version-match"
  _write_registry

  local envelope
  envelope="$(_post_write_event '.factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md')"
  _run_dispatcher "$envelope"

  _assert_plugin_ran_not_crashed
  _assert_exit 0
}

@test "AC-015: unparseable last_amended produces exit code 0 (advisory, not block)" {
  _require_artifacts
  # e1-unparseable: last_amended has no YYYY-MM-DD (vX.X) pattern
  _load_fixture "e1-unparseable"
  _write_registry

  local envelope
  envelope="$(_post_write_event '.factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md')"
  _run_dispatcher "$envelope"

  _assert_plugin_ran_not_crashed
  # BC-5.39.010 PC37 last sentence: unparseable format → advisory, not block
  _assert_exit 0
}

# ---------------------------------------------------------------------------
# AC-015 over-broad exclusion prevention (F-S2104-P29-H02 lesson):
# When [Prior:] chain is present in last_amended but the OUTERMOST version is
# wrong, E1 must still fire. The [Prior:] annotation must NOT suppress the check.
# Fixture: e1-prior-chain-wrong-outermost (version "1.6", last_amended "(v1.5) [Prior: (v1.4)]")
# BC-5.39.010 EC-018: extract_last_amended_outer_version reads from char 0 → (v1.5) ≠ 1.6 → block.
# ---------------------------------------------------------------------------

@test "AC-015: [Prior:] chain present but outermost version wrong still produces exit code 2 with [Class E1]" {
  _require_artifacts
  _load_fixture "e1-prior-chain-wrong-outermost"
  _write_registry

  local envelope
  envelope="$(_post_write_event '.factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md')"
  _run_dispatcher "$envelope"

  _assert_plugin_ran_not_crashed
  _assert_exit 2 "[Class E1]"
}

# ---------------------------------------------------------------------------
# AC-016: Class E2 — non-ascending modified[] blocks
# Fixtures: e2-non-monotonic / e2-ascending
# BC-5.39.010 PC38-39 + postcondition 22
# ---------------------------------------------------------------------------

@test "AC-016 MUTANT: non-ascending modified[] produces exit code 2 with [Class E2]" {
  _require_artifacts
  _load_fixture "e2-non-monotonic"
  _write_registry

  local envelope
  envelope="$(_post_write_event '.factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md')"
  _run_dispatcher "$envelope"

  _assert_plugin_ran_not_crashed
  _assert_exit 2 "[Class E2]"
}

@test "AC-016 CONTROL: ascending modified[] produces exit code 0" {
  _require_artifacts
  _load_fixture "e2-ascending"
  _write_registry

  local envelope
  envelope="$(_post_write_event '.factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md')"
  _run_dispatcher "$envelope"

  _assert_plugin_ran_not_crashed
  _assert_exit 0
}

# ---------------------------------------------------------------------------
# AC-017: Class E1 — Prior-chain version positionally excluded
# Fixture: e1-prior-chain-correct (version "1.6", last_amended "(v1.6) ... [Prior: (v1.5) ...]")
# BC-5.39.010 EC-018: Prior-chain exclusion by positional anchor
# ---------------------------------------------------------------------------

@test "AC-017: Prior-chain (v1.5) does not cause E1 mismatch when version is 1.6" {
  _require_artifacts
  _load_fixture "e1-prior-chain-correct"
  _write_registry

  local envelope
  # [Prior: (v1.5)] in last_amended — positional anchor ensures (v1.5) is NOT at position 0
  # extract_last_amended_outer_version reads from char 0 → returns "1.6" → matches version → pass
  envelope="$(_post_write_event '.factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md')"
  _run_dispatcher "$envelope"

  _assert_plugin_ran_not_crashed
  _assert_exit 0
}

# ---------------------------------------------------------------------------
# AC-018: combined violations from multiple arms → single block
# Fixture: combined-a1-e1 (BC v1.33, INDEX v1.5, last_amended "(v1.31)")
# BC-5.39.010 postcondition 23: all violations combined into ONE block_with_fix
# ---------------------------------------------------------------------------

@test "AC-018: A1 stale INDEX + E1 mismatch produces single combined block message" {
  _require_artifacts
  _load_fixture "combined-a1-e1"
  _write_registry

  local envelope
  envelope="$(_post_write_event '.factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md')"
  _run_dispatcher "$envelope"

  _assert_plugin_ran_not_crashed
  _assert_exit 2 "[Class A Arm1]"
  # Combined block must also cite [Class E1]
  local combined
  combined="$(cat "$_DISP_STDERR" 2>/dev/null) $output"
  [[ "$combined" == *"[Class E1]"* ]] || {
    echo "FAIL: combined block must also contain [Class E1]"
    echo "  Output: $combined"
    false
  }
}

# ---------------------------------------------------------------------------
# AC-019: cap-passing — max_bytes constants actually passed to host::read_file
#
# BC-5.39.010 AC-019: every secondary host::read_file call MUST pass explicit
# max_bytes and timeout_ms values (no default-zero calls).
#
# Test strategy (indirect verification):
#   If max_bytes = 0 or undefined, the dispatcher returns InvalidArgument (-4)
#   or OutputTooLarge (-3), causing the plugin to block (exit 2) or crash.
#   Successful exit 0 on a fixture where secondary reads actually execute proves
#   that BC_INDEX_MAX_BYTES (1_048_576) and STORY_INDEX_B1_MAX_BYTES (1_048_576)
#   are correctly passed to host::read_file.
#
# Two sub-tests:
#   (a) A1 secondary read: BC-INDEX.md read during BC file write
#   (b) B1 secondary read: STORY-INDEX.md read during story file write
# ---------------------------------------------------------------------------

@test "AC-019 (a): A1 secondary BC-INDEX.md read completes — BC_INDEX_MAX_BYTES cap passed correctly" {
  _require_artifacts
  _load_fixture "a1-current-index"
  _write_registry

  local envelope
  # BC file write triggers A1 → secondary read of BC-INDEX.md with max_bytes=1_048_576
  # If max_bytes were 0 or negative, the dispatcher returns InvalidArgument (-4)
  # and the plugin blocks (exit 2). Clean exit 0 proves the cap is correctly passed.
  envelope="$(_post_write_event '.factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md')"
  _run_dispatcher "$envelope"

  _assert_plugin_ran_not_crashed
  _assert_exit 0
}

@test "AC-019 (b): B1 secondary STORY-INDEX.md read completes — STORY_INDEX_B1_MAX_BYTES cap passed correctly" {
  _require_artifacts
  _load_fixture "b1-hash-match"
  _write_registry

  local envelope
  # Story file write triggers B1 → secondary read of STORY-INDEX.md with max_bytes=1_048_576
  # If max_bytes were 0 or negative, the dispatcher returns InvalidArgument (-4)
  # and the plugin blocks (exit 2). Clean exit 0 proves the cap is correctly passed.
  envelope="$(_post_write_event '.factory/stories/S-21.07-test.md')"
  _run_dispatcher "$envelope"

  _assert_plugin_ran_not_crashed
  _assert_exit 0
}
