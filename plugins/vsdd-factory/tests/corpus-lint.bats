#!/usr/bin/env bats
# corpus-lint.bats — tests for corpus lint hooks:
# validate-table-cell-count, validate-changelog-monotonicity,
# validate-input-hash (format extension), validate-state-pin-freshness,
# validate-index-self-reference

setup() {
  PLUGIN_ROOT="$(cd "$BATS_TEST_DIRNAME/.." && pwd)"
  HOOKS="$PLUGIN_ROOT/hooks"
  WORK=$(mktemp -d)
  mkdir -p "$WORK/.factory/specs/behavioral-contracts"
  mkdir -p "$WORK/.factory/specs/verification-properties"
  mkdir -p "$WORK/.factory/specs/architecture"
  mkdir -p "$WORK/.factory/specs/prd-supplements"
  mkdir -p "$WORK/.factory/stories"
  mkdir -p "$WORK/.factory/cycles/phase-2-patch"
}

teardown() {
  rm -rf "$WORK"
}

_run_hook() {
  local hook="$1"
  local file="$2"
  INPUT=$(jq -nc --arg fp "$file" '{tool_input: {file_path: $fp}}')
  run bash -c "echo '$INPUT' | '$HOOKS/$hook' 2>&1"
}

# ========================================================================
# validate-table-cell-count.sh
# ========================================================================

@test "table-cell-count: passes syntax check" {
  run bash -n "$HOOKS/validate-table-cell-count.sh"
  [ "$status" -eq 0 ]
}

@test "table-cell-count: passes valid 5-col table" {
  cat > "$WORK/.factory/specs/test.md" << 'EOF'
| Version | Burst | Date | Author | Change |
|---------|-------|------|--------|--------|
| 1.2 | pass-5 | 2026-04-20 | architect | Updated spec |
| 1.1 | pass-3 | 2026-04-19 | product-owner | Initial |
EOF
  _run_hook validate-table-cell-count.sh "$WORK/.factory/specs/test.md"
  [ "$status" -eq 0 ]
}

@test "table-cell-count: blocks broken pipe in cell" {
  cat > "$WORK/.factory/specs/test.md" << 'EOF'
| Version | Burst | Date | Author | Change |
|---------|-------|------|--------|--------|
| 1.2 | pass-5 | 2026-04-20 | architect | Normalized to canonical 5-col Version | Burst | Date | Author | Change form. |
EOF
  _run_hook validate-table-cell-count.sh "$WORK/.factory/specs/test.md"
  [ "$status" -eq 2 ]
  [[ "$output" == *"pipes vs header"* ]]
}

@test "table-cell-count: allows escaped pipes" {
  cat > "$WORK/.factory/specs/test.md" << 'EOF'
| Version | Burst | Date | Author | Change |
|---------|-------|------|--------|--------|
| 1.2 | pass-5 | 2026-04-20 | architect | Normalized \| Burst \| Date form. |
EOF
  _run_hook validate-table-cell-count.sh "$WORK/.factory/specs/test.md"
  [ "$status" -eq 0 ]
}

@test "table-cell-count: ignores non-factory files" {
  echo "not a factory file" > "$WORK/README.md"
  _run_hook validate-table-cell-count.sh "$WORK/README.md"
  [ "$status" -eq 0 ]
}

@test "table-cell-count: passes file with no tables" {
  cat > "$WORK/.factory/specs/test.md" << 'EOF'
# No tables here
Just text.
EOF
  _run_hook validate-table-cell-count.sh "$WORK/.factory/specs/test.md"
  [ "$status" -eq 0 ]
}

@test "table-cell-count: handles multiple tables in one file" {
  cat > "$WORK/.factory/specs/test.md" << 'EOF'
| A | B |
|---|---|
| 1 | 2 |

Some text.

| X | Y | Z |
|---|---|---|
| 1 | 2 | 3 |
EOF
  _run_hook validate-table-cell-count.sh "$WORK/.factory/specs/test.md"
  [ "$status" -eq 0 ]
}

@test "table-cell-count: catches broken row in second table" {
  cat > "$WORK/.factory/specs/test.md" << 'EOF'
| A | B |
|---|---|
| 1 | 2 |

| X | Y | Z |
|---|---|---|
| 1 | broken | extra | pipe |
EOF
  _run_hook validate-table-cell-count.sh "$WORK/.factory/specs/test.md"
  [ "$status" -eq 2 ]
}

@test "table-cell-count: passes header-only table (no data rows)" {
  cat > "$WORK/.factory/specs/test.md" << 'EOF'
| A | B | C |
|---|---|---|
EOF
  _run_hook validate-table-cell-count.sh "$WORK/.factory/specs/test.md"
  [ "$status" -eq 0 ]
}

# ========================================================================
# validate-changelog-monotonicity.sh
# ========================================================================

@test "changelog-monotonicity: passes syntax check" {
  run bash -n "$HOOKS/validate-changelog-monotonicity.sh"
  [ "$status" -eq 0 ]
}

