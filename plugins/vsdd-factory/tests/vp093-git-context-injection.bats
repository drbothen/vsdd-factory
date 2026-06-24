#!/usr/bin/env bats
# vp093-git-context-injection.bats — VP-093 proof harness for S-18.04b-prereq
#
# Verifies the factory-dispatcher binary injects `git_context` into `payload.extra`
# on qualifying PostToolUse Bash git-commit events targeting the factory-artifacts
# worktree (ADR-029 §Decision 1–3; BC-1.16.001 PC1–PC6).
#
# ANTI-TAUTOLOGY DISCIPLINE (lesson from S-18.04b VP-084):
#   The sibling story S-18.04b VP-084 passed VACUOUSLY because bats setup never
#   git-init'd the factory-artifacts repo. This harness avoids that failure mode by:
#   1. Using legacy-bash-adapter.wasm + a payload-capture shell script so the actual
#      JSON delivered to the plugin is read and asserted. Tests do NOT rely on log
#      event types that might not exist in the implementation.
#   2. VP-093-A: asserts the EXACT values of all four fields from the real synthetic
#      git repo's HEAD/HEAD^ (not merely "no error" or "exit 0").
#   3. VP-093-B: asserts each field is the empty-string JSON value "" (not just absent)
#      — distinguishing fail-open injection from no injection.
#   4. VP-093-C / VP-093-D: use a POSITIVE-COVERAGE control assertion. The capture
#      script writes the payload to CAPTURE_FILE. If git_context were injected for
#      these non-qualifying events, the assertion would FAIL (not vacuously pass).
#      The test verifies absence of "git_context" key in the captured payload JSON,
#      then also verifies the plugin WAS invoked at all (capture file non-empty).
#
# GREEN STATE (S-18.04b-prereq delivered):
#   VP-093-A, VP-093-B, VP-093-E PASS because inject_git_context_if_qualifying is
#   fully implemented in invoke.rs and wired into main.rs. The captured payload
#   contains git_context with the correct field values.
#   VP-093-C, VP-093-D PASS verifying absence of git_context on non-qualifying events;
#   the positive-coverage sentinel (CAPTURE_FILE non-empty) confirms the plugin is
#   exercised and the absence is intentional, not a routing failure.
#
# Test cases (VP-093-A through VP-093-E per AC-013):
#   VP-093-A  Four-field injection on qualifying PostToolUse Bash git-commit event.
#             Two-commit synthetic factory-artifacts repo. Captures payload delivered
#             to plugin. Asserts all four field VALUES match real HEAD/HEAD^.
#   VP-093-B  All-empty injection on git error (non-git factory dir). Asserts all
#             four fields are "" (not null, not absent). Dispatcher exits 0.
#   VP-093-C  No injection on non-qualifying Bash event (git push). Asserts
#             git_context key is ABSENT from captured payload; plugin IS invoked.
#   VP-093-D  No injection on PostToolUse Edit event. Asserts git_context key
#             is ABSENT from captured payload; plugin IS invoked.
#   VP-093-E  Initial-commit edge case: head_parent_subject="" and head_parent_sha=""
#             (not null, not absent) on a single-commit factory-artifacts repo.
#
# Bats harness pattern: follows precompact-routing.bats (VP-086).
# Payload capture: legacy-bash-adapter.wasm + capture.sh (writes stdin to CAPTURE_FILE).
# Dispatcher binary: target/release/factory-dispatcher
# WASM plugin: plugins/vsdd-factory/hook-plugins/legacy-bash-adapter.wasm
#
# AC traces:
#   AC-001 / BC-1.16.001 PC1 (four-field injection on qualifying event) → VP-093-A
#   AC-002 / BC-1.16.001 PC2 (fail-open on git error)                   → VP-093-B
#   AC-003 / BC-1.16.001 PC3 (no injection on non-qualifying Bash)       → VP-093-C
#   AC-004 / BC-1.16.001 PC4 (no injection on non-Bash PostToolUse)      → VP-093-D
#   AC-006 / BC-1.16.001 INV5 (four-field completeness)                  → VP-093-A, VP-093-E
#   AC-007 / BC-1.16.001 INV1 (exec-free WASM; end-to-end via dispatcher) → all
#   AC-009 / BC-1.16.001 INV3 (fail-open preserves pipeline)             → VP-093-B
#   AC-011 / BC-1.16.001 INV5 (initial-commit: "" not null)              → VP-093-E
#   AC-012 / BC-1.16.001 EC-004 (git push is not qualifying)             → VP-093-C
#   AC-013 / VP-093 (bats harness via dispatcher binary stdin/IPC)        → all
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
  ADAPTER_WASM="$REPO_ROOT/plugins/vsdd-factory/hook-plugins/legacy-bash-adapter.wasm"
  WORK="$(mktemp -d)"

  # CLAUDE_PLUGIN_ROOT is the plugin root (holds hooks-registry.toml + hook-plugins/).
  # CLAUDE_PROJECT_DIR is the simulated project root.
  # factory_dir = CLAUDE_PROJECT_DIR/.factory (derived by dispatcher host context via cwd).
  PROJECT_DIR="$WORK/project"
  FACTORY_DIR="$PROJECT_DIR/.factory"

  mkdir -p "$WORK/hook-plugins" "$WORK/hooks"
  mkdir -p "$FACTORY_DIR/logs"

  # Copy legacy-bash-adapter.wasm into WORK/hook-plugins so the registry resolves it.
  if [ -f "$ADAPTER_WASM" ]; then
    cp "$ADAPTER_WASM" "$WORK/hook-plugins/legacy-bash-adapter.wasm"
  fi

  # CAPTURE_FILE: the shell script writes the plugin's stdin (the enriched payload) here.
  # Each test clears and re-reads CAPTURE_FILE after dispatcher invocation.
  CAPTURE_FILE="$WORK/captured-payload.json"

  # capture.sh: dumps the plugin's stdin (the full enriched payload JSON) to CAPTURE_FILE,
  # then exits 0. The dispatcher passes the entire payload_value to the plugin as stdin.
  # This is the primary non-tautological signal: if git_context is injected, it appears
  # in CAPTURE_FILE because the plugin receives the full payload including extra fields.
  cat > "$WORK/hooks/capture.sh" <<SCRIPT_EOF
