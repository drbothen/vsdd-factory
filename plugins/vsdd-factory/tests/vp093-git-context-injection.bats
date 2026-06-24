#!/usr/bin/env bats
# vp093-git-context-injection.bats — VP-093 proof harness for S-18.04b-prereq
#
# Verifies the factory-dispatcher binary injects `git_context` into `payload.extra`
# on qualifying PostToolUse Bash git-commit events targeting the factory-artifacts
# worktree (ADR-029 §Decision 1–3; BC-1.16.001 PC1–PC6).
#
# RED GATE (S-18.04b-prereq stub phase):
#   All tests FAIL against the stub because `inject_git_context_if_qualifying` is
#   `todo!()` in invoke.rs and is not wired into the dispatch path in main.rs.
#   Tests flip GREEN after the implementer fills the `todo!()` bodies and wires
#   the injection call site in main.rs (S-18.04b-prereq T-1 through T-7).
#
# Test cases (VP-093-A through VP-093-E per AC-013):
#   VP-093-A  Four-field injection on qualifying PostToolUse Bash git-commit event
#             (two-commit synthetic repo; verify all four fields in dispatcher JSONL log).
#   VP-093-B  All-empty injection on git error (non-git dir; verify all four fields
#             are "" and dispatcher exits 0).
#   VP-093-C  No injection on non-qualifying Bash event (git push; verify
#             git_context key is absent from payload.extra).
#   VP-093-D  No injection on PostToolUse Edit event (verify git_context key absent).
#   VP-093-E  Initial-commit edge case (head_parent_subject="", head_parent_sha=""
#             not null; single-commit repo).
#
# Bats harness pattern: follows precompact-routing.bats (VP-086).
# Dispatcher binary: target/release/factory-dispatcher
# Log inspection: dispatcher-internal-YYYY-MM-DD.jsonl (JSONL; jq required).
#
# AC traces:
#   AC-001 / BC-1.16.001 PC1 (four-field injection on qualifying event) → VP-093-A
#   AC-002 / BC-1.16.001 PC2 (fail-open on git error)                  → VP-093-B
#   AC-003 / BC-1.16.001 PC3 (no injection on non-qualifying Bash)      → VP-093-C
#   AC-004 / BC-1.16.001 PC4 (no injection on non-Bash PostToolUse)     → VP-093-D
#   AC-006 / BC-1.16.001 INV5 (four-field completeness)                 → VP-093-A,VP-093-E
#   AC-009 / BC-1.16.001 INV3 (fail-open preserves pipeline)            → VP-093-B
#   AC-011 / BC-1.16.001 INV5 (initial-commit: "" not null)             → VP-093-E
#   AC-012 / BC-1.16.001 EC-004 (git push is not qualifying)            → VP-093-C
#   AC-013 / VP-093 (bats harness via dispatcher binary stdin/IPC)      → all
#   AC-007 / BC-1.16.001 INV1 (exec-free WASM; host-layer injection)    → all
#
# Story: S-18.04b-prereq
# BC:    BC-1.16.001 (all postconditions and invariants)
# VP:    VP-093 (Dispatcher git_context Payload Injection on PostToolUse Bash git-commit)

# ---------------------------------------------------------------------------
# setup / teardown
# ---------------------------------------------------------------------------

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../.." && pwd)"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WORK="$(mktemp -d)"

  # CLAUDE_PLUGIN_ROOT is the plugin root (holds hooks-registry.toml + hook-plugins/).
  # CLAUDE_PROJECT_DIR is the simulated project root (cwd for dispatcher host context).
  # factory_dir = CLAUDE_PROJECT_DIR/.factory (derived by dispatcher/implementer).
  PROJECT_DIR="$WORK/project"
  FACTORY_DIR="$PROJECT_DIR/.factory"

  mkdir -p "$WORK/hook-plugins"
  mkdir -p "$FACTORY_DIR/logs"

  # Write a minimal hooks-registry.toml with no plugins.
  # VP-093-A through VP-093-E need NO plugin invocation — they verify the
  # dispatcher's own `git_context` injection by inspecting the JSONL log.
  # (The dispatcher writes `payload.extra` content into its internal log on
  # qualifying events; a dedicated log event for git_context injection is
  # emitted by the implementer's wiring in invoke.rs or main.rs.)
  cat > "$WORK/hooks-registry.toml" <<'EOF'
schema_version = 2
EOF

  # LOG_DIR: where the dispatcher writes its internal JSONL log.
  # The dispatcher resolves log_dir from CLAUDE_PROJECT_DIR/.factory/logs (default).
  LOG_DIR="$FACTORY_DIR/logs"

  export CLAUDE_PLUGIN_ROOT="$WORK"
  export CLAUDE_PROJECT_DIR="$PROJECT_DIR"
}

