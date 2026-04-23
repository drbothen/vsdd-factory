#!/usr/bin/env bats
# policy-validators-emission.bats — emission tests for the 4 Policy 6/7/8/9
# PostToolUse validators instrumented in observability phase 2d.1 (v0.60.0).
#
# Existing behavior tests cover the block/pass logic in policy9.bats and
# policy-enforcement.bats. These tests verify that (a) a hook.block event
# is written when the validator fires, and (b) the validator still blocks
# when emit-event is unreachable.

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

_input() {
  # $1 = file path
  jq -nc --arg f "$1" '{tool_name: "Edit", tool_input: {file_path: $f}}'
}

# ---------- Policy 6: validate-subsystem-names.sh ----------

_setup_policy6_scenario() {
  # Create an ARCH-INDEX with canonical SS-01 and SS-02, then a BC file
  # with a non-canonical subsystem. Returns the BC file path.
  local arch_dir="$SCRATCH/.factory/specs/architecture"
  local bc_dir="$SCRATCH/.factory/specs/behavioral-contracts"
  mkdir -p "$arch_dir" "$bc_dir"
  cat > "$arch_dir/ARCH-INDEX.md" <<EOF
# ARCH-INDEX

## Subsystem Registry

| SS ID | Name | Architecture Doc | Implementing Modules | Phase |
|-------|------|------------------|----------------------|-------|
| SS-01 | Auth | arch/auth.md | src/auth/ | impl |
| SS-02 | Data | arch/data.md | src/data/ | impl |
EOF
  cat > "$bc_dir/BC-1.01.001.md" <<EOF
---
subsystem: SS-99
---
# BC-1.01.001: Example
EOF
  echo "$bc_dir/BC-1.01.001.md"
}

@test "validate-subsystem-names: emits policy6_subsystem_name_mismatch" {
  local bc
  bc=$(_setup_policy6_scenario)
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  run bash -c "echo '$(_input "$bc")' | '$HOOKS_DIR/validate-subsystem-names.sh' 2>&1"
  [ "$status" -eq 2 ]
  local f
  f=$(_logfile)
  [ -n "$f" ]
  [ "$(jq -r '.reason' < "$f")" = "policy6_subsystem_name_mismatch" ]
  [ "$(jq -r '.hook' < "$f")" = "validate-subsystem-names" ]
  [ "$(jq -r '.file_path' < "$f")" = "$bc" ]
}

@test "validate-subsystem-names: still blocks when CLAUDE_PLUGIN_ROOT unset" {
  local bc
  bc=$(_setup_policy6_scenario)
  run bash -c "unset CLAUDE_PLUGIN_ROOT; echo '$(_input "$bc")' | '$HOOKS_DIR/validate-subsystem-names.sh' 2>&1"
  [ "$status" -eq 2 ]
}

@test "validate-subsystem-names: canonical SS ID emits no event" {
  _setup_policy6_scenario >/dev/null
  local bc="$SCRATCH/.factory/specs/behavioral-contracts/BC-1.01.001.md"
  # Fix the subsystem to a canonical one
  sed -i.bak 's/SS-99/SS-01/' "$bc" && rm "$bc.bak"
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  run bash -c "echo '$(_input "$bc")' | '$HOOKS_DIR/validate-subsystem-names.sh' 2>&1"
  [ "$status" -eq 0 ]
  [ -z "$(_logfile)" ]
}

# ---------- Policy 7: validate-bc-title.sh ----------

_setup_policy7_scenario() {
  # Create BC-INDEX listing BC-1.01.001 with title "Registered Title",
  # and BC file with H1 "Drifted Title".
  local bc_dir="$SCRATCH/.factory/specs/behavioral-contracts"
  mkdir -p "$bc_dir"
  cat > "$bc_dir/BC-INDEX.md" <<EOF
# BC-INDEX

| ID          | Title            | Status |
|-------------|------------------|--------|
| BC-1.01.001 | Registered Title | green  |
EOF
  cat > "$bc_dir/BC-1.01.001.md" <<EOF
---
status: draft
---
# BC-1.01.001: Drifted Title

Body.
EOF
  echo "$bc_dir/BC-1.01.001.md"
}

@test "validate-bc-title: emits policy7_bc_title_mismatch" {
  local bc
  bc=$(_setup_policy7_scenario)
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  run bash -c "echo '$(_input "$bc")' | '$HOOKS_DIR/validate-bc-title.sh' 2>&1"
  [ "$status" -eq 2 ]
  local f
  f=$(_logfile)
  [ -n "$f" ]
  [ "$(jq -r '.reason' < "$f")" = "policy7_bc_title_mismatch" ]
  [ "$(jq -r '.bc_id' < "$f")" = "BC-1.01.001" ]
  [ "$(jq -r '.h1_title' < "$f")" = "Drifted Title" ]
  [ "$(jq -r '.index_title' < "$f")" = "Registered Title" ]
}

@test "validate-bc-title: still blocks when emit-event path broken" {
  local bc
  bc=$(_setup_policy7_scenario)
  run bash -c "CLAUDE_PLUGIN_ROOT='/nonexistent' echo '$(_input "$bc")' | '$HOOKS_DIR/validate-bc-title.sh' 2>&1"
  [ "$status" -eq 2 ]
}