@test "changelog-monotonicity: passes descending versions" {
  cat > "$WORK/.factory/specs/behavioral-contracts/BC-test.md" << 'EOF'
---
version: "1.3"
---
# BC Test
## Changelog
| Version | Burst | Date | Author | Change |
|---------|-------|------|--------|--------|
| 1.3 | pass-10 | 2026-04-20 | po | Third |
| 1.2 | pass-5 | 2026-04-19 | po | Second |
| 1.1 | pass-1 | 2026-04-18 | po | Initial |
EOF
  _run_hook validate-changelog-monotonicity.sh "$WORK/.factory/specs/behavioral-contracts/BC-test.md"
  [ "$status" -eq 0 ]
}

@test "changelog-monotonicity: blocks duplicate versions" {
  cat > "$WORK/.factory/specs/behavioral-contracts/BC-test.md" << 'EOF'
---
version: "1.2"
---
# BC Test
## Changelog
| Version | Burst | Date | Author | Change |
|---------|-------|------|--------|--------|
| 1.2 | pass-10 | 2026-04-20 | po | Fix |
| 1.2 | pass-5 | 2026-04-19 | po | Initial |
EOF
  _run_hook validate-changelog-monotonicity.sh "$WORK/.factory/specs/behavioral-contracts/BC-test.md"
  [ "$status" -eq 2 ]
  [[ "$output" == *"duplicate version"* ]]
}

@test "changelog-monotonicity: blocks date inversion" {
  cat > "$WORK/.factory/specs/behavioral-contracts/BC-test.md" << 'EOF'
---
version: "1.2"
---
# BC Test
## Changelog
| Version | Burst | Date | Author | Change |
|---------|-------|------|--------|--------|
| 1.2 | pass-10 | 2026-04-18 | po | Older date on newer version |
| 1.1 | pass-5 | 2026-04-20 | po | Newer date on older version |
EOF
  _run_hook validate-changelog-monotonicity.sh "$WORK/.factory/specs/behavioral-contracts/BC-test.md"
  [ "$status" -eq 2 ]
  [[ "$output" == *"newer than prior"* ]]
}

@test "changelog-monotonicity: blocks frontmatter/changelog version mismatch" {
  cat > "$WORK/.factory/specs/behavioral-contracts/BC-test.md" << 'EOF'
---
version: "1.1"
---
# BC Test
## Changelog
| Version | Burst | Date | Author | Change |
|---------|-------|------|--------|--------|
| 1.3 | pass-10 | 2026-04-20 | po | Latest |
| 1.2 | pass-5 | 2026-04-19 | po | Previous |
EOF
  _run_hook validate-changelog-monotonicity.sh "$WORK/.factory/specs/behavioral-contracts/BC-test.md"
  [ "$status" -eq 2 ]
  [[ "$output" == *"Frontmatter version"* ]]
}

@test "changelog-monotonicity: ignores STATE.md" {
  cat > "$WORK/.factory/STATE.md" << 'EOF'
---
version: "1.0"
---
## Changelog
| Version | Date |
|---------|------|
| 1.0 | 2026-04-20 |
| 1.0 | 2026-04-19 |
EOF
  _run_hook validate-changelog-monotonicity.sh "$WORK/.factory/STATE.md"
  [ "$status" -eq 0 ]
}

@test "changelog-monotonicity: blocks ascending version order" {
  cat > "$WORK/.factory/specs/behavioral-contracts/BC-test.md" << 'EOF'
---
version: "1.3"
---
# BC Test
## Changelog
| Version | Burst | Date | Author | Change |
|---------|-------|------|--------|--------|
| 1.1 | pass-1 | 2026-04-18 | po | First |
| 1.2 | pass-5 | 2026-04-19 | po | Second |
| 1.3 | pass-10 | 2026-04-20 | po | Third |
EOF
  _run_hook validate-changelog-monotonicity.sh "$WORK/.factory/specs/behavioral-contracts/BC-test.md"
  [ "$status" -eq 2 ]
  [[ "$output" == *"Frontmatter version"* ]]
}

@test "changelog-monotonicity: allows same-day entries" {
  cat > "$WORK/.factory/specs/behavioral-contracts/BC-test.md" << 'EOF'
---
version: "1.3"
---
# BC Test
## Changelog
| Version | Burst | Date | Author | Change |
|---------|-------|------|--------|--------|
| 1.3 | pass-10 | 2026-04-20 | po | Third |
| 1.2 | pass-5 | 2026-04-20 | po | Second |
| 1.1 | pass-1 | 2026-04-20 | po | Initial |
EOF
  _run_hook validate-changelog-monotonicity.sh "$WORK/.factory/specs/behavioral-contracts/BC-test.md"
  [ "$status" -eq 0 ]
}

@test "changelog-monotonicity: passes file without changelog" {
  cat > "$WORK/.factory/specs/behavioral-contracts/BC-test.md" << 'EOF'
---
version: "1.0"
---
# BC with no changelog section
Just content.
EOF
  _run_hook validate-changelog-monotonicity.sh "$WORK/.factory/specs/behavioral-contracts/BC-test.md"
  [ "$status" -eq 0 ]
}

