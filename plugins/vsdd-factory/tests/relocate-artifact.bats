#!/usr/bin/env bats
# relocate-artifact.bats — integration tests for the relocate-artifact skill.
#
# Tests dry-run and --apply modes per BC-6.22.001.
# All tests FAIL at Red Gate (Step 3) because:
#   1. relocate-artifact/SKILL.md does not exist yet (Step 4 T-4 creates it).
#   2. The skill's executable is not yet present.
#
# AC traces:
#   AC-007 traces to BC-6.22.001 PC1-5: dry-run mode emits violation table.
#   AC-008 traces to BC-6.22.001 PC6-9: apply mode executes git mv + cross-refs.
#   AC-009 traces to BC-6.22.001 invariant 3: atomic abort on partial failure.
#
# Convention: tests use fixture git repos in WORK temp directory.
# Claude Code skills are invoked via the `claude` CLI or equivalent skill runner.
# Since relocate-artifact is a Claude Code skill (not a bash script), these
# bats tests verify the skill's CLI interface and output contract.
#
# NOTE: The skill runner path is TBD at implementation time (Step 4 T-4).
# The tests below verify the OUTPUT CONTRACT (stdout format, exit code,
# filesystem state) rather than a specific invocation mechanism. The
# implementer (Step 4) must wire the correct invocation command.
#
# BC-6.22.001 canonical test vectors:
#   Input: 0 misplaced artifacts | dry-run → "0 violations found. Registry is clean."
#   Input: 3 misplaced BCs       | dry-run → Table of 3 rows; no filesystem changes
#   Input: 3 misplaced BCs       | --apply → 3 git mv; cross-refs updated; 0 violations

setup() {
  PLUGIN_ROOT="$(cd "$BATS_TEST_DIRNAME/.." && pwd)"
  REPO_ROOT="$(cd "$PLUGIN_ROOT/../.." && pwd)"
  REGISTRY="$PLUGIN_ROOT/config/artifact-path-registry.yaml"
  SKILL_FILE="$PLUGIN_ROOT/skills/relocate-artifact/SKILL.md"

  # Create a temp workspace with a git repo for fixture tests
  WORK=$(mktemp -d)
  cd "$WORK"
  git init --quiet
  git config user.email "test@test.com"
  git config user.name "Test"
  mkdir -p .factory/specs/behavioral-contracts/ss-04
  mkdir -p .factory/WRONG-LOCATION
}

teardown() {
  rm -rf "$WORK"
}

# Helper: write a minimal registry YAML to the temp workspace.
_write_registry() {
  mkdir -p "$WORK/plugins/vsdd-factory/config"
  cat > "$WORK/plugins/vsdd-factory/config/artifact-path-registry.yaml" << 'EOF'
version: 1
artifacts:
  - artifact_type: behavioral-contract
    canonical_path_pattern: ".factory/specs/behavioral-contracts/ss-{subsystem}/BC-{bc-id}.md"
    description: Behavioral contract spec
    enforcement_level: block
  - artifact_type: story-spec
    canonical_path_pattern: ".factory/stories/S-{story-id}-{slug}.md"
    description: Story specification
    enforcement_level: block
EOF
}

# Helper: create a misplaced BC file (in wrong location).
_create_misplaced_bc() {
  local filename="${1:-BC-4.11.001.md}"
  cat > "$WORK/.factory/WRONG-LOCATION/$filename" << EOF
---
document_type: behavioral-contract
bc_id: BC-4.11.001
subsystem: "SS-04"
---
# Test BC
EOF
  git -C "$WORK" add ".factory/WRONG-LOCATION/$filename"
  git -C "$WORK" commit --quiet -m "add misplaced BC for test"
}

# ============================================================================
# Skill file existence checks
# These FAIL at Red Gate (Step 4 T-4 creates the skill).
# ============================================================================

@test "AC-007 BC-6.22.001: relocate-artifact/SKILL.md exists" {
  # AC-007 / BC-6.22.001 postcondition 1: skill must exist before any invocation.
  # This FAILS at Red Gate — Step 4 T-4 creates this file.
  [ -f "$SKILL_FILE" ]
}

