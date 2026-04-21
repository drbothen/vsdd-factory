#!/usr/bin/env bats
# input-hash-scan.bats — tests for compute-input-hash --scan mode
#
# Tests the batch scanning mode that walks a directory and reports/fixes
# input-hash drift across all artifacts.

setup() {
  PLUGIN_ROOT="$(cd "$BATS_TEST_DIRNAME/.." && pwd)"
  BIN="$PLUGIN_ROOT/bin/compute-input-hash"
  WORK=$(mktemp -d)

  # Create a minimal .factory/ structure
  mkdir -p "$WORK/.factory/specs/behavioral-contracts"
  mkdir -p "$WORK/.factory/specs/architecture"
  mkdir -p "$WORK/.factory/stories"

  # Create a source file that artifacts reference
  cat > "$WORK/.factory/specs/source.md" << 'SRCEOF'
---
document_type: source
---
# Source doc
Original content.
SRCEOF
}

teardown() {
  rm -rf "$WORK"
}

_make_artifact() {
  local file="$1"
  local hash="$2"
  local inputs="$3"
  cat > "$file" << EOF
---
document_type: test-artifact
inputs: [$inputs]
input-hash: "$hash"
---
# Test artifact
EOF
}

_compute_fresh_hash() {
  # Compute the correct hash for source.md
  "$BIN" "$WORK/.factory/specs/behavioral-contracts/fresh.md" 2>/dev/null
}

# ========================================================================
# Back-compat: existing per-file invocations unchanged
# ========================================================================

@test "scan: per-file print mode still works" {
  _make_artifact "$WORK/.factory/specs/behavioral-contracts/test.md" "[md5]" "source.md"
  run "$BIN" "$WORK/.factory/specs/behavioral-contracts/test.md"
  [ "$status" -eq 0 ]
  [[ "$output" =~ ^[0-9a-f]{7}$ ]]
}

@test "scan: per-file --update mode still works" {
  _make_artifact "$WORK/.factory/specs/behavioral-contracts/test.md" "[md5]" "source.md"
  run "$BIN" "$WORK/.factory/specs/behavioral-contracts/test.md" --update
  [ "$status" -eq 0 ]
  # Verify hash was written to frontmatter
  run grep "^input-hash:" "$WORK/.factory/specs/behavioral-contracts/test.md"
  [[ "$output" != *"[md5]"* ]]
}

@test "scan: per-file --check mode still works (match)" {
  # First compute the correct hash
  _make_artifact "$WORK/.factory/specs/behavioral-contracts/test.md" "[md5]" "source.md"
  "$BIN" "$WORK/.factory/specs/behavioral-contracts/test.md" --update
  # Now check — should match
  run "$BIN" "$WORK/.factory/specs/behavioral-contracts/test.md" --check
  [ "$status" -eq 0 ]
}

@test "scan: per-file --check mode still works (mismatch)" {
  _make_artifact "$WORK/.factory/specs/behavioral-contracts/test.md" "aaaaaaa" "source.md"
  run "$BIN" "$WORK/.factory/specs/behavioral-contracts/test.md" --check
  [ "$status" -eq 2 ]
}

# ========================================================================
# --scan reports drift
# ========================================================================

@test "scan: reports drift with exit 2" {
  # Fresh artifact (correct hash)
  _make_artifact "$WORK/.factory/specs/behavioral-contracts/fresh.md" "[md5]" "source.md"
  "$BIN" "$WORK/.factory/specs/behavioral-contracts/fresh.md" --update

  # Stale artifact (wrong hash)
  _make_artifact "$WORK/.factory/specs/behavioral-contracts/stale.md" "aaaaaaa" "source.md"

  run "$BIN" --scan "$WORK/.factory"
  [ "$status" -eq 2 ]
  [[ "$output" == *"STALE=1"* ]]
  [[ "$output" == *"MATCH=1"* ]]
}

@test "scan: stderr lists stale file path" {
  _make_artifact "$WORK/.factory/specs/behavioral-contracts/stale.md" "aaaaaaa" "source.md"

  run "$BIN" --scan "$WORK/.factory" 2>&1
  [[ "$output" == *"stale.md"* ]]
}