# ========================================================================
# validate-input-hash.sh (format extension)
# ========================================================================

@test "input-hash: passes syntax check" {
  run bash -n "$HOOKS/validate-input-hash.sh"
  [ "$status" -eq 0 ]
}

@test "input-hash: passes valid 7-char hash" {
  cat > "$WORK/.factory/specs/test.md" << 'EOF'
---
inputs: [prd.md]
input-hash: "abc1234"
---
# Test
EOF
  _run_hook validate-input-hash.sh "$WORK/.factory/specs/test.md"
  [ "$status" -eq 0 ]
}

@test "input-hash: blocks 32-char hash" {
  cat > "$WORK/.factory/specs/test.md" << 'EOF'
---
inputs: [prd.md]
input-hash: "954a3238916e29e1e72e10758b6c91a7"
---
# Test
EOF
  _run_hook validate-input-hash.sh "$WORK/.factory/specs/test.md"
  [ "$status" -eq 2 ]
  [[ "$output" == *"32 chars"* ]]
}

@test "input-hash: blocks uppercase hex" {
  cat > "$WORK/.factory/specs/test.md" << 'EOF'
---
inputs: [prd.md]
input-hash: "ABC1234"
---
# Test
EOF
  _run_hook validate-input-hash.sh "$WORK/.factory/specs/test.md"
  [ "$status" -eq 2 ]
  [[ "$output" == *"invalid chars"* ]]
}

@test "input-hash: allows pending-recompute placeholder" {
  cat > "$WORK/.factory/specs/test.md" << 'EOF'
---
inputs: [prd.md]
input-hash: "[pending-recompute]"
---
# Test
EOF
  _run_hook validate-input-hash.sh "$WORK/.factory/specs/test.md"
  [ "$status" -eq 0 ]
}

@test "input-hash: blocks 6-char hash (too short)" {
  cat > "$WORK/.factory/specs/test.md" << 'EOF'
---
inputs: [prd.md]
input-hash: "abc123"
---
# Test
EOF
  _run_hook validate-input-hash.sh "$WORK/.factory/specs/test.md"
  [ "$status" -eq 2 ]
  [[ "$output" == *"6 chars"* ]]
}

@test "input-hash: allows live-state placeholder" {
  cat > "$WORK/.factory/specs/test.md" << 'EOF'
---
inputs: []
input-hash: "[live-state]"
---
# Test
EOF
  _run_hook validate-input-hash.sh "$WORK/.factory/specs/test.md"
  [ "$status" -eq 0 ]
}

# ========================================================================
# validate-state-pin-freshness.sh
# ========================================================================

@test "state-pin-freshness: passes syntax check" {
  run bash -n "$HOOKS/validate-state-pin-freshness.sh"
  [ "$status" -eq 0 ]
}

@test "state-pin-freshness: ignores non-STATE files" {
  echo "---" > "$WORK/.factory/specs/test.md"
  echo "version: '1.0'" >> "$WORK/.factory/specs/test.md"
  echo "---" >> "$WORK/.factory/specs/test.md"
  _run_hook validate-state-pin-freshness.sh "$WORK/.factory/specs/test.md"
  [ "$status" -eq 0 ]
}

@test "state-pin-freshness: passes when pins match actual versions" {
  # Create actual artifact
  cat > "$WORK/.factory/stories/STORY-INDEX.md" << 'EOF'
---
version: "1.30"
---
# STORY-INDEX
EOF
  # Create STATE.md with matching pin
  cat > "$WORK/.factory/STATE.md" << 'EOF'
---
story_index_version: "1.30"
---
# STATE
EOF
  _run_hook validate-state-pin-freshness.sh "$WORK/.factory/STATE.md"
  [ "$status" -eq 0 ]
}

@test "state-pin-freshness: blocks when pin is stale" {
  cat > "$WORK/.factory/stories/STORY-INDEX.md" << 'EOF'
---
version: "1.30"
---
# STORY-INDEX
EOF
  cat > "$WORK/.factory/STATE.md" << 'EOF'
---
story_index_version: "1.29"
---
# STATE
EOF
  _run_hook validate-state-pin-freshness.sh "$WORK/.factory/STATE.md"
  [ "$status" -eq 2 ]
  [[ "$output" == *"cites '1.29'"* ]]
  [[ "$output" == *"version '1.30'"* ]]
}

@test "state-pin-freshness: handles v-prefix normalization" {
  cat > "$WORK/.factory/stories/STORY-INDEX.md" << 'EOF'
---
version: "1.30"
---
EOF
  cat > "$WORK/.factory/STATE.md" << 'EOF'
---
story_index_version: "v1.30"
---
EOF
  _run_hook validate-state-pin-freshness.sh "$WORK/.factory/STATE.md"
  [ "$status" -eq 0 ]
}

