#!/usr/bin/env bats
# vp084-proof.bats — VP-084 dispatcher-invocation proof test for S-18.04b.
#
# Exercises BC-5.41.003 INV2 / AC-007: the exemption proof MUST invoke
# validate-burst-log and validate-dispatch-advance via the factory-dispatcher
# (real WASM dispatch path), NOT by calling wasmtime directly or by calling
# any shell script.
#
# # ADR-029 Design (corrected from pre-ADR-029 proof harness)
# Per AC-007 and S-18.04b AC-007 (VP-084 proof-model update):
#   - Trigger: PostToolUse Bash (tool="Bash", tool_input.command contains "git commit")
#   - NOT Edit/Write — ADR-029 §Decision 1 flips the trigger to Bash.
#   - git_context MUST supply all 4 fields: head_subject, head_sha,
#     head_parent_subject, head_parent_sha (BC-1.16.001 4-field schema).
#   - Negative control: MUST supply sentinel subjects via git_context (NOT
#     rely on fail-open=empty path — that was the pass-1 F-1 tautology finding).
#
# # Red Gate condition
# - If WASM binaries are absent: test SKIPS (not pass).
# - If dispatcher is absent: test SKIPS (not pass).
# - If WASM binaries are present but exemption logic is not implemented (still
#   uses exec_subprocess / Edit trigger): positive tests FAIL (wrong Continue
#   for non-Bash trigger, or Block when exempt) and negative control PASSES
#   vacuously (the whole test is the Red Gate).
#
# # VP / BC trace
#   VP-084: PreCompact Flush Commit Is Lifecycle-Distinct From State-Manager Burst Commit
#   VP-093: Dispatcher Injects git_context Into payload.extra on PostToolUse Bash git-commit
#   BC-5.41.003 PC4: bats test coverage via dispatcher invocation
#   BC-5.41.003 INV2: symmetric implementation; both gates exercised
#   BC-1.16.001: git_context 4-field injection contract
#   AC-007: proof MUST use dispatcher, NOT wasmtime/direct WASM invocation

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../.." && pwd)"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  BURST_LOG_WASM="$REPO_ROOT/plugins/vsdd-factory/hook-plugins/validate-burst-log.wasm"
  DISPATCH_ADVANCE_WASM="$REPO_ROOT/plugins/vsdd-factory/hook-plugins/validate-dispatch-advance.wasm"

  WORK="$(mktemp -d)"

  # AC-019 / F-R3-004 pattern: CLAUDE_PROJECT_DIR is a DISTINCT subdirectory of WORK.
  PROJECT_DIR="$WORK/project"

  mkdir -p "$WORK/.factory/logs"
  mkdir -p "$WORK/hook-plugins"
  mkdir -p "$PROJECT_DIR/.factory/hooks"
  mkdir -p "$PROJECT_DIR/.factory/cycles/v1.0-feature-context-durability-E18"

  # Synthetic git-repo fixture (BC-1.16.001 INV3 + negative control requirement).
  # The git repo represents the "real project dir" seen by the dispatcher.
  # We initialise it so git_context-like fields can be verified via the log.
  FIXTURE_REPO="$WORK/fixture-git"
  mkdir -p "$FIXTURE_REPO"
  git -C "$FIXTURE_REPO" init -b main 2>/dev/null || git -C "$FIXTURE_REPO" init 2>/dev/null

  export CLAUDE_PLUGIN_ROOT="$WORK"
  export CLAUDE_PROJECT_DIR="$PROJECT_DIR"
}

teardown() {
  rm -rf "$WORK"
}

# ---------------------------------------------------------------------------
# Preflight skip helpers
# ---------------------------------------------------------------------------

_require_dispatcher() {
  if [ ! -x "$DISPATCHER" ]; then
    skip "factory-dispatcher not built — run: cargo build --release -p factory-dispatcher"
  fi
}

_require_burst_log_wasm() {
  _require_dispatcher
  if [ ! -f "$BURST_LOG_WASM" ]; then
    skip "validate-burst-log.wasm not compiled — run: cargo build --target wasm32-wasip1 --release -p validate-burst-log && cp target/wasm32-wasip1/release/validate-burst-log.wasm plugins/vsdd-factory/hook-plugins/"
  fi
  cp "$BURST_LOG_WASM" "$WORK/hook-plugins/validate-burst-log.wasm"
}

_require_dispatch_advance_wasm() {
  _require_dispatcher
  if [ ! -f "$DISPATCH_ADVANCE_WASM" ]; then
    skip "validate-dispatch-advance.wasm not compiled — run: cargo build --target wasm32-wasip1 --release -p validate-dispatch-advance && cp target/wasm32-wasip1/release/validate-dispatch-advance.wasm plugins/vsdd-factory/hook-plugins/"
  fi
  cp "$DISPATCH_ADVANCE_WASM" "$WORK/hook-plugins/validate-dispatch-advance.wasm"
}

