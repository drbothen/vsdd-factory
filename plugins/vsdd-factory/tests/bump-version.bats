#!/usr/bin/env bats
# bump-version.bats — tests for scripts/bump-version.sh
#
# Covers semver validation (stable + prerelease + malformed), the lockstep
# write to plugin.json + marketplace.json, and the CHANGELOG idempotent
# stub guard. The script expects to live at <root>/scripts/bump-version.sh
# and to derive REPO_ROOT relative to that, so each test stages a minimal
# fake repo in a temp directory.

setup() {
  REAL_SCRIPT="${BATS_TEST_DIRNAME}/../../../scripts/bump-version.sh"
  WORK="$(mktemp -d)"
  mkdir -p "$WORK/scripts"
  mkdir -p "$WORK/plugins/vsdd-factory/.claude-plugin"
  mkdir -p "$WORK/.claude-plugin"
  cp "$REAL_SCRIPT" "$WORK/scripts/bump-version.sh"

  cat > "$WORK/plugins/vsdd-factory/.claude-plugin/plugin.json" <<'JSON'
{
  "name": "vsdd-factory",
  "version": "0.79.4",
  "description": "fixture"
}
JSON

  cat > "$WORK/.claude-plugin/marketplace.json" <<'JSON'
{
  "plugins": [
    {
      "name": "vsdd-factory",
      "version": "0.79.4"
    }
  ]
}
JSON

  cat > "$WORK/CHANGELOG.md" <<'MD'
# Changelog

## 0.79.4 — last config workaround (2026-04-23)

Stuff happened.
MD

  # The script refuses to run if the JSON files have uncommitted changes,
  # so the workdir must be a clean git repo.
  git -C "$WORK" init -q
  git -C "$WORK" -c user.email=t@t -c user.name=t add -A
  git -C "$WORK" -c user.email=t@t -c user.name=t commit -q -m fixture
}

teardown() {
  rm -rf "$WORK"
}

# ---------- Stable semver (no regression) ----------

@test "bump-version: accepts N.N.N (stable)" {
  run "$WORK/scripts/bump-version.sh" 0.80.0 "test stable"
  [ "$status" -eq 0 ]
  [[ "$output" == *"Bumped to 0.80.0"* ]]
}

@test "bump-version: writes stable version to plugin.json" {
  run "$WORK/scripts/bump-version.sh" 0.80.0 "test"
  [ "$status" -eq 0 ]
  v=$(jq -r .version "$WORK/plugins/vsdd-factory/.claude-plugin/plugin.json")
  [ "$v" = "0.80.0" ]
}

@test "bump-version: writes stable version to marketplace.json" {
  run "$WORK/scripts/bump-version.sh" 0.80.0 "test"
  [ "$status" -eq 0 ]
  v=$(jq -r '.plugins[0].version' "$WORK/.claude-plugin/marketplace.json")
  [ "$v" = "0.80.0" ]
}

# ---------- Prerelease accepted ----------

@test "bump-version: accepts N.N.N-beta.N" {
  run "$WORK/scripts/bump-version.sh" 1.0.0-beta.1 "Factory Plugin Kit beta"
  [ "$status" -eq 0 ]
  v=$(jq -r .version "$WORK/plugins/vsdd-factory/.claude-plugin/plugin.json")
  [ "$v" = "1.0.0-beta.1" ]
  m=$(jq -r '.plugins[0].version' "$WORK/.claude-plugin/marketplace.json")
  [ "$m" = "1.0.0-beta.1" ]
}

@test "bump-version: accepts N.N.N-rc.N" {
  run "$WORK/scripts/bump-version.sh" 1.0.0-rc.2 "rc"
  [ "$status" -eq 0 ]
  v=$(jq -r .version "$WORK/plugins/vsdd-factory/.claude-plugin/plugin.json")
  [ "$v" = "1.0.0-rc.2" ]
}