@test "state-pin-freshness: reports multiple stale pins" {
  cat > "$WORK/.factory/stories/STORY-INDEX.md" << 'EOF'
---
version: "1.30"
---
EOF
  cat > "$WORK/.factory/specs/behavioral-contracts/BC-INDEX.md" << 'EOF'
---
version: "4.10"
---
EOF
  cat > "$WORK/.factory/STATE.md" << 'EOF'
---
story_index_version: "1.28"
bc_index_version: "4.08"
---
EOF
  _run_hook validate-state-pin-freshness.sh "$WORK/.factory/STATE.md"
  [ "$status" -eq 2 ]
  [[ "$output" == *"story_index_version"* ]]
  [[ "$output" == *"bc_index_version"* ]]
}

@test "state-pin-freshness: checks vp_index_version" {
  cat > "$WORK/.factory/specs/verification-properties/VP-INDEX.md" << 'EOF'
---
version: "1.6"
---
EOF
  cat > "$WORK/.factory/STATE.md" << 'EOF'
---
vp_index_version: "1.5"
---
EOF
  _run_hook validate-state-pin-freshness.sh "$WORK/.factory/STATE.md"
  [ "$status" -eq 2 ]
  [[ "$output" == *"vp_index_version"* ]]
}

@test "state-pin-freshness: skips missing artifacts" {
  cat > "$WORK/.factory/STATE.md" << 'EOF'
---
story_index_version: "1.30"
bc_index_version: "4.10"
---
EOF
  # Only create STORY-INDEX, not BC-INDEX
  cat > "$WORK/.factory/stories/STORY-INDEX.md" << 'EOF'
---
version: "1.30"
---
EOF
  _run_hook validate-state-pin-freshness.sh "$WORK/.factory/STATE.md"
  [ "$status" -eq 0 ]
}

# ========================================================================
# validate-index-self-reference.sh
# ========================================================================

@test "index-self-reference: passes syntax check" {
  run bash -n "$HOOKS/validate-index-self-reference.sh"
  [ "$status" -eq 0 ]
}

@test "index-self-reference: ignores non-index files" {
  echo "test" > "$WORK/.factory/specs/test.md"
  _run_hook validate-index-self-reference.sh "$WORK/.factory/specs/test.md"
  [ "$status" -eq 0 ]
}

@test "index-self-reference: passes when current pass referenced" {
  cat > "$WORK/.factory/STATE.md" << 'EOF'
---
current_step: "Pass 72 remediation"
---
EOF
  cat > "$WORK/.factory/cycles/phase-2-patch/INDEX.md" << 'EOF'
| pass-71 | complete | 3 |
| pass-72 | in-progress | — |
EOF
  _run_hook validate-index-self-reference.sh "$WORK/.factory/cycles/phase-2-patch/INDEX.md"
  [ "$status" -eq 0 ]
  [[ "$output" != *"SELF-REFERENCE"* ]]
}

@test "index-self-reference: warns when current pass missing" {
  cat > "$WORK/.factory/STATE.md" << 'EOF'
---
current_step: "Pass 72 remediation"
---
EOF
  cat > "$WORK/.factory/cycles/phase-2-patch/INDEX.md" << 'EOF'
| pass-70 | complete | 8 |
| pass-71 | complete | 3 |
EOF
  _run_hook validate-index-self-reference.sh "$WORK/.factory/cycles/phase-2-patch/INDEX.md"
  [ "$status" -eq 0 ]  # advisory only
  [[ "$output" == *"SELF-REFERENCE"* ]]
  [[ "$output" == *"pass-72"* ]]
}

@test "index-self-reference: warns when burst-log missing current burst" {
  cat > "$WORK/.factory/STATE.md" << 'EOF'
---
current_step: "Burst 39 complete"
---
EOF
  cat > "$WORK/.factory/cycles/phase-2-patch/burst-log.md" << 'EOF'
## Burst 37
Fixed things.
## Burst 38
More fixes.
EOF
  _run_hook validate-index-self-reference.sh "$WORK/.factory/cycles/phase-2-patch/burst-log.md"
  [ "$status" -eq 0 ]
  [[ "$output" == *"SELF-REFERENCE"* ]]
  [[ "$output" == *"burst-39"* ]]
}

@test "index-self-reference: handles compound current_step" {
  cat > "$WORK/.factory/STATE.md" << 'EOF'
---
current_step: "Phase 2 patch cycle ��� pass-72 remediation landed; counter 0/3; pass-73 pending"
---
EOF
  cat > "$WORK/.factory/cycles/phase-2-patch/INDEX.md" << 'EOF'
| pass-71 | complete | 3 |
EOF
  _run_hook validate-index-self-reference.sh "$WORK/.factory/cycles/phase-2-patch/INDEX.md"
  [ "$status" -eq 0 ]
  [[ "$output" == *"SELF-REFERENCE"* ]]
  [[ "$output" == *"pass-72"* ]]
}

@test "index-self-reference: skips when no current_step in STATE" {
  cat > "$WORK/.factory/STATE.md" << 'EOF'
---
phase: 2
status: in_progress
---
EOF
  cat > "$WORK/.factory/cycles/phase-2-patch/INDEX.md" << 'EOF'
| pass-71 | complete | 3 |
EOF
  _run_hook validate-index-self-reference.sh "$WORK/.factory/cycles/phase-2-patch/INDEX.md"
  [ "$status" -eq 0 ]
  [[ "$output" != *"SELF-REFERENCE"* ]]
}

