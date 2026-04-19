#!/usr/bin/env bats
# input-hash.bats — tests for compute-input-hash bin helper and validate-input-hash hook

setup() {
  PLUGIN_ROOT="${BATS_TEST_DIRNAME}/.."
  BIN="$PLUGIN_ROOT/bin/compute-input-hash"
  HOOK="$PLUGIN_ROOT/hooks/validate-input-hash.sh"
  WORK=$(mktemp -d)

  # Create a mini .factory/ structure
  mkdir -p "$WORK/.factory/specs/domain-spec"
  mkdir -p "$WORK/.factory/specs/behavioral-contracts"
  mkdir -p "$WORK/.factory/stories"

  # Create source files
  echo "# Product Brief" > "$WORK/.factory/specs/product-brief.md"
  echo "# L2 Index" > "$WORK/.factory/specs/domain-spec/L2-INDEX.md"
}

teardown() {
  rm -rf "$WORK"
}

# ===== bin/compute-input-hash =====

@test "compute-input-hash: prints hash for valid artifact" {
  cat > "$WORK/.factory/specs/prd.md" << 'EOF'
---
document_type: prd
inputs: [product-brief.md]
input-hash: "[md5]"
---
# PRD
EOF
  run "$BIN" "$WORK/.factory/specs/prd.md"
  [ "$status" -eq 0 ]
  # Hash should be 7 chars
  [[ "${#output}" -eq 7 ]]
}

@test "compute-input-hash: hash is deterministic" {
  cat > "$WORK/.factory/specs/prd.md" << 'EOF'
---
inputs: [product-brief.md]
input-hash: "[md5]"
---
EOF
  hash1=$("$BIN" "$WORK/.factory/specs/prd.md")
  hash2=$("$BIN" "$WORK/.factory/specs/prd.md")
  [ "$hash1" = "$hash2" ]
}

@test "compute-input-hash: hash changes when input changes" {
  cat > "$WORK/.factory/specs/prd.md" << 'EOF'
---
inputs: [product-brief.md]
input-hash: "[md5]"
---
EOF
  hash1=$("$BIN" "$WORK/.factory/specs/prd.md")
  echo "Updated content" >> "$WORK/.factory/specs/product-brief.md"
  hash2=$("$BIN" "$WORK/.factory/specs/prd.md")
  [ "$hash1" != "$hash2" ]
}

@test "compute-input-hash: --update writes hash to frontmatter" {
  cat > "$WORK/.factory/specs/prd.md" << 'EOF'
---
inputs: [product-brief.md]
input-hash: "[md5]"
---
EOF
  run "$BIN" "$WORK/.factory/specs/prd.md" --update
  [ "$status" -eq 0 ]
  # Verify frontmatter was updated
  stored=$(awk '/^input-hash:/ { sub(/.*: *"?/, ""); sub(/"?$/, ""); print; exit }' "$WORK/.factory/specs/prd.md")
  [[ "$stored" != "[md5]" ]]
  [[ "${#stored}" -eq 7 ]]
}

@test "compute-input-hash: --check passes when hash matches" {
  cat > "$WORK/.factory/specs/prd.md" << 'EOF'
---
inputs: [product-brief.md]
input-hash: "[md5]"
---
EOF
  "$BIN" "$WORK/.factory/specs/prd.md" --update
  run "$BIN" "$WORK/.factory/specs/prd.md" --check
  [ "$status" -eq 0 ]
}

@test "compute-input-hash: --check fails when hash stale" {
  cat > "$WORK/.factory/specs/prd.md" << 'EOF'
---
inputs: [product-brief.md]
input-hash: "0000000"
---
EOF
  run "$BIN" "$WORK/.factory/specs/prd.md" --check
  [ "$status" -eq 2 ]
  [[ "$output" == *"DRIFT"* ]]
}

@test "compute-input-hash: --check skips null hash" {
  cat > "$WORK/.factory/specs/prd.md" << 'EOF'
---
inputs: [product-brief.md]
input-hash: null
---
EOF
  run "$BIN" "$WORK/.factory/specs/prd.md" --check
  [ "$status" -eq 0 ]
}

@test "compute-input-hash: resolves inputs relative to specs/" {
  cat > "$WORK/.factory/specs/behavioral-contracts/BC-1.01.001.md" << 'EOF'
---
inputs: [domain-spec/L2-INDEX.md]
input-hash: "[md5]"
---
EOF
  run "$BIN" "$WORK/.factory/specs/behavioral-contracts/BC-1.01.001.md"
  [ "$status" -eq 0 ]
  [[ "${#output}" -eq 7 ]]
}

