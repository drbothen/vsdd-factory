#!/usr/bin/env bats
# validate-factory-path-staging.bats — integration tests for S-21.01.
#
# Tests AC-001..AC-009 for the validate-factory-path-staging WASM guard
# (Layer-1) and the §Main-Checkout Sync Protocol in per-story-delivery.md
# (Layer-2). Together these implement the two-layer INV-E21-001 defense
# per ADR-031 §Decision 2.
#
# Story:  S-21.01 (E-21 Wave 1 — factory state data-loss hardening)
# BC:     BC-4.16.001 (Layer-1 WASM guard) + BC-5.43.001 (Layer-2 protocol)
# ADR:    ADR-031 §Decision 2+3
#
# Skip-guard strategy (AC-001..AC-006 WASM tests):
#   Tests require two artifacts:
#     1. plugins/vsdd-factory/hook-plugins/validate-factory-path-staging.wasm
#     2. target/release/factory-dispatcher
#   If absent, tests SKIP with an actionable message.
#   Set CI_REQUIRE_ARTIFACTS=1 to convert skips to hard failures.
#
# Tests AC-007..AC-009 (Layer-2 per-story-delivery.md content checks) run
# unconditionally — they assert protocol documentation is present and do not
# require any built artifact.
#
# Test Plan table (S-21.01 bats subset):
#
#   | Test name                                                        | AC    |
#   |------------------------------------------------------------------|-------|
#   | T-001 guard blocks git-add factory path on develop               | AC-001|
#   | T-002 guard passes git-add factory path on factory-artifacts      | AC-002|
#   | T-003 guard passes non-git-add command on develop                 | AC-003|
#   | T-004 guard passes git-add non-factory path on develop            | AC-004|
#   | T-005 guard fail-open on branch detection failure                 | AC-005|
#   | T-006 registry entry has canonical shape                          | AC-001|
#   | T-007 per-story-delivery.md has Main-Checkout Sync Protocol       | AC-007|
#   | T-008 per-story-delivery.md mandates git diff --name-only gate    | AC-007|
#   | T-009 per-story-delivery.md halts with FactoryPathDeletionInMergeDiff | AC-007|
#   | T-010 per-story-delivery.md covers git pull or git merge          | AC-007|
#   | T-011 per-story-delivery.md documents pass on clean diff          | AC-008|
#   | T-012 per-story-delivery.md documents fail-open on diff failure   | AC-009|

# ---------------------------------------------------------------------------
# setup / teardown
# ---------------------------------------------------------------------------

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  GUARD_WASM="$PLUGIN_ROOT/hook-plugins/validate-factory-path-staging.wasm"
  PRODUCTION_REGISTRY="$PLUGIN_ROOT/hooks-registry.toml"
  PER_STORY_DELIVERY_MD="$PLUGIN_ROOT/agents/orchestrator/per-story-delivery.md"

  WORK="$(mktemp -d)"
  mkdir -p "$WORK/.factory/logs"
  mkdir -p "$WORK/hook-plugins"

  # Copy the guard WASM into the synthetic plugin root if it exists.
  # Absent WASM → _require_artifacts will skip (Red Gate graceful skip).
  if [ -f "$GUARD_WASM" ]; then
    cp "$GUARD_WASM" "$WORK/hook-plugins/validate-factory-path-staging.wasm"
  fi

  export VSDD_LOG_DIR="$WORK/.factory/logs"
  export CLAUDE_PROJECT_DIR="$WORK"
  export CLAUDE_PLUGIN_ROOT="$WORK"
}

teardown() {
  [ -n "${WORK:-}" ] && rm -rf "$WORK"
}

# ---------------------------------------------------------------------------
# Preflight helpers
# ---------------------------------------------------------------------------

# Skip if dispatcher binary or guard WASM is not present.
# SKIP != PASS — tests that skip are still RED at Red Gate time.
# Set CI_REQUIRE_ARTIFACTS=1 to convert skips to hard failures.
_require_artifacts() {
  if [ ! -x "$DISPATCHER" ]; then
    [ -z "${CI_REQUIRE_ARTIFACTS:-}" ] || {
      echo "FAIL: factory-dispatcher binary not present (CI_REQUIRE_ARTIFACTS=1)."
      echo "  Run: cargo build --release -p factory-dispatcher"
      return 1
    }
    skip "factory-dispatcher binary not built — run: cargo build --release -p factory-dispatcher"
  fi
  if [ ! -f "$WORK/hook-plugins/validate-factory-path-staging.wasm" ]; then
    [ -z "${CI_REQUIRE_ARTIFACTS:-}" ] || {
      echo "FAIL: validate-factory-path-staging.wasm not present (CI_REQUIRE_ARTIFACTS=1)."
      echo "  Run: cargo build --target wasm32-wasip1 -p validate-factory-path-staging"
      return 1
    }
    skip "validate-factory-path-staging.wasm not present — run: cargo build --target wasm32-wasip1 -p validate-factory-path-staging"
  fi
}