#!/usr/bin/env bash
# Capture the plugin payload (received on stdin) to CAPTURE_FILE for assertion.
# Exit 0 so the dispatcher does not block.
cat > "$CAPTURE_FILE"
exit 0
SCRIPT_EOF
  chmod +x "$WORK/hooks/capture.sh"

  export CLAUDE_PLUGIN_ROOT="$WORK"
  export CLAUDE_PROJECT_DIR="$PROJECT_DIR"

  # Export for test functions.
  export WORK PROJECT_DIR FACTORY_DIR CAPTURE_FILE DISPATCHER
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

# Skip if legacy-bash-adapter.wasm is absent (required for payload-capture).
_require_adapter() {
  if [ ! -f "$WORK/hook-plugins/legacy-bash-adapter.wasm" ]; then
    skip "legacy-bash-adapter.wasm not present — build hook-plugins or copy to $REPO_ROOT/plugins/vsdd-factory/hook-plugins/"
  fi
}

# Skip if jq is not available (required for JSON field extraction).
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

# ---------------------------------------------------------------------------
# Registry helpers
# ---------------------------------------------------------------------------

# Write a PostToolUse Bash capture registry.
# The capture plugin (legacy-bash-adapter + capture.sh) receives the full payload
# via stdin and writes it to CAPTURE_FILE for assertion.
_write_posttooluse_capture_registry() {
  cat > "$WORK/hooks-registry.toml" <<EOF
schema_version = 2

[[hooks]]
name = "payload-capture"
event = "PostToolUse"
plugin = "hook-plugins/legacy-bash-adapter.wasm"
timeout_ms = 10000
on_error = "continue"

[hooks.capabilities.exec_subprocess]
binary_allow = ["bash"]
shell_bypass_acknowledged = "yes"
cwd_allow = ["."]

[hooks.config]
script_path = "hooks/capture.sh"
EOF
}