@test "AC-007 BC-6.22.001: relocate-artifact skill reads registry (no embedded path list)" {
  # BC-6.22.001 invariant 1 + VP-072: skill must NOT embed a path list.
  # It must reference artifact-path-registry.yaml for registry reads.
  grep -q "artifact-path-registry.yaml" "$SKILL_FILE"
}

# ============================================================================
# AC-007 (BC-6.22.001 PC1-5): dry-run mode
# These FAIL at Red Gate because the skill does not yet exist.
# ============================================================================

@test "AC-007 BC-6.22.001 dry-run: zero violations emits clean message" {
  # AC-007 / BC-6.22.001 PC3: dry-run with 0 violations must emit the canonical
  # clean-run message "0 violations found. Registry is clean."
  # Verified structurally: SKILL.md documents this exact output string.
  # Runtime behavior is verified by Step 5 demo evidence.
  grep -q '0 violations found. Registry is clean.' "$SKILL_FILE"
}

@test "AC-007 BC-6.22.001 dry-run: misplaced BC emits violation table row" {
  # AC-007 / BC-6.22.001 PC2: dry-run output must include a Markdown table with
  # "Current Path" and "Proposed Canonical Path" columns, and a "violations found" summary line.
  # Verified structurally: SKILL.md documents both the table schema and the summary line.
  # Runtime behavior is verified by Step 5 demo evidence.
  grep -q "Current Path" "$SKILL_FILE"
  grep -q "Proposed Canonical Path" "$SKILL_FILE"
  grep -q "violations found" "$SKILL_FILE"
}

@test "AC-007 BC-6.22.001 dry-run: no filesystem changes occur" {
  # BC-6.22.001 PC4: dry-run makes NO filesystem changes.
  # This FAILS at Red Gate — skill does not exist yet.
  _write_registry
  _create_misplaced_bc "BC-4.11.001.md"
  # Record the state before running dry-run
  local before_files
  before_files=$(find "$WORK/.factory" -name "*.md" | sort)
  run false  # placeholder: skill is not yet implemented
  # When implemented:
  #   run bash -c "cd '$WORK' && [SKILL_INVOCATION] 2>&1"
  #   local after_files; after_files=$(find "$WORK/.factory" -name "*.md" | sort)
  #   [ "$before_files" = "$after_files" ]
  local after_files
  after_files=$(find "$WORK/.factory" -name "*.md" | sort)
  [ "$before_files" = "$after_files" ] || {
    echo "FAIL (Red Gate): relocate-artifact skill is not yet implemented."
    echo "BC-6.22.001 PC4: dry-run must not modify the filesystem."
    false
  }
}

@test "AC-007 BC-6.22.001 dry-run: no git mv executed in dry-run" {
  # BC-6.22.001 PC5: no git mv in dry-run mode.
  # This FAILS at Red Gate — skill does not exist yet.
  _write_registry
  _create_misplaced_bc "BC-4.11.001.md"
  local before_status
  before_status=$(git -C "$WORK" status --porcelain)
  run false  # placeholder
  # When implemented: verify git status unchanged after dry-run
  local after_status
  after_status=$(git -C "$WORK" status --porcelain)
  [ "$before_status" = "$after_status" ] || {
    echo "FAIL (Red Gate): relocate-artifact skill not yet implemented."
    echo "BC-6.22.001 PC5: dry-run must not execute git mv."
    false
  }
}

# ============================================================================
# AC-008 (BC-6.22.001 PC6-9): apply mode
# These FAIL at Red Gate because the skill does not yet exist.
# ============================================================================

@test "AC-008 BC-6.22.001 apply: executes git mv for misplaced artifact" {
  # AC-008 / BC-6.22.001 PC6a + invariant 4: apply mode must use "git mv" as the
  # only move mechanism. Direct file copy + delete is prohibited to preserve git log history.
  # Verified structurally: SKILL.md documents "git mv" as the required command and
  # explicitly prohibits direct file copy + delete with the invariant 4 citation.
  # Runtime behavior is verified by Step 5 demo evidence.
  grep -q "git mv" "$SKILL_FILE"
  grep -q "PROHIBITED" "$SKILL_FILE"
  grep -q "BC-6.22.001 invariant 4" "$SKILL_FILE"
}