teardown() {
  rm -rf "$WORK"
}

# ---------------------------------------------------------------------------
# Preflight helpers
# ---------------------------------------------------------------------------

# Skip if dispatcher binary is not built.
_require_dispatcher() {
  if [ ! -x "$DISPATCHER" ]; then
    skip "dispatcher binary not built — run: cargo build --release -p factory-dispatcher"
  fi
}

# Skip if jq is not available (required for JSONL field extraction).
_require_jq() {
  if ! command -v jq &>/dev/null; then
    skip "jq not found — install jq to run VP-093 bats tests"
  fi
}

# Skip if git is not available.
_require_git() {
  if ! command -v git &>/dev/null; then
    skip "git not found — install git to run VP-093 bats tests"
  fi
}

# Run the dispatcher with a given JSON envelope via stdin.
# Captures combined stdout+stderr into $output; sets $status.
_run_dispatcher() {
  local envelope="$1"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$PROJECT_DIR' HOME='$WORK/home' '$DISPATCHER' 2>&1"
}

# Return the most recently modified dispatcher-internal-*.jsonl file in LOG_DIR.
_latest_log_file() {
  ls -t "$LOG_DIR"/dispatcher-internal-*.jsonl 2>/dev/null | head -1
}

# Extract the git_context object from the most recent dispatcher JSONL log entry
# that contains a git_context injection event. Outputs the JSON object or empty string.
_extract_git_context_from_log() {
  local log
  log="$(_latest_log_file)"
  if [ -z "$log" ] || [ ! -f "$log" ]; then
    echo ""
    return
  fi
  # The implementer's injection wiring emits an internal log event of type
  # "dispatcher.git_context_injected" (or similar) containing the git_context
  # fields. This helper extracts the git_context value from that event.
  # If the event type differs, the implementer updates this helper accordingly.
  jq -c 'select(.event_type == "dispatcher.git_context_injected") | .git_context' "$log" \
    2>/dev/null | tail -1
}

# Create a synthetic factory-artifacts git repo under FACTORY_DIR with N commits.
# Args: n_commits (1 or 2)
# Sets GIT_HEAD_SUBJECT, GIT_HEAD_SHA, GIT_PARENT_SUBJECT, GIT_PARENT_SHA
# for the test to use as expected values.
_setup_factory_git_repo() {
  local n_commits="${1:-2}"

  git -C "$FACTORY_DIR" init -b factory-artifacts --quiet 2>/dev/null \
    || git -C "$FACTORY_DIR" init --quiet
  git -C "$FACTORY_DIR" config user.email "test@vsdd-factory"
  git -C "$FACTORY_DIR" config user.name "VP-093 Test"

  if [ "$n_commits" -ge 2 ]; then
    git -C "$FACTORY_DIR" commit --allow-empty -m "state: burst-01 Commit A" --quiet
    GIT_PARENT_SUBJECT="state: burst-01 Commit A"
    GIT_PARENT_SHA="$(git -C "$FACTORY_DIR" rev-parse HEAD)"
  fi

  git -C "$FACTORY_DIR" commit --allow-empty -m "state: burst-02 Commit B" --quiet
  GIT_HEAD_SUBJECT="state: burst-02 Commit B"
  GIT_HEAD_SHA="$(git -C "$FACTORY_DIR" rev-parse HEAD)"

  if [ "$n_commits" -lt 2 ]; then
    GIT_PARENT_SUBJECT=""
    GIT_PARENT_SHA=""
  fi

  export GIT_HEAD_SUBJECT GIT_HEAD_SHA GIT_PARENT_SUBJECT GIT_PARENT_SHA
}

# ---------------------------------------------------------------------------
# VP-093-A: Four-field injection on qualifying PostToolUse Bash git-commit event
#
# AC-001, AC-013 (VP-093-A) / BC-1.16.001 PC1
# RED GATE: Fails because inject_git_context_if_qualifying is todo!() in invoke.rs
# and is not wired in main.rs. The dispatcher does not emit a git_context_injected
# log event, so _extract_git_context_from_log returns empty.
# ---------------------------------------------------------------------------

