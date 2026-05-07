#!/usr/bin/env bats
# vp-072-sot-invariant.bats — VP-072 single-source-of-truth invariant.
#
# Verifies that no skill SKILL.md file or WASM hook source embeds a
# hardcoded duplicate artifact path list that overlaps with
# plugins/vsdd-factory/config/artifact-path-registry.yaml.
#
# This test FAILS today (Step 3, Red Gate) because:
#   1. artifact-path-registry.yaml does not yet exist (Step 2 created the
#      file stub; Step 4 populates it).
#   2. The 9 creation skills have not yet been updated with registry-read
#      preambles (Step 4 task T-3).
#   3. validate-artifact-path hook source does not exist as a non-stub.
#
# AC traces:
#   AC-011 traces to BC-4.11.001 PC8: 9 creation skills updated with registry-read preamble.
#   AC-012 traces to BC-4.11.001 PC8: writing-agent prompt preambles updated.
#   AC-013 traces to BC-6.22.001 invariant 1 + VP-072: no duplicate path lists.
#   AC-015 traces to BC-4.11.001 invariant 1: hook source has no hardcoded paths.
#
# Bats convention: PLUGIN_ROOT is the plugins/vsdd-factory directory.
# Tests run from the repo root (set by bats setup or CI).

setup() {
  # Navigate to repo root from the test file's location:
  # plugins/vsdd-factory/tests/ → ../../.. = repo root
  PLUGIN_ROOT="$(cd "$BATS_TEST_DIRNAME/.." && pwd)"
  REPO_ROOT="$(cd "$PLUGIN_ROOT/../.." && pwd)"
  REGISTRY="$PLUGIN_ROOT/config/artifact-path-registry.yaml"
  SKILLS_DIR="$PLUGIN_ROOT/skills"
  HOOKS_DIR="$PLUGIN_ROOT/hooks"
  HOOK_SRC_DIR="$REPO_ROOT/crates/hook-plugins"
}

# ============================================================================
# AC-013 / VP-072: artifact-path-registry.yaml exists and is parseable
# This FAILS until Step 4 (T-1) creates the registry file with >= 6 entries.
# ============================================================================

@test "VP-072 AC-013: artifact-path-registry.yaml exists" {
  # AC-013 traces to BC-6.22.001 invariant 1 + VP-072.
  # This test FAILS at Red Gate because the registry file does not exist
  # (Step 4 T-1 creates it with >= 6 artifact type entries).
  [ -f "$REGISTRY" ]
}

@test "VP-072 AC-013: artifact-path-registry.yaml has at least 6 artifact_type entries" {
  # AC-001 (falsifiable test): grep artifact_type returns >= 6 entries.
  # This FAILS at Red Gate (registry file absent or has < 6 entries).
  local count
  count=$(grep -c "artifact_type:" "$REGISTRY" 2>/dev/null || echo "0")
  [ "$count" -ge 6 ]
}

@test "VP-072 AC-013: artifact-path-registry.yaml contains behavioral-contract entry" {
  # AC-013: registry must have at least the behavioral-contract artifact type.
  grep -q "behavioral-contract" "$REGISTRY"
}

@test "VP-072 AC-013: artifact-path-registry.yaml contains adr entry" {
  # AC-013: registry must have the adr artifact type.
  grep -q "artifact_type:.*adr\|artifact_type: adr" "$REGISTRY"
}

@test "VP-072 AC-013: artifact-path-registry.yaml contains verification-property entry" {
  # AC-013: registry must have the verification-property artifact type.
  grep -q "verification-property" "$REGISTRY"
}

@test "VP-072 AC-013: artifact-path-registry.yaml contains story-spec entry" {
  # AC-013: registry must have the story-spec artifact type.
  grep -q "story-spec\|story_spec" "$REGISTRY"
}

@test "VP-072 AC-013: artifact-path-registry.yaml contains enforcement_level field on all entries" {
  # AC-005 / BC-4.11.001 invariant 2: every registry entry must have enforcement_level.
  # This FAILS until Step 4 T-1 populates the registry with enforcement_level on each entry.
  local entry_count level_count
  entry_count=$(grep -c "artifact_type:" "$REGISTRY" 2>/dev/null || echo "0")
  level_count=$(grep -c "enforcement_level:" "$REGISTRY" 2>/dev/null || echo "0")
  [ "$entry_count" -gt 0 ]
  [ "$level_count" -eq "$entry_count" ]
}

# ============================================================================
# AC-011 / VP-072: 9 creation skills have registry-read preamble
# These FAIL at Red Gate because Step 4 (T-3) adds the preambles.
# ============================================================================