@test "index-self-reference: passes when burst-log has current burst" {
  cat > "$WORK/.factory/STATE.md" << 'EOF'
---
current_step: "Burst 39 complete"
---
EOF
  cat > "$WORK/.factory/cycles/phase-2-patch/burst-log.md" << 'EOF'
## Burst 38
More fixes.
## Burst 39
Latest fixes.
EOF
  _run_hook validate-index-self-reference.sh "$WORK/.factory/cycles/phase-2-patch/burst-log.md"
  [ "$status" -eq 0 ]
  [[ "$output" != *"SELF-REFERENCE"* ]]
}

# ========================================================================
# hooks.json wiring
# ========================================================================

@test "corpus-lint: hooks.json wires validate-table-cell-count" {
  grep -q "validate-table-cell-count.sh" "$PLUGIN_ROOT/hooks/hooks.json"
}

@test "corpus-lint: hooks.json wires validate-changelog-monotonicity" {
  grep -q "validate-changelog-monotonicity.sh" "$PLUGIN_ROOT/hooks/hooks.json"
}

@test "corpus-lint: hooks.json wires validate-state-pin-freshness" {
  grep -q "validate-state-pin-freshness.sh" "$PLUGIN_ROOT/hooks/hooks.json"
}

@test "corpus-lint: hooks.json wires validate-index-self-reference" {
  grep -q "validate-index-self-reference.sh" "$PLUGIN_ROOT/hooks/hooks.json"
}

# ========================================================================
# validate-state-index-status-coherence.sh
# ========================================================================

@test "state-index-coherence: passes syntax check" {
  run bash -n "$HOOKS/validate-state-index-status-coherence.sh"
  [ "$status" -eq 0 ]
}

@test "state-index-coherence: passes when STATE and INDEX agree" {
  cat > "$WORK/.factory/STATE.md" << 'EOF'
---
convergence_status: PASS_98_REMEDIATED_AWAITING_PASS_99
---
# STATE
EOF
  mkdir -p "$WORK/.factory/cycles/phase-2-patch"
  cat > "$WORK/.factory/cycles/phase-2-patch/INDEX.md" << 'EOF'
# Phase 2 Patch Cycle
**Status:** PASS-98-REMEDIATED-AWAITING-PASS-99
EOF
  _run_hook validate-state-index-status-coherence.sh "$WORK/.factory/STATE.md"
  [ "$status" -eq 0 ]
}

@test "state-index-coherence: passes when no cycle INDEX exists" {
  cat > "$WORK/.factory/STATE.md" << 'EOF'
---
convergence_status: PASS_5_IN_PROGRESS
---
# STATE
EOF
  # No cycles directory — new project
  _run_hook validate-state-index-status-coherence.sh "$WORK/.factory/STATE.md"
  [ "$status" -eq 0 ]
}

@test "state-index-coherence: warns when INDEX lags behind STATE" {
  cat > "$WORK/.factory/STATE.md" << 'EOF'
---
convergence_status: PASS_99_IN_PROGRESS
---
EOF
  mkdir -p "$WORK/.factory/cycles/phase-2-patch"
  cat > "$WORK/.factory/cycles/phase-2-patch/INDEX.md" << 'EOF'
# Phase 2 Patch Cycle
**Status:** PASS-97-REMEDIATED — awaiting pass-98
EOF
  _run_hook validate-state-index-status-coherence.sh "$WORK/.factory/STATE.md"
  [ "$status" -eq 1 ]
  [[ "$output" == *"COHERENCE WARNING"* ]]
}

@test "state-index-coherence: warns when STATE lags behind INDEX" {
  cat > "$WORK/.factory/STATE.md" << 'EOF'
---
convergence_status: PASS_97_REMEDIATED
---
EOF
  mkdir -p "$WORK/.factory/cycles/phase-2-patch"
  cat > "$WORK/.factory/cycles/phase-2-patch/INDEX.md" << 'EOF'
# Phase 2 Patch Cycle
**Status:** PASS-99-IN-PROGRESS
EOF
  _run_hook validate-state-index-status-coherence.sh "$WORK/.factory/STATE.md"
  [ "$status" -eq 1 ]
  [[ "$output" == *"COHERENCE WARNING"* ]]
}

@test "state-index-coherence: fires when INDEX.md is edited" {
  cat > "$WORK/.factory/STATE.md" << 'EOF'
---
convergence_status: PASS_99_IN_PROGRESS
---
EOF
  mkdir -p "$WORK/.factory/cycles/phase-2-patch"
  cat > "$WORK/.factory/cycles/phase-2-patch/INDEX.md" << 'EOF'
# Phase 2 Patch Cycle
**Status:** PASS-97-REMEDIATED
EOF
  # Fire on INDEX.md edit (not STATE.md)
  _run_hook validate-state-index-status-coherence.sh "$WORK/.factory/cycles/phase-2-patch/INDEX.md"
  [ "$status" -eq 1 ]
}

