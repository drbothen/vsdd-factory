#!/usr/bin/env bats
# pass-real-state-md-snapshot.bats — F-P2-002: full-surface integration test against
#                                     the LIVE .factory/STATE.md (auto-copied at run time).
#
# Traces to:
#   BC-5.39.005 postcondition 1 (all properties hold => HookResult::Continue)
#   F-P2-001: tighter trajectory predicate (canonical-tail discriminator)
#   F-P2-002: real STATE.md exercises the FULL validator surface
#   F-P3-002: snapshot-vs-live drift class structurally eliminated (auto-copy)
#
# Setup: setup() copies REPO_ROOT/.factory/STATE.md into the test work directory
#        at run time — the test always exercises the CURRENT live content.
#        The frozen fixture at fixtures/validate-state-structure/pass-real-state-md-snapshot/
#        is retained as a documentation reference for the pass-2 fix-burst baseline.
# Expected: hook exits 0 (Continue — no block).
#
# This is the LOAD-BEARING bats evidence that F-P2-001 + F-P2-002 are closed:
# the real STATE.md banner contains `(363→310 lines)` (1 component, non-adjacent)
# and tracker lines with spread arrows — neither is a false-positive trajectory tail.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-state-structure.wasm"
  WORK="$(mktemp -d)"
  mkdir -p "$WORK/hook-plugins"
  mkdir -p "$WORK/.factory/logs"
  # F-P3-002: auto-copy LIVE STATE.md at run time — eliminates snapshot-vs-live drift class.
  # The test always exercises current .factory/STATE.md content rather than a frozen fixture.
  cp "$REPO_ROOT/.factory/STATE.md" "$WORK/.factory/STATE.md"
}

teardown() {
  [ -n "${WORK:-}" ] && [ -d "$WORK" ] && find "$WORK" -type f -delete && find "$WORK" -type d -mindepth 1 | sort -r | xargs rmdir 2>/dev/null && rmdir "$WORK" 2>/dev/null || true
}

_write_registry() {
  cat > "$WORK/hooks-registry.toml" << 'TOML'
schema_version = 2

[[hooks]]
name = "validate-state-structure"
event = "PostToolUse"
tool = "Edit|Write"
plugin = "hook-plugins/validate-state-structure.wasm"
timeout_ms = 5000
on_error = "continue"

[hooks.capabilities.read_file]
path_allow = [
  ".factory",
]
TOML
}

_require_artifacts() {
  if [ ! -x "$DISPATCHER" ]; then
    skip "dispatcher binary not built -- run: cargo build --release -p factory-dispatcher"
  fi
  if [ ! -f "$WASM_PLUGIN" ]; then
    skip "validate-state-structure.wasm not built"
  fi
}

_state_md_envelope() {
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"pass-real-state-md-snapshot","tool_input":{"file_path":".factory/STATE.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# F-P2-002: real STATE.md snapshot exercises full validator surface => Continue
# ---------------------------------------------------------------------------

@test "F-P2-002 PASS: real STATE.md (live, auto-copied) passes full validator surface (no false-positive block)" {
  _require_artifacts
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 0: no block signal emitted (Continue)
  [ "$status" -eq 0 ]

  # No blocking_plugins= for a clean pass
  [[ "$output" != *"blocking_plugins="* ]]
}

@test "F-P2-002 PASS: banner (363→310 lines) narrative arrow does NOT trigger trajectory-tail false positive" {
  _require_artifacts
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Must exit 0 — if the (363→310) line were picked up as trajectory tail (1 component),
  # the hook would exit 2 with "1 components; required LENGTH=4". Exit 0 proves it is not.
  [ "$status" -eq 0 ]

  # Specifically: no trajectory-tail violation in block_reason
  [[ "$output" != *"trajectory-tail has"* ]]
  [[ "$output" != *"trajectory-tail"*"components"* ]]
}

# ---------------------------------------------------------------------------
# F-P3-001: body-narrative digit-before-first-arrow forms do NOT block
# Regression-prevention: STATE.md body contains "Trajectory 11→9→8→7→5" (line 69)
# which has digit '1' immediately before the first →. The hook must NOT false-positive-
# block on this even when it is the FIRST trajectory-like line in document order.
# ---------------------------------------------------------------------------

@test "F-P3-001 PASS: body narrative 'Trajectory 11->9->8->7->5' does NOT trigger trajectory-tail false positive" {
  _require_artifacts
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  # Inject the narrative form verbatim from STATE.md line 69 into the live fixture BEFORE
  # the canonical tail. This ensures the body scan encounters digit-before-arrow FIRST.
  # sed inserts the injected line before the first line containing "trajectory →" (canonical).
  local state_md="$WORK/.factory/STATE.md"
  local injected_line="| F5-passes-3-7-injected-regression | COMPLETE | Trajectory 11→9→8→7→5; burst-not-tail |"
  # Use awk to insert before the first line matching canonical form (space-then-arrow)
  awk '/trajectory →/{if(!injected){print "'"$injected_line"'"; injected=1}} {print}' \
    "$state_md" > "${state_md}.tmp" && mv "${state_md}.tmp" "$state_md"

  # Banner wc-l is now stale after the line injection — update it to match new count.
  # Count new lines, then update the last "NNN lines (wc-l" occurrence in the banner.
  local new_count
  new_count=$(wc -l < "$state_md" | tr -d ' ')
  # Replace the last occurrence of a number before " lines (wc-l" in the file.
  # Use Python to avoid complex sed/awk portability issues.
  python3 - "$state_md" "$new_count" << 'PYEOF'
import sys, re
path, new_count = sys.argv[1], sys.argv[2]
content = open(path).read()
# Replace last occurrence of "NNN lines (wc-l;" with "NEW_COUNT lines (wc-l;"
pattern = r'(\d+)( lines \(wc-l)'
matches = list(re.finditer(pattern, content))
if matches:
    m = matches[-1]
    content = content[:m.start(1)] + new_count + content[m.end(1):]
open(path, 'w').write(content)
PYEOF

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Must exit 0 — if "Trajectory 11→9→8→7→5" were picked up as the trajectory tail
  # (4 components matching D-433(e)) the hook would pass, but if another body line with
  # digit-before-arrow were found and counted incorrectly, it would block.
  # The hook must continue (exit 0) with the canonical →9→9→9→9 still found.
  [ "$status" -eq 0 ]

  # No trajectory-tail violation in output
  [[ "$output" != *"trajectory-tail has"* ]]
  [[ "$output" != *"blocking_plugins="* ]]
}
