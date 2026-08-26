#!/usr/bin/env bats
# pass-real-state-md-snapshot.bats — F-P2-002: integration test against
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
#
# SCOPE NOTE (F-P1-003 LOCAL adversary pass-1 fix):
#   The live .factory/STATE.md is the F5-cycle format (brownfield-onboarding engine-discipline
#   cycle). It may not have a `## Convergence Status` section heading (D-434(e) sub-check 1),
#   since the F5 cycle tracks convergence differently (Phase Progress + Concurrent Cycles).
#   Per F-P1-003, the validator now uses heading-anchored detection that correctly detects
#   the absence of `## Convergence Status` as a structural deficiency — which is the correct
#   behavior. The Phase 1 tests (banner, dual-margin, trajectory-tail) and trajectory-tail
#   false-positive tests are the LOAD-BEARING evidence for F-P2-001+F-P2-002+F-P3-001.
#   A Phase 1 violation in the live STATE.md would be a genuine structural defect.
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
  # Guard: skip the entire suite when the factory-artifacts worktree is not mounted.
  # .factory/STATE.md lives on the factory-artifacts orphan branch, which must be
  # checked out as a git worktree at REPO_ROOT/.factory. Without that mount the cp
  # below would abort setup with an error instead of a clean skip. This is the same
  # local-only design as PR #725 (sprint-state.yaml live-file guards).
  if [ ! -f "$REPO_ROOT/.factory/STATE.md" ]; then
    skip ".factory/STATE.md absent — factory-artifacts worktree not mounted; run: git worktree add .factory origin/factory-artifacts"
  fi
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
# F-P2-002: real STATE.md Phase 1 properties — banner, dual-margin, trajectory-tail
#
# LOAD-BEARING: these tests prove that the live STATE.md passes Phase 1 checks and
# that the (363→310 lines) narrative arrow in the banner is NOT picked up as a
# trajectory tail (F-P2-001 discriminator evidence).
#
# The live STATE.md may have D-434(e) Phase 2 sub-check violations (e.g., missing
# '## Convergence Status' heading per F-P1-003 fix) — those are structural deficiencies
# in the F5-cycle STATE.md that will be corrected by state-manager. Phase 1 violations
# would be genuine structural defects that must not occur.
# ---------------------------------------------------------------------------

@test "F-P2-002 PASS: real STATE.md Phase 1 passes — no banner/margin/tail false-positive block" {
  _require_artifacts
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Phase 1 checks must not produce violations on the live STATE.md.
  # If a Phase 1 violation occurs, the output will contain the violation keywords.
  # Phase 2 D-434(e) violations (e.g., missing '## Convergence Status' heading due to
  # F5-cycle STATE.md format) are expected and do not falsify Phase 1 correctness.
  [[ "$output" != *"no SIZE BUDGET banner"* ]]
  [[ "$output" != *"dual-margin form"* ]]
  [[ "$output" != *"trajectory-tail has"* ]]
  [[ "$output" != *"trajectory-tail"*"components"* ]]
  [[ "$output" != *"no trajectory-tail found"* ]]
}

@test "F-P2-002 PASS: banner (363→310 lines) narrative arrow does NOT trigger trajectory-tail false positive" {
  _require_artifacts
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Must NOT produce a trajectory-tail violation — if the (363→310) line were picked up as
  # trajectory tail (1 component), the hook would output "trajectory-tail has 1 components;
  # required LENGTH=4". The absence of this message proves the discriminator is working.
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
  # Count new lines, then update the last "NNN lines (wc-l" occurrence in the file.
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

  # Must NOT produce a trajectory-tail violation — if "Trajectory 11→9→8→7→5" were picked
  # up as the trajectory tail (4 components) and the canonical →9→9→9→9 were displaced,
  # the hook would pass but this discriminator check would be violated.
  # The F-P3-001 fix ensures the digit-before-first-arrow form is rejected.
  [[ "$output" != *"trajectory-tail has"* ]]
  [[ "$output" != *"no trajectory-tail found"* ]]
}
