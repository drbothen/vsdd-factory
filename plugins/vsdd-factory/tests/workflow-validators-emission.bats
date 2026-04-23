#!/usr/bin/env bats
# workflow-validators-emission.bats — emission tests for the 10 workflow /
# specialized PostToolUse validators instrumented in obs phase 2d.3 (v0.62.0).
#
# Covers both exit-2 block events and exit-0 severity=warn events.

setup() {
  HOOKS_DIR="${BATS_TEST_DIRNAME}/../hooks"
  PLUGIN_ROOT="${BATS_TEST_DIRNAME}/.."
  EMIT_TMPDIR="$(mktemp -d)"
  SCRATCH="$(mktemp -d)"
  export VSDD_LOG_DIR="$EMIT_TMPDIR"
  unset VSDD_TELEMETRY
}

teardown() {
  rm -rf "$EMIT_TMPDIR" "$SCRATCH"
  unset VSDD_LOG_DIR CLAUDE_PLUGIN_ROOT VSDD_TELEMETRY
}

_logfile() {
  ls "$EMIT_TMPDIR"/events-*.jsonl 2>/dev/null | head -1
}

_edit_input() {
  jq -nc --arg f "$1" '{tool_name: "Edit", tool_input: {file_path: $f}}'
}

_bash_input() {
  # $1 = command, $2 = exit_code
  jq -nc --arg c "$1" --arg e "$2" '{
    tool_name: "Bash",
    tool_input: {command: $c},
    tool_response: {exit_code: ($e | tonumber)}
  }'
}

# ---------- purity-check.sh (severity=warn) ----------

@test "purity-check: emits pure_core_boundary_violation with severity=warn" {
  local dir="$SCRATCH/pure"
  mkdir -p "$dir"
  local f="$dir/impure.rs"
  cat > "$f" <<EOF
use std::fs;
fn main() {
    println!("hello");
}
EOF
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  run bash -c "echo '$(_edit_input "$f")' | '$HOOKS_DIR/purity-check.sh' 2>&1"
  [ "$status" -eq 0 ]
  local lf
  lf=$(_logfile)
  [ -n "$lf" ]
  [ "$(jq -r '.reason' < "$lf")" = "pure_core_boundary_violation" ]
  [ "$(jq -r '.severity' < "$lf")" = "warn" ]
}

# ---------- validate-input-hash.sh ----------

@test "input-hash: emits input_hash_invalid_format on wrong length" {
  local dir="$SCRATCH/.factory"
  mkdir -p "$dir"
  local f="$dir/story.md"
  cat > "$f" <<EOF
---
inputs:
  - foo.md
input-hash: abc123
---
# story
EOF
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  run bash -c "echo '$(_edit_input "$f")' | '$HOOKS_DIR/validate-input-hash.sh' 2>&1"
  [ "$status" -eq 2 ]
  local lf
  lf=$(_logfile)
  [ -n "$lf" ]
  [ "$(jq -r '.reason' < "$lf")" = "input_hash_invalid_format" ]
  [ "$(jq -r '.issue' < "$lf")" = "length" ]
}

@test "input-hash: emits with issue=chars on non-hex" {
  local dir="$SCRATCH/.factory"
  mkdir -p "$dir"
  local f="$dir/story.md"
  cat > "$f" <<EOF
---
inputs:
  - foo.md
input-hash: ZZZZZZZ
---
# story
EOF
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  run bash -c "echo '$(_edit_input "$f")' | '$HOOKS_DIR/validate-input-hash.sh' 2>&1"
  [ "$status" -eq 2 ]
  local lf
  lf=$(_logfile)
  [ "$(jq -r '.issue' < "$lf")" = "chars" ]
}

# ---------- validate-novelty-assessment.sh ----------

@test "novelty-assessment: emits novelty_assessment_incomplete" {
  local dir="$SCRATCH/.factory"
  mkdir -p "$dir"
  local f="$dir/pass-01.md"
  cat > "$f" <<EOF
# pass-01

Body with no novelty assessment at all.
EOF
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  run bash -c "echo '$(_edit_input "$f")' | '$HOOKS_DIR/validate-novelty-assessment.sh' 2>&1"
  [ "$status" -eq 2 ]
  local lf
  lf=$(_logfile)
  [ "$(jq -r '.reason' < "$lf")" = "novelty_assessment_incomplete" ]
}

# ---------- convergence-tracker.sh ----------

@test "convergence-tracker: emits convergence_rule_violation on premature CONVERGENCE_REACHED" {
  local dir="$SCRATCH/.factory/cycles/v1/adversarial-reviews"
  mkdir -p "$dir"
  local f="$dir/pass-01.md"
  cat > "$f" <<EOF
# pass-01

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 1 |
| **Novelty score** | 0.80 |
| **Trajectory** | 5 |
| **Verdict** | CONVERGENCE_REACHED |
| **New findings** | 5 |
| **Duplicate/variant findings** | 0 |

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 2 |
| HIGH | 3 |
EOF
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  run bash -c "echo '$(_edit_input "$f")' | '$HOOKS_DIR/convergence-tracker.sh' 2>&1"
  [ "$status" -eq 2 ]
  local lf
  lf=$(_logfile)
  [ "$(jq -r '.reason' < "$lf")" = "convergence_rule_violation" ]
  [ "$(jq -r '.verdict' < "$lf")" = "CONVERGENCE_REACHED" ]
}

# ---------- validate-anchor-capabilities-union.sh ----------

