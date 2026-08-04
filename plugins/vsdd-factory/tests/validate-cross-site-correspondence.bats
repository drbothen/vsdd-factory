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
#   (a) a plugin.completed record exists for validate-cross-site-correspondence, AND
#   (b) no plugin.crashed OR plugin.timeout record exists for it.
# Under the stub, condition (a) always fails (stub panics → plugin.crashed, not plugin.completed).
# → ALL 37+ payload tests FAIL in Red Gate.
# After implementation, the stub is replaced with real logic, plugin.completed fires,
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
# In Red Gate: FAILS because the stub panics → plugin.crashed (not plugin.completed) in log.
# Post-implementation: PASSES when the plugin runs to completion (plugin.completed fires).
#
# RG-006 (T-036): This helper is itself tested by a permanent mutant below that injects
# a plugin.crashed-only log and asserts the helper fails. See T-036 comment block.
#
# This makes ALL 37+ payload-driven tests fail in Red Gate — even controls asserting
# exit 0, which previously passed vacuously because on_error="continue" swallows panics.
# BC-5.38.001 Red Gate: every test must fail for the right reason, not by exit-code coincidence.
_assert_plugin_ran_not_crashed() {
  local log; log="$(_plugin_log)"

  # Must have a plugin.completed record — confirms execution ran to completion.
  # plugin.invoked is written BEFORE execution starts; only plugin.completed proves
  # the plugin ran its full code path without crashing or timing out.
  if ! grep '"plugin_name":"validate-cross-site-correspondence"' "$log" 2>/dev/null \
       | grep -q '"type":"plugin.completed"'; then
    echo "FAIL: no plugin.completed record for validate-cross-site-correspondence"
    echo "  Plugin never ran, crashed before completion, or timed out."
    echo "  Log path: $log"
    grep '"plugin_name":"validate-cross-site-correspondence"' "$log" 2>/dev/null | head -5 \
      || echo "  (no matching records in log)"
    false
    return
  fi

  # Must NOT have a plugin.crashed record — confirms the plugin did not panic.
  if grep '"plugin_name":"validate-cross-site-correspondence"' "$log" 2>/dev/null \
       | grep -q '"type":"plugin.crashed"'; then
    echo "FAIL: plugin.crashed found for validate-cross-site-correspondence"
    echo "  The plugin panicked (todo!() body). Correct Red Gate failure."
    grep '"plugin_name":"validate-cross-site-correspondence"' "$log" 2>/dev/null \
      | grep '"type":"plugin.crashed"' | head -3
    false
    return
  fi

  # Must NOT have a plugin.timeout record — confirms the plugin did not exceed budget.
  if grep '"plugin_name":"validate-cross-site-correspondence"' "$log" 2>/dev/null \
       | grep -q '"type":"plugin.timeout"'; then
    echo "FAIL: plugin.timeout found for validate-cross-site-correspondence"
    echo "  Plugin exceeded its time budget — check WASM fuel or timeout_ms setting."
    grep '"plugin_name":"validate-cross-site-correspondence"' "$log" 2>/dev/null \
      | grep '"type":"plugin.timeout"' | head -3
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
    # Guard: entry must exist — empty output means the section was never found,
    # which would make the fuel_cap absence check vacuously true.
    [ -n "$output" ] || {
      echo "FAIL: no 'validate-cross-site-correspondence' section found in registry"
      false
    }
    # fuel_cap must NOT be present (BC-5.39.010 v1.2 §Gate Spec; ADR-035 §Decision 5)
    [[ "$output" != *"fuel_cap"* ]]
}

# ---------------------------------------------------------------------------
# T-036 (RG-006) PERMANENT MUTANT: _assert_plugin_ran_not_crashed self-test.
#
# This test validates the helper function itself — not a plugin execution.
# It ALWAYS PASSES (both in Red Gate and after implementation) because it tests
# that the helper correctly detects a plugin.crashed-only log as a failure.
#
# RG-006: without this mutant, a bug that makes _assert_plugin_ran_not_crashed
# return success on a crashed-only log would silently invalidate ALL 37+ tests.
# The helper must fail when:
#   (a) only plugin.crashed record exists (no plugin.completed), AND
#   (b) only plugin.timeout record exists (no plugin.completed).
#
# ALWAYS PASSES: This is a meta-test of the Red Gate mechanism, not of plugin
# behavior. It passes both in Red Gate and post-implementation.
# ---------------------------------------------------------------------------