@test "state-index-coherence: ignores non-factory files" {
  echo "---" > "$WORK/STATE.md"
  echo "convergence_status: PASS_1" >> "$WORK/STATE.md"
  echo "---" >> "$WORK/STATE.md"
  _run_hook validate-state-index-status-coherence.sh "$WORK/STATE.md"
  [ "$status" -eq 0 ]
}

@test "state-index-coherence: skips when no convergence_status field" {
  cat > "$WORK/.factory/STATE.md" << 'EOF'
---
phase: 2
status: in_progress
---
EOF
  mkdir -p "$WORK/.factory/cycles/phase-2-patch"
  cat > "$WORK/.factory/cycles/phase-2-patch/INDEX.md" << 'EOF'
**Status:** PASS-5-IN-PROGRESS
EOF
  _run_hook validate-state-index-status-coherence.sh "$WORK/.factory/STATE.md"
  [ "$status" -eq 0 ]
}

@test "state-index-coherence: handles case-insensitive comparison" {
  cat > "$WORK/.factory/STATE.md" << 'EOF'
---
convergence_status: Pass_5_In_Progress
---
EOF
  mkdir -p "$WORK/.factory/cycles/phase-2-patch"
  cat > "$WORK/.factory/cycles/phase-2-patch/INDEX.md" << 'EOF'
**Status:** pass-5-in-progress
EOF
  _run_hook validate-state-index-status-coherence.sh "$WORK/.factory/STATE.md"
  [ "$status" -eq 0 ]
}

@test "state-index-coherence: trims description after em-dash" {
  cat > "$WORK/.factory/STATE.md" << 'EOF'
---
convergence_status: PASS_10_COMPLETE
---
EOF
  mkdir -p "$WORK/.factory/cycles/phase-2-patch"
  cat > "$WORK/.factory/cycles/phase-2-patch/INDEX.md" << 'EOF'
**Status:** PASS-10-COMPLETE — all findings resolved
EOF
  _run_hook validate-state-index-status-coherence.sh "$WORK/.factory/STATE.md"
  [ "$status" -eq 0 ]
}

@test "state-index-coherence: hooks.json wires the hook" {
  grep -q "validate-state-index-status-coherence.sh" "$PLUGIN_ROOT/hooks/hooks.json"
}

# ========================================================================
# validate-anchor-capabilities-union.sh
# ========================================================================

@test "anchor-caps-union: passes syntax check" {
  run bash -n "$HOOKS/validate-anchor-capabilities-union.sh"
  [ "$status" -eq 0 ]
}

@test "anchor-caps-union: passes single-anchor all same CAP" {
  # Create 3 BCs all with CAP-007
  for i in 001 002 003; do
    cat > "$WORK/.factory/specs/behavioral-contracts/BC-2.04.${i}-test.md" << EOF
---
capability: CAP-007
---
# BC
EOF
  done
  cat > "$WORK/.factory/stories/S-001-test.md" << 'EOF'
---
anchor_bcs: [BC-2.04.001, BC-2.04.002, BC-2.04.003]
anchor_capabilities: [CAP-007]
---
# Story
EOF
  _run_hook validate-anchor-capabilities-union.sh "$WORK/.factory/stories/S-001-test.md"
  [ "$status" -eq 0 ]
}

@test "anchor-caps-union: passes multi-anchor with sorted CAPs" {
  cat > "$WORK/.factory/specs/behavioral-contracts/BC-2.04.001-a.md" << 'EOF'
---
capability: CAP-005
---
EOF
  cat > "$WORK/.factory/specs/behavioral-contracts/BC-2.04.002-b.md" << 'EOF'
---
capability: CAP-006
---
EOF
  cat > "$WORK/.factory/specs/behavioral-contracts/BC-2.04.003-c.md" << 'EOF'
---
capability: CAP-007
---
EOF
  cat > "$WORK/.factory/stories/S-002-test.md" << 'EOF'
---
anchor_bcs: [BC-2.04.001, BC-2.04.002, BC-2.04.003]
anchor_capabilities: [CAP-005, CAP-006, CAP-007]
---
# Story
EOF
  _run_hook validate-anchor-capabilities-union.sh "$WORK/.factory/stories/S-002-test.md"
  [ "$status" -eq 0 ]
}

@test "anchor-caps-union: passes dual-anchor BC with CSV capability" {
  cat > "$WORK/.factory/specs/behavioral-contracts/BC-2.04.001-dual.md" << 'EOF'
---
capability: "CAP-030, CAP-032"
---
EOF
  cat > "$WORK/.factory/stories/S-003-test.md" << 'EOF'
---
anchor_bcs: [BC-2.04.001]
anchor_capabilities: [CAP-030, CAP-032]
---
# Story
EOF
  _run_hook validate-anchor-capabilities-union.sh "$WORK/.factory/stories/S-003-test.md"
  [ "$status" -eq 0 ]
}