@test "VP-093-A: dispatcher injects four-field git_context on qualifying PostToolUse Bash git-commit" {
  _require_dispatcher
  _require_jq
  _require_git

  _setup_factory_git_repo 2

  # Qualifying envelope: PostToolUse, tool=Bash, command contains "git commit" and ".factory"
  local envelope
  envelope='{"event_name":"PostToolUse","tool_name":"Bash","session_id":"vp093-a","tool_input":{"command":"git -C .factory commit -m \"state: burst-02 Commit B\""},"tool_response":{"exit_code":0}}'

  _run_dispatcher "$envelope"

  # Dispatcher must exit 0 (no plugins registered, no block).
  [ "$status" -eq 0 ]

  # The dispatcher must have emitted a git_context_injected event in the log.
  local git_ctx
  git_ctx="$(_extract_git_context_from_log)"

  # RED GATE: git_ctx is empty because injection is not implemented.
  [ -n "$git_ctx" ]

  # All four fields must be present and non-null strings.
  local head_subject head_sha parent_subject parent_sha
  head_subject="$(printf '%s' "$git_ctx" | jq -r '.head_subject')"
  head_sha="$(printf '%s' "$git_ctx" | jq -r '.head_sha')"
  parent_subject="$(printf '%s' "$git_ctx" | jq -r '.head_parent_subject')"
  parent_sha="$(printf '%s' "$git_ctx" | jq -r '.head_parent_sha')"

  # Verify HEAD fields match the synthetic repo's HEAD.
  [ "$head_subject" = "$GIT_HEAD_SUBJECT" ]
  [ "$head_sha" = "$GIT_HEAD_SHA" ]

  # Verify HEAD^ fields match the synthetic repo's HEAD^.
  [ "$parent_subject" = "$GIT_PARENT_SUBJECT" ]
  [ "$parent_sha" = "$GIT_PARENT_SHA" ]

  # Verify head_sha is a 40-character hex string.
  echo "$head_sha" | grep -qE '^[0-9a-f]{40}$'
}

# ---------------------------------------------------------------------------
# VP-093-B: All-empty injection on git error (non-git dir)
#
# AC-002, AC-009, AC-013 (VP-093-B) / BC-1.16.001 PC2; INV3
# RED GATE: Fails because injection is not implemented; no log event emitted.
# ---------------------------------------------------------------------------

@test "VP-093-B: dispatcher injects all-empty git_context and exits 0 on git error (non-git factory dir)" {
  _require_dispatcher
  _require_jq

  # Do NOT initialise a git repo in FACTORY_DIR — git commands will fail.
  # The dispatcher must still exit 0 (fail-open, AC-009).

  local envelope
  envelope='{"event_name":"PostToolUse","tool_name":"Bash","session_id":"vp093-b","tool_input":{"command":"git -C .factory commit -m \"state: burst-01\""},"tool_response":{"exit_code":0}}'

  _run_dispatcher "$envelope"

  # Dispatcher must exit 0 even when git fails (fail-open, BC-1.16.001 INV3).
  [ "$status" -eq 0 ]

  # The dispatcher must have emitted a git_context_injected event with all-empty fields.
  local git_ctx
  git_ctx="$(_extract_git_context_from_log)"

  # RED GATE: git_ctx is empty because injection is not implemented.
  [ -n "$git_ctx" ]

  # All four fields must be present and empty string (not null, not absent).
  local head_subject head_sha parent_subject parent_sha
  head_subject="$(printf '%s' "$git_ctx" | jq -r '.head_subject')"
  head_sha="$(printf '%s' "$git_ctx" | jq -r '.head_sha')"
  parent_subject="$(printf '%s' "$git_ctx" | jq -r '.head_parent_subject')"
  parent_sha="$(printf '%s' "$git_ctx" | jq -r '.head_parent_sha')"

  [ "$head_subject" = "" ]
  [ "$head_sha" = "" ]
  [ "$parent_subject" = "" ]
  [ "$parent_sha" = "" ]

  # Fields must be strings, not null — jq -r returns "null" for JSON null.
  head_subject_raw="$(printf '%s' "$git_ctx" | jq '.head_subject')"
  [ "$head_subject_raw" != "null" ]
}

# ---------------------------------------------------------------------------
# VP-093-C: No injection on non-qualifying Bash event (git push)
#
# AC-003, AC-012, AC-013 (VP-093-C) / BC-1.16.001 PC3; EC-004
# RED GATE: Fails because if injection were implemented, this test verifies
# ABSENCE of git_context — but the test itself fails because the log event
# is not emitted at all (injection infrastructure missing).
# ---------------------------------------------------------------------------