# ---------------------------------------------------------------------------
# Dispatcher invocation helper
# ---------------------------------------------------------------------------

# Run the dispatcher with a given JSON envelope via stdin.
# Captures combined stdout+stderr into $output; sets $status.
_run_dispatcher() {
  local envelope="$1"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$PROJECT_DIR' HOME='$WORK/home' '$DISPATCHER' 2>&1"
}

# ---------------------------------------------------------------------------
# Git repo setup helper
# ---------------------------------------------------------------------------

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
# AC-001, AC-013 (VP-093-A) / BC-1.16.001 PC1; INV1; INV5
#
# GREEN: inject_git_context_if_qualifying is fully implemented in invoke.rs and
# wired into main.rs. The captured payload contains git_context with all four
# fields populated, matching the synthetic repo's real HEAD and HEAD^ values.
#
# NON-TAUTOLOGY signal: asserts EXACT field VALUES (head_sha, head_subject,
# head_parent_sha, head_parent_subject) from the real synthetic git repo.
# A fail-open implementation producing all-empty fields would fail this test.
# A non-injection implementation produces no git_context key, failing this test.
# ---------------------------------------------------------------------------

@test "VP-093-A: dispatcher injects four-field git_context on qualifying PostToolUse Bash git-commit" {
  _require_dispatcher
  _require_adapter
  _require_jq
  _require_git

  _setup_factory_git_repo 2
  _write_posttooluse_capture_registry

  # Qualifying envelope: PostToolUse, tool=Bash, command contains "git commit" and ".factory".
  # The factory_dir is derived from CLAUDE_PROJECT_DIR/.factory by the dispatcher.
  local envelope
  envelope='{"event_name":"PostToolUse","tool_name":"Bash","session_id":"vp093-a","tool_input":{"command":"git -C .factory commit -m \"state: burst-02 Commit B\""},"tool_response":{"exit_code":0}}'

  _run_dispatcher "$envelope"

  # Dispatcher must exit 0.
  [ "$status" -eq 0 ]

  # CAPTURE_FILE must exist and be non-empty (confirms the plugin was invoked).
  # A non-empty CAPTURE_FILE is the positive-coverage sentinel: if the plugin is
  # never invoked (registry mismatch, timeout, skip), CAPTURE_FILE is empty and
  # the test fails rather than vacuously passing.
  [ -s "$CAPTURE_FILE" ]

  # git_context must be present in the captured payload.
  # Verifies injection ran for this qualifying PostToolUse Bash git-commit event.
  local git_ctx_present
  git_ctx_present="$(jq 'has("git_context")' "$CAPTURE_FILE")"
  [ "$git_ctx_present" = "true" ]

  # All four fields must be present as strings.
  local head_subject head_sha parent_subject parent_sha
  head_subject="$(jq -r '.git_context.head_subject' "$CAPTURE_FILE")"
  head_sha="$(jq -r '.git_context.head_sha' "$CAPTURE_FILE")"
  parent_subject="$(jq -r '.git_context.head_parent_subject' "$CAPTURE_FILE")"
  parent_sha="$(jq -r '.git_context.head_parent_sha' "$CAPTURE_FILE")"

  # NON-TAUTOLOGY: verify EXACT VALUES from the real synthetic repo.
  # Fail-open (all-empty) would fail these assertions.
  [ "$head_subject" = "$GIT_HEAD_SUBJECT" ]
  [ "$head_sha" = "$GIT_HEAD_SHA" ]
  [ "$parent_subject" = "$GIT_PARENT_SUBJECT" ]
  [ "$parent_sha" = "$GIT_PARENT_SHA" ]

  # head_sha must be a 40-char hex string (not empty, not garbage).
  echo "$head_sha" | grep -qE '^[0-9a-f]{40}$'

  # parent_sha must also be a 40-char hex string (two-commit repo, so HEAD^ exists).
  echo "$parent_sha" | grep -qE '^[0-9a-f]{40}$'

  # Confirm no field is the string "null" (distinguishes JSON null from empty string).
  [ "$head_subject" != "null" ]
  [ "$head_sha" != "null" ]
  [ "$parent_subject" != "null" ]
  [ "$parent_sha" != "null" ]
}