@test "compute-input-hash: fails with no inputs field" {
  cat > "$WORK/.factory/specs/prd.md" << 'EOF'
---
document_type: prd
---
EOF
  run "$BIN" "$WORK/.factory/specs/prd.md"
  [ "$status" -eq 1 ]
}

@test "compute-input-hash: is executable" {
  [ -x "$BIN" ]
}

@test "compute-input-hash: passes syntax check" {
  bash -n "$BIN"
}

# ===== hooks/validate-input-hash.sh =====

@test "input-hash hook: warns when hash is placeholder" {
  cat > "$WORK/.factory/specs/prd.md" << 'EOF'
---
inputs: [product-brief.md]
input-hash: "[md5]"
---
EOF
  INPUT=$(jq -nc --arg fp "$WORK/.factory/specs/prd.md" '{tool_input: {file_path: $fp}}')
  run bash -c "echo '$INPUT' | CLAUDE_PLUGIN_ROOT='$PLUGIN_ROOT' '$HOOK' 2>&1"
  [ "$status" -eq 0 ]
  [[ "$output" == *"no computed input-hash"* ]]
}

@test "input-hash hook: warns when hash is null" {
  cat > "$WORK/.factory/specs/prd.md" << 'EOF'
---
inputs: [product-brief.md]
input-hash: null
---
EOF
  INPUT=$(jq -nc --arg fp "$WORK/.factory/specs/prd.md" '{tool_input: {file_path: $fp}}')
  run bash -c "echo '$INPUT' | CLAUDE_PLUGIN_ROOT='$PLUGIN_ROOT' '$HOOK' 2>&1"
  [ "$status" -eq 0 ]
  [[ "$output" == *"no computed input-hash"* ]]
}

@test "input-hash hook: silent on non-.factory file" {
  INPUT='{"tool_input":{"file_path":"src/main.rs"}}'
  run bash -c "echo '$INPUT' | CLAUDE_PLUGIN_ROOT='$PLUGIN_ROOT' '$HOOK' 2>&1"
  [ "$status" -eq 0 ]
  [[ -z "$output" ]]
}

@test "input-hash hook: silent when no inputs: field" {
  cat > "$WORK/.factory/specs/notes.md" << 'EOF'
---
document_type: notes
---
EOF
  INPUT=$(jq -nc --arg fp "$WORK/.factory/specs/notes.md" '{tool_input: {file_path: $fp}}')
  run bash -c "echo '$INPUT' | CLAUDE_PLUGIN_ROOT='$PLUGIN_ROOT' '$HOOK' 2>&1"
  [ "$status" -eq 0 ]
  [[ -z "$output" ]]
}

@test "input-hash hook: suggests compute-input-hash command" {
  cat > "$WORK/.factory/specs/prd.md" << 'EOF'
---
inputs: [product-brief.md]
input-hash: "[md5]"
---
EOF
  INPUT=$(jq -nc --arg fp "$WORK/.factory/specs/prd.md" '{tool_input: {file_path: $fp}}')
  run bash -c "echo '$INPUT' | CLAUDE_PLUGIN_ROOT='$PLUGIN_ROOT' '$HOOK' 2>&1"
  [[ "$output" == *"compute-input-hash"* ]]
}

@test "input-hash hook: is executable" {
  [ -x "$HOOK" ]
}

@test "input-hash hook: passes syntax check" {
  bash -n "$HOOK"
}

@test "input-hash hook: hooks.json wires validate-input-hash" {
  jq -e '.hooks.PostToolUse[0].hooks[] | select(.command | contains("validate-input-hash"))' "$PLUGIN_ROOT/hooks/hooks.json" >/dev/null
}

# ===== check-input-drift skill =====

@test "check-input-drift skill exists" {
  [ -f "$PLUGIN_ROOT/skills/check-input-drift/SKILL.md" ]
}

@test "check-input-drift command file exists" {
  [ -f "$PLUGIN_ROOT/commands/check-input-drift.md" ]
}

@test "check-input-drift references compute-input-hash" {
  grep -qF "compute-input-hash" "$PLUGIN_ROOT/skills/check-input-drift/SKILL.md"
}
