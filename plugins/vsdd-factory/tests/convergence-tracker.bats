#!/usr/bin/env bats
# convergence-tracker.bats — tests for convergence-tracker PostToolUse hook
#
# Tests lightweight convergence rule enforcement:
# 1. Trajectory monotonicity
# 2. Minimum 3 clean passes for CONVERGENCE_REACHED
# 3. Novelty score vs verdict consistency
# 4. Zero-findings first pass warning

setup() {
  HOOK="${BATS_TEST_DIRNAME}/../hooks/convergence-tracker.sh"
  WORK=$(mktemp -d)
  mkdir -p "$WORK/.factory/cycles/v1/adversarial-reviews"
  mkdir -p "$WORK/.factory/phase-f5-adversarial"
  mkdir -p "$WORK/.factory/specs"
}

teardown() {
  rm -rf "$WORK"
}

_write_and_check() {
  local file="$1"
  local content="$2"
  printf '%s' "$content" > "$file"
  INPUT=$(jq -nc --arg fp "$file" '{tool_input: {file_path: $fp}}')
  run bash -c "echo '$INPUT' | '$HOOK' 2>&1"
}

_make_review() {
  local pass="$1"
  local crit="$2"
  local high="$3"
  local med="$4"
  local low="$5"
  local novelty="$6"
  local trajectory="$7"
  local verdict="$8"
  local new_findings="${9:-$((crit + high + med + low))}"
  local dup="${10:-0}"

  cat <<EOF
# Adversarial Review — Pass $pass

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | $crit |
| HIGH | $high |
| MEDIUM | $med |
| LOW | $low |

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | $pass |
| **New findings** | $new_findings |
| **Duplicate/variant findings** | $dup |
| **Novelty score** | $novelty |
| **Median severity** | 2.0 |
| **Trajectory** | $trajectory |
| **Verdict** | $verdict |
EOF
}

# ---------- Syntax ----------

@test "convergence-tracker: hook passes syntax check" {
  run bash -n "$HOOK"
  [ "$status" -eq 0 ]
}

# ---------- Ignores non-review files ----------

@test "convergence-tracker: ignores non-review files" {
  echo "not a review" > "$WORK/.factory/specs/prd.md"
  INPUT=$(jq -nc --arg fp "$WORK/.factory/specs/prd.md" '{tool_input: {file_path: $fp}}')
  run bash -c "echo '$INPUT' | '$HOOK' 2>&1"
  [ "$status" -eq 0 ]
}

@test "convergence-tracker: ignores INDEX files" {
  echo "index" > "$WORK/.factory/cycles/v1/adversarial-reviews/ADV-P3-INDEX.md"
  INPUT=$(jq -nc --arg fp "$WORK/.factory/cycles/v1/adversarial-reviews/ADV-P3-INDEX.md" '{tool_input: {file_path: $fp}}')
  run bash -c "echo '$INPUT' | '$HOOK' 2>&1"
  [ "$status" -eq 0 ]
}

# ---------- Rule 1: Zero-findings first pass ----------

@test "convergence-tracker: warns on zero findings pass 1" {
  local review
  review=$(_make_review 1 0 0 0 0 "0.00" "0" "FINDINGS_REMAIN" 0 0)
  _write_and_check "$WORK/.factory/cycles/v1/adversarial-reviews/pass-1.md" "$review"
  [ "$status" -eq 0 ]  # warn only, not block
  [[ "$output" == *"Zero findings on first adversary pass"* ]]
}

@test "convergence-tracker: no warning when pass 1 has findings" {
  local review
  review=$(_make_review 1 2 3 4 1 "1.00" "10" "FINDINGS_REMAIN")
  _write_and_check "$WORK/.factory/cycles/v1/adversarial-reviews/pass-1.md" "$review"
  [ "$status" -eq 0 ]
  [[ "$output" != *"Zero findings"* ]]
}

# ---------- Rule 2: Trajectory monotonicity ----------

@test "convergence-tracker: warns on trajectory regression" {
  local review
  review=$(_make_review 5 0 0 2 1 "0.30" "29→24→21→7→12" "FINDINGS_REMAIN")
  _write_and_check "$WORK/.factory/cycles/v1/adversarial-reviews/pass-5.md" "$review"
  [ "$status" -eq 0 ]  # warn only
  [[ "$output" == *"Trajectory monotonicity violation"* ]]
}

