#!/usr/bin/env bats
# tdd-discipline-gate.bats — RED-phase test suite for S-7.03 TDD Discipline Hardening
#
# Traces to: BC-5.38.001..006, BC-8.29.001..003, BC-8.30.001..002, BC-6.21.001..002
# VPs: VP-063 (integration), VP-064 (manual)
# AC-011 enumerates all 18 tests (a) through (r).
#
# Run from plugin root:
#   bats tests/tdd-discipline-gate.bats

setup() {
  PLUGIN_ROOT="$(cd "$BATS_TEST_DIRNAME/.." && pwd)"
  WORK="$(mktemp -d)"
}

teardown() {
  rm -rf "$WORK"
}

# ============================================================
# Layer 1 — Stub-commit obligations (AC-001, AC-002, AC-003)
# ============================================================

# (a) AC-001: stub-architect.md must contain the todo!() obligation guard text
#     citing BC-5.38.001.
@test "test_stub_architect_uses_todo_for_nontrivial_bodies" {
  local file="$PLUGIN_ROOT/agents/stub-architect.md"
  run grep -F 'todo!()' "$file"
  [ "$status" -eq 0 ]
  run grep -F 'BC-5.38.001' "$file"
  [ "$status" -eq 0 ]
}

# (b) AC-002: per-story-delivery.md must document that GREEN-BY-DESIGN tests are
#     excluded from the RED_RATIO denominator.
@test "test_green_by_design_excluded_from_red_ratio_denominator" {
  local file="$PLUGIN_ROOT/workflows/phases/per-story-delivery.md"
  run grep -F 'GREEN-BY-DESIGN' "$file"
  [ "$status" -eq 0 ]
  run grep -E 'denominator|exclud' "$file"
  [ "$status" -eq 0 ]
}

# (c) AC-003: per-story-delivery.md must document that WIRING-EXEMPT tests are
#     excluded from the RED_RATIO denominator.
@test "test_wiring_exempt_excluded_from_red_ratio_denominator" {
  local file="$PLUGIN_ROOT/workflows/phases/per-story-delivery.md"
  run grep -F 'WIRING-EXEMPT' "$file"
  [ "$status" -eq 0 ]
  run grep -E 'denominator|exclud' "$file"
  [ "$status" -eq 0 ]
}

# ============================================================
# Layer 2 — Anti-precedent guard (AC-004, AC-005)
# ============================================================

# (d) AC-004: deliver-story SKILL.md must contain the verbatim ANTI-PRECEDENT GUARD
#     block including all four SHA commits.
@test "test_anti_precedent_guard_in_deliver_story_skill" {
  local file="$PLUGIN_ROOT/skills/deliver-story/SKILL.md"
  run grep -F 'ANTI-PRECEDENT GUARD:' "$file"
  [ "$status" -eq 0 ]
  run grep -F 'aa706543' "$file"
  [ "$status" -eq 0 ]
  run grep -F '6d2d005e' "$file"
  [ "$status" -eq 0 ]
  run grep -F '20b4a12a' "$file"
  [ "$status" -eq 0 ]
  run grep -F 'e86d03f2' "$file"
  [ "$status" -eq 0 ]
}

# (e) AC-005: per-story-delivery.md Step 2 must contain the verbatim
#     ANTI-PRECEDENT GUARD block.
@test "test_anti_precedent_guard_in_per_story_delivery" {
  local file="$PLUGIN_ROOT/workflows/phases/per-story-delivery.md"
  run grep -F 'ANTI-PRECEDENT GUARD:' "$file"
  [ "$status" -eq 0 ]
}

# (f) AC-005: stub-architect.md must contain the VERBATIM self-check question
#     from BC-5.38.005 invariant 1. Exact fixed-string match required per
#     BC-5.38.005 invariant 2 — paraphrase is not acceptable.
@test "test_self_check_question_in_stub_architect_prompt" {
  local file="$PLUGIN_ROOT/agents/stub-architect.md"
  run grep -F 'If I include this real implementation, will the test for this function pass trivially without any implementer work?' "$file"
  [ "$status" -eq 0 ]
}

# ============================================================
# Layer 3 — Red Gate density check (AC-006, AC-007)
# ============================================================

# (g) AC-006: per-story-delivery.md must contain a "Red Gate Density Check" section.
@test "test_red_ratio_threshold_section_present" {
  local file="$PLUGIN_ROOT/workflows/phases/per-story-delivery.md"
  run grep -F 'Red Gate Density Check' "$file"
  [ "$status" -eq 0 ]
}

# (h) AC-006: per-story-delivery.md must contain the RED_RATIO formula and 0.5
#     threshold.
@test "test_red_ratio_formula_present" {
  local file="$PLUGIN_ROOT/workflows/phases/per-story-delivery.md"
  run grep -F 'RED_RATIO' "$file"
  [ "$status" -eq 0 ]
  run grep -F '0.5' "$file"
  [ "$status" -eq 0 ]
}

# (i) AC-006: per-story-delivery.md must document both remediation options A and B.
@test "test_remediation_options_ab_present" {
  local file="$PLUGIN_ROOT/workflows/phases/per-story-delivery.md"
  run grep -F 'Option A' "$file"
  [ "$status" -eq 0 ]
  run grep -F 'Option B' "$file"
  [ "$status" -eq 0 ]
}