# ---------------------------------------------------------------------------
# Registry writers — ADR-029: trigger is Bash, NOT Edit|Write
# ---------------------------------------------------------------------------

_write_burst_log_registry() {
  # ADR-029 §Decision 1: PostToolUse Bash (git commit events), NOT Edit|Write.
  cat > "$WORK/hooks-registry.toml" <<EOF
schema_version = 2

[[hooks]]
name = "validate-burst-log"
event = "PostToolUse"
tool = "Bash"
plugin = "hook-plugins/validate-burst-log.wasm"
priority = 100
timeout_ms = 10000
on_error = "block"
async = false

[hooks.capabilities.read_file]
path_allow = ["${PROJECT_DIR}/"]
EOF
}

_write_dispatch_advance_registry() {
  # ADR-029 §Decision 1: PostToolUse Bash (git commit events), NOT Edit|Write.
  cat > "$WORK/hooks-registry.toml" <<EOF
schema_version = 2

[[hooks]]
name = "validate-dispatch-advance"
event = "PostToolUse"
tool = "Bash"
plugin = "hook-plugins/validate-dispatch-advance.wasm"
priority = 100
timeout_ms = 10000
on_error = "block"
async = false

[hooks.capabilities.read_file]
path_allow = ["${PROJECT_DIR}/"]
EOF
}

# ---------------------------------------------------------------------------
# Dispatcher invocation helper
# ---------------------------------------------------------------------------

_run_dispatcher() {
  local envelope="$1"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$PROJECT_DIR' HOME='$WORK/home' '$DISPATCHER' 2>&1"
}

# ---------------------------------------------------------------------------
# Setup helper: write a precompact-flush-log entry.
# 4-field canonical format: <ISO-TS> <SHA> <cycle>/<step> commit
# ---------------------------------------------------------------------------

_setup_precompact_flush_log() {
  local flush_sha="$1"
  local log_file="$PROJECT_DIR/.factory/hooks/precompact-flush-log"
  printf "2026-06-14T00:00:00Z %s v1.0-feature-context-durability-E18/S-18.04 commit\n" "$flush_sha" > "$log_file"
}

# ---------------------------------------------------------------------------
# Setup helper: initialise $PROJECT_DIR/.factory as a real git repo with
# sentinel multi-commit chain subjects.
#
# Creates two commits so that:
#   HEAD subject       = "stage 1 backfill"
#   HEAD^ subject      = "stage 2 backfill"
#
# This satisfies the ADR-029 requirement that the negative control test
# supply REAL sentinel subjects via a git repo the dispatcher can query —
# not synthetic JSON fields that the dispatcher will overwrite with real
# git output (or empty on failure).
# ---------------------------------------------------------------------------
_setup_sentinel_git_chain() {
  local factory_dir="$PROJECT_DIR/.factory"
  # Configure git identity for the temp repo (no global config in temp HOME).
  export GIT_AUTHOR_NAME="vp084-test"
  export GIT_AUTHOR_EMAIL="test@vp084"
  export GIT_COMMITTER_NAME="vp084-test"
  export GIT_COMMITTER_EMAIL="test@vp084"

  git -C "$factory_dir" init -b main 2>/dev/null || git -C "$factory_dir" init 2>/dev/null

  # First commit — becomes HEAD^ after the second commit.
  printf 'fixture\n' > "$factory_dir/.gitkeep"
  git -C "$factory_dir" add .gitkeep
  git -C "$factory_dir" commit --no-gpg-sign -m "stage 2 backfill" 2>/dev/null

  # Second commit — becomes HEAD.
  printf 'fixture2\n' >> "$factory_dir/.gitkeep"
  git -C "$factory_dir" add .gitkeep
  git -C "$factory_dir" commit --no-gpg-sign -m "stage 1 backfill" 2>/dev/null
}

# ---------------------------------------------------------------------------
# Setup helper: write a structurally valid burst-log.md.
# Used so the burst-log structural validation passes — the only remaining
# possible block is MULTI_COMMIT_CHAIN_NOT_ALLOWED.
# ---------------------------------------------------------------------------