@test "VP-072 AC-011: create-adr/SKILL.md references artifact-path-registry.yaml" {
  # AC-011 traces to BC-4.11.001 PC8: create-adr must read registry before Write.
  grep -q "artifact-path-registry.yaml" "$SKILLS_DIR/create-adr/SKILL.md"
}

@test "VP-072 AC-011: create-architecture/SKILL.md references artifact-path-registry.yaml" {
  # AC-011: create-architecture must read registry before Write.
  grep -q "artifact-path-registry.yaml" "$SKILLS_DIR/create-architecture/SKILL.md"
}

@test "VP-072 AC-011: create-brief/SKILL.md references artifact-path-registry.yaml" {
  # AC-011: create-brief must read registry before Write.
  grep -q "artifact-path-registry.yaml" "$SKILLS_DIR/create-brief/SKILL.md"
}

@test "VP-072 AC-011: create-domain-spec/SKILL.md references artifact-path-registry.yaml" {
  # AC-011: create-domain-spec must read registry before Write.
  grep -q "artifact-path-registry.yaml" "$SKILLS_DIR/create-domain-spec/SKILL.md"
}

@test "VP-072 AC-011: create-excalidraw/SKILL.md references artifact-path-registry.yaml" {
  # AC-011: create-excalidraw must read registry before Write.
  grep -q "artifact-path-registry.yaml" "$SKILLS_DIR/create-excalidraw/SKILL.md"
}

@test "VP-072 AC-011: create-prd/SKILL.md references artifact-path-registry.yaml" {
  # AC-011: create-prd must read registry before Write.
  grep -q "artifact-path-registry.yaml" "$SKILLS_DIR/create-prd/SKILL.md"
}

@test "VP-072 AC-011: create-story/SKILL.md references artifact-path-registry.yaml" {
  # AC-011: create-story must read registry before Write.
  grep -q "artifact-path-registry.yaml" "$SKILLS_DIR/create-story/SKILL.md"
}

@test "VP-072 AC-011: register-artifact/SKILL.md references artifact-path-registry.yaml" {
  # AC-011: register-artifact must read registry before Write.
  grep -q "artifact-path-registry.yaml" "$SKILLS_DIR/register-artifact/SKILL.md"
}

@test "VP-072 AC-011: conform-to-template/SKILL.md references artifact-path-registry.yaml" {
  # AC-011: conform-to-template must read registry before Write.
  grep -q "artifact-path-registry.yaml" "$SKILLS_DIR/conform-to-template/SKILL.md"
}

# ============================================================================
# AC-012 / VP-072: writing-agent prompt preambles reference the registry
# These FAIL at Red Gate because Step 4 (T-3) adds the preambles.
# ============================================================================

@test "VP-072 AC-012: agents/architect.md references artifact-path-registry.yaml" {
  # AC-012 traces to BC-4.11.001 PC8 (F1 Section 7): writing-agent preamble.
  grep -q "artifact-path-registry.yaml" "$PLUGIN_ROOT/agents/architect.md"
}

@test "VP-072 AC-012: agents/product-owner.md references artifact-path-registry.yaml" {
  # AC-012: product-owner agent preamble required.
  grep -q "artifact-path-registry.yaml" "$PLUGIN_ROOT/agents/product-owner.md"
}

@test "VP-072 AC-012: agents/business-analyst.md references artifact-path-registry.yaml" {
  # AC-012: business-analyst agent preamble required.
  grep -q "artifact-path-registry.yaml" "$PLUGIN_ROOT/agents/business-analyst.md"
}

@test "VP-072 AC-012: agents/story-writer.md references artifact-path-registry.yaml" {
  # AC-012: story-writer agent preamble required.
  grep -q "artifact-path-registry.yaml" "$PLUGIN_ROOT/agents/story-writer.md"
}

@test "VP-072 AC-012: agents/technical-writer.md references artifact-path-registry.yaml" {
  # AC-012: technical-writer agent preamble required.
  grep -q "artifact-path-registry.yaml" "$PLUGIN_ROOT/agents/technical-writer.md"
}

# ============================================================================
# AC-013 / VP-072: no skill file embeds a duplicate .factory path list
# These tests PASS today (no embedded path lists exist yet) but serve as
# a regression guard. They will catch violations if Step 4 accidentally
# embeds path lists in skill files.
# ============================================================================