@test "convergence-tracker: passes with decreasing trajectory" {
  local review
  review=$(_make_review 5 0 0 2 1 "0.30" "29→24→21→7→4" "FINDINGS_REMAIN")
  _write_and_check "$WORK/.factory/cycles/v1/adversarial-reviews/pass-5.md" "$review"
  [ "$status" -eq 0 ]
  [[ "$output" != *"monotonicity"* ]]
}

@test "convergence-tracker: passes with flat trajectory" {
  local review
  review=$(_make_review 5 0 0 2 1 "0.30" "29→24→21→7→7" "FINDINGS_REMAIN")
  _write_and_check "$WORK/.factory/cycles/v1/adversarial-reviews/pass-5.md" "$review"
  [ "$status" -eq 0 ]
  [[ "$output" != *"monotonicity"* ]]
}

# ---------- Rule 3: Novelty score vs verdict ----------

@test "convergence-tracker: blocks CONVERGENCE_REACHED with high novelty" {
  local review
  review=$(_make_review 5 0 0 0 0 "0.45" "29→24→21→7→4" "CONVERGENCE_REACHED" 0 0)
  # Need 3 clean prior passes for clean-pass check
  _make_review 2 0 0 1 0 "0.30" "29→24" "FINDINGS_REMAIN" > "$WORK/.factory/cycles/v1/adversarial-reviews/pass-2.md"
  _make_review 3 0 0 0 1 "0.20" "29→24→21" "FINDINGS_REMAIN" > "$WORK/.factory/cycles/v1/adversarial-reviews/pass-3.md"
  _make_review 4 0 0 0 0 "0.10" "29→24→21→7" "FINDINGS_REMAIN" 0 0 > "$WORK/.factory/cycles/v1/adversarial-reviews/pass-4.md"
  _write_and_check "$WORK/.factory/cycles/v1/adversarial-reviews/pass-5.md" "$review"
  [ "$status" -eq 2 ]
  [[ "$output" == *"novelty score is 0.45"* ]]
}

@test "convergence-tracker: allows CONVERGENCE_REACHED with low novelty" {
  # Create 3 prior clean passes
  _make_review 3 0 0 0 1 "0.20" "29→24→21" "FINDINGS_REMAIN" > "$WORK/.factory/cycles/v1/adversarial-reviews/pass-3.md"
  _make_review 4 0 0 0 0 "0.10" "29→24→21→7" "FINDINGS_REMAIN" 0 0 > "$WORK/.factory/cycles/v1/adversarial-reviews/pass-4.md"
  _make_review 5 0 0 0 0 "0.05" "29→24→21→7→0" "FINDINGS_REMAIN" 0 0 > "$WORK/.factory/cycles/v1/adversarial-reviews/pass-5.md"

  local review
  review=$(_make_review 6 0 0 0 0 "0.05" "29→24→21→7→0→0" "CONVERGENCE_REACHED" 0 0)
  _write_and_check "$WORK/.factory/cycles/v1/adversarial-reviews/pass-6.md" "$review"
  [ "$status" -eq 0 ]
}

@test "convergence-tracker: blocks CONVERGENCE_REACHED with CRIT findings" {
  _make_review 3 0 0 0 0 "0.10" "29→24→21" "FINDINGS_REMAIN" 0 0 > "$WORK/.factory/cycles/v1/adversarial-reviews/pass-3.md"
  _make_review 4 0 0 0 0 "0.05" "29→24→21→0" "FINDINGS_REMAIN" 0 0 > "$WORK/.factory/cycles/v1/adversarial-reviews/pass-4.md"

  local review
  review=$(_make_review 5 1 0 0 0 "0.10" "29→24→21→0→1" "CONVERGENCE_REACHED")
  _write_and_check "$WORK/.factory/cycles/v1/adversarial-reviews/pass-5.md" "$review"
  [ "$status" -eq 2 ]
  [[ "$output" == *"CRITICAL findings"* ]]
}