@test "T-036 RG-006 PERMANENT MUTANT: _assert_plugin_ran_not_crashed fails on crashed-only log" {
  # Create a fake internal log with ONLY plugin.crashed (no plugin.completed)
  local fake_log
  fake_log="$WORK/.factory/logs/dispatcher-internal-$(date +%Y-%m-%d).jsonl"
  mkdir -p "$(dirname "$fake_log")"
  printf '{"type":"plugin.crashed","plugin_name":"validate-cross-site-correspondence","message":"todo!() panic"}\n' > "$fake_log"

  # _assert_plugin_ran_not_crashed must FAIL with this log
  # (no plugin.completed record → first check fails)
  if _assert_plugin_ran_not_crashed 2>/dev/null; then
    echo "FAIL: _assert_plugin_ran_not_crashed must fail when only plugin.crashed in log"
    echo "  (no plugin.completed record found — stub panic not detected)"
    false
  fi
  # Test passes: helper correctly returned failure for crashed-only log
}

@test "T-036b RG-006 PERMANENT MUTANT: _assert_plugin_ran_not_crashed fails on timeout-only log" {
  # Create a fake internal log with ONLY plugin.timeout (no plugin.completed)
  local fake_log
  fake_log="$WORK/.factory/logs/dispatcher-internal-$(date +%Y-%m-%d).jsonl"
  mkdir -p "$(dirname "$fake_log")"
  printf '{"type":"plugin.timeout","plugin_name":"validate-cross-site-correspondence","message":"exceeded budget"}\n' > "$fake_log"

  # _assert_plugin_ran_not_crashed must FAIL (no plugin.completed AND has timeout record)
  if _assert_plugin_ran_not_crashed 2>/dev/null; then
    echo "FAIL: _assert_plugin_ran_not_crashed must fail when only plugin.timeout in log"
    false
  fi
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
# T-039: Class A Arm1 — escaped-pipe version chain CONTROL
# Fixture: a1-escaped-pipe-current (BC-1.13.001 v1.12; INDEX row v1.3 \| ... \| v1.12)
# F-S2107-P1B-006: escaped-pipe chain → extract_bc_index_version returns FIRST token "1.3"
# F-S2107-P1B-007: frontmatter changelog line with pipe also false-matches before body row
# BC-5.39.010 v1.3 invariant 10: LAST version token in escaped-pipe chain is authoritative.
# ---------------------------------------------------------------------------

@test "T-039 CONTROL: escaped-pipe version chain last token matches — exit code 0" {
  _require_artifacts
  _load_fixture "a1-escaped-pipe-current"
  _write_registry

  local envelope
  # BC-1.13.001 at v1.12; INDEX row v1.3 \| ... \| v1.12
  # After fix: last token "1.12" matched → no violation → exit 0
  # F-S2107-P1B-006 RED GATE: split('|') returns first token "1.3" → "1.3"≠"1.12" → exit 2 → FAILS
  # F-S2107-P1B-007 RED GATE: frontmatter changelog line matches first → "4.43"≠"1.12" → exit 2 → FAILS
  envelope="$(_post_write_event '.factory/specs/behavioral-contracts/ss-01/BC-1.13.001.md')"
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
# T-037: Class B — B1=B2 agree but B3 mismatch blocks (B3-only mismatch)
# Fixture: b1-b3-only-mismatch (B1=47a65c9, B2=47a65c9, B3=deadbee)
# F-S2107-P1B-003: parse_story_index_blockquote_hash uses starts_with("> S-21.07=")
#   → production prose blockquote shape never matches → B3=None → inert (no block)
# BC-5.39.010 v1.3 invariant 11: B3≠B1 must block (blockquote hash mismatch).
# ---------------------------------------------------------------------------

@test "T-037 MUTANT: B1=B2 agree but B3 mismatch in production blockquote blocks" {
  _require_artifacts
  _load_fixture "b1-b3-only-mismatch"
  _write_registry

  local envelope
  # story frontmatter B1=47a65c9, STORY-INDEX catalog B2=47a65c9, blockquote B3=deadbee
  # After fix: B3 extracted → B3≠B1 → exit 2 [Class B]
  # F-S2107-P1B-003 RED GATE: blockquote parser inert on production shape → B3=None
  # → no three-way comparison → exit 0 → test expects 2 → FAILS
  envelope="$(_post_write_event '.factory/stories/S-21.07-test.md')"
  _run_dispatcher "$envelope"

  _assert_plugin_ran_not_crashed
  _assert_exit 2 "[Class B]"
}

# ---------------------------------------------------------------------------
# T-038: Class B — cross-story catalog row correct lookup
# Fixture: b1-cross-story-catalog (S-18.00 row mentions S-18.01 in blocks column)
# F-S2107-P1B-008: parse_story_index_catalog_hash naive contains(story_id) matches
#   S-18.00 row first (because it mentions "S-18.01" in blocks/depends column)
# BC-5.39.010 v1.3 PC16: catalog lookup must return hash from CANONICAL story row only.
# ---------------------------------------------------------------------------

@test "T-038 CONTROL: cross-story catalog row lookup returns own-story hash (exit code 0)" {
  _require_artifacts
  _load_fixture "b1-cross-story-catalog"
  _write_registry

  local envelope
  # S-18.01-test.md B1=1b4ea21; STORY-INDEX: S-18.00 row mentions S-18.01 (hash e5bc551)
  # before S-18.01 row (hash 1b4ea21). After fix: S-18.01 row matched → B2=1b4ea21=B1 → exit 0
  # F-S2107-P1B-008 RED GATE: naive contains("S-18.01") hits S-18.00 row → "e5bc551"≠"1b4ea21" → exit 2
  envelope="$(_post_write_event '.factory/stories/S-18.01-test.md')"
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
  # [DEFERRED v1.6 — Class D] BC-5.39.010 v1.6 defers Class D entirely.
  # is_cycle_artifact() will return None after Class D removal; burst-log.md writes
  # become unclassified → Continue (no Class D advisory arm runs).
  # Restore this test when Class D is re-implemented in a future wave.
  skip "[DEFERRED v1.6 — Class D] burst-log.md unclassified after Class D arm removal; test preserved per POLICY 1 for future re-activation"
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

@test "AC-012: excluded-namespace tokens produce exit code 0 AND no advisory in log" {
  # [DEFERRED v1.6 — Class D] BC-5.39.010 v1.6 defers Class D entirely.
  # Class D advisory arm removed; burst-log.md writes are unclassified → Continue.
  # Restore this test when Class D is re-implemented in a future wave.
  skip "[DEFERRED v1.6 — Class D] burst-log.md unclassified after Class D arm removal; test preserved per POLICY 1 for future re-activation"
  # Strengthened: also verifies NO advisory is emitted for excluded-namespace tokens.
  # First AC-012 tests only exit code; this test additionally checks the dispatcher log
  # to confirm no spurious `plugin.log warn` record was emitted.
  # F-S2107-P1C-020 overlap: if "discloses:" triggers advisory for excluded tokens,
  # this test would catch it because advisories would appear in the log.
  _require_artifacts
  _load_fixture "d-clean-tokens"
  _write_registry

  local envelope
  envelope="$(_post_write_event '.factory/cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md')"
  _run_dispatcher "$envelope"

  _assert_plugin_ran_not_crashed
  _assert_exit 0

  # No advisory must be emitted for excluded-namespace tokens (D-, S-, BC-, VP-, etc.)
  local log; log="$(_plugin_log)"
  if grep '"plugin_name":"validate-cross-site-correspondence"' "$log" 2>/dev/null \
       | grep '"type":"plugin.log"' | grep -q '"level":"warn"'; then
    echo "FAIL: unexpected advisory emitted for excluded-namespace tokens in Closes/Refs"
    echo "  All tokens in d-clean-tokens are excluded-namespace; no advisory expected."
    grep '"plugin_name":"validate-cross-site-correspondence"' "$log" \
      | grep '"type":"plugin.log"' | grep '"level":"warn"' | head -3
    false
  fi
}

# ---------------------------------------------------------------------------
# AC-013: Class D — non-F- token triggers advisory, not block
# Fixtures: d-non-f-token (B01 in Closes) / d-all-f-token (only F- tokens)
# BC-5.39.010 invariant 6: Class D NEVER blocks; advisory-only
# ---------------------------------------------------------------------------

@test "AC-013 MUTANT: B01 in burst-log Closes produces advisory in log (exit code 0, not 2)" {
  # [DEFERRED v1.6 — Class D] BC-5.39.010 v1.6 defers Class D entirely.
  # Class D advisory arm removed; burst-log.md writes are unclassified → Continue.
  # Restore when Class D is re-implemented.
  skip "[DEFERRED v1.6 — Class D] burst-log.md unclassified after Class D arm removal; test preserved per POLICY 1 for future re-activation"
  _require_artifacts
  _load_fixture "d-non-f-token"
  _write_registry

  local envelope
  envelope="$(_post_write_event '.factory/cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md')"
  _run_dispatcher "$envelope"

  _assert_plugin_ran_not_crashed
  # Class D is advisory-only per BC-5.39.010 invariant 6: NEVER blocks → exit 0 always.
  _assert_exit 0

  # Advisory for "B01" must be present in the dispatcher internal log.
  # BC-5.39.010 PC33: advisory must be emitted for non-F- tokens; message cites the token.
  # F-S2107-P1C-020 overlap guard: if "closes:" false-matches for excluded content, no advisory
  # would fire for B01 → this assertion would catch the wrong behavior.
  # RED GATE: stub panics → _assert_plugin_ran_not_crashed fails before this check.
  # Post-stub: advisory must appear in log as plugin.log warn record mentioning "B01".
  local log; log="$(_plugin_log)"
  grep '"plugin_name":"validate-cross-site-correspondence"' "$log" 2>/dev/null \
    | grep '"type":"plugin.log"' \
    | grep '"level":"warn"' \
    | grep -q 'B01' || {
      echo "FAIL: expected advisory mentioning 'B01' not found in dispatcher log"
      echo "  BC-5.39.010 invariant 6: Class D must emit advisory for non-F- token B01"
      grep '"plugin_name":"validate-cross-site-correspondence"' "$log" \
        | grep '"type":"plugin.log"' | head -5 || echo "  (no plugin.log records)"
      false
    }
}

@test "AC-013 CONTROL: only F- tokens in burst-log Closes produces exit code 0" {
  # [DEFERRED v1.6 — Class D] BC-5.39.010 v1.6 defers Class D entirely.
  # Restore when Class D is re-implemented.
  skip "[DEFERRED v1.6 — Class D] burst-log.md unclassified after Class D arm removal; test preserved per POLICY 1 for future re-activation"
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
  # [DEFERRED v1.6 — Class D] BC-5.39.010 v1.6 defers Class D entirely.
  # is_cycle_artifact() returns None after removal; lessons.md writes are unclassified → Continue.
  # Restore when Class D is re-implemented.
  skip "[DEFERRED v1.6 — Class D] lessons.md unclassified after Class D arm removal; test preserved per POLICY 1 for future re-activation"
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
# T-035: Class A Arm1 — BC-INDEX.md must NOT be classified as a BC file
# Fixture: a1-current-index (reuse — BC-INDEX.md exists in this fixture)
# F-S2107-P1B-005: dispatch::is_bc_file uses starts_with("BC-") && ends_with(".md")
#   → "BC-INDEX.md" matches → A1 runs with bc_id="BC-INDEX" → spurious violations.
# BC-5.39.010 v1.3 §Classification invariant: index files excluded from BC classification.
# ---------------------------------------------------------------------------

@test "T-035 CONTROL: BC-INDEX.md write event produces exit code 0 (not classified as BC file)" {
  _require_artifacts
  _load_fixture "a1-current-index"
  _write_registry

  local envelope
  # Writing BC-INDEX.md must NOT trigger Arm A1 (not a behavioral contract file).
  # After fix: dispatch skips BC-INDEX.md → Continue → exit 0
  # F-S2107-P1B-005 RED GATE: is_bc_file returns true for BC-INDEX.md → A1 runs →
  #   bc_id="BC-INDEX", bc_version=index-version → no row for "BC-INDEX" or wrong version
  #   → exit 2 → FAILS
  envelope="$(_post_write_event '.factory/specs/behavioral-contracts/BC-INDEX.md')"
  _run_dispatcher "$envelope"

  _assert_plugin_ran_not_crashed
  _assert_exit 0
}

# ---------------------------------------------------------------------------
# T-038: Class B Arm1 — cross-story catalog row correct lookup
# Fixture: b1-cross-story-catalog (S-18.00 row mentions S-18.01 in blocks column)
# F-S2107-P1B-008: parse_story_index_catalog_hash naive contains("S-18.01") matches
#   S-18.00 row first (mentions "S-18.01" in blocks/depends column; hash e5bc551)
#   → "e5bc551" ≠ S-18.01 story frontmatter "1b4ea21" → spurious exit 2.
# BC-5.39.010 v1.3 PC16: catalog lookup must match CANONICAL story row (first cell).
# ---------------------------------------------------------------------------

@test "T-038 CONTROL: cross-story catalog lookup returns own-story hash (exit code 0)" {
  _require_artifacts
  _load_fixture "b1-cross-story-catalog"
  _write_registry

  local envelope
  # S-18.01-test.md B1=1b4ea21; S-18.00 row mentions "S-18.01" in blocks column (hash e5bc551)
  # S-18.01 catalog row hash = 1b4ea21. After fix: S-18.01 row matched → B2=1b4ea21=B1 → exit 0
  # F-S2107-P1B-008 RED GATE: naive contains("S-18.01") hits S-18.00 first → "e5bc551"≠"1b4ea21" → exit 2
  envelope="$(_post_write_event '.factory/stories/S-18.01-test.md')"
  _run_dispatcher "$envelope"

  _assert_plugin_ran_not_crashed
  _assert_exit 0
}

# ---------------------------------------------------------------------------
# T-045: Class E1 — 15-byte last_amended format accepted (no spurious advisory)
# Fixture: e1-15-byte-last-amended (VP-9999-test; version "2"; last_amended "2026-07-30 (v2)")
# Isolation strategy: VP file write is used (NOT a BC file write) because VP files do
# NOT trigger Arm A1 (A1 is BC-file-only). Arm A1 can't parse single-integer versions
# like "v2" in BC-INDEX (extract_version_token requires vN.N format). Using a VP file
# isolates E1 so the test is genuinely discriminating.
# F-S2107-P1C-014: extract_last_amended_outer_version `if len < 17 { return None }`
#   → 15-byte "2026-07-30 (v2)" → None → advisory "unparseable format" fires.
# BC-5.39.010 v1.3 §E1: YYYY-MM-DD (vN) with single-digit outer version is valid.
# Fixture renamed: VP-039.md → VP-9999-test.md (VP-039 is a live corpus ID; F-P2-corpus-hygiene).
# ---------------------------------------------------------------------------

@test "T-045 CONTROL: 15-byte last_amended 2026-07-30 (v2) produces no advisory (exit 0)" {
  _require_artifacts
  _load_fixture "e1-15-byte-last-amended"
  _write_registry

  local envelope
  # VP-9999-test; version "2"; last_amended "2026-07-30 (v2)" = 15 bytes.
  # VP files skip Arm A1 entirely — only Class E runs. This isolates the 15-byte E1 check.
  # After fix: len threshold lowered → Some("2") extracted → "2"=="2" → no E1 advisory → exit 0
  # F-S2107-P1C-014 RED GATE: len < 17 → None → advisory fires → log has warn record → FAILS
  # Fixture uses VP-9999-test.md (non-live ID) to avoid corpus-churn pollution.
  envelope="$(_post_write_event '.factory/specs/verification-properties/VP-9999-test.md')"
  _run_dispatcher "$envelope"

  _assert_plugin_ran_not_crashed
  _assert_exit 0

  # No Class E1 advisory must appear in the dispatcher log.
  # The 15-byte format is valid; no "unparseable" advisory expected.
  local log; log="$(_plugin_log)"
  if grep '"plugin_name":"validate-cross-site-correspondence"' "$log" 2>/dev/null \
       | grep '"type":"plugin.log"' | grep '"level":"warn"' | grep -qi 'last.amended\|unparseable\|unrecognized'; then
    echo "FAIL: unexpected E1 advisory about last_amended format for valid 15-byte string"
    echo "  '2026-07-30 (v2)' is a valid format per BC-5.39.010 v1.3 §E1 (F-S2107-P1C-014)"
    grep '"plugin_name":"validate-cross-site-correspondence"' "$log" \
      | grep '"type":"plugin.log"' | grep '"level":"warn"' | head -3
    false
  fi
}

# ---------------------------------------------------------------------------
# T-046: Class E1 positive-coverage mutant — VP file version/last_amended mismatch
# Fixture: e1-vp-version-mismatch (VP-9999-test; version "1.7"; last_amended "2026-07-30 (v1.6)")
# F-P2-013: T-045 only asserts exit 0 + absence of warn advisory. A mutation where
# Class E never ran (or E1 was disabled) would still pass T-045. T-046 adds a MUTANT
# that asserts exit 2 when there IS a version/last_amended mismatch.
# Isolation: VP file → no Arm A1 trigger → pure Class E1 isolation.
# RED GATE: before fix (Class E1 not yet implemented): plugin.crashed → T-046 FAILS.
# Post-fix: E1 detects "1.7" ≠ "1.6" → violation → exit 2 → PASSES.
# ---------------------------------------------------------------------------

@test "T-046 MUTANT (F-P2-013): VP file version/last_amended mismatch produces exit 2" {
  _require_artifacts
  _load_fixture "e1-vp-version-mismatch"
  _write_registry

  local envelope
  # VP-9999-test: version "1.7", last_amended "2026-07-30 (v1.6)".
  # extract_last_amended_outer_version("2026-07-30 (v1.6)") → "1.6".
  # "1.7" ≠ "1.6" → Class E1 violation → exit 2.
  # VP file → no Arm A1 (A1 is BC-file-only). Class E1 runs in isolation.
  envelope="$(_post_write_event '.factory/specs/verification-properties/VP-9999-test.md')"
  _run_dispatcher "$envelope"

  _assert_plugin_ran_not_crashed
  _assert_exit 2 "[Class E1]"
}

# ---------------------------------------------------------------------------
# T-047: PC40 — story with volatile input-hash inputs emits advisory, exits 0
# Fixture: b1-volatile-input (S-21.07-test: input-hash + inputs: [.factory/STATE.md];
#   STORY-INDEX.md with MISMATCHED hash to prove PC40 suppresses the comparison)
# BC-5.39.010 v1.6 PC40: volatile inputs → advisory + Continue, NOT three-way comparison.
# RED GATE: without PC40 fix, arm B1 runs three-way comparison → finds mismatch → exit 2.
# Post-fix: volatile inputs detected → advisory → exit 0.
# ---------------------------------------------------------------------------

@test "T-047 (PC40): story with volatile input path emits advisory and exits 0 not 2" {
  _require_artifacts
  _load_fixture "b1-volatile-input"
  _write_registry

  local envelope
  # S-21.07-test.md: input-hash abc123; inputs: [".factory/STATE.md"] (volatile).
  # STORY-INDEX.md: S-21.07 catalog row has hash xyz789 (≠ abc123) — deliberate mismatch
  # to prove PC40 suppresses the comparison (without PC40, this would produce exit 2).
  envelope="$(_post_write_event '.factory/stories/S-21.07-test.md')"
  _run_dispatcher "$envelope"

  _assert_plugin_ran_not_crashed
  # PC40: volatile input path → advisory + Continue, NOT block.
  # Without fix: three-way comparison runs → B1≠B2 → exit 2. With fix: exit 0.
  _assert_exit 0

  # Advisory mentioning volatile must be present in the dispatcher log.
  local log; log="$(_plugin_log)"
  grep '"plugin_name":"validate-cross-site-correspondence"' "$log" 2>/dev/null \
    | grep '"type":"plugin.log"' \
    | grep '"level":"warn"' \
    | grep -qi 'volatile' || {
      echo "FAIL: expected advisory mentioning 'volatile' not found in dispatcher log"
      echo "  PC40: story with volatile inputs must emit advisory + Continue (BC-5.39.010 v1.6)"
      grep '"plugin_name":"validate-cross-site-correspondence"' "$log" \
        | grep '"type":"plugin.log"' | head -5 || echo "  (no plugin.log records)"
      false
    }
}