@test "scan: exit 0 when no drift" {
  _make_artifact "$WORK/.factory/specs/behavioral-contracts/fresh.md" "[md5]" "source.md"
  "$BIN" "$WORK/.factory/specs/behavioral-contracts/fresh.md" --update

  run "$BIN" --scan "$WORK/.factory"
  [ "$status" -eq 0 ]
  [[ "$output" == *"STALE=0"* ]]
}

# ========================================================================
# --scan --update remediates
# ========================================================================

@test "scan: --update fixes stale hashes" {
  _make_artifact "$WORK/.factory/specs/behavioral-contracts/stale.md" "aaaaaaa" "source.md"

  run "$BIN" --scan "$WORK/.factory" --update
  [ "$status" -eq 0 ]
  [[ "$output" == *"UPDATED=1"* ]]
}

@test "scan: subsequent --scan after --update shows no drift" {
  _make_artifact "$WORK/.factory/specs/behavioral-contracts/stale.md" "aaaaaaa" "source.md"

  # Fix it
  "$BIN" --scan "$WORK/.factory" --update >/dev/null 2>&1

  # Re-scan — should be clean
  run "$BIN" --scan "$WORK/.factory"
  [ "$status" -eq 0 ]
  [[ "$output" == *"STALE=0"* ]]
}

# ========================================================================
# --scan skips INDEX files
# ========================================================================

@test "scan: skips INDEX.md files" {
  # Create INDEX files that should be skipped
  _make_artifact "$WORK/.factory/specs/behavioral-contracts/BC-INDEX.md" "aaaaaaa" "source.md"
  _make_artifact "$WORK/.factory/stories/STORY-INDEX.md" "aaaaaaa" "source.md"
  cat > "$WORK/.factory/specs/architecture/INDEX.md" << 'EOF'
---
inputs: [source.md]
input-hash: "aaaaaaa"
---
EOF

  # Create one real artifact
  _make_artifact "$WORK/.factory/specs/behavioral-contracts/real.md" "[md5]" "source.md"
  "$BIN" "$WORK/.factory/specs/behavioral-contracts/real.md" --update

  run "$BIN" --scan "$WORK/.factory"
  [ "$status" -eq 0 ]
  [[ "$output" == *"TOTAL=1"* ]]
}

# ========================================================================
# --scan handles no-inputs artifacts
# ========================================================================

@test "scan: handles artifacts with missing input files" {
  _make_artifact "$WORK/.factory/specs/behavioral-contracts/orphan.md" "aaaaaaa" "nonexistent.md"

  run "$BIN" --scan "$WORK/.factory"
  [ "$status" -eq 0 ]
  [[ "$output" == *"NOINPUT=1"* ]]
}

# ========================================================================
# --scan handles uncomputed hashes
# ========================================================================

@test "scan: handles null input-hash as uncomputed" {
  cat > "$WORK/.factory/specs/behavioral-contracts/nullhash.md" << 'EOF'
---
inputs: [source.md]
input-hash: null
---
# Test
EOF

  run "$BIN" --scan "$WORK/.factory"
  [ "$status" -eq 0 ]
  [[ "$output" == *"UNCOMPUTED=1"* ]]
}

@test "scan: handles [md5] placeholder as uncomputed" {
  _make_artifact "$WORK/.factory/specs/behavioral-contracts/placeholder.md" "[md5]" "source.md"

  run "$BIN" --scan "$WORK/.factory"
  [ "$status" -eq 0 ]
  [[ "$output" == *"UNCOMPUTED=1"* ]]
}

# ========================================================================
# --scan summary line format
# ========================================================================

@test "scan: stdout is single summary line with all fields" {
  _make_artifact "$WORK/.factory/specs/behavioral-contracts/test.md" "[md5]" "source.md"
  "$BIN" "$WORK/.factory/specs/behavioral-contracts/test.md" --update

  run "$BIN" --scan "$WORK/.factory"
  # Should be exactly one non-empty stdout line with all fields
  local summary
  summary=$(echo "$output" | grep "^TOTAL=")
  [[ "$summary" == *"TOTAL="* ]]
  [[ "$summary" == *"MATCH="* ]]
  [[ "$summary" == *"STALE="* ]]
  [[ "$summary" == *"UNCOMPUTED="* ]]
  [[ "$summary" == *"NOINPUT="* ]]
  [[ "$summary" == *"UPDATED="* ]]
  [[ "$summary" == *"UPDATE_FAILED="* ]]
}