@test "convergence-tracker: blocks CONVERGENCE_REACHED with HIGH findings" {
  _make_review 3 0 0 0 0 "0.10" "29→24→21" "FINDINGS_REMAIN" 0 0 > "$WORK/.factory/cycles/v1/adversarial-reviews/pass-3.md"
  _make_review 4 0 0 0 0 "0.05" "29→24→21→0" "FINDINGS_REMAIN" 0 0 > "$WORK/.factory/cycles/v1/adversarial-reviews/pass-4.md"

  local review
  review=$(_make_review 5 0 2 0 0 "0.10" "29→24→21→0→2" "CONVERGENCE_REACHED")
  _write_and_check "$WORK/.factory/cycles/v1/adversarial-reviews/pass-5.md" "$review"
  [ "$status" -eq 2 ]
  [[ "$output" == *"HIGH findings"* ]]
}

# ---------- Rule 4: Minimum 3 clean passes ----------

@test "convergence-tracker: blocks CONVERGENCE_REACHED with only 1 clean pass" {
  # Only 1 prior clean pass
  _make_review 3 0 1 0 0 "0.30" "29→24→21" "FINDINGS_REMAIN" > "$WORK/.factory/cycles/v1/adversarial-reviews/pass-3.md"
  _make_review 4 0 0 0 0 "0.10" "29→24→21→0" "FINDINGS_REMAIN" 0 0 > "$WORK/.factory/cycles/v1/adversarial-reviews/pass-4.md"

  local review
  review=$(_make_review 5 0 0 0 0 "0.05" "29→24→21→0→0" "CONVERGENCE_REACHED" 0 0)
  _write_and_check "$WORK/.factory/cycles/v1/adversarial-reviews/pass-5.md" "$review"
  [ "$status" -eq 2 ]
  [[ "$output" == *"only 2 consecutive clean passes"* ]]
}

@test "convergence-tracker: allows CONVERGENCE_REACHED with 3+ clean passes" {
  # 3 prior clean passes + current = 4 clean streak
  _make_review 3 0 0 0 1 "0.20" "29→24→21" "FINDINGS_REMAIN" > "$WORK/.factory/cycles/v1/adversarial-reviews/pass-3.md"
  _make_review 4 0 0 0 0 "0.10" "29→24→21→0" "FINDINGS_REMAIN" 0 0 > "$WORK/.factory/cycles/v1/adversarial-reviews/pass-4.md"
  _make_review 5 0 0 0 0 "0.05" "29→24→21→0→0" "FINDINGS_REMAIN" 0 0 > "$WORK/.factory/cycles/v1/adversarial-reviews/pass-5.md"

  local review
  review=$(_make_review 6 0 0 0 0 "0.03" "29→24→21→0→0→0" "CONVERGENCE_REACHED" 0 0)
  _write_and_check "$WORK/.factory/cycles/v1/adversarial-reviews/pass-6.md" "$review"
  [ "$status" -eq 0 ]
}

# ---------- FINDINGS_REMAIN is always OK ----------

@test "convergence-tracker: FINDINGS_REMAIN always passes (no convergence claims)" {
  local review
  review=$(_make_review 3 2 5 3 1 "0.80" "29→24→21" "FINDINGS_REMAIN")
  _write_and_check "$WORK/.factory/cycles/v1/adversarial-reviews/pass-3.md" "$review"
  [ "$status" -eq 0 ]
}

# ---------- Phase F5 files ----------

@test "convergence-tracker: validates phase-f5 adversarial-delta-review" {
  local review
  review=$(_make_review 1 1 0 0 0 "1.00" "1" "CONVERGENCE_REACHED")
  _write_and_check "$WORK/.factory/phase-f5-adversarial/adversarial-delta-review.md" "$review"
  [ "$status" -eq 2 ]  # CRIT + no clean streak
}

# ---------- hooks.json ----------

@test "convergence-tracker: hooks-registry.toml wires convergence-tracker" {
  run grep -c "script_path = \"hooks/convergence-tracker.sh\"" "${BATS_TEST_DIRNAME}/../hooks-registry.toml"
  [ "$status" -eq 0 ]
  [ "$output" -ge 1 ]
}