# ---------------------------------------------------------------------------
# VP-093-B: All-empty injection on git error (non-git factory dir)
#
# AC-002, AC-009, AC-013 (VP-093-B) / BC-1.16.001 PC2; INV3
#
# GREEN: injection is implemented; git_context is present with all four fields
# set to "" when git commands fail (non-git directory). Dispatcher exits 0
# (fail-open, no block on git error; BC-1.16.001 INV3).
#
# NON-TAUTOLOGY signal: the test asserts each field IS present and IS the empty
# string. A no-injection implementation leaves git_context absent (has("git_context")
# = false → failure). A partial-inject implementation with null fields fails the
# jq-r output check (jq -r returns "null" string for JSON null).
# ---------------------------------------------------------------------------

@test "VP-093-B: dispatcher injects all-empty git_context and exits 0 on git error (non-git factory dir)" {
  _require_dispatcher
  _require_adapter
  _require_jq

  # Do NOT initialise a git repo in FACTORY_DIR — git commands will fail with non-zero exit.
  # The dispatcher must still exit 0 (fail-open; AC-009 / BC-1.16.001 INV3).
  _write_posttooluse_capture_registry

  local envelope
  envelope='{"event_name":"PostToolUse","tool_name":"Bash","session_id":"vp093-b","tool_input":{"command":"git -C .factory commit -m \"state: burst-01\""},"tool_response":{"exit_code":0}}'

  _run_dispatcher "$envelope"

  # Dispatcher must exit 0 even when git fails (fail-open, BC-1.16.001 INV3).
  [ "$status" -eq 0 ]

  # Plugin must be invoked (positive-coverage sentinel).
  [ -s "$CAPTURE_FILE" ]

  # git_context must be present in the captured payload (all-empty fail-open form).
  # Verifies the fail-open path populates all four fields as "" rather than omitting git_context.
  local git_ctx_present
  git_ctx_present="$(jq 'has("git_context")' "$CAPTURE_FILE")"
  [ "$git_ctx_present" = "true" ]

  # All four fields must be present and equal to "" (empty string, not null, not absent).
  local head_subject head_sha parent_subject parent_sha
  head_subject="$(jq -r '.git_context.head_subject' "$CAPTURE_FILE")"
  head_sha="$(jq -r '.git_context.head_sha' "$CAPTURE_FILE")"
  parent_subject="$(jq -r '.git_context.head_parent_subject' "$CAPTURE_FILE")"
  parent_sha="$(jq -r '.git_context.head_parent_sha' "$CAPTURE_FILE")"

  [ "$head_subject" = "" ]
  [ "$head_sha" = "" ]
  [ "$parent_subject" = "" ]
  [ "$parent_sha" = "" ]

  # Fields must be JSON strings (not null). jq -r emits "null" string for JSON null.
  head_subject_raw="$(jq '.git_context.head_subject' "$CAPTURE_FILE")"
  head_sha_raw="$(jq '.git_context.head_sha' "$CAPTURE_FILE")"
  parent_subject_raw="$(jq '.git_context.head_parent_subject' "$CAPTURE_FILE")"
  parent_sha_raw="$(jq '.git_context.head_parent_sha' "$CAPTURE_FILE")"

  [ "$head_subject_raw" = '""' ]
  [ "$head_sha_raw" = '""' ]
  [ "$parent_subject_raw" = '""' ]
  [ "$parent_sha_raw" = '""' ]
}