@test "bump-version: accepts N.N.N-alpha (no numeric suffix)" {
  run "$WORK/scripts/bump-version.sh" 1.0.0-alpha "alpha"
  [ "$status" -eq 0 ]
  v=$(jq -r .version "$WORK/plugins/vsdd-factory/.claude-plugin/plugin.json")
  [ "$v" = "1.0.0-alpha" ]
}

@test "bump-version: accepts dotted prerelease identifiers" {
  run "$WORK/scripts/bump-version.sh" 1.0.0-beta.1.dev3 "exotic"
  [ "$status" -eq 0 ]
  v=$(jq -r .version "$WORK/plugins/vsdd-factory/.claude-plugin/plugin.json")
  [ "$v" = "1.0.0-beta.1.dev3" ]
}

# ---------- Malformed rejected ----------

@test "bump-version: rejects N.N (missing patch)" {
  run "$WORK/scripts/bump-version.sh" 1.0
  [ "$status" -ne 0 ]
  [[ "$output" == *"version must match"* ]]
}

@test "bump-version: rejects trailing hyphen" {
  run "$WORK/scripts/bump-version.sh" 1.0.0-
  [ "$status" -ne 0 ]
  [[ "$output" == *"version must match"* ]]
}

@test "bump-version: rejects empty version" {
  run "$WORK/scripts/bump-version.sh" ""
  [ "$status" -ne 0 ]
  [[ "$output" == *"version must match"* ]]
}

@test "bump-version: rejects build metadata (+...)" {
  # We deliberately do not accept build metadata.
  run "$WORK/scripts/bump-version.sh" 1.0.0+build.1
  [ "$status" -ne 0 ]
  [[ "$output" == *"version must match"* ]]
}

# ---------- Argument validation ----------

@test "bump-version: rejects missing version arg" {
  run "$WORK/scripts/bump-version.sh"
  [ "$status" -ne 0 ]
  [[ "$output" == *"usage:"* ]]
}

@test "bump-version: usage hint mentions prerelease example" {
  run "$WORK/scripts/bump-version.sh"
  [ "$status" -ne 0 ]
  [[ "$output" == *"1.0.0-beta.1"* ]]
}

# ---------- CHANGELOG idempotent guard for prerelease headings ----------

@test "bump-version: prepends CHANGELOG stub for new prerelease" {
  run "$WORK/scripts/bump-version.sh" 1.0.0-beta.1 "Factory Plugin Kit beta"
  [ "$status" -eq 0 ]
  grep -q '^## 1.0.0-beta.1 — Factory Plugin Kit beta' "$WORK/CHANGELOG.md"
}

@test "bump-version: skips CHANGELOG stub when prerelease heading already present" {
  # First write a real entry so the idempotent guard fires the next call.
  cat > "$WORK/CHANGELOG.md" <<'MD'
# Changelog

## 1.0.0-beta.1 — Factory Plugin Kit beta (2026-04-25)

Real release notes already authored.

## 0.79.4 — last config workaround (2026-04-23)

Stuff happened.
MD
  git -C "$WORK" -c user.email=t@t -c user.name=t add CHANGELOG.md
  git -C "$WORK" -c user.email=t@t -c user.name=t commit -q -m "pre-author"

  run "$WORK/scripts/bump-version.sh" 1.0.0-beta.1 "Factory Plugin Kit beta"
  [ "$status" -eq 0 ]
  [[ "$output" == *"already present"* ]]

  # Verify only one heading remains.
  count=$(grep -c '^## 1.0.0-beta.1 ' "$WORK/CHANGELOG.md")
  [ "$count" -eq 1 ]
}

# ---------- Idempotency on JSON files ----------

@test "bump-version: re-running with same prerelease version is a no-op" {
  run "$WORK/scripts/bump-version.sh" 1.0.0-beta.1 "test"
  [ "$status" -eq 0 ]
  git -C "$WORK" -c user.email=t@t -c user.name=t add -A
  git -C "$WORK" -c user.email=t@t -c user.name=t commit -q -m bump

  run "$WORK/scripts/bump-version.sh" 1.0.0-beta.1 "test"
  [ "$status" -eq 0 ]
  [[ "$output" == *"already at 1.0.0-beta.1"* ]]
}