_write_valid_burst_log() {
  local log_file="$PROJECT_DIR/.factory/cycles/v1.0-feature-context-durability-E18/burst-log.md"
  cat > "$log_file" <<'EOF'
## Burst: vp084-proof test fixture (2026-06-14)

**Parent-commit:** abc1234

**Adversary verdict:** NITPICK — no blockers

**Files touched (Dim-1): 1 unique files**

- .factory/STATE.md

**Codifications:** D-500(a)

**Dim-2 Attestation:** gate invoked via literal shell; output captured.

**Dim-5 Attestation:** no security-relevant changes.

**Dim-6 Attestation:** no performance regressions.

**Dim-7 Attestation:** no accessibility changes.

**Closes:** D-500(a)
EOF
}

# ---------------------------------------------------------------------------
# VP-084 Test 1: validate-burst-log exempts PreCompact flush commit via dispatcher
# BC-5.41.003 PC4 / AC-007
# Red Gate Test Table row: test_vp084_exemption_via_dispatcher_not_wasmtime
#
# ADR-029: envelope uses tool="Bash" with command containing "git commit".
# git_context carries all 4 fields (BC-1.16.001 PC1).
# head_subject="PreCompact flush ..." → exemption must fire → dispatcher exits 0.
# ---------------------------------------------------------------------------

@test "test_vp084_exemption_via_dispatcher_not_wasmtime: validate-burst-log exempts PreCompact via dispatcher" {
  # AC-007: MUST invoke via dispatcher, NOT wasmtime.
  _require_burst_log_wasm
  _write_burst_log_registry
  _write_valid_burst_log

  local flush_sha="abc1234def5678abc1234def5678abc1234def56"
  local parent_sha="999aaabbbccc000111222333444555666777888f"
  _setup_precompact_flush_log "$flush_sha"

  # ADR-029 envelope: PostToolUse Bash, git commit command.
  # git_context: HEAD=PreCompact flush (exempt), HEAD^=state-manager burst.
  # All 4 git_context fields present per BC-1.16.001 PC1.
  local envelope
  envelope=$(printf '%s' "{\"event_name\":\"PostToolUse\",\"tool_name\":\"Bash\",\"session_id\":\"vp084-proof-session\",\"dispatcher_trace_id\":\"vp084-trace-1\",\"tool_input\":{\"command\":\"git -C .factory commit -m 'state: burst-24 Commit E'\"},\"git_context\":{\"head_subject\":\"PreCompact flush v1.0-feature-context-durability-E18/S-18.04 2026-06-14T00:00:00Z\",\"head_sha\":\"${flush_sha}\",\"head_parent_subject\":\"state: burst-24 Commit E — D-477 codification\",\"head_parent_sha\":\"${parent_sha}\"}}")

  _run_dispatcher "$envelope"

  # Assert: dispatcher exits 0 (no block_intent for MULTI_COMMIT_CHAIN_NOT_ALLOWED).
  # With ADR-029 corrected impl: head_subject starts with "PreCompact flush " → exempt →
  # chain check skips → Continue → exit 0.
  # With current exec-based impl (or wrong Edit trigger): either the plugin doesn't fire
  # (Bash event not matched by Edit|Write registry), exits 0 vacuously — which means the
  # positive test passes incorrectly. See negative control for the real gate.
  [ "$status" -eq 0 ]
  # Assert: no MULTI_COMMIT_CHAIN block in output.
  if echo "$output" | grep -q "MULTI_COMMIT_CHAIN_NOT_ALLOWED"; then
    echo "FAIL: output contains MULTI_COMMIT_CHAIN_NOT_ALLOWED — exemption not working" >&3
    false
  fi
}

# ---------------------------------------------------------------------------
# VP-084 Test 2: validate-dispatch-advance exempts PreCompact flush commit via dispatcher
# BC-5.41.003 INV2 / AC-006 / AC-007
#
# ADR-029: same Bash envelope + 4-field git_context, symmetric with Test 1.
# ---------------------------------------------------------------------------

@test "test_vp084_dispatch_advance_exemption_via_dispatcher: validate-dispatch-advance exempts PreCompact via dispatcher" {
  # AC-007 + INV2: validate-dispatch-advance also MUST use dispatcher path.
  _require_dispatch_advance_wasm
  _write_dispatch_advance_registry

  local flush_sha="abc1234def5678abc1234def5678abc1234def56"
  local parent_sha="999aaabbbccc000111222333444555666777888f"
  _setup_precompact_flush_log "$flush_sha"

  # ADR-029 envelope: PostToolUse Bash, git commit command.
  # git_context: HEAD=PreCompact flush (exempt), HEAD^=state-manager burst.
  local envelope
  envelope=$(printf '%s' "{\"event_name\":\"PostToolUse\",\"tool_name\":\"Bash\",\"session_id\":\"vp084-proof-session-da\",\"dispatcher_trace_id\":\"vp084-trace-2\",\"tool_input\":{\"command\":\"git -C .factory commit -m 'PreCompact flush cycle'\"},\"git_context\":{\"head_subject\":\"PreCompact flush v1.0-feature-context-durability-E18/S-18.04 2026-06-14T00:00:00Z\",\"head_sha\":\"${flush_sha}\",\"head_parent_subject\":\"state: burst-24 Commit E — D-477 codification\",\"head_parent_sha\":\"${parent_sha}\"}}")

  _run_dispatcher "$envelope"

  # Assert: dispatcher exits 0 (no block from MULTI_COMMIT_CHAIN_NOT_ALLOWED).
  [ "$status" -eq 0 ]
  if echo "$output" | grep -q "MULTI_COMMIT_CHAIN_NOT_ALLOWED"; then
    echo "FAIL: output contains MULTI_COMMIT_CHAIN_NOT_ALLOWED — dispatch-advance exemption not working" >&3
    false
  fi
}