# ---------------------------------------------------------------------------
# Registry writer
# ---------------------------------------------------------------------------

# Write a minimal synthetic hooks-registry.toml with only the
# validate-factory-path-staging entry. Matches the canonical shape
# from the production registry (AC-001: priority 140, on_error=continue,
# async=false, tool=^Bash$, exec_subprocess.binary_allow=[git]).
_write_guard_registry() {
  local git_allow="${1:-git}"
  cat > "$WORK/hooks-registry.toml" <<TOML
schema_version = 2

[[hooks]]
name = "validate-factory-path-staging"
event = "PreToolUse"
tool = "^Bash\$"
plugin = "hook-plugins/validate-factory-path-staging.wasm"
priority = 140
timeout_ms = 5000
on_error = "continue"
async = false

[hooks.capabilities.exec_subprocess]
binary_allow = ["${git_allow}"]
env_allow = ["HOME", "GIT_CONFIG_GLOBAL", "XDG_CONFIG_HOME"]
TOML
}

# ---------------------------------------------------------------------------
# Git repo helpers
# ---------------------------------------------------------------------------

# Initialise a minimal git repo in WORK with the given branch name.
# This lets the plugin call `git branch --show-current` and receive
# the expected branch output.
_init_git_repo_on_branch() {
  local branch="$1"
  ( cd "$WORK" && \
    git init -q -b "$branch" && \
    git config user.email "test@test.local" && \
    git config user.name "Test" && \
    # An initial commit is required so the branch resolves
    touch .gitkeep && git add .gitkeep && git commit -q -m "init" )
}

# ---------------------------------------------------------------------------
# Dispatcher invocation helper
# ---------------------------------------------------------------------------

# Build a synthetic PreToolUse Bash JSON envelope.
_bash_event() {
  local cmd="$1"
  local session="${2:-test-vfps}"
  local escaped_cmd
  escaped_cmd="$(printf '%s' "$cmd" | sed 's/\\/\\\\/g; s/"/\\"/g')"
  printf '{"event_name":"PreToolUse","tool_name":"Bash","session_id":"%s","dispatcher_trace_id":"%s-trace","tool_input":{"command":"%s"}}' \
    "$session" "$session" "$escaped_cmd"
}

# Invoke the factory-dispatcher with a PreToolUse Bash envelope.
# Sets $status and $output (combined) per bats conventions.
_run_dispatcher() {
  local envelope="$1"
  STDERR_FILE="$WORK/dispatcher-stderr-$$.txt"
  run bash -c "printf '%s' '$(printf '%s' "$envelope" | sed "s/'/'\\\\''/g")' | \
    VSDD_LOG_DIR='$WORK/.factory/logs' \
    CLAUDE_PLUGIN_ROOT='$WORK' \
    CLAUDE_PROJECT_DIR='$WORK' \
    '$DISPATCHER' 2>'$STDERR_FILE'"
}

# ---------------------------------------------------------------------------
# T-001 / AC-001: Guard blocks git add .factory/<path> on product branch (develop)
# BC-4.16.001 PC1, Invariant 1
# ---------------------------------------------------------------------------

@test "T-001 S-21.01 AC-001: guard blocks git add .factory/STATE.md on develop branch" {
  _require_artifacts
  _write_guard_registry
  _init_git_repo_on_branch "develop"

  local envelope
  envelope="$(_bash_event 'git add .factory/STATE.md')"
  _run_dispatcher "$envelope"

  [ "$status" -eq 2 ] || {
    echo "FAIL: AC-001 / BC-4.16.001 PC1: expected exit 2 (block), got $status"
    echo "  Dispatcher stderr: $(cat "$STDERR_FILE" 2>/dev/null)"
    false
  }

  grep -q "FactoryPathOnProductBranch" "$STDERR_FILE" 2>/dev/null || \
  echo "$output" | grep -q "FactoryPathOnProductBranch" || {
    echo "FAIL: AC-001 / BC-4.16.001 PC1: 'FactoryPathOnProductBranch' not found in output."
    echo "  Expected: block reason must contain the error variant."
    echo "  Stderr: $(cat "$STDERR_FILE" 2>/dev/null)"
    echo "  Stdout: $output"
    false
  }
}