@test "validate-bc-title: matching title emits no event" {
  _setup_policy7_scenario >/dev/null
  local bc="$SCRATCH/.factory/specs/behavioral-contracts/BC-1.01.001.md"
  sed -i.bak 's/Drifted Title/Registered Title/' "$bc" && rm "$bc.bak"
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  run bash -c "echo '$(_input "$bc")' | '$HOOKS_DIR/validate-bc-title.sh' 2>&1"
  [ "$status" -eq 0 ]
  [ -z "$(_logfile)" ]
}

# ---------- Policy 8: validate-story-bc-sync.sh ----------

_setup_policy8_scenario() {
  # Story frontmatter declares BC-1.01.001 but body BC table references a
  # different BC (BC-2.01.001) — triggers check 3 (body BC not in frontmatter).
  local story_dir="$SCRATCH/.factory/stories"
  mkdir -p "$story_dir"
  cat > "$story_dir/STORY-042.md" <<EOF
---
behavioral_contracts:
  - BC-1.01.001
---
# STORY-042

## Behavioral Contracts

| BC | Status |
|----|--------|
| BC-2.01.001 | green |

## Acceptance Criteria

- AC-1: Some criterion (traces to BC-2.01.001)
EOF
  echo "$story_dir/STORY-042.md"
}

@test "validate-story-bc-sync: emits policy8_bc_array_desync" {
  local story
  story=$(_setup_policy8_scenario)
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  run bash -c "echo '$(_input "$story")' | '$HOOKS_DIR/validate-story-bc-sync.sh' 2>&1"
  [ "$status" -eq 2 ]
  local f
  f=$(_logfile)
  [ -n "$f" ]
  [ "$(jq -r '.reason' < "$f")" = "policy8_bc_array_desync" ]
  [ "$(jq -r '.hook' < "$f")" = "validate-story-bc-sync" ]
}

@test "validate-story-bc-sync: still blocks when CLAUDE_PLUGIN_ROOT unset" {
  local story
  story=$(_setup_policy8_scenario)
  run bash -c "unset CLAUDE_PLUGIN_ROOT; echo '$(_input "$story")' | '$HOOKS_DIR/validate-story-bc-sync.sh' 2>&1"
  [ "$status" -eq 2 ]
}

# ---------- Policy 9: validate-vp-consistency.sh ----------

_setup_policy9_scenario() {
  # VP-INDEX lists VP-001 but verification-architecture.md doesn't mention it.
  local vp_dir="$SCRATCH/.factory/specs/verification-properties"
  local arch_dir="$SCRATCH/.factory/specs/architecture"
  mkdir -p "$vp_dir" "$arch_dir"
  cat > "$vp_dir/VP-INDEX.md" <<EOF
# VP-INDEX

| VP ID  | Name | Status | Tool | Proof Method |
|--------|------|--------|------|--------------|
| VP-001 | Safe | green  | Kani | bounded      |

## Summary

| Tool | Count |
|------|-------|
| Kani | 1     |
EOF
  cat > "$arch_dir/verification-architecture.md" <<EOF
# Verification Architecture

(No VPs referenced here — triggers inconsistency.)
EOF
  cat > "$arch_dir/verification-coverage-matrix.md" <<EOF
# Verification Coverage Matrix

## Coverage by Module

| Module | Criticality | Kani | VPs |
|--------|-------------|------|-----|
| core   | HIGH        | 1    | 1   |
EOF
  echo "$vp_dir/VP-INDEX.md"
}

@test "validate-vp-consistency: emits policy9_vp_inconsistency" {
  local vp
  vp=$(_setup_policy9_scenario)
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  run bash -c "echo '$(_input "$vp")' | '$HOOKS_DIR/validate-vp-consistency.sh' 2>&1"
  [ "$status" -eq 2 ]
  local f
  f=$(_logfile)
  [ -n "$f" ]
  [ "$(jq -r '.reason' < "$f")" = "policy9_vp_inconsistency" ]
  [ "$(jq -r '.hook' < "$f")" = "validate-vp-consistency" ]
}

@test "validate-vp-consistency: still blocks when emit-event path broken" {
  local vp
  vp=$(_setup_policy9_scenario)
  run bash -c "CLAUDE_PLUGIN_ROOT='/nonexistent' echo '$(_input "$vp")' | '$HOOKS_DIR/validate-vp-consistency.sh' 2>&1"
  [ "$status" -eq 2 ]
}

# ---------- VSDD_TELEMETRY=off across all four ----------

@test "VSDD_TELEMETRY=off: subsystem-names still blocks, no event" {
  local bc
  bc=$(_setup_policy6_scenario)
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  export VSDD_TELEMETRY=off
  run bash -c "echo '$(_input "$bc")' | '$HOOKS_DIR/validate-subsystem-names.sh' 2>&1"
  [ "$status" -eq 2 ]
  [ -z "$(_logfile)" ]
}

@test "VSDD_TELEMETRY=off: vp-consistency still blocks, no event" {
  local vp
  vp=$(_setup_policy9_scenario)
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  export VSDD_TELEMETRY=off
  run bash -c "echo '$(_input "$vp")' | '$HOOKS_DIR/validate-vp-consistency.sh' 2>&1"
  [ "$status" -eq 2 ]
  [ -z "$(_logfile)" ]
}
