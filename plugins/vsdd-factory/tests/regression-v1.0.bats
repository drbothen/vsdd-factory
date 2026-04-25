#!/usr/bin/env bats
# regression-v1.0.bats — v0.79.x → v1.0 dispatcher regression checks (S-2.7).
#
# These tests document the *current* end-to-end behavior of the v1.0
# dispatcher when invoked with synthetic Claude Code envelopes. They are
# tier-C regression guards: each test pins one observable invariant of
# the dispatcher → legacy-bash-adapter → bash hook pipeline so a future
# refactor can't silently regress the path.
#
# IMPORTANT: as of v1.0.0-beta.1, S-2.7 discovered a bug where
# legacy-bash-adapter never round-trips bash output back to the wasm
# module — the host writes the result envelope to wasm linear memory
# offset 0 and the SDK's read_owned_bytes short-circuits ptr==0 to an
# empty Vec. Tests in this file assert the *observed* behavior so the
# bug surface is durable; once the bug is fixed (S-2.1 / S-2.7 follow-
# up), the assertions flip to the desired-behavior set marked with
# TODO(S-2.7-fixup) below.
#
# Cross-references:
# - crates/factory-dispatcher/src/host/exec_subprocess.rs:87 (host bug)
# - crates/factory-dispatcher/src/invoke.rs:489 (StoreData mirror)
# - crates/hook-sdk/src/host.rs:283 (SDK ptr==0 short-circuit)

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  ADAPTER_WASM="$PLUGIN_ROOT/hook-plugins/legacy-bash-adapter.wasm"
  WORK="$BATS_TEST_TMPDIR/proj"
  mkdir -p "$WORK/.factory/logs"
}

# ---------- preflight ---------------------------------------------------

@test "regression-v1.0: dispatcher binary exists (cargo build --workspace --release)" {
  if [ ! -x "$DISPATCHER" ]; then
    skip "dispatcher not built — run 'cargo build --workspace --release' first"
  fi
  [ -x "$DISPATCHER" ]
}

@test "regression-v1.0: legacy-bash-adapter.wasm exists at the registry-resolved path" {
  if [ ! -f "$ADAPTER_WASM" ]; then
    skip "adapter wasm not present — copy from target/wasm32-wasip1/release/"
  fi
  [ -f "$ADAPTER_WASM" ]
}

# ---------- end-to-end smoke -------------------------------------------

@test "regression-v1.0: dispatcher exits 0 for an event with no matching plugins" {
  if [ ! -x "$DISPATCHER" ] || [ ! -f "$ADAPTER_WASM" ]; then
    skip "preflight artifacts missing"
  fi
  run env CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT" CLAUDE_PROJECT_DIR="$WORK" \
    bash -c "echo '{\"event_name\":\"NoSuch\",\"tool_name\":\"X\",\"session_id\":\"s\"}' | '$DISPATCHER'"
  [ "$status" -eq 0 ]
}

@test "regression-v1.0: dispatcher writes dispatcher.started event for every invocation" {
  if [ ! -x "$DISPATCHER" ] || [ ! -f "$ADAPTER_WASM" ]; then
    skip "preflight artifacts missing"
  fi
  env CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT" CLAUDE_PROJECT_DIR="$WORK" \
    bash -c "echo '{\"event_name\":\"NoSuch\",\"tool_name\":\"X\",\"session_id\":\"s\"}' | '$DISPATCHER'" \
    >/dev/null 2>&1
  log="$(ls "$WORK/.factory/logs/dispatcher-internal-"*.jsonl 2>/dev/null | head -1)"
  [ -n "$log" ]
  grep -q '"type":"dispatcher.started"' "$log"
}

@test "regression-v1.0: dispatcher matches at least one plugin for PostToolUse/Bash" {
  if [ ! -x "$DISPATCHER" ] || [ ! -f "$ADAPTER_WASM" ]; then
    skip "preflight artifacts missing"
  fi
  envelope='{"event_name":"PostToolUse","tool_name":"Bash","session_id":"s","tool_input":{"command":"git commit"},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
  env CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT" CLAUDE_PROJECT_DIR="$WORK" \
    bash -c "printf '%s' '$envelope' | '$DISPATCHER'" \
    >/dev/null 2>&1
  log="$(ls "$WORK/.factory/logs/dispatcher-internal-"*.jsonl 2>/dev/null | head -1)"
  [ -n "$log" ]
  # plugin.invoked entries — at least one
  count="$(grep -c '"type":"plugin.invoked"' "$log" || true)"
  [ "$count" -ge 1 ]
}

@test "regression-v1.0: dispatcher emits plugin.invoked + plugin.completed lifecycle pairs" {
  if [ ! -x "$DISPATCHER" ] || [ ! -f "$ADAPTER_WASM" ]; then
    skip "preflight artifacts missing"
  fi
  envelope='{"event_name":"PreToolUse","tool_name":"Bash","session_id":"s","tool_input":{"command":"echo hi"}}'
  env CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT" CLAUDE_PROJECT_DIR="$WORK" \
    bash -c "printf '%s' '$envelope' | '$DISPATCHER'" \
    >/dev/null 2>&1
  log="$(ls "$WORK/.factory/logs/dispatcher-internal-"*.jsonl 2>/dev/null | head -1)"
  [ -n "$log" ]
  invoked="$(grep -c '"type":"plugin.invoked"' "$log" || true)"
  completed="$(grep -c '"type":"plugin.completed"' "$log" || true)"
  [ "$invoked" -ge 1 ]
  [ "$invoked" -eq "$completed" ]
}