@test "T-001b S-21.01 AC-001: guard blocks git add .factory/<path> on main branch" {
  _require_artifacts
  _write_guard_registry
  _init_git_repo_on_branch "main"

  local envelope
  envelope="$(_bash_event 'git add .factory/stories/S-21.01.md')"
  _run_dispatcher "$envelope"

  [ "$status" -eq 2 ] || {
    echo "FAIL: AC-001 / BC-4.16.001 PC1: main is a product branch — must exit 2. Got $status."
    false
  }
}

@test "T-001c S-21.01 AC-001: guard blocks git add -A (conservative) on develop branch" {
  _require_artifacts
  _write_guard_registry
  _init_git_repo_on_branch "develop"

  local envelope
  envelope="$(_bash_event 'git add -A')"
  _run_dispatcher "$envelope"

  [ "$status" -eq 2 ] || {
    echo "FAIL: AC-001 / BC-4.16.001 Invariant 4: 'git add -A' on develop must exit 2 (conservative block). Got $status."
    false
  }
}

# ---------------------------------------------------------------------------
# T-002 / AC-002: Guard passes on factory-artifacts branch (PC3)
# BC-4.16.001 PC3
# ---------------------------------------------------------------------------

@test "T-002 S-21.01 AC-002: guard passes git add .factory/STATE.md on factory-artifacts" {
  _require_artifacts
  _write_guard_registry
  _init_git_repo_on_branch "factory-artifacts"

  local envelope
  envelope="$(_bash_event 'git add .factory/STATE.md')"
  _run_dispatcher "$envelope"

  [ "$status" -eq 0 ] || {
    echo "FAIL: AC-002 / BC-4.16.001 PC3: factory-artifacts branch must return exit 0 (Continue). Got $status."
    echo "  Stderr: $(cat "$STDERR_FILE" 2>/dev/null)"
    false
  }
}

# ---------------------------------------------------------------------------
# T-003 / AC-003: Guard passes non-git-add commands on product branch (PC4)
# BC-4.16.001 PC4
# ---------------------------------------------------------------------------

@test "T-003 S-21.01 AC-003: guard passes git commit on develop (non-git-add PC4)" {
  _require_artifacts
  _write_guard_registry
  _init_git_repo_on_branch "develop"

  local envelope
  envelope="$(_bash_event 'git commit -m "test"')"
  _run_dispatcher "$envelope"

  [ "$status" -eq 0 ] || {
    echo "FAIL: AC-003 / BC-4.16.001 PC4: 'git commit' is not in scope — must return exit 0. Got $status."
    false
  }
}

@test "T-003b S-21.01 AC-003: guard passes git merge on develop (Layer-1 scope narrow)" {
  _require_artifacts
  _write_guard_registry
  _init_git_repo_on_branch "develop"

  local envelope
  envelope="$(_bash_event 'git merge feature/S-21.01')"
  _run_dispatcher "$envelope"

  [ "$status" -eq 0 ] || {
    echo "FAIL: AC-003 / BC-4.16.001 PC4: 'git merge' is Layer-2 domain (BC-5.43.001);"
    echo "  Layer-1 scope is git-add only — must return exit 0. Got $status."
    false
  }
}

# ---------------------------------------------------------------------------
# T-004 / AC-004: Guard passes git add of non-.factory/ paths (PC2)
# BC-4.16.001 PC2
# ---------------------------------------------------------------------------

@test "T-004 S-21.01 AC-004: guard passes git add src/main.rs on develop (non-factory PC2)" {
  _require_artifacts
  _write_guard_registry
  _init_git_repo_on_branch "develop"

  local envelope
  envelope="$(_bash_event 'git add src/main.rs')"
  _run_dispatcher "$envelope"

  [ "$status" -eq 0 ] || {
    echo "FAIL: AC-004 / BC-4.16.001 PC2: 'git add src/main.rs' on develop must return exit 0 (non-.factory/ path). Got $status."
    false
  }
}

# ---------------------------------------------------------------------------
# T-005 / AC-005: Guard fails open on branch detection failure (Invariant 3)
# BC-4.16.001 Invariant 3
# ---------------------------------------------------------------------------