# (j) AC-007: validate-red-ratio.sh blocks (exits non-zero) when red_ratio is
#     below threshold and no exception path is recorded.
#     The hook file must exist (implementation required) AND return non-zero.
@test "test_validate_red_ratio_blocks_on_low_ratio" {
  local hook="$PLUGIN_ROOT/hooks/validate-red-ratio.sh"
  # Hook must exist — if absent this assertion fails loudly (RED gate).
  [ -f "$hook" ]
  local log_file="$WORK/red-gate-log-S-7.03.md"
  cat > "$log_file" <<'EOF'
# Red Gate Log — S-7.03

| Field | Value |
|-------|-------|
| red_ratio: 0.3 |
| total_new_tests: 10 |
| exempt_count: 0 |
| red_count: 3 |
EOF
  INPUT=$(jq -nc --arg fp "$log_file" '{"tool_input":{"file_path":$fp}}')
  run bash -c "echo '$INPUT' | '$hook' 2>&1"
  [ "$status" -ne 0 ]
}

# (k) AC-007: validate-red-ratio.sh exits 0 when red_ratio meets the >= 0.5
#     threshold.
@test "test_validate_red_ratio_passes_on_sufficient_ratio" {
  local hook="$PLUGIN_ROOT/hooks/validate-red-ratio.sh"
  local log_file="$WORK/red-gate-log-S-7.03.md"
  cat > "$log_file" <<'EOF'
# Red Gate Log — S-7.03

| Field | Value |
|-------|-------|
| red_ratio: 0.5 |
| total_new_tests: 10 |
| exempt_count: 0 |
| red_count: 5 |
EOF
  INPUT=$(jq -nc --arg fp "$log_file" '{"tool_input":{"file_path":$fp}}')
  run bash -c "echo '$INPUT' | '$hook' 2>&1"
  [ "$status" -eq 0 ]
}

# (l) AC-007: validate-red-ratio.sh exits 0 when remediation: option_b is recorded
#     even with a low red_ratio.
@test "test_validate_red_ratio_passes_on_option_b_election" {
  local hook="$PLUGIN_ROOT/hooks/validate-red-ratio.sh"
  local log_file="$WORK/red-gate-log-S-7.03.md"
  cat > "$log_file" <<'EOF'
# Red Gate Log — S-7.03

| Field | Value |
|-------|-------|
| red_ratio: 0.3 |
| total_new_tests: 10 |
| exempt_count: 0 |
| red_count: 3 |
| remediation: option_b |
EOF
  INPUT=$(jq -nc --arg fp "$log_file" '{"tool_input":{"file_path":$fp}}')
  run bash -c "echo '$INPUT' | '$hook' 2>&1"
  [ "$status" -eq 0 ]
}

# (m) AC-007: validate-red-ratio must be registered in hooks-registry.toml.
@test "test_validate_red_ratio_registered_in_hooks_registry" {
  local file="$PLUGIN_ROOT/hooks-registry.toml"
  run grep -F 'validate-red-ratio' "$file"
  [ "$status" -eq 0 ]
}

# ============================================================
# Layer 4 — tdd_mode frontmatter (AC-008, AC-009)
# ============================================================

# (n) AC-008: story-template.md must contain the tdd_mode: frontmatter field.
@test "test_tdd_mode_field_in_story_template" {
  local file="$PLUGIN_ROOT/templates/story-template.md"
  run grep -F 'tdd_mode:' "$file"
  [ "$status" -eq 0 ]
}

# (o) AC-008: the tdd_mode: line in story-template.md must document the facade
#     value (inline comment documents both valid values per BC-8.30.001).
@test "test_tdd_mode_comment_documents_both_values" {
  local file="$PLUGIN_ROOT/templates/story-template.md"
  run grep -F 'tdd_mode:' "$file"
  [ "$status" -eq 0 ]
  # The line containing tdd_mode: must also contain "facade" on the same line.
  run grep -E '^[^#]*tdd_mode:.*facade' "$file"
  [ "$status" -eq 0 ]
}

# (p) AC-009: per-story-delivery.md must contain a section describing facade-mode
#     delivery semantics.
@test "test_facade_mode_section_present" {
  local file="$PLUGIN_ROOT/workflows/phases/per-story-delivery.md"
  run grep -E 'tdd_mode: facade|facade-mode' "$file"
  [ "$status" -eq 0 ]
}

# ============================================================
# Layer 5 — Mutation testing wave-gate (AC-010)
# ============================================================

# (q) AC-010: wave-gate SKILL.md must contain a Mutation Testing section that
#     invokes cargo mutants.
@test "test_wave_gate_mutation_section_present" {
  local file="$PLUGIN_ROOT/skills/wave-gate/SKILL.md"
  run grep -F 'cargo mutants' "$file"
  [ "$status" -eq 0 ]
}

# (r) AC-010: wave-gate SKILL.md must document the 80% kill-rate threshold.
#     Both "80" and "kill" must appear in the file (same mutation-testing block).
@test "test_wave_gate_mutation_threshold_80_present" {
  local file="$PLUGIN_ROOT/skills/wave-gate/SKILL.md"
  run grep -F '80' "$file"
  [ "$status" -eq 0 ]
  run grep -E 'kill' "$file"
  [ "$status" -eq 0 ]
}
