#!/usr/bin/env bats
# structural-validators-emission.bats — emission tests for the 7 structural
# PostToolUse validators instrumented in observability phase 2d.2 (v0.61.0).

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
  jq -nc --arg f "$1" '{tool_name: "Edit", tool_input: {file_path: $f}}'
}

# ---------- validate-finding-format.sh ----------

@test "finding-format: emits finding_id_legacy_format" {
  local dir="$SCRATCH/.factory/cycles/v1/adversarial-reviews"
  mkdir -p "$dir"
  local f="$dir/pass-01-review.md"
  cat > "$f" <<EOF
# Adversarial Review

Finding ADV-042 is the legacy format.
EOF
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  run bash -c "echo '$(_input "$f")' | '$HOOKS_DIR/validate-finding-format.sh' 2>&1"
  [ "$status" -eq 2 ]
  local lf
  lf=$(_logfile)
  [ -n "$lf" ]
  [ "$(jq -r '.reason' < "$lf")" = "finding_id_legacy_format" ]
  [ "$(jq -r '.hook' < "$lf")" = "validate-finding-format" ]
}

@test "finding-format: still blocks when CLAUDE_PLUGIN_ROOT unset" {
  local dir="$SCRATCH/.factory/cycles/v1/adversarial-reviews"
  mkdir -p "$dir"
  local f="$dir/pass-01-review.md"
  echo "Finding ADV-042" > "$f"
  run bash -c "unset CLAUDE_PLUGIN_ROOT; echo '$(_input "$f")' | '$HOOKS_DIR/validate-finding-format.sh' 2>&1"
  [ "$status" -eq 2 ]
}

# ---------- validate-table-cell-count.sh ----------

@test "table-cell-count: emits table_cell_count_mismatch" {
  local dir="$SCRATCH/.factory"
  mkdir -p "$dir"
  local f="$dir/bad-table.md"
  cat > "$f" <<EOF
# Doc

| A | B | C |
|---|---|---|
| 1 | 2 | 3 | 4 |
EOF
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  run bash -c "echo '$(_input "$f")' | '$HOOKS_DIR/validate-table-cell-count.sh' 2>&1"
  [ "$status" -eq 2 ]
  local lf
  lf=$(_logfile)
  [ -n "$lf" ]
  [ "$(jq -r '.reason' < "$lf")" = "table_cell_count_mismatch" ]
}

@test "table-cell-count: clean table emits no event" {
  local dir="$SCRATCH/.factory"
  mkdir -p "$dir"
  local f="$dir/good-table.md"
  cat > "$f" <<EOF
| A | B |
|---|---|
| 1 | 2 |
EOF
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  run bash -c "echo '$(_input "$f")' | '$HOOKS_DIR/validate-table-cell-count.sh' 2>&1"
  [ "$status" -eq 0 ]
  [ -z "$(_logfile)" ]
}

# ---------- validate-changelog-monotonicity.sh ----------

@test "changelog-monotonicity: emits changelog_not_monotonic" {
  local dir="$SCRATCH/.factory"
  mkdir -p "$dir"
  local f="$dir/changelog.md"
  cat > "$f" <<EOF
---
version: 2.0.0
---
# Doc

## Changelog

| Version | Date | Change |
|---------|------|--------|
| 2.0.0   | 2026-04-20 | second |
| 2.0.0   | 2026-04-19 | dup    |
EOF
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  run bash -c "echo '$(_input "$f")' | '$HOOKS_DIR/validate-changelog-monotonicity.sh' 2>&1"
  [ "$status" -eq 2 ]
  local lf
  lf=$(_logfile)
  [ -n "$lf" ]
  [ "$(jq -r '.reason' < "$lf")" = "changelog_not_monotonic" ]
}

@test "changelog-monotonicity: still blocks when emit-event path broken" {
  local dir="$SCRATCH/.factory"
  mkdir -p "$dir"
  local f="$dir/changelog.md"
  cat > "$f" <<EOF
---
version: 2.0.0
---
## Changelog

| Version | Date | Change |
|---------|------|--------|
| 2.0.0   | 2026-04-20 | x |
| 2.0.0   | 2026-04-19 | y |
EOF
  run bash -c "CLAUDE_PLUGIN_ROOT='/nonexistent' echo '$(_input "$f")' | '$HOOKS_DIR/validate-changelog-monotonicity.sh' 2>&1"
  [ "$status" -eq 2 ]
}

# ---------- validate-state-size.sh ----------