@test "T-005 S-21.01 AC-005: guard fails open when run outside a git repo" {
  _require_artifacts
  _write_guard_registry
  # Intentionally NOT initialising a git repo — git branch --show-current will fail.
  # Guard must fail-open (exit 0) per BC-4.16.001 Invariant 3.

  local envelope
  envelope="$(_bash_event 'git add .factory/STATE.md')"
  _run_dispatcher "$envelope"

  [ "$status" -eq 0 ] || {
    echo "FAIL: AC-005 / BC-4.16.001 Invariant 3: branch detection failure (no git repo) must"
    echo "  fail-open (exit 0). Got $status."
    echo "  Uncertain branch state is NOT a blocking condition."
    false
  }
}

# ---------------------------------------------------------------------------
# T-006 / AC-001: Registry entry has canonical shape (runs without WASM)
# BC-4.16.001 — hooks-registry.toml shape
# ---------------------------------------------------------------------------

@test "T-006 S-21.01 AC-001: production registry has validate-factory-path-staging entry" {
  grep -q 'name = "validate-factory-path-staging"' "$PRODUCTION_REGISTRY" || {
    echo "FAIL: AC-001: hooks-registry.toml must contain an entry for 'validate-factory-path-staging'."
    echo "  No entry found. S-21.01 implementer task: add [[hooks]] entry per ADR-031 §Decision 2."
    false
  }
}

@test "T-006b S-21.01 AC-001: registry entry has tool=^Bash$ (anchored)" {
  # Get the block starting at validate-factory-path-staging and check fields
  local entry_block
  entry_block="$(awk '/name = "validate-factory-path-staging"/{found=1} found && /^\[\[/{if(!first){first=1;next} exit} found{print}' "$PRODUCTION_REGISTRY")"

  echo "$entry_block" | grep -q 'tool = "^Bash\$"' || {
    echo "FAIL: AC-001: validate-factory-path-staging entry must have tool = \"^\Bash\$\""
    echo "  (anchored tool filter per S-19.04 anchoring convention)."
    echo "  Entry block:"
    echo "$entry_block"
    false
  }
}

@test "T-006c S-21.01 AC-001: registry entry has priority=140 and on_error=continue" {
  local entry_block
  entry_block="$(awk '/name = "validate-factory-path-staging"/{found=1} found && /^\[\[hooks/{if(!first){first=1;next} exit} found{print}' "$PRODUCTION_REGISTRY")"

  echo "$entry_block" | grep -q 'priority = 140' || {
    echo "FAIL: AC-001: entry must have priority = 140 (ADR-031 §Decision 2)."
    echo "  Entry block: $entry_block"
    false
  }

  echo "$entry_block" | grep -q 'on_error = "continue"' || {
    echo "FAIL: AC-001 / BC-4.16.001 Invariant 2: entry must have on_error = \"continue\""
    echo "  (fail-open per ADR-031 §Decision 2 rationale: crashed guard never wedges session)."
    echo "  Entry block: $entry_block"
    false
  }
}

@test "T-006d S-21.01 AC-001: registry entry has exec_subprocess.binary_allow=[git]" {
  local entry_block
  entry_block="$(awk '/name = "validate-factory-path-staging"/{found=1} found && /^\[\[hooks/{if(!first){first=1;next} exit} found{print}' "$PRODUCTION_REGISTRY")"

  echo "$entry_block" | grep -q 'binary_allow.*git' || {
    echo "FAIL: AC-001: registry entry must declare [hooks.capabilities.exec_subprocess]"
    echo "  binary_allow = [\"git\"] — required for branch detection via git branch --show-current."
    echo "  Entry block: $entry_block"
    false
  }
}

# ---------------------------------------------------------------------------
# T-007 / AC-007: per-story-delivery.md §Main-Checkout Sync Protocol — section presence
# BC-5.43.001 PC2, Invariant 1 — Layer-2 enforcement site
# ---------------------------------------------------------------------------

@test "T-007 S-21.01 AC-007: per-story-delivery.md has Main-Checkout Sync Protocol section" {
  [ -f "$PER_STORY_DELIVERY_MD" ] || {
    echo "FAIL: AC-007: $PER_STORY_DELIVERY_MD not found."
    false
  }

  grep -q "Main-Checkout Sync Protocol" "$PER_STORY_DELIVERY_MD" || {
    echo "FAIL: AC-007 / BC-5.43.001 / ADR-031 §Decision 2:"
    echo "  plugins/vsdd-factory/agents/orchestrator/per-story-delivery.md must contain"
    echo "  a '§Main-Checkout Sync Protocol' section as the S-21.01 Layer-2 deliverable."
    echo "  The section is absent — S-21.01 implementer must add it."
    false
  }
}