@test "VP-072 AC-013: no skill SKILL.md contains 2+ .factory/specs/behavioral-contracts references" {
  # VP-072 invariant: skills must not embed a path list for BC paths.
  # A single reference (e.g., documentation) is allowed; 2+ is a potential duplicate list.
  local bc_pattern=".factory/specs/behavioral-contracts"
  local violations=0
  while IFS= read -r -d '' skill_file; do
    local count
    count=$(grep -c "$bc_pattern" "$skill_file" 2>/dev/null || true)
    if [ "$count" -ge 2 ]; then
      echo "VIOLATION: $skill_file contains $count references to $bc_pattern"
      violations=$((violations + 1))
    fi
  done < <(find "$SKILLS_DIR" -name "SKILL.md" -print0 2>/dev/null)
  [ "$violations" -eq 0 ]
}

@test "VP-072 AC-013: no skill SKILL.md contains 2+ .factory/specs/verification-properties references" {
  # VP-072 invariant: skills must not embed a path list for VP paths.
  local vp_pattern=".factory/specs/verification-properties"
  local violations=0
  while IFS= read -r -d '' skill_file; do
    local count
    count=$(grep -c "$vp_pattern" "$skill_file" 2>/dev/null || true)
    if [ "$count" -ge 2 ]; then
      echo "VIOLATION: $skill_file contains $count references to $vp_pattern"
      violations=$((violations + 1))
    fi
  done < <(find "$SKILLS_DIR" -name "SKILL.md" -print0 2>/dev/null)
  [ "$violations" -eq 0 ]
}

# ============================================================================
# AC-015 / VP-072: validate-artifact-path hook source has no hardcoded paths
# This PASSES today (lib.rs stub has no hardcoded .factory/ path patterns).
# Kept as a regression guard.
# ============================================================================

@test "VP-072 AC-015: validate-artifact-path lib.rs has no hardcoded .factory/specs/behavioral-contracts paths" {
  # AC-015 traces to BC-4.11.001 invariant 1: hook reads registry at runtime;
  # must not embed any .factory/specs/* path literals.
  local hook_src="$HOOK_SRC_DIR/validate-artifact-path/src/lib.rs"
  if [ -f "$hook_src" ]; then
    local bc_count
    bc_count=$(grep -c ".factory/specs/behavioral-contracts" "$hook_src" 2>/dev/null || true)
    [ "$bc_count" -eq 0 ]
  fi
  # If source file doesn't exist yet, this test passes (no hardcoded paths = compliant).
}

@test "VP-072 AC-015: validate-artifact-path lib.rs has no hardcoded .factory/specs/verification-properties paths" {
  # AC-015: hook source must not embed VP path literals.
  local hook_src="$HOOK_SRC_DIR/validate-artifact-path/src/lib.rs"
  if [ -f "$hook_src" ]; then
    local vp_count
    vp_count=$(grep -c ".factory/specs/verification-properties" "$hook_src" 2>/dev/null || true)
    [ "$vp_count" -eq 0 ]
  fi
}

# ============================================================================
# AC-014 / BC-4.11.001: hooks-registry.toml registration
# This FAILS at Red Gate because Step 4 (T-6) adds the registration
# (ONLY after relocate-artifact reports 0 violations per AC-010/BC-4.11.001 PC5).
# ============================================================================

@test "VP-072 AC-014: hooks-registry.toml contains validate-artifact-path entry" {
  # AC-014 traces to BC-4.11.001 PC1 (precondition 1): hook must be registered
  # in hooks-registry.toml for PreToolUse events.
  # This FAILS at Red Gate — Step 4 T-6 adds the registration (last task).
  grep -q "validate-artifact-path" "$PLUGIN_ROOT/hooks-registry.toml"
}

# ============================================================================
# AC-010 / VP-072: delivery sequencing gate
# relocate-artifact must be registered as a skill (Step 4 T-4 creates it).
# ============================================================================

@test "VP-072 AC-010: relocate-artifact/SKILL.md exists" {
  # AC-010 / BC-6.22.001 PC5/PC10: relocate-artifact skill must exist before
  # validate-artifact-path hook can be registered.
  # This FAILS at Red Gate because Step 4 T-4 creates the skill file.
  [ -f "$SKILLS_DIR/relocate-artifact/SKILL.md" ]
}

@test "VP-072 AC-010: relocate-artifact/SKILL.md references the registry" {
  # BC-6.22.001 invariant 1: relocate-artifact must NOT embed a path list;
  # it reads the registry at runtime.
  grep -q "artifact-path-registry.yaml" "$SKILLS_DIR/relocate-artifact/SKILL.md"
}