@test "anchor-caps-union: blocks wrong capability" {
  cat > "$WORK/.factory/specs/behavioral-contracts/BC-2.04.001-x.md" << 'EOF'
---
capability: CAP-006
---
EOF
  cat > "$WORK/.factory/stories/S-004-test.md" << 'EOF'
---
anchor_bcs: [BC-2.04.001]
anchor_capabilities: [CAP-005]
---
# Story
EOF
  _run_hook validate-anchor-capabilities-union.sh "$WORK/.factory/stories/S-004-test.md"
  [ "$status" -eq 2 ]
  [[ "$output" == *"UNION VIOLATION"* ]]
  [[ "$output" == *"CAP-006"* ]]
}

@test "anchor-caps-union: blocks incomplete capabilities" {
  cat > "$WORK/.factory/specs/behavioral-contracts/BC-2.04.001-y.md" << 'EOF'
---
capability: CAP-005
---
EOF
  cat > "$WORK/.factory/specs/behavioral-contracts/BC-2.04.002-y.md" << 'EOF'
---
capability: CAP-006
---
EOF
  cat > "$WORK/.factory/stories/S-005-test.md" << 'EOF'
---
anchor_bcs: [BC-2.04.001, BC-2.04.002]
anchor_capabilities: [CAP-005]
---
# Story
EOF
  _run_hook validate-anchor-capabilities-union.sh "$WORK/.factory/stories/S-005-test.md"
  [ "$status" -eq 2 ]
  [[ "$output" == *"UNION VIOLATION"* ]]
  [[ "$output" == *"CAP-006"* ]]
}

@test "anchor-caps-union: blocks spurious capability" {
  cat > "$WORK/.factory/specs/behavioral-contracts/BC-2.04.001-z.md" << 'EOF'
---
capability: CAP-005
---
EOF
  cat > "$WORK/.factory/stories/S-006-test.md" << 'EOF'
---
anchor_bcs: [BC-2.04.001]
anchor_capabilities: [CAP-004, CAP-005]
---
# Story
EOF
  _run_hook validate-anchor-capabilities-union.sh "$WORK/.factory/stories/S-006-test.md"
  [ "$status" -eq 2 ]
  [[ "$output" == *"UNION VIOLATION"* ]]
}

@test "anchor-caps-union: skips story with no anchor_bcs" {
  cat > "$WORK/.factory/stories/S-007-test.md" << 'EOF'
---
anchor_capabilities: [CAP-005]
---
# Story with no anchor_bcs
EOF
  _run_hook validate-anchor-capabilities-union.sh "$WORK/.factory/stories/S-007-test.md"
  [ "$status" -eq 0 ]
}

@test "anchor-caps-union: skips non-story files" {
  cat > "$WORK/.factory/specs/behavioral-contracts/BC-test.md" << 'EOF'
---
capability: CAP-005
---
EOF
  _run_hook validate-anchor-capabilities-union.sh "$WORK/.factory/specs/behavioral-contracts/BC-test.md"
  [ "$status" -eq 0 ]
}

@test "anchor-caps-union: warns but passes when BC file not found" {
  cat > "$WORK/.factory/stories/S-008-test.md" << 'EOF'
---
anchor_bcs: [BC-9.99.999]
anchor_capabilities: [CAP-001]
---
# Story
EOF
  _run_hook validate-anchor-capabilities-union.sh "$WORK/.factory/stories/S-008-test.md"
  # Should pass (not block) — BC may be new/not yet created
  [ "$status" -eq 0 ]
  [[ "$output" == *"not found"* ]]
}

@test "anchor-caps-union: handles behavioral_contracts field name" {
  cat > "$WORK/.factory/specs/behavioral-contracts/BC-2.04.001-alt.md" << 'EOF'
---
capability: CAP-007
---
EOF
  cat > "$WORK/.factory/stories/S-009-test.md" << 'EOF'
---
behavioral_contracts: [BC-2.04.001]
anchor_capabilities: [CAP-007]
---
# Story
EOF
  _run_hook validate-anchor-capabilities-union.sh "$WORK/.factory/stories/S-009-test.md"
  [ "$status" -eq 0 ]
}

@test "anchor-caps-union: handles STORY- prefix files" {
  cat > "$WORK/.factory/specs/behavioral-contracts/BC-2.04.001-st.md" << 'EOF'
---
capability: CAP-007
---
EOF
  cat > "$WORK/.factory/stories/STORY-001-test.md" << 'EOF'
---
anchor_bcs: [BC-2.04.001]
anchor_capabilities: [CAP-007]
---
# Story
EOF
  _run_hook validate-anchor-capabilities-union.sh "$WORK/.factory/stories/STORY-001-test.md"
  [ "$status" -eq 0 ]
}

@test "anchor-caps-union: shows BC→CAP mapping in error" {
  cat > "$WORK/.factory/specs/behavioral-contracts/BC-2.04.001-map.md" << 'EOF'
---
capability: CAP-006
---
EOF
  cat > "$WORK/.factory/stories/S-010-test.md" << 'EOF'
---
anchor_bcs: [BC-2.04.001]
anchor_capabilities: [CAP-005]
---
EOF
  _run_hook validate-anchor-capabilities-union.sh "$WORK/.factory/stories/S-010-test.md"
  [ "$status" -eq 2 ]
  [[ "$output" == *"BC-2.04.001:CAP-006"* ]]
}