# ---------------------------------------------------------------------------
# T-008 / AC-007(a): section mandates git diff --name-only gate
# BC-5.43.001 §Description + PC2
# ---------------------------------------------------------------------------

@test "T-008 S-21.01 AC-007(a): per-story-delivery.md mandates git diff --name-only pre-check" {
  grep -q "git diff --name-only" "$PER_STORY_DELIVERY_MD" || {
    echo "FAIL: AC-007(a) / BC-5.43.001 PC2: §Main-Checkout Sync Protocol must mandate"
    echo "  'git diff --name-only HEAD..<target-ref>' as the required pre-check command."
    echo "  The command is absent from per-story-delivery.md."
    false
  }
}

# ---------------------------------------------------------------------------
# T-009 / AC-007(b): section halts with FactoryPathDeletionInMergeDiff
# BC-5.43.001 PC2 error variant
# ---------------------------------------------------------------------------

@test "T-009 S-21.01 AC-007(b): per-story-delivery.md specifies FactoryPathDeletionInMergeDiff halt" {
  grep -q "FactoryPathDeletionInMergeDiff" "$PER_STORY_DELIVERY_MD" || {
    echo "FAIL: AC-007(b) / BC-5.43.001 PC2: §Main-Checkout Sync Protocol must specify halt"
    echo "  with 'FactoryPathDeletionInMergeDiff' error variant when .factory/ path detected."
    echo "  The error variant is absent from per-story-delivery.md."
    false
  }
}

# ---------------------------------------------------------------------------
# T-010 / AC-007(d): section covers git pull and/or git merge
# BC-5.43.001 §Description: covers both documented steps and ad-hoc operator Bash
# ---------------------------------------------------------------------------

@test "T-010 S-21.01 AC-007(d): per-story-delivery.md covers git pull or git merge" {
  grep -q "git pull\|git merge" "$PER_STORY_DELIVERY_MD" || {
    echo "FAIL: AC-007(d) / BC-5.43.001: §Main-Checkout Sync Protocol must cover"
    echo "  'git pull' and/or 'git merge' on the main product checkout."
    echo "  Neither appears in per-story-delivery.md in context of the pre-check protocol."
    false
  }
}

# ---------------------------------------------------------------------------
# T-011 / AC-008: section documents pass-through on clean diff (PC1)
# BC-5.43.001 PC1: merge proceeds when no .factory/ paths in diff
# ---------------------------------------------------------------------------

@test "T-011 S-21.01 AC-008: per-story-delivery.md documents pass on clean diff" {
  # The section must document that the operation PROCEEDS when diff is clean.
  local has_pass_case
  has_pass_case="$(grep -c "proceed\|passes\|safe to\|no .factory\|Proceed\|transparently" "$PER_STORY_DELIVERY_MD" || true)"

  [ "$has_pass_case" -gt 0 ] || {
    echo "FAIL: AC-008 / BC-5.43.001 PC1: §Main-Checkout Sync Protocol must document"
    echo "  that the operation PROCEEDS (passes transparently) when git diff --name-only"
    echo "  returns no .factory/ paths. The pass-through condition is absent."
    false
  }
}

# ---------------------------------------------------------------------------
# T-012 / AC-009: section documents fail-open on git diff failure (Invariant 4)
# BC-5.43.001 Invariant 4: log warning AND proceed when git diff fails
# ---------------------------------------------------------------------------

@test "T-012 S-21.01 AC-009: per-story-delivery.md documents fail-open on diff failure" {
  # The section must document fail-open: log warning + proceed when git diff fails.
  local has_fail_open
  has_fail_open="$(grep -c "warning\|fail-open\|log\|proceed\|Warning\|Fail-open" "$PER_STORY_DELIVERY_MD" || true)"

  [ "$has_fail_open" -gt 0 ] || {
    echo "FAIL: AC-009 / BC-5.43.001 Invariant 4: §Main-Checkout Sync Protocol must"
    echo "  document fail-open behavior: log a warning AND proceed when git diff fails."
    echo "  The fail-open clause is absent from per-story-delivery.md."
    false
  }
}