@test "regression-v1.0: every internal-log event has dispatcher_trace_id and session_id" {
  if [ ! -x "$DISPATCHER" ] || [ ! -f "$ADAPTER_WASM" ]; then
    skip "preflight artifacts missing"
  fi
  envelope='{"event_name":"PostToolUse","tool_name":"Bash","session_id":"trace-test","tool_input":{"command":"echo hi"},"tool_response":{"exit_code":0}}'
  env CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT" CLAUDE_PROJECT_DIR="$WORK" \
    bash -c "printf '%s' '$envelope' | '$DISPATCHER'" \
    >/dev/null 2>&1
  log="$(ls "$WORK/.factory/logs/dispatcher-internal-"*.jsonl 2>/dev/null | head -1)"
  [ -n "$log" ]
  total="$(wc -l < "$log")"
  with_trace="$(grep -c '"dispatcher_trace_id":"' "$log" || true)"
  with_session="$(grep -c '"session_id":"trace-test"' "$log" || true)"
  [ "$total" -gt 0 ]
  [ "$with_trace" -eq "$total" ]
  [ "$with_session" -eq "$total" ]
}

# ---------- known-bug pins (S-2.7-beta) --------------------------------
# These tests pin the *current observed* behavior of the broken adapter
# path so the bug-surface is detectable in CI and a fix flips them
# obviously. When the host-memory offset-0 bug is fixed (S-2.1
# follow-up), invert the assertion and remove the TODO line.

@test "regression-v1.0[BUG]: legacy adapter currently exits non-zero (HookResult::Error) for matched bash hooks" {
  # TODO(S-2.7-fixup): once exec_subprocess writes envelope at a real
  # guest-allocated buffer instead of wasm memory offset 0, this
  # assertion flips to expect exit_code: 0 (Continue) for hooks that
  # exited 0 from bash.
  if [ ! -x "$DISPATCHER" ] || [ ! -f "$ADAPTER_WASM" ]; then
    skip "preflight artifacts missing"
  fi
  envelope='{"event_name":"PostToolUse","tool_name":"Bash","session_id":"bug-test","tool_input":{"command":"echo hi"},"tool_response":{"exit_code":0}}'
  env CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT" CLAUDE_PROJECT_DIR="$WORK" \
    bash -c "printf '%s' '$envelope' | '$DISPATCHER'" \
    >/dev/null 2>&1
  log="$(ls "$WORK/.factory/logs/dispatcher-internal-"*.jsonl 2>/dev/null | head -1)"
  [ -n "$log" ]
  # Every plugin.completed should currently report exit_code:1 — that is
  # SDK exit-code-1 → HookResult::Error returned by the adapter when
  # exec_subprocess fails to round-trip its envelope.
  ok="$(grep -c '"exit_code":0' "$log" || true)"
  err="$(grep -c '"exit_code":1' "$log" || true)"
  # When the bug is present, ok==0 and err>=1.
  [ "$ok" -eq 0 ]
  [ "$err" -ge 1 ]
}

@test "regression-v1.0[BUG]: bash-side events do NOT land in events-*.jsonl when routed through the adapter" {
  # TODO(S-2.7-fixup): once the adapter actually runs bash, hooks like
  # capture-commit-activity will write commit.made events to
  # events-YYYY-MM-DD.jsonl and this assertion flips to "do land".
  if [ ! -x "$DISPATCHER" ] || [ ! -f "$ADAPTER_WASM" ]; then
    skip "preflight artifacts missing"
  fi
  envelope='{"event_name":"PostToolUse","tool_name":"Bash","session_id":"sx","tool_input":{"command":"git commit -m x"},"tool_response":{"exit_code":0,"stdout":"[main abc1234] x","stderr":""}}'
  env CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT" CLAUDE_PROJECT_DIR="$WORK" \
    bash -c "printf '%s' '$envelope' | '$DISPATCHER'" \
    >/dev/null 2>&1
  count="$(ls "$WORK/.factory/logs/events-"*.jsonl 2>/dev/null | wc -l | tr -d ' ')"
  [ "$count" -eq 0 ]
}

# ---------- direct-bash side-effect baseline ---------------------------
# Sanity tests that the bash hooks *themselves* still work outside the
# dispatcher path. If these fail, the regression is in the bash script,
# not the dispatcher.

@test "regression-v1.0: capture-commit-activity (direct) emits commit.made for a successful git commit" {
  envelope='{"event_name":"PostToolUse","tool_name":"Bash","session_id":"d","tool_input":{"command":"git commit -m \"x\""},"tool_response":{"exit_code":0,"stdout":"[main abc1234] x","stderr":""}}'
  run env CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT" \
        CLAUDE_PROJECT_DIR="$WORK" \
        VSDD_LOG_DIR="$WORK/.factory/logs" \
    bash -c "printf '%s' '$envelope' | bash '$PLUGIN_ROOT/hooks/capture-commit-activity.sh'"
  [ "$status" -eq 0 ]
  log="$(ls "$WORK/.factory/logs/events-"*.jsonl 2>/dev/null | head -1)"
  [ -n "$log" ]
  grep -q '"type":"commit.made"' "$log"
  grep -q '"commit_sha":"abc1234"' "$log"
}

@test "regression-v1.0: block-ai-attribution (direct) blocks (exit 2) on Co-Authored-By: Claude" {
  envelope='{"event_name":"PreToolUse","tool_name":"Bash","session_id":"d","tool_input":{"command":"git commit -m foo\n\nCo-Authored-By: Claude <noreply@anthropic.com>"}}'
  run env CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT" \
        CLAUDE_PROJECT_DIR="$WORK" \
        VSDD_LOG_DIR="$WORK/.factory/logs" \
    bash -c "printf '%s' '$envelope' | bash '$PLUGIN_ROOT/hooks/block-ai-attribution.sh'"
  [ "$status" -eq 2 ]
}
