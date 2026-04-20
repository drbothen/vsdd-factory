#!/usr/bin/env bats
# novelty-assessment.bats — tests for validate-novelty-assessment PostToolUse hook

setup() {
  HOOK="${BATS_TEST_DIRNAME}/../hooks/validate-novelty-assessment.sh"
  WORK=$(mktemp -d)
  mkdir -p "$WORK/.factory/cycles/v1/adversarial-reviews"
  mkdir -p "$WORK/.factory/phase-f5-adversarial"
  mkdir -p "$WORK/.factory/stories/adversarial-reviews"
  mkdir -p "$WORK/.factory/specs"
}

teardown() {
  rm -rf "$WORK"
}

_write_and_check() {
  local file="$1"
  local content="$2"
  echo "$content" > "$file"
  INPUT=$(jq -nc --arg fp "$file" '{tool_input: {file_path: $fp}}')
  run bash -c "echo '$INPUT' | '$HOOK' 2>&1"
}

VALID_REVIEW="# Adversarial Review — Pass 3

## Critical Findings
None.

## Summary
| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 0 |
| MEDIUM | 2 |
| LOW | 1 |

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 3 |
| **New findings** | 1 |
| **Duplicate/variant findings** | 2 |
| **Novelty score** | 0.33 |
| **Median severity** | 1.5 |
| **Trajectory** | 29→24→21 |
| **Verdict** | FINDINGS_REMAIN |"

CONVERGED_REVIEW="# Adversarial Review — Pass 8

## Summary
No findings.

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 8 |
| **New findings** | 0 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 0.00 |
| **Median severity** | 0.0 |
| **Trajectory** | 29→24→21→7→4→3→2→0 |
| **Verdict** | CONVERGENCE_REACHED |"

MISSING_SECTION="# Adversarial Review — Pass 3

## Critical Findings
None.

## Summary
All good."

MISSING_SCORE="# Adversarial Review — Pass 3

## Summary
| Severity | Count |
|----------|-------|
| CRITICAL | 0 |

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 3 |
| **Trajectory** | 29→24→21 |
| **Verdict** | FINDINGS_REMAIN |"

MISSING_VERDICT="# Adversarial Review — Pass 3

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 3 |
| **Novelty score** | 0.33 |
| **Trajectory** | 29→24→21 |"

# ---------- Passes ----------

@test "novelty-assessment: hook passes syntax check" {
  run bash -n "$HOOK"
  [ "$status" -eq 0 ]
}

@test "novelty-assessment: valid review with all fields passes" {
  _write_and_check "$WORK/.factory/cycles/v1/adversarial-reviews/pass-3.md" "$VALID_REVIEW"
  [ "$status" -eq 0 ]
}

@test "novelty-assessment: converged review passes" {
  _write_and_check "$WORK/.factory/cycles/v1/adversarial-reviews/pass-8.md" "$CONVERGED_REVIEW"
  [ "$status" -eq 0 ]
}

@test "novelty-assessment: ignores non-review files" {
  _write_and_check "$WORK/.factory/specs/prd.md" "# PRD\nNo novelty here."
  [ "$status" -eq 0 ]
}

@test "novelty-assessment: ignores INDEX files" {
  _write_and_check "$WORK/.factory/cycles/v1/adversarial-reviews/ADV-P3-INDEX.md" "# Index"
  [ "$status" -eq 0 ]
}

@test "novelty-assessment: ignores FINDINGS.md" {
  _write_and_check "$WORK/.factory/cycles/v1/adversarial-reviews/FINDINGS.md" "# Findings tracker"
  [ "$status" -eq 0 ]
}

@test "novelty-assessment: ignores individual finding files" {
  _write_and_check "$WORK/.factory/cycles/v1/adversarial-reviews/ADV-P1CONV-P03-CRIT-001.md" "# Finding"
  [ "$status" -eq 0 ]
}

@test "novelty-assessment: ignores convergence-summary files" {
  _write_and_check "$WORK/.factory/phase-f5-adversarial/convergence-summary.md" "# Summary"
  [ "$status" -eq 0 ]
}

@test "novelty-assessment: ignores convergence-trajectory files" {
  _write_and_check "$WORK/.factory/cycles/v1/convergence-trajectory.md" "# Trajectory"
  [ "$status" -eq 0 ]
}

# ---------- Blocks ----------

@test "novelty-assessment: blocks review missing Novelty Assessment section" {
  _write_and_check "$WORK/.factory/cycles/v1/adversarial-reviews/pass-3.md" "$MISSING_SECTION"
  [ "$status" -eq 2 ]
  [[ "$output" == *"Missing '## Novelty Assessment' section"* ]]
}

@test "novelty-assessment: blocks review missing novelty score" {
  _write_and_check "$WORK/.factory/cycles/v1/adversarial-reviews/pass-3.md" "$MISSING_SCORE"
  [ "$status" -eq 2 ]
  [[ "$output" == *"Missing 'Novelty score'"* ]]
}

@test "novelty-assessment: blocks review missing verdict" {
  _write_and_check "$WORK/.factory/cycles/v1/adversarial-reviews/pass-3.md" "$MISSING_VERDICT"
  [ "$status" -eq 2 ]
  [[ "$output" == *"Missing verdict"* ]]
}

# ---------- Delta review files ----------

@test "novelty-assessment: validates adversarial-delta-review files" {
  _write_and_check "$WORK/.factory/phase-f5-adversarial/adversarial-delta-review.md" "$MISSING_SECTION"
  [ "$status" -eq 2 ]
}

@test "novelty-assessment: validates round-N-review files" {
  _write_and_check "$WORK/.factory/phase-f5-adversarial/round-2-review.md" "$MISSING_SECTION"
  [ "$status" -eq 2 ]
}

@test "novelty-assessment: validates gemini-review files" {
  _write_and_check "$WORK/.factory/phase-f5-adversarial/gemini-review.md" "$MISSING_SECTION"
  [ "$status" -eq 2 ]
}

@test "novelty-assessment: validates story adversarial review files" {
  _write_and_check "$WORK/.factory/stories/adversarial-reviews/pass-1.md" "$MISSING_SECTION"
  [ "$status" -eq 2 ]
}

@test "novelty-assessment: valid delta review passes" {
  _write_and_check "$WORK/.factory/phase-f5-adversarial/adversarial-delta-review.md" "$VALID_REVIEW"
  [ "$status" -eq 0 ]
}

# ---------- hooks.json wiring ----------

@test "novelty-assessment: hooks.json wires validate-novelty-assessment" {
  run grep -c "validate-novelty-assessment.sh" "${BATS_TEST_DIRNAME}/../hooks/hooks.json"
  [ "$status" -eq 0 ]
  [ "$output" -ge 1 ]
}