@test "anchor-caps-union: hooks.json wires the hook" {
  grep -q "validate-anchor-capabilities-union.sh" "$PLUGIN_ROOT/hooks/hooks.json"
}

# ========================================================================
# validate-demo-evidence-story-scoped.sh
# ========================================================================

@test "demo-evidence-scoped: passes syntax check" {
  run bash -n "$HOOKS/validate-demo-evidence-story-scoped.sh"
  [ "$status" -eq 0 ]
}

@test "demo-evidence-scoped: passes file in story subdirectory" {
  mkdir -p "$WORK/docs/demo-evidence/S-0.02"
  echo "# Evidence" > "$WORK/docs/demo-evidence/S-0.02/evidence-report.md"
  _run_hook validate-demo-evidence-story-scoped.sh "$WORK/docs/demo-evidence/S-0.02/evidence-report.md"
  [ "$status" -eq 0 ]
}

@test "demo-evidence-scoped: blocks flat-level file" {
  mkdir -p "$WORK/docs/demo-evidence"
  echo "# Evidence" > "$WORK/docs/demo-evidence/evidence-report.md"
  _run_hook validate-demo-evidence-story-scoped.sh "$WORK/docs/demo-evidence/evidence-report.md"
  [ "$status" -eq 2 ]
  [[ "$output" == *"POL-010"* ]]
}

@test "demo-evidence-scoped: blocks flat AC file" {
  mkdir -p "$WORK/docs/demo-evidence"
  echo "# AC" > "$WORK/docs/demo-evidence/AC-001-test.md"
  _run_hook validate-demo-evidence-story-scoped.sh "$WORK/docs/demo-evidence/AC-001-test.md"
  [ "$status" -eq 2 ]
  [[ "$output" == *"POL-010"* ]]
}

@test "demo-evidence-scoped: passes non-demo-evidence file" {
  echo "test" > "$WORK/.factory/specs/test.md"
  _run_hook validate-demo-evidence-story-scoped.sh "$WORK/.factory/specs/test.md"
  [ "$status" -eq 0 ]
}

@test "demo-evidence-scoped: passes gif in story subdirectory" {
  mkdir -p "$WORK/docs/demo-evidence/S-6.06"
  echo "gif content" > "$WORK/docs/demo-evidence/S-6.06/AC-001-test.gif"
  _run_hook validate-demo-evidence-story-scoped.sh "$WORK/docs/demo-evidence/S-6.06/AC-001-test.gif"
  [ "$status" -eq 0 ]
}

@test "demo-evidence-scoped: hooks.json wires the hook" {
  grep -q "validate-demo-evidence-story-scoped.sh" "$PLUGIN_ROOT/hooks/hooks.json"
}

# ========================================================================
# validate-factory-path-root.sh
# ========================================================================

@test "factory-path-root: passes syntax check" {
  run bash -n "$HOOKS/validate-factory-path-root.sh"
  [ "$status" -eq 0 ]
}

@test "factory-path-root: passes write to project root .factory/" {
  _run_hook validate-factory-path-root.sh "/home/user/project/.factory/STATE.md"
  [ "$status" -eq 0 ]
}

@test "factory-path-root: passes write to nested .factory/ path" {
  _run_hook validate-factory-path-root.sh "/home/user/project/.factory/cycles/phase-3/STORY-001/implementation/red-gate-log.md"
  [ "$status" -eq 0 ]
}

@test "factory-path-root: blocks write to .worktrees/STORY-NNN/.factory/" {
  _run_hook validate-factory-path-root.sh "/home/user/project/.worktrees/STORY-001/.factory/STATE.md"
  [ "$status" -eq 2 ]
  [[ "$output" == *"FACTORY PATH ERROR"* ]]
  [[ "$output" == *"worktree instead of project root"* ]]
}

@test "factory-path-root: blocks worktree .factory/ with nested path" {
  _run_hook validate-factory-path-root.sh "/home/user/project/.worktrees/STORY-042/.factory/cycles/phase-3/STORY-042/implementation/red-gate-log.md"
  [ "$status" -eq 2 ]
  [[ "$output" == *".worktrees/STORY-042"* ]]
}

@test "factory-path-root: shows expected path in error" {
  _run_hook validate-factory-path-root.sh "/home/user/project/.worktrees/STORY-001/.factory/stories/red-gate-log.md"
  [ "$status" -eq 2 ]
  [[ "$output" == *"stories/red-gate-log.md"* ]]
}

@test "factory-path-root: ignores non-.factory/ writes" {
  _run_hook validate-factory-path-root.sh "/home/user/project/src/main.rs"
  [ "$status" -eq 0 ]
}

@test "factory-path-root: ignores docs/ writes in worktrees" {
  _run_hook validate-factory-path-root.sh "/home/user/project/.worktrees/STORY-001/docs/demo-evidence/S-0.01/AC-001.gif"
  [ "$status" -eq 0 ]
}

@test "factory-path-root: hooks.json wires the hook" {
  grep -q "validate-factory-path-root.sh" "$PLUGIN_ROOT/hooks/hooks.json"
}