# ---------------------------------------------------------------------------
# VP-084 Negative control: non-PreCompact chain DOES trigger block via dispatcher.
# BC-5.41.003 PC3 / AC-007
#
# ADR-029 correction: This test MUST NOT rely on fail-open=empty git_context.
# It MUST supply real sentinel subjects via git_context (all 4 fields).
# This closes pass-1 F-1 tautology finding.
#
# head_subject="stage 1 backfill", head_parent_subject="stage 2 backfill" →
# both subjects contain sentinels → MULTI_COMMIT_CHAIN_NOT_ALLOWED → block.
#
# Negative control is un-skipped. It will fail (get Continue instead of Block)
# until the ADR-029 wiring is implemented (T-4..T-7). That failure IS the
# Red Gate confirming the test is non-tautological.
# ---------------------------------------------------------------------------

@test "test_vp084_non_precompact_chain_blocks_via_dispatcher: normal backfill chain triggers MULTI_COMMIT_CHAIN" {
  # This test is the non-tautological negative control (pass-1 F-1 fix).
  # git_context supplies real sentinel subjects — NOT empty fields (fail-open).
  # Before ADR-029 impl: plugin either doesn't fire (Edit trigger, wrong event)
  # or ignores git_context and reads exec_subprocess → non-deterministic.
  # After ADR-029 impl: plugin fires on Bash event, reads git_context sentinels
  # → MULTI_COMMIT_CHAIN_NOT_ALLOWED block.

  _require_burst_log_wasm
  _write_burst_log_registry
  _write_valid_burst_log

  # Set up a REAL git repo at $PROJECT_DIR/.factory with the sentinel chain.
  # ADR-029: the dispatcher overwrites caller-supplied git_context with real git
  # output from factory_dir ($CLAUDE_PROJECT_DIR/.factory). The WASM plugin reads
  # the dispatcher-injected git_context — NOT the JSON envelope field. So the
  # sentinel subjects must exist as REAL git commits, not JSON-only fields.
  _setup_sentinel_git_chain

  # Do NOT write precompact-flush-log → log-absent → case (c): prefix-only exemption.
  # But head_subject is NOT a PreCompact prefix → no exemption applies.
  # With both subjects as sentinels → MULTI_COMMIT_CHAIN_NOT_ALLOWED.

  # ADR-029 envelope: Bash git commit command (qualifying event). The dispatcher
  # reads real sentinel subjects from the git repo, overwriting any git_context
  # fields in this envelope (which serve only as documentation here).
  local envelope
  envelope=$(printf '%s' "{\"event_name\":\"PostToolUse\",\"tool_name\":\"Bash\",\"session_id\":\"vp084-negative-control\",\"dispatcher_trace_id\":\"vp084-trace-neg\",\"tool_input\":{\"command\":\"git -C .factory commit -m 'stage 1 backfill'\"}}")

  _run_dispatcher "$envelope"

  # Assert: MULTI_COMMIT_CHAIN_NOT_ALLOWED block fired.
  # Corrected ADR-029 impl: reads sentinel subjects from git_context → blocks.
  # Current impl (exec-based or wrong trigger): may exit 0 vacuously (Red Gate).
  # If dispatcher exits 0 AND output has no MULTI_COMMIT_CHAIN → test FAILS correctly.
  if echo "$output" | grep -q "MULTI_COMMIT_CHAIN_NOT_ALLOWED"; then
    # Block fired — negative control passes.
    true
  else
    # No block — the negative control confirms the impl doesn't read git_context.
    echo "RED GATE (negative control): expected MULTI_COMMIT_CHAIN_NOT_ALLOWED block for" \
         "sentinel chain in git_context; got Continue (exit=$status)." \
         "ADR-029 wiring not yet implemented." >&3
    false
  fi
}
