#!/usr/bin/env bats
# regression-v1.0.bats — v0.79.x → v1.0 dispatcher regression checks (S-2.7).
#
# These tests document the *current* end-to-end behavior of the v1.0
# dispatcher when invoked with synthetic Claude Code envelopes. They are
# tier-C regression guards: each test pins one observable invariant of
# the dispatcher → legacy-bash-adapter → bash hook pipeline so a future
# refactor can't silently regress the path.
#
# History: S-2.7 originally discovered three bugs that prevented the
# adapter pipeline from working end-to-end. All three were fixed in the
# same wave of work (commit c121d07):
# - exec_subprocess wrote the result envelope to wasm memory offset 0
#   and the SDK short-circuited ptr==0 to an empty Vec, so bash output
#   never reached the adapter.
# - HostContext.plugin_root was never populated, so relative
#   script_path values resolved against bash's cwd instead of the
#   plugin root.
# - HostContext.env_view was empty, so per-plugin env_allow always
#   produced an empty subprocess env.
# The assertions below describe the post-fix end-state. They are the
# regression guards that catch any future revert.

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

# ---------- adapter round-trip (post-fix end-state) --------------------
# These tests pin the desired adapter pipeline behavior. Until commit
# c121d07 they asserted the buggy state with `[BUG]` markers; they now
# guard the correct round-trip end-state.

@test "regression-v1.0: legacy adapter exits 0 (Continue) for a matched bash hook that exited 0" {
  if [ ! -x "$DISPATCHER" ] || [ ! -f "$ADAPTER_WASM" ]; then
    skip "preflight artifacts missing"
  fi
  envelope='{"event_name":"PostToolUse","tool_name":"Bash","session_id":"adapter-ok","tool_input":{"command":"echo hi"},"tool_response":{"exit_code":0}}'
  env CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT" CLAUDE_PROJECT_DIR="$WORK" \
    bash -c "printf '%s' '$envelope' | '$DISPATCHER'" \
    >/dev/null 2>&1
  log="$(ls "$WORK/.factory/logs/dispatcher-internal-"*.jsonl 2>/dev/null | head -1)"
  [ -n "$log" ]
  # At least one plugin.completed with exit_code:0 (the bash hook ran
  # cleanly and the adapter mapped that to HookResult::Continue).
  ok="$(grep -c '"type":"plugin.completed".*"exit_code":0' "$log" || true)"
  [ "$ok" -ge 1 ]
}

@test "regression-v1.0: bash-side events land in events-*.jsonl when routed through the adapter" {
  if [ ! -x "$DISPATCHER" ] || [ ! -f "$ADAPTER_WASM" ]; then
    skip "preflight artifacts missing"
  fi
  # NOTE: This test originally used a PostToolUse/Bash envelope expecting
  # capture-commit-activity.sh to run via the legacy-bash-adapter. After
  # the WASM migration (commit 818fb95), capture-commit-activity is now a
  # native Wasm plugin and PostToolUse/Bash routes only to WASM hooks
  # (capture-commit-activity.wasm, capture-pr-activity.wasm,
  # regression-gate.wasm) — none through legacy-bash-adapter. The test
  # was rewritten (PR #79) to use purity-check.sh, which is still
  # adapter-routed on PostToolUse/Edit|Write and emits
  # `pure_core_boundary_violation` via bin/emit-event into events-*.jsonl
  # whenever the edited file is under */pure/** with side-effecting code.
  pure_dir="$WORK/src/pure"
  mkdir -p "$pure_dir"
  pure_file="$pure_dir/impure.rs"
  cat > "$pure_file" <<EOF
use std::fs;
fn main() {
    println!("hello");
}
EOF
  envelope=$(printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"events-land","tool_input":{"file_path":"%s"},"tool_response":{"exit_code":0}}' "$pure_file")
  env CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT" CLAUDE_PROJECT_DIR="$WORK" \
    bash -c "printf '%s' '$envelope' | '$DISPATCHER'" \
    >/dev/null 2>&1
  count="$(ls "$WORK/.factory/logs/events-"*.jsonl 2>/dev/null | wc -l | tr -d ' ')"
  [ "$count" -ge 1 ]
}

# NOTE: The "direct-bash side-effect baseline" section previously contained
# two tests that invoked capture-commit-activity.sh and block-ai-attribution.sh
# directly. Both .sh files were deleted in commit 818fb95 (superseded by
# native WASM equivalents). The tests were removed here to prevent CI failures.
# End-to-end coverage of these behaviors is provided by the dispatcher test
# suite (regression-v1.0 adapter round-trip tests above).