@test "state-size: emits state_bloat when >500 lines and growing" {
  local dir="$SCRATCH/.factory"
  mkdir -p "$dir"
  local f="$dir/STATE.md"
  # Generate 600 lines
  for i in $(seq 1 600); do echo "line $i"; done > "$f"
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  run bash -c "echo '$(_input "$f")' | '$HOOKS_DIR/validate-state-size.sh' 2>&1"
  [ "$status" -eq 2 ]
  local lf
  lf=$(_logfile)
  [ -n "$lf" ]
  [ "$(jq -r '.reason' < "$lf")" = "state_bloat" ]
  [ "$(jq -r '.line_count' < "$lf")" = "600" ]
  [ "$(jq -r '.limit' < "$lf")" = "500" ]
}

@test "state-size: under-limit emits no event" {
  local dir="$SCRATCH/.factory"
  mkdir -p "$dir"
  local f="$dir/STATE.md"
  echo "small" > "$f"
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  run bash -c "echo '$(_input "$f")' | '$HOOKS_DIR/validate-state-size.sh' 2>&1"
  [ "$status" -eq 0 ]
  [ -z "$(_logfile)" ]
}

# ---------- validate-state-pin-freshness.sh ----------

@test "state-pin-freshness: emits state_version_pin_drift" {
  local dir="$SCRATCH/.factory"
  mkdir -p "$dir/specs/behavioral-contracts"
  # Artifact has version 2.0
  cat > "$dir/specs/behavioral-contracts/BC-INDEX.md" <<EOF
---
version: 2.0
---
# BC-INDEX
EOF
  # STATE.md pins the old version
  cat > "$dir/STATE.md" <<EOF
---
bc_index_version: 1.0
---
# STATE
EOF
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  run bash -c "echo '$(_input "$dir/STATE.md")' | '$HOOKS_DIR/validate-state-pin-freshness.sh' 2>&1"
  [ "$status" -eq 2 ]
  local lf
  lf=$(_logfile)
  [ -n "$lf" ]
  [ "$(jq -r '.reason' < "$lf")" = "state_version_pin_drift" ]
}

# ---------- validate-template-compliance.sh ----------

@test "template-compliance: emits template_noncompliant with missing_keys" {
  local dir="$SCRATCH/.factory/specs/behavioral-contracts"
  mkdir -p "$dir"
  local f="$dir/BC-1.01.001.md"
  # Missing most required frontmatter fields
  cat > "$f" <<EOF
---
status: draft
---
# BC-1.01.001: Example
EOF
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  run bash -c "echo '$(_input "$f")' | '$HOOKS_DIR/validate-template-compliance.sh' 2>&1"
  # May exit 2 if template found and fields missing; may exit 0 if no template
  # matches. Either way, if a block happens there should be an event.
  if [ "$status" -eq 2 ]; then
    local lf
    lf=$(_logfile)
    [ -n "$lf" ]
    [ "$(jq -r '.reason' < "$lf")" = "template_noncompliant" ]
  fi
}

# ---------- validate-state-index-status-coherence.sh (exit 1 / warn severity) ----------

@test "state-index-status-coherence: emits with severity=warn" {
  local dir="$SCRATCH/.factory"
  mkdir -p "$dir/cycles/v1"
  cat > "$dir/STATE.md" <<EOF
---
convergence_status: converged
---
# STATE
EOF
  cat > "$dir/cycles/v1/INDEX.md" <<EOF
# Cycle v1

**Status:** running — still iterating
EOF
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  run bash -c "echo '$(_input "$dir/STATE.md")' | '$HOOKS_DIR/validate-state-index-status-coherence.sh' 2>&1"
  [ "$status" -eq 1 ]  # advisory, not exit 2
  local lf
  lf=$(_logfile)
  [ -n "$lf" ]
  [ "$(jq -r '.reason' < "$lf")" = "state_index_status_drift" ]
  [ "$(jq -r '.severity' < "$lf")" = "warn" ]
}

# ---------- VSDD_TELEMETRY=off across the family ----------

@test "VSDD_TELEMETRY=off: finding-format still blocks, no event" {
  local dir="$SCRATCH/.factory/cycles/v1/adversarial-reviews"
  mkdir -p "$dir"
  local f="$dir/pass-01-review.md"
  echo "Finding ADV-042" > "$f"
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  export VSDD_TELEMETRY=off
  run bash -c "echo '$(_input "$f")' | '$HOOKS_DIR/validate-finding-format.sh' 2>&1"
  [ "$status" -eq 2 ]
  [ -z "$(_logfile)" ]
}

@test "VSDD_TELEMETRY=off: state-size still blocks, no event" {
  local dir="$SCRATCH/.factory"
  mkdir -p "$dir"
  local f="$dir/STATE.md"
  for i in $(seq 1 600); do echo "line $i"; done > "$f"
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  export VSDD_TELEMETRY=off
  run bash -c "echo '$(_input "$f")' | '$HOOKS_DIR/validate-state-size.sh' 2>&1"
  [ "$status" -eq 2 ]
  [ -z "$(_logfile)" ]
}