# ---------------------------------------------------------------------------
# VP-093-C: No injection on non-qualifying Bash event (git push)
#
# AC-003, AC-012, AC-013 (VP-093-C) / BC-1.16.001 PC3; EC-004
#
# DESIGN NOTE: This test asserts ABSENCE of git_context. A stub (no injection)
# also produces absence — so this test may appear GREEN during Red Gate phase.
# Non-tautology discipline is maintained via the positive-coverage sentinel:
# CAPTURE_FILE must be non-empty (plugin WAS invoked). If the plugin is never
# called, the test fails for the right reason (registry/routing failure), not
# vacuously. After implementation, the absence assertion remains correct.
# ---------------------------------------------------------------------------

@test "VP-093-C: dispatcher does NOT inject git_context on non-qualifying Bash event (git push)" {
  _require_dispatcher
  _require_adapter
  _require_jq
  _require_git

  _setup_factory_git_repo 2
  _write_posttooluse_capture_registry

  # Non-qualifying envelope: PostToolUse, tool=Bash, but command is git push (not commit).
  local envelope
  envelope='{"event_name":"PostToolUse","tool_name":"Bash","session_id":"vp093-c","tool_input":{"command":"git -C .factory push origin factory-artifacts"},"tool_response":{"exit_code":0}}'

  _run_dispatcher "$envelope"

  # Dispatcher exits 0.
  [ "$status" -eq 0 ]

  # Positive-coverage sentinel: plugin must be invoked (CAPTURE_FILE non-empty).
  # Without this, the test could pass vacuously because the plugin was never called.
  [ -s "$CAPTURE_FILE" ]

  # git_context key must be ABSENT from the captured payload for git push.
  local git_ctx_present
  git_ctx_present="$(jq 'has("git_context")' "$CAPTURE_FILE")"
  [ "$git_ctx_present" = "false" ]
}

# ---------------------------------------------------------------------------
# VP-093-D: No injection on PostToolUse Edit event
#
# AC-004, AC-008, AC-013 (VP-093-D) / BC-1.16.001 PC4; INV2
#
# DESIGN NOTE: Same non-tautology discipline as VP-093-C. Edit events route to
# the PostToolUse plugin registered without a tool= filter. The positive-coverage
# sentinel (CAPTURE_FILE non-empty) confirms the plugin received the Edit payload.
# After implementation, absence of git_context in Edit events is verified correctly.
# ---------------------------------------------------------------------------

@test "VP-093-D: dispatcher does NOT inject git_context on PostToolUse Edit event" {
  _require_dispatcher
  _require_adapter
  _require_jq

  _write_posttooluse_capture_registry

  # PostToolUse Edit event: tool_name=Edit. Dispatcher MUST NOT inspect command or inject.
  local envelope
  envelope='{"event_name":"PostToolUse","tool_name":"Edit","session_id":"vp093-d","tool_input":{"file_path":".factory/STATE.md","old_string":"a","new_string":"b"},"tool_response":{"success":true}}'

  _run_dispatcher "$envelope"

  [ "$status" -eq 0 ]

  # Positive-coverage sentinel: plugin must be invoked for PostToolUse Edit events.
  [ -s "$CAPTURE_FILE" ]

  # git_context key must be ABSENT for non-Bash tool events (AC-004 / INV2).
  local git_ctx_present
  git_ctx_present="$(jq 'has("git_context")' "$CAPTURE_FILE")"
  [ "$git_ctx_present" = "false" ]
}

