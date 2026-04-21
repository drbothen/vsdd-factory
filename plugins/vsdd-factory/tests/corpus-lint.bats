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
