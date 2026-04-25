#!/usr/bin/env bats
# generate-hooks-json.bats — tests for scripts/generate-hooks-json.sh
#
# Covers placeholder substitution correctness, JSON validity of generated
# variants, drift detection (--check), and platform-suffix handling
# (windows-x64 → .exe, others → no suffix). Each test stages a minimal fake
# repo in a temp directory so the script's REPO_ROOT discovery resolves
# without polluting the real workspace.

setup() {
  REAL_SCRIPT="${BATS_TEST_DIRNAME}/../../../scripts/generate-hooks-json.sh"
  WORK="$(mktemp -d)"
  mkdir -p "$WORK/scripts"
  mkdir -p "$WORK/plugins/vsdd-factory/hooks"
  cp "$REAL_SCRIPT" "$WORK/scripts/generate-hooks-json.sh"
  chmod +x "$WORK/scripts/generate-hooks-json.sh"

  # Minimal fixture template — same structural shape as the real one but
  # smaller so failures are easier to read.
  cat > "$WORK/plugins/vsdd-factory/hooks/hooks.json.template" <<'JSON'
{
  "hooks": {
    "PreToolUse": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "${CLAUDE_PLUGIN_ROOT}/hooks/dispatcher/bin/{{PLATFORM}}/factory-dispatcher{{EXE_SUFFIX}}",
            "timeout": 10000
          }
        ]
      }
    ]
  }
}
JSON
}

teardown() {
  rm -rf "$WORK"
}

# ---------- Generation produces every platform ----------

@test "generator: writes all 5 platform variants" {
  run "$WORK/scripts/generate-hooks-json.sh"
  [ "$status" -eq 0 ]
  for p in darwin-arm64 darwin-x64 linux-x64 linux-arm64 windows-x64; do
    [ -f "$WORK/plugins/vsdd-factory/hooks/hooks.json.$p" ]
  done
}

@test "generator: each generated variant is valid JSON" {
  run "$WORK/scripts/generate-hooks-json.sh"
  [ "$status" -eq 0 ]
  for p in darwin-arm64 darwin-x64 linux-x64 linux-arm64 windows-x64; do
    jq empty "$WORK/plugins/vsdd-factory/hooks/hooks.json.$p"
  done
}

# ---------- Placeholder substitution ----------

@test "generator: substitutes PLATFORM placeholder" {
  run "$WORK/scripts/generate-hooks-json.sh"
  [ "$status" -eq 0 ]
  cmd=$(jq -r '.hooks.PreToolUse[0].hooks[0].command' "$WORK/plugins/vsdd-factory/hooks/hooks.json.darwin-arm64")
  [[ "$cmd" == *"/darwin-arm64/"* ]]
}

@test "generator: leaves no PLATFORM placeholder behind" {
  run "$WORK/scripts/generate-hooks-json.sh"
  [ "$status" -eq 0 ]
  for p in darwin-arm64 darwin-x64 linux-x64 linux-arm64 windows-x64; do
    ! grep -q '{{PLATFORM}}' "$WORK/plugins/vsdd-factory/hooks/hooks.json.$p"
  done
}

@test "generator: leaves no EXE_SUFFIX placeholder behind" {
  run "$WORK/scripts/generate-hooks-json.sh"
  [ "$status" -eq 0 ]
  for p in darwin-arm64 darwin-x64 linux-x64 linux-arm64 windows-x64; do
    ! grep -q '{{EXE_SUFFIX}}' "$WORK/plugins/vsdd-factory/hooks/hooks.json.$p"
  done
}

# ---------- EXE_SUFFIX semantics ----------

@test "generator: windows-x64 variant has .exe suffix" {
  run "$WORK/scripts/generate-hooks-json.sh"
  [ "$status" -eq 0 ]
  cmd=$(jq -r '.hooks.PreToolUse[0].hooks[0].command' "$WORK/plugins/vsdd-factory/hooks/hooks.json.windows-x64")
  [[ "$cmd" == *"/factory-dispatcher.exe" ]]
}

@test "generator: linux-x64 variant has no .exe suffix" {
  run "$WORK/scripts/generate-hooks-json.sh"
  [ "$status" -eq 0 ]
  cmd=$(jq -r '.hooks.PreToolUse[0].hooks[0].command' "$WORK/plugins/vsdd-factory/hooks/hooks.json.linux-x64")
  [[ "$cmd" == *"/factory-dispatcher" ]]
  [[ "$cmd" != *".exe" ]]
}

@test "generator: darwin-arm64 variant has no .exe suffix" {
  run "$WORK/scripts/generate-hooks-json.sh"
  [ "$status" -eq 0 ]
  cmd=$(jq -r '.hooks.PreToolUse[0].hooks[0].command' "$WORK/plugins/vsdd-factory/hooks/hooks.json.darwin-arm64")
  [[ "$cmd" != *".exe" ]]
}

# ---------- ${CLAUDE_PLUGIN_ROOT} pass-through ----------

@test "generator: preserves \${CLAUDE_PLUGIN_ROOT} variable verbatim" {
  run "$WORK/scripts/generate-hooks-json.sh"
  [ "$status" -eq 0 ]
  for p in darwin-arm64 darwin-x64 linux-x64 linux-arm64 windows-x64; do
    cmd=$(jq -r '.hooks.PreToolUse[0].hooks[0].command' "$WORK/plugins/vsdd-factory/hooks/hooks.json.$p")
    [[ "$cmd" == \$\{CLAUDE_PLUGIN_ROOT\}* ]]
  done
}

# ---------- Drift check ----------

@test "generator --check: passes when committed variants match template" {
  run "$WORK/scripts/generate-hooks-json.sh"
  [ "$status" -eq 0 ]
  run "$WORK/scripts/generate-hooks-json.sh" --check
  [ "$status" -eq 0 ]
  [[ "$output" == *"no drift"* ]]
}

@test "generator --check: fails when a variant has been hand-edited" {
  run "$WORK/scripts/generate-hooks-json.sh"
  [ "$status" -eq 0 ]
  # Tamper.
  echo '{"hooks":{"hand-edited":true}}' > "$WORK/plugins/vsdd-factory/hooks/hooks.json.darwin-arm64"
  run "$WORK/scripts/generate-hooks-json.sh" --check
  [ "$status" -ne 0 ]
  [[ "$output" == *"drift detected"* ]]
}

@test "generator --check: fails when a committed variant is missing" {
  run "$WORK/scripts/generate-hooks-json.sh"
  [ "$status" -eq 0 ]
  rm "$WORK/plugins/vsdd-factory/hooks/hooks.json.linux-arm64"
  run "$WORK/scripts/generate-hooks-json.sh" --check
  [ "$status" -ne 0 ]
  [[ "$output" == *"missing committed variant"* ]]
}

# ---------- Argument handling ----------

@test "generator: rejects unknown flag" {
  run "$WORK/scripts/generate-hooks-json.sh" --bogus
  [ "$status" -ne 0 ]
  [[ "$output" == *"unknown argument"* ]]
}

@test "generator: --help prints docstring" {
  run "$WORK/scripts/generate-hooks-json.sh" --help
  [ "$status" -eq 0 ]
  [[ "$output" == *"hooks.json.template"* ]]
}

# ---------- Missing template ----------

@test "generator: errors clearly when template is missing" {
  rm "$WORK/plugins/vsdd-factory/hooks/hooks.json.template"
  run "$WORK/scripts/generate-hooks-json.sh"
  [ "$status" -ne 0 ]
  [[ "$output" == *"template not found"* ]]
}