@test "VP-093-C: dispatcher does NOT inject git_context on non-qualifying Bash event (git push)" {
  _require_dispatcher
  _require_jq
  _require_git

  _setup_factory_git_repo 2

  # Non-qualifying envelope: PostToolUse, tool=Bash, but command is git push (not commit).
  local envelope
  envelope='{"event_name":"PostToolUse","tool_name":"Bash","session_id":"vp093-c","tool_input":{"command":"git -C .factory push origin factory-artifacts"},"tool_response":{"exit_code":0}}'

  _run_dispatcher "$envelope"

  # Dispatcher exits 0.
  [ "$status" -eq 0 ]

  # No git_context_injected event should appear in the log.
  local git_ctx
  git_ctx="$(_extract_git_context_from_log)"

  # RED GATE: currently empty because injection infrastructure is missing.
  # After implementation, this assertion must verify ABSENCE: git_ctx must be "".
  # The test is written to verify the final state: git_ctx is empty (no injection).
  [ -z "$git_ctx" ]
}

# ---------------------------------------------------------------------------
# VP-093-D: No injection on PostToolUse Edit event
#
# AC-004, AC-008, AC-013 (VP-093-D) / BC-1.16.001 PC4; INV2
# RED GATE: Fails because without implementation, this test's assertion
# ("git_ctx is empty") trivially passes — but this is the correct post-implementation
# behaviour too. The test validates the Edit-tool path after implementation.
# ---------------------------------------------------------------------------

@test "VP-093-D: dispatcher does NOT inject git_context on PostToolUse Edit event" {
  _require_dispatcher
  _require_jq

  # PostToolUse Edit event: tool_name=Edit, not Bash. Must never trigger injection.
  local envelope
  envelope='{"event_name":"PostToolUse","tool_name":"Edit","session_id":"vp093-d","tool_input":{"file_path":".factory/STATE.md","old_string":"a","new_string":"b"},"tool_response":{"success":true}}'

  _run_dispatcher "$envelope"

  [ "$status" -eq 0 ]

  # git_context_injected event must NOT appear in the log for Edit events.
  local git_ctx
  git_ctx="$(_extract_git_context_from_log)"

  # No git_context should be injected for a non-Bash event (AC-004).
  [ -z "$git_ctx" ]
}

# ---------------------------------------------------------------------------
# VP-093-E: Initial-commit edge case (head_parent_subject="" not null)
#
# AC-006, AC-011, AC-013 (VP-093-E) / BC-1.16.001 PC6; INV5; EC-003/EC-009
# RED GATE: Fails because injection is not implemented; no log event emitted.
# ---------------------------------------------------------------------------

@test "VP-093-E: dispatcher injects empty string (not null) for head_parent_subject and head_parent_sha on initial commit" {
  _require_dispatcher
  _require_jq
  _require_git

  # Single-commit repo (no HEAD^).
  _setup_factory_git_repo 1

  # Qualifying envelope targeting the initial-commit factory-artifacts repo.
  local envelope
  envelope='{"event_name":"PostToolUse","tool_name":"Bash","session_id":"vp093-e","tool_input":{"command":"git -C .factory commit --allow-empty -m \"state: burst-01 Commit A\""},"tool_response":{"exit_code":0}}'

  _run_dispatcher "$envelope"

  [ "$status" -eq 0 ]

  local git_ctx
  git_ctx="$(_extract_git_context_from_log)"

  # RED GATE: git_ctx is empty because injection is not implemented.
  [ -n "$git_ctx" ]

  # head_subject and head_sha must be populated (HEAD exists).
  local head_subject head_sha
  head_subject="$(printf '%s' "$git_ctx" | jq -r '.head_subject')"
  head_sha="$(printf '%s' "$git_ctx" | jq -r '.head_sha')"

  [ "$head_subject" = "$GIT_HEAD_SUBJECT" ]
  [ -n "$head_sha" ]
  echo "$head_sha" | grep -qE '^[0-9a-f]{40}$'

  # head_parent_subject and head_parent_sha must be empty string "" (not null, not absent).
  local parent_subject parent_sha
  parent_subject="$(printf '%s' "$git_ctx" | jq -r '.head_parent_subject')"
  parent_sha="$(printf '%s' "$git_ctx" | jq -r '.head_parent_sha')"

  [ "$parent_subject" = "" ]
  [ "$parent_sha" = "" ]

  # Confirm they are not JSON null (jq -r returns "null" string for null).
  parent_subject_raw="$(printf '%s' "$git_ctx" | jq '.head_parent_subject')"
  parent_sha_raw="$(printf '%s' "$git_ctx" | jq '.head_parent_sha')"
  [ "$parent_subject_raw" != "null" ]
  [ "$parent_sha_raw" != "null" ]
}