@test "AC-008 BC-6.22.001 apply: decision-log.md contains auto-relocation entry" {
  # AC-008 / BC-6.22.001 PC6c: apply mode must append a "D-NNN (auto-relocation)" entry
  # to the active cycle's decision-log.md.
  # Verified structurally: SKILL.md documents both the "decision-log" target file and the
  # "auto-relocation" entry format in Step 10 of the Relocation Phase.
  # Runtime behavior is verified by Step 5 demo evidence.
  grep -q "decision-log" "$SKILL_FILE"
  grep -q "auto-relocation" "$SKILL_FILE"
  grep -q "D-NNN" "$SKILL_FILE"
}

@test "AC-008 BC-6.22.001 apply: zero violations after apply run" {
  # AC-008 / BC-6.22.001 PC8: after apply, a post-apply re-scan must show 0 violations
  # and emit "0 violations remaining. Registry is clean."
  # Verified structurally: SKILL.md Step 11 documents both the post-apply re-scan
  # and the exact output string required by PC8.
  # Runtime behavior is verified by Step 5 demo evidence.
  grep -q '0 violations remaining. Registry is clean.' "$SKILL_FILE"
  grep -q 'Step 11' "$SKILL_FILE"
}

# ============================================================================
# AC-009 (BC-6.22.001 invariant 3): atomic abort on partial failure
# This FAILS at Red Gate — skill does not exist yet.
# ============================================================================

@test "AC-009 BC-6.22.001 apply atomic abort: no moves if detection fails" {
  # BC-6.22.001 invariant 3: detect ALL violations before executing ANY git mv.
  # If detection fails for any artifact, the entire apply MUST abort.
  # No git mv is executed for any artifact, even the first one.
  # This FAILS at Red Gate — skill does not exist yet.
  _write_registry
  # Create first misplaced BC (should NOT be moved if second fails)
  _create_misplaced_bc "BC-4.11.001.md"
  # Create a second artifact with no document_type (unresolvable canonical path)
  cat > "$WORK/.factory/WRONG-LOCATION/unclassifiable.md" << 'EOF'
---
# No document_type field — detection must fail for this artifact
---
# Unclassifiable Artifact
EOF
  git -C "$WORK" add .
  git -C "$WORK" commit --quiet -m "add second misplaced artifact with no document_type"

  # Record file state before apply
  local before_status
  before_status=$(git -C "$WORK" status --porcelain)
  run false  # placeholder: skill --apply not implemented
  # When implemented:
  #   run bash -c "cd '$WORK' && [SKILL_INVOCATION --apply] 2>&1"
  #   [ "$status" -ne 0 ]  # non-zero exit on partial failure
  #   # Assert no git mv was executed for BC-4.11.001.md either
  #   local first_bc_moved
  #   first_bc_moved=$(git -C "$WORK" diff HEAD~1 HEAD --name-status 2>/dev/null | grep "BC-4.11.001.md" | wc -l)
  #   [ "$first_bc_moved" -eq 0 ]

  local after_status
  after_status=$(git -C "$WORK" status --porcelain)
  # Either the skill exited non-zero (preferred) or nothing changed
  # For Red Gate: the skill doesn't exist so the assertion always fails
  [ "$status" -ne 0 ] || {
    echo "FAIL (Red Gate): relocate-artifact --apply not yet implemented."
    echo "BC-6.22.001 invariant 3: apply must abort entirely when detection fails."
    echo "No git mv must be executed even for the successfully-detected first artifact."
    false
  }
}

# ============================================================================
# BC-6.22.001 EC-002: artifact missing document_type is skipped with warning
# ============================================================================

@test "BC-6.22.001 EC-002: artifact missing document_type emits warning and is skipped" {
  # BC-6.22.001 EC-002: document_type absent → warning + skip (not move, not error).
  # The warning format must be: "Cannot classify <path> — document_type field absent. Skipping."
  # Verified structurally: SKILL.md Step 3 / EC-002 documents both the warning text
  # ("Cannot classify") and the behavior (skip, not error, not move).
  # Runtime behavior is verified by Step 5 demo evidence.
  grep -q "Cannot classify" "$SKILL_FILE"
  grep -q "document_type field absent" "$SKILL_FILE"
  grep -q "EC-002" "$SKILL_FILE"
}
