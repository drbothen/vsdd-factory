#!/usr/bin/env bats
# s-5.06-semver-commitment.bats — RED-gate tests for AC-1..AC-6
# Target file: docs/guide/semver-commitment.md
# All tests MUST FAIL until the implementer creates the target file.

setup() {
  # Resolve repo root: four levels up from tests/docs-completeness/
  REPO_ROOT="$(cd "$BATS_TEST_DIRNAME/../../../.." && pwd)"
  SEMVER_DOC="$REPO_ROOT/docs/guide/semver-commitment.md"
  V10_INDEX="$REPO_ROOT/docs/guide/v1.0-index.md"
  README="$REPO_ROOT/README.md"
}

# -----------------------------------------------------------------------
# AC-1: docs/guide/semver-commitment.md exists with full content
# -----------------------------------------------------------------------

@test "AC-1: docs/guide/semver-commitment.md exists" {
  [ -f "$SEMVER_DOC" ]
}

@test "AC-1: semver-commitment.md has at least 100 non-blank lines" {
  local count
  count=$(grep -c '[^[:space:]]' "$SEMVER_DOC")
  [ "$count" -ge 100 ]
}

# -----------------------------------------------------------------------
# AC-2: Section "What's stable" listing stable surfaces
# -----------------------------------------------------------------------

@test "AC-2: stable surface section heading exists" {
  grep -qi "what.*stable" "$SEMVER_DOC"
}

@test "AC-2: stable surface lists hook-sdk ABI" {
  grep -qi "hook.sdk.*abi\|hook_sdk.*abi\|vsdd_hook_sdk" "$SEMVER_DOC"
}

@test "AC-2: stable surface lists registry schema" {
  grep -qi "registry.*schema" "$SEMVER_DOC"
}

@test "AC-2: stable surface lists hooks.json format" {
  grep -qi "hooks\.json" "$SEMVER_DOC"
}

@test "AC-2: stable surface lists event type namespaces" {
  grep -qi "event.*type.*namespace\|event.*namespace" "$SEMVER_DOC"
}

# -----------------------------------------------------------------------
# AC-3: Section "What's NOT stable" listing unstable surfaces
# -----------------------------------------------------------------------

@test "AC-3: unstable surface section heading exists" {
  grep -qi "not.*stable\|unstable" "$SEMVER_DOC"
}

@test "AC-3: unstable surface lists internal JSONL format" {
  grep -qi "jsonl\|internal.*format" "$SEMVER_DOC"
}

@test "AC-3: unstable surface lists dispatcher invocation args" {
  grep -qi "dispatcher.*invocation\|dispatcher.*arg\|invocation.*arg" "$SEMVER_DOC"
}

# -----------------------------------------------------------------------
# AC-4: Breaking change policy section
# -----------------------------------------------------------------------

@test "AC-4: breaking change policy section heading exists" {
  grep -qi "breaking.*change.*policy\|breaking.*change" "$SEMVER_DOC"
}

@test "AC-4: breaking change policy mentions major version bump" {
  grep -qi "major.*version\|major.*bump" "$SEMVER_DOC"
}

@test "AC-4: breaking change policy mentions migration guide" {
  grep -qi "migration.*guide\|migrate" "$SEMVER_DOC"
}

# -----------------------------------------------------------------------
# AC-5: Plugin backward compat policy section
# -----------------------------------------------------------------------

@test "AC-5: plugin backward compat section heading exists" {
  grep -qi "plugin.*compat\|plugin.*backward\|backward.*compat" "$SEMVER_DOC"
}

@test "AC-5: plugin compat mentions HOST_ABI_VERSION" {
  grep -q "HOST_ABI_VERSION" "$SEMVER_DOC"
}

# -----------------------------------------------------------------------
# AC-6: Cross-links in v1.0-index.md and README.md
# -----------------------------------------------------------------------

@test "AC-6: v1.0-index.md For operators table contains semver-commitment.md row" {
  grep -q "semver-commitment\.md" "$V10_INDEX"
}

@test "AC-6: README.md v1.0 Factory Plugin Kit section contains semver-commitment.md row" {
  grep -q "semver-commitment\.md" "$README"
}

@test "AC-6: README.md L261 reads 'links the five below'" {
  grep -q "links the five below" "$README"
}
