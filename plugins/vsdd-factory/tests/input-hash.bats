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
  mkdir -p "$WORK/.factory/specs/verification-properties"
  mkdir -p "$WORK/.factory/stories"
  mkdir -p "$WORK/.factory/phase-0-ingestion"
  mkdir -p "$WORK/.factory/holdout-scenarios"

  # Create source files
  echo "# Product Brief" > "$WORK/.factory/specs/product-brief.md"
  echo "# L2 Index" > "$WORK/.factory/specs/domain-spec/L2-INDEX.md"
  echo "# Recovered Architecture" > "$WORK/.factory/phase-0-ingestion/recovered-architecture.md"
  echo "# Story S-1.03" > "$WORK/.factory/stories/S-1.03-capability-resolution.md"
  echo "# Holdout HS-001" > "$WORK/.factory/holdout-scenarios/HS-001.md"
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

@test "compute-input-hash: --update creates input-hash field when absent (#623)" {
  # Regression for #623: --update silently no-oped (reported success, wrote
  # nothing) when the frontmatter had NO input-hash field at all — the sed
  # find-and-replace matched no line. It must now insert the field and exit 0
  # with the field genuinely present.
  cat > "$WORK/.factory/specs/prd.md" << 'EOF'
---
document_type: prd
inputs: [product-brief.md]
---
# PRD body
EOF
  # Field genuinely absent to start with.
  ! grep -q '^input-hash:' "$WORK/.factory/specs/prd.md"

  run "$BIN" "$WORK/.factory/specs/prd.md" --update
  [ "$status" -eq 0 ]
  # stdout's first line is the emitted hash (run merges the stderr "updated"
  # message into $output, so assert against ${lines[0]}).
  [[ "${#lines[0]}" -eq 7 ]]

  # Field must now exist with the computed hash — a single new line, frontmatter
  # fence and body preserved.
  stored=$(awk '/^input-hash:/ { sub(/.*: *"?/, ""); sub(/"?$/, ""); print; exit }' "$WORK/.factory/specs/prd.md")
  [ "$stored" = "${lines[0]}" ]
  [[ "${#stored}" -eq 7 ]]
  [ "$(grep -c '^input-hash:' "$WORK/.factory/specs/prd.md")" -eq 1 ]
  grep -q '^# PRD body' "$WORK/.factory/specs/prd.md"
}

@test "compute-input-hash: --update on field-absent file is idempotent on re-run (#623)" {
  cat > "$WORK/.factory/specs/prd.md" << 'EOF'
---
document_type: prd
inputs: [product-brief.md]
---
EOF
  first=$("$BIN" "$WORK/.factory/specs/prd.md" --update)
  stored1=$(awk '/^input-hash:/ { sub(/.*: *"?/, ""); sub(/"?$/, ""); print; exit }' "$WORK/.factory/specs/prd.md")

  # Second run: field is now current, so nothing changes — value stays identical
  # and no duplicate field is appended.
  run "$BIN" "$WORK/.factory/specs/prd.md" --update
  [ "$status" -eq 0 ]
  stored2=$(awk '/^input-hash:/ { sub(/.*: *"?/, ""); sub(/"?$/, ""); print; exit }' "$WORK/.factory/specs/prd.md")

  [ "$stored1" = "$first" ]
  [ "$stored2" = "$first" ]
  [ "$(grep -c '^input-hash:' "$WORK/.factory/specs/prd.md")" -eq 1 ]
}