# ---------------------------------------------------------------------------
# VP-093-E: Initial-commit edge case (head_parent_subject="" not null)
#
# AC-006, AC-011, AC-013 (VP-093-E) / BC-1.16.001 PC6; INV5; EC-003; EC-009
#
# GREEN: injection is implemented; git_context is present with:
#   head_subject = GIT_HEAD_SUBJECT (non-empty, from real HEAD commit)
#   head_sha = 40-char hex (non-empty, real HEAD SHA)
#   head_parent_subject = "" (empty string, NOT null, NOT absent — HEAD^ does not exist)
#   head_parent_sha = "" (empty string, NOT null, NOT absent — HEAD^ does not exist)
#
# NON-TAUTOLOGY signal: asserts head_sha is non-empty AND a 40-char hex (proving
# HEAD was populated correctly), while asserting parent fields are "" (proving the
# HEAD^-non-existent path ran, not a generic fail-open). An all-empty fail-open
# (from a git error on HEAD itself) would fail the head_sha hex assertion.
# ---------------------------------------------------------------------------

@test "VP-093-E: initial commit — head_parent_subject and head_parent_sha are empty strings (not null)" {
  _require_dispatcher
  _require_adapter
  _require_jq
  _require_git

  # Single-commit factory-artifacts repo (no HEAD^).
  _setup_factory_git_repo 1
  _write_posttooluse_capture_registry

  # Qualifying envelope targeting the single-commit factory-artifacts repo.
  local envelope
  envelope='{"event_name":"PostToolUse","tool_name":"Bash","session_id":"vp093-e","tool_input":{"command":"git -C .factory commit --allow-empty -m \"state: burst-01 Commit A\""},"tool_response":{"exit_code":0}}'

  _run_dispatcher "$envelope"

  [ "$status" -eq 0 ]

  # Positive-coverage sentinel.
  [ -s "$CAPTURE_FILE" ]

  # git_context must be present (injection ran for this qualifying event).
  # Verifies the initial-commit path is handled: HEAD is populated, HEAD^ yields "".
  local git_ctx_present
  git_ctx_present="$(jq 'has("git_context")' "$CAPTURE_FILE")"
  [ "$git_ctx_present" = "true" ]

  # head_subject and head_sha must be populated (HEAD exists on initial commit).
  local head_subject head_sha
  head_subject="$(jq -r '.git_context.head_subject' "$CAPTURE_FILE")"
  head_sha="$(jq -r '.git_context.head_sha' "$CAPTURE_FILE")"

  [ "$head_subject" = "$GIT_HEAD_SUBJECT" ]
  [ -n "$head_sha" ]
  echo "$head_sha" | grep -qE '^[0-9a-f]{40}$'

  # NON-TAUTOLOGY for HEAD population: head_sha must not be "" or "null".
  # An all-empty fail-open (git failed on HEAD itself) would produce head_sha="" → test fails.
  [ "$head_sha" != "" ]
  [ "$head_sha" != "null" ]

  # head_parent_subject and head_parent_sha must be empty string "" (not null, not absent).
  # This distinguishes the initial-commit path (HEAD^ non-existent → "") from a general
  # git error (all-empty including HEAD fields) — the NON-TAUTOLOGY for the parent path.
  local parent_subject parent_sha
  parent_subject="$(jq -r '.git_context.head_parent_subject' "$CAPTURE_FILE")"
  parent_sha="$(jq -r '.git_context.head_parent_sha' "$CAPTURE_FILE")"

  [ "$parent_subject" = "" ]
  [ "$parent_sha" = "" ]

  # Confirm parent fields are JSON "" not JSON null (jq raw output of null = "null").
  parent_subject_raw="$(jq '.git_context.head_parent_subject' "$CAPTURE_FILE")"
  parent_sha_raw="$(jq '.git_context.head_parent_sha' "$CAPTURE_FILE")"
  [ "$parent_subject_raw" = '""' ]
  [ "$parent_sha_raw" = '""' ]
}