@test "anchor-capabilities: emits anchor_capabilities_mismatch" {
  local dir="$SCRATCH/.factory"
  mkdir -p "$dir/stories" "$dir/specs/behavioral-contracts"
  cat > "$dir/specs/behavioral-contracts/BC-1.01.001.md" <<EOF
---
capability: CAP-005
---
# BC-1.01.001
EOF
  cat > "$dir/stories/STORY-042.md" <<EOF
---
anchor_bcs:
  - BC-1.01.001
anchor_capabilities:
  - CAP-999
---
# STORY-042
EOF
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  run bash -c "echo '$(_edit_input "$dir/stories/STORY-042.md")' | '$HOOKS_DIR/validate-anchor-capabilities-union.sh' 2>&1"
  [ "$status" -eq 2 ]
  local lf
  lf=$(_logfile)
  [ "$(jq -r '.reason' < "$lf")" = "anchor_capabilities_mismatch" ]
  [ "$(jq -r '.expected' < "$lf")" = "CAP-005" ]
  [ "$(jq -r '.actual' < "$lf")" = "CAP-999" ]
}

# ---------- validate-demo-evidence-story-scoped.sh ----------

@test "demo-evidence-story-scoped: emits demo_evidence_not_story_scoped" {
  local dir="$SCRATCH/docs/demo-evidence"
  mkdir -p "$dir"
  local f="$dir/evidence-report.md"
  echo "report" > "$f"
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  run bash -c "echo '$(_edit_input "$f")' | '$HOOKS_DIR/validate-demo-evidence-story-scoped.sh' 2>&1"
  [ "$status" -eq 2 ]
  local lf
  lf=$(_logfile)
  [ "$(jq -r '.reason' < "$lf")" = "demo_evidence_not_story_scoped" ]
}

# ---------- validate-pr-description-completeness.sh ----------

@test "pr-description-completeness: emits pr_description_incomplete" {
  local dir="$SCRATCH/.factory/code-delivery/STORY-042"
  mkdir -p "$dir"
  local f="$dir/pr-description.md"
  cat > "$f" <<EOF
# PR description

Body without required sections and with {story_id} placeholder.
EOF
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  run bash -c "echo '$(_edit_input "$f")' | '$HOOKS_DIR/validate-pr-description-completeness.sh' 2>&1"
  [ "$status" -eq 2 ]
  local lf
  lf=$(_logfile)
  [ "$(jq -r '.reason' < "$lf")" = "pr_description_incomplete" ]
}

# ---------- validate-wave-gate-completeness.sh ----------

@test "wave-gate-completeness: emits wave_gate_incomplete (no gate_report)" {
  if ! command -v python3 >/dev/null 2>&1; then
    skip "python3 unavailable"
  fi
  local dir="$SCRATCH/.factory"
  mkdir -p "$dir"
  local f="$dir/wave-state.yaml"
  cat > "$f" <<EOF
waves:
  wave-1:
    gate_status: passed
EOF
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  run bash -c "echo '$(_edit_input "$f")' | '$HOOKS_DIR/validate-wave-gate-completeness.sh' 2>&1"
  [ "$status" -eq 2 ]
  local lf
  lf=$(_logfile)
  [ "$(jq -r '.reason' < "$lf")" = "wave_gate_incomplete" ]
}

# ---------- validate-factory-path-root.sh ----------

@test "factory-path-root: emits factory_path_worktree_relative" {
  # The hook checks the PATH pattern, not file existence, so we can test
  # with a fabricated path.
  local input
  input=$(jq -nc '{
    tool_name: "Write",
    tool_input: {file_path: "/tmp/.worktrees/STORY-001/.factory/stories/S-1.md"}
  }')
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  run bash -c "echo '$input' | '$HOOKS_DIR/validate-factory-path-root.sh' 2>&1"
  [ "$status" -eq 2 ]
  local lf
  lf=$(_logfile)
  [ "$(jq -r '.reason' < "$lf")" = "factory_path_worktree_relative" ]
  [ "$(jq -r '.worktree' < "$lf")" = ".worktrees/STORY-001" ]
}

# ---------- regression-gate.sh (severity=warn) ----------

@test "regression-gate: emits regression_gate_pass_to_fail with severity=warn" {
  # Seed prior state as pass, then run a failing test command.
  mkdir -p "$SCRATCH/.factory"
  echo '{"status": "pass"}' > "$SCRATCH/.factory/regression-state.json"
  local input
  input=$(_bash_input "cargo test" "1")
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  run bash -c "cd '$SCRATCH' && echo '$input' | '$HOOKS_DIR/regression-gate.sh' 2>&1"
  [ "$status" -eq 0 ]
  local lf
  lf=$(_logfile)
  [ -n "$lf" ]
  [ "$(jq -r '.reason' < "$lf")" = "regression_gate_pass_to_fail" ]
  [ "$(jq -r '.severity' < "$lf")" = "warn" ]
}

@test "regression-gate: pass->pass emits no event" {
  mkdir -p "$SCRATCH/.factory"
  echo '{"status": "pass"}' > "$SCRATCH/.factory/regression-state.json"
  local input
  input=$(_bash_input "cargo test" "0")
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  run bash -c "cd '$SCRATCH' && echo '$input' | '$HOOKS_DIR/regression-gate.sh' 2>&1"
  [ "$status" -eq 0 ]
  [ -z "$(_logfile)" ]
}

# ---------- VSDD_TELEMETRY=off spot checks ----------

@test "VSDD_TELEMETRY=off: factory-path-root still blocks, no event" {
  local input
  input=$(jq -nc '{
    tool_name: "Write",
    tool_input: {file_path: "/tmp/.worktrees/STORY-001/.factory/x.md"}
  }')
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  export VSDD_TELEMETRY=off
  run bash -c "echo '$input' | '$HOOKS_DIR/validate-factory-path-root.sh' 2>&1"
  [ "$status" -eq 2 ]
  [ -z "$(_logfile)" ]
}