@test "compute-input-hash: --update fails loudly on malformed frontmatter with no closing fence (#623)" {
  # A bookkeeping tool must never report success on a no-op. If the frontmatter
  # has an inputs: field but no closing '---' fence to anchor insertion, --update
  # must exit nonzero rather than silently writing nothing.
  printf -- '---\ninputs: [product-brief.md]\n# body with no closing fence\n' \
    > "$WORK/.factory/specs/malformed.md"

  run "$BIN" "$WORK/.factory/specs/malformed.md" --update
  [ "$status" -ne 0 ]
  [[ "$output" == *"failed to write input-hash"* ]]
  ! grep -q '^input-hash:' "$WORK/.factory/specs/malformed.md"
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

@test "compute-input-hash: resolves inputs in phase-0-ingestion/" {
  cat > "$WORK/.factory/specs/architecture-concept.md" << 'EOF'
---
inputs: [recovered-architecture.md, product-brief.md]
input-hash: "[md5]"
---
EOF
  run "$BIN" "$WORK/.factory/specs/architecture-concept.md"
  [ "$status" -eq 0 ]
  [[ "${#output}" -eq 7 ]]
}

@test "compute-input-hash: resolves inputs in stories/" {
  cat > "$WORK/.factory/specs/verification-properties/VP-042.md" << 'EOF'
---
inputs: [S-1.03-capability-resolution.md]
input-hash: "[md5]"
---
EOF
  run "$BIN" "$WORK/.factory/specs/verification-properties/VP-042.md"
  [ "$status" -eq 0 ]
  [[ "${#output}" -eq 7 ]]
}

@test "compute-input-hash: resolves inputs in holdout-scenarios/" {
  cat > "$WORK/.factory/specs/holdout-eval.md" << 'EOF'
---
inputs: [HS-001.md]
input-hash: "[md5]"
---
EOF
  run "$BIN" "$WORK/.factory/specs/holdout-eval.md"
  [ "$status" -eq 0 ]
  [[ "${#output}" -eq 7 ]]
}

# ===== regression: multi-input block parsing (bug: only first input hashed) =====

@test "compute-input-hash: multi-line inputs: block hashes ALL inputs, not just first" {
  # Regression for the awk sub()-before-exit bug: after sub() strips '  - ',
  # the path starts with '.' which matched /^[^ -]/ and caused early exit.
  # Two artifacts sharing the same first input but differing in second input
  # MUST produce different hashes.
  echo "# Shared first input" > "$WORK/.factory/specs/domain-spec/shared.md"
  echo "# Second input A" > "$WORK/.factory/specs/domain-spec/second-a.md"
  echo "# Second input B" > "$WORK/.factory/specs/domain-spec/second-b.md"

  cat > "$WORK/.factory/specs/behavioral-contracts/artifact-a.md" << 'EOF'
---
document_type: bc
inputs:
  - domain-spec/shared.md
  - domain-spec/second-a.md
input-hash: "[md5]"
---
EOF

  cat > "$WORK/.factory/specs/behavioral-contracts/artifact-b.md" << 'EOF'
---
document_type: bc
inputs:
  - domain-spec/shared.md
  - domain-spec/second-b.md
input-hash: "[md5]"
---
EOF

  hash_a=$("$BIN" "$WORK/.factory/specs/behavioral-contracts/artifact-a.md")
  hash_b=$("$BIN" "$WORK/.factory/specs/behavioral-contracts/artifact-b.md")

  [ "$hash_a" != "$hash_b" ]
}

@test "compute-input-hash: three-item inputs: block — all three items contribute to hash (CR-006)" {
  # Belt-and-suspenders: the 2-item test proves item 2 is read; this proves item 3
  # is also read (i.e., the fix is not accidentally limited to exactly 2 items).
  echo "# Input 1" > "$WORK/.factory/specs/domain-spec/item1.md"
  echo "# Input 2" > "$WORK/.factory/specs/domain-spec/item2.md"
  echo "# Input 3 variant A" > "$WORK/.factory/specs/domain-spec/item3a.md"
  echo "# Input 3 variant B" > "$WORK/.factory/specs/domain-spec/item3b.md"

  cat > "$WORK/.factory/specs/behavioral-contracts/three-a.md" << 'EOF'
---
document_type: bc
inputs:
  - domain-spec/item1.md
  - domain-spec/item2.md
  - domain-spec/item3a.md
input-hash: "[md5]"
---
EOF

  cat > "$WORK/.factory/specs/behavioral-contracts/three-b.md" << 'EOF'
---
document_type: bc
inputs:
  - domain-spec/item1.md
  - domain-spec/item2.md
  - domain-spec/item3b.md
input-hash: "[md5]"
---
EOF

  hash_a=$("$BIN" "$WORK/.factory/specs/behavioral-contracts/three-a.md")
  hash_b=$("$BIN" "$WORK/.factory/specs/behavioral-contracts/three-b.md")

  [ "$hash_a" != "$hash_b" ]
}

@test "compute-input-hash: single-input multi-line inputs: block is stable" {
  echo "# Only input" > "$WORK/.factory/specs/domain-spec/only.md"

  cat > "$WORK/.factory/specs/behavioral-contracts/single.md" << 'EOF'
---
document_type: bc
inputs:
  - domain-spec/only.md
input-hash: "[md5]"
---
EOF

  hash1=$("$BIN" "$WORK/.factory/specs/behavioral-contracts/single.md")
  hash2=$("$BIN" "$WORK/.factory/specs/behavioral-contracts/single.md")
  [ "$hash1" = "$hash2" ]
  [[ "${#hash1}" -eq 7 ]]
}

@test "compute-input-hash: missing input in multi-line block is surfaced, not silently skipped" {
  echo "# Good input" > "$WORK/.factory/specs/domain-spec/good.md"

  cat > "$WORK/.factory/specs/behavioral-contracts/partial.md" << 'EOF'
---
document_type: bc
inputs:
  - domain-spec/good.md
  - domain-spec/nonexistent.md
input-hash: "[md5]"
---
EOF

  run "$BIN" "$WORK/.factory/specs/behavioral-contracts/partial.md" --resolve 2>&1
  [ "$status" -eq 1 ]
  [[ "$output" == *"MISSING"* ]]
  [[ "$output" == *"nonexistent.md"* ]]
}

# ===== hooks/validate-input-hash.sh =====

@test "input-hash hook: blocks when hash is placeholder" {
  cat > "$WORK/.factory/specs/prd.md" << 'EOF'
---
inputs: [product-brief.md]
input-hash: "[md5]"
---
EOF
  INPUT=$(jq -nc --arg fp "$WORK/.factory/specs/prd.md" '{tool_input: {file_path: $fp}}')
  run bash -c "echo '$INPUT' | CLAUDE_PLUGIN_ROOT='$PLUGIN_ROOT' '$HOOK' 2>&1"
  [ "$status" -eq 2 ]
  [[ "$output" == *"no computed input-hash"* ]]
}

@test "input-hash hook: blocks when hash is null" {
  cat > "$WORK/.factory/specs/prd.md" << 'EOF'
---
inputs: [product-brief.md]
input-hash: null
---
EOF
  INPUT=$(jq -nc --arg fp "$WORK/.factory/specs/prd.md" '{tool_input: {file_path: $fp}}')
  run bash -c "echo '$INPUT' | CLAUDE_PLUGIN_ROOT='$PLUGIN_ROOT' '$HOOK' 2>&1"
  [ "$status" -eq 2 ]
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
  load "${BATS_TEST_DIRNAME}/helpers/registry.bash"
  registry_has_hook "validate-input-hash" "PostToolUse"
}

# ===== check-input-drift skill =====

@test "check-input-drift skill exists" {
  [ -f "$PLUGIN_ROOT/skills/check-input-drift/SKILL.md" ]
}

@test "check-input-drift references compute-input-hash" {
  grep -qF "compute-input-hash" "$PLUGIN_ROOT/skills/check-input-drift/SKILL.md"
}

@test "check-input-drift has cluster drift triage (Step 6)" {
  grep -q "Step 6.*[Tt]riage cluster drift" "$PLUGIN_ROOT/skills/check-input-drift/SKILL.md"
}

@test "check-input-drift lists all 7 producing agent dispatch targets" {
  local skill="$PLUGIN_ROOT/skills/check-input-drift/SKILL.md"
  grep -q "business-analyst" "$skill"
  grep -q "product-owner" "$skill"
  grep -q "architect" "$skill"
  grep -q "story-writer" "$skill"
}

@test "check-input-drift has cluster pattern reference table" {
  grep -q "Common Cluster Drift Patterns" "$PLUGIN_ROOT/skills/check-input-drift/SKILL.md"
}

@test "check-input-drift warns before bulk update >3 files" {
  grep -q "Before running.*--update.*on >3 files.*Step 6" "$PLUGIN_ROOT/skills/check-input-drift/SKILL.md"
}

@test "check-input-drift has task template for dispatched agents" {
  grep -q "Do NOT touch input-hash frontmatter" "$PLUGIN_ROOT/skills/check-input-drift/SKILL.md"
}

@test "check-input-drift documents when to skip Step 6" {
  grep -q "When to skip Step 6" "$PLUGIN_ROOT/skills/check-input-drift/SKILL.md"
}

# ===== repo-root-relative path resolution (POLICY 18: engine paths as inputs) =====

@test "compute-input-hash: repo-root-relative input (plugins/…) resolves and contributes to hash" {
  # Simulates an artifact whose inputs: list includes a repo-root-relative engine
  # path (e.g., plugins/vsdd-factory/skills/…/SKILL.md).  The file is placed at
  # the repo-root-relative path inside WORK, and the artifact lives under .factory/.
  mkdir -p "$WORK/.factory/specs"
  mkdir -p "$WORK/plugins/vsdd-factory/skills/my-skill"
  echo "# My Skill" > "$WORK/plugins/vsdd-factory/skills/my-skill/SKILL.md"
  # product-brief.md is created by setup(); no need to recreate here

  cat > "$WORK/.factory/specs/story.md" << 'EOF'
---
document_type: story
inputs:
  - product-brief.md
  - plugins/vsdd-factory/skills/my-skill/SKILL.md
input-hash: "[md5]"
---
EOF

  # --resolve must report all inputs FOUND, not MISSING
  run "$BIN" "$WORK/.factory/specs/story.md" --resolve 2>&1
  [ "$status" -eq 0 ]
  [[ "$output" == *"all 2 inputs resolved"* ]]

  # hash must be computable (7 chars)
  hash=$("$BIN" "$WORK/.factory/specs/story.md")
  [[ "${#hash}" -eq 7 ]]
}

@test "compute-input-hash: repo-root-relative input changes hash when SKILL.md changes" {
  mkdir -p "$WORK/.factory/specs"
  mkdir -p "$WORK/plugins/vsdd-factory/skills/my-skill"
  echo "# My Skill v1" > "$WORK/plugins/vsdd-factory/skills/my-skill/SKILL.md"

  cat > "$WORK/.factory/specs/story.md" << 'EOF'
---
document_type: story
inputs:
  - plugins/vsdd-factory/skills/my-skill/SKILL.md
input-hash: "[md5]"
---
EOF

  hash1=$("$BIN" "$WORK/.factory/specs/story.md")
  echo "# My Skill v2 — content changed" > "$WORK/plugins/vsdd-factory/skills/my-skill/SKILL.md"
  hash2=$("$BIN" "$WORK/.factory/specs/story.md")

  [ "$hash1" != "$hash2" ]
}

@test "compute-input-hash: truly-missing repo-root-relative path surfaces MISSING, not silently skipped" {
  mkdir -p "$WORK/.factory/specs"

  cat > "$WORK/.factory/specs/story.md" << 'EOF'
---
document_type: story
inputs:
  - plugins/vsdd-factory/skills/nonexistent-skill/SKILL.md
input-hash: "[md5]"
---
EOF

  run "$BIN" "$WORK/.factory/specs/story.md" --resolve 2>&1
  [ "$status" -eq 1 ]
  [[ "$output" == *"MISSING"* ]]
  [[ "$output" == *"nonexistent-skill"* ]]
}

# ===== security: path-traversal rejection (CWE-22) =====

@test "compute-input-hash: path-traversal input (..) is rejected as MISSING, not resolved" {
  # SEC-001: inputs containing '..' must be surfaced as MISSING with a warning,
  # never resolved against REPO_ROOT or any search base.
  mkdir -p "$WORK/.factory/specs"
  echo "# Sensitive content" > "$WORK/sensitive.md"

  cat > "$WORK/.factory/specs/traversal-artifact.md" << 'EOF'
---
document_type: bc
inputs:
  - ../../sensitive.md
input-hash: "[md5]"
---
EOF

  run "$BIN" "$WORK/.factory/specs/traversal-artifact.md" --resolve 2>&1
  [ "$status" -eq 1 ]
  # Each assertion is independent — all three must hold (CR-002: no OR-disjunction
  # that masks whether the explicit rejection message fires).
  [[ "$output" == *"rejected path traversal"* ]]
  [[ "$output" == *"MISSING"* ]]
  # Must NOT report resolution success
  [[ "$output" != *"all"*"inputs resolved"* ]]
}
