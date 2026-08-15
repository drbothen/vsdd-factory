#!/usr/bin/env bash
# run-scenario.sh — demo-evidence harness for S-21.07
# (validate-cross-site-correspondence PostToolUse WASM governance gate)
#
# Invokes the REAL factory-dispatcher binary against the REAL compiled
# validate-cross-site-correspondence.wasm plugin, using the exact
# dispatcher-invocation pattern that
# plugins/vsdd-factory/tests/validate-cross-site-correspondence.bats uses
# (PostToolUse envelope on stdin, hooks-registry.toml capability grant,
# VSDD_LOG_DIR internal JSONL log for advisory-level telemetry).
#
# Usage:
#   ./run-scenario.sh block          # AC-001 / PC2b — index-newer-than-primary BLOCK
#   ./run-scenario.sh indeterminate  # AC-028 / PC15b / PC26 — secondary-index UTF-8
#                                     # decode-failure INDETERMINATE advisory
#   ./run-scenario.sh continue       # well-formed artifacts — silent CONTINUE (pass)
#   ./run-scenario.sh all            # runs all three in sequence (used by the recording)
#
# Exits non-zero if the dispatcher binary or the compiled WASM plugin is not
# present locally — run `cargo build -p factory-dispatcher` and
# `cargo build --release --target wasm32-wasip1 -p validate-cross-site-correspondence`
# first (see plugins/vsdd-factory/tests/validate-cross-site-correspondence.bats
# for the exact build + staging commands).

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
FIXTURE_BASE="$PLUGIN_ROOT/tests/fixtures/validate-cross-site-correspondence"

DISPATCHER="$REPO_ROOT/target/debug/factory-dispatcher"
if [[ ! -x "$DISPATCHER" ]]; then
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
fi
if [[ ! -x "$DISPATCHER" ]]; then
  echo "FATAL: factory-dispatcher not found (tried target/debug and target/release)." >&2
  echo "  Run: cargo build -p factory-dispatcher" >&2
  exit 1
fi

GUARD_WASM="$PLUGIN_ROOT/hook-plugins/validate-cross-site-correspondence.wasm"
if [[ ! -f "$GUARD_WASM" ]]; then
  echo "FATAL: validate-cross-site-correspondence.wasm not staged at $GUARD_WASM" >&2
  echo "  Run: cargo build --release --target wasm32-wasip1 -p validate-cross-site-correspondence" >&2
  echo "  Then: cp target/wasm32-wasip1/release/validate_cross_site_correspondence.wasm \\" >&2
  echo "        plugins/vsdd-factory/hook-plugins/validate-cross-site-correspondence.wasm" >&2
  exit 1
fi

_write_registry() {
  local work="$1"
  cat > "$work/hooks-registry.toml" <<TOML
schema_version = 2

[[hooks]]
name = "validate-cross-site-correspondence"
event = "PostToolUse"
tool = "^(Edit|Write|MultiEdit)\$"
plugin = "hook-plugins/validate-cross-site-correspondence.wasm"
priority = 460
timeout_ms = 8000
on_error = "continue"
async = false

[hooks.capabilities.read_file]
path_allow = [
  ".factory/specs/behavioral-contracts/",
  ".factory/specs/verification-properties/",
  ".factory/stories/"
]
TOML
}

_run_dispatcher() {
  local work="$1" file_path="$2"
  local envelope escaped env_file
  escaped="$(printf '%s' "$file_path" | sed 's/\\/\\\\/g; s/"/\\"/g')"
  envelope="$(printf '{"event_name":"PostToolUse","tool_name":"Write","session_id":"demo","dispatcher_trace_id":"demo-trace","tool_input":{"file_path":"%s","content":""},"tool_response":{}}' "$escaped")"
  env_file="$work/envelope.json"
  printf '%s' "$envelope" > "$env_file"
  VSDD_LOG_DIR="$work/.factory/logs" \
    CLAUDE_PLUGIN_ROOT="$work" \
    CLAUDE_PROJECT_DIR="$work" \
    "$DISPATCHER" < "$env_file" 1>"$work/stdout.txt" 2>"$work/stderr.txt"
}

_setup_work() {
  local work; work="$(mktemp -d)"
  mkdir -p "$work/.factory/logs" "$work/hook-plugins"
  cp "$GUARD_WASM" "$work/hook-plugins/validate-cross-site-correspondence.wasm"
  _write_registry "$work"
  echo "$work"
}

_today_log() {
  local work="$1"
  echo "$work/.factory/logs/dispatcher-internal-$(date +%Y-%m-%d).jsonl"
}

run_block() {
  echo "=== SCENARIO: BLOCK  (AC-001 / BC-5.39.010 PC2b) ==="
  echo "    Fixture: a1-index-ahead-of-primary — BC-INDEX.md row cites v1.11,"
  echo "    frontmatter version: \"1.10\" (index newer than primary — anomalous)."
  local work; work="$(_setup_work)"
  cp -r "$FIXTURE_BASE/a1-index-ahead-of-primary/factory/." "$work/.factory/"
  set +e
  _run_dispatcher "$work" ".factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md"
  local rc=$?
  set -e
  echo "exit code: $rc"
  # block_reason is the last field on the stderr summary line and its value
  # itself contains unescaped double quotes (e.g. `version: is "1.10"`), so
  # anchor on the field name and take the rest of the line rather than a
  # naive [^"]* grep (which would truncate at the first embedded quote).
  grep -o 'block_reason=".*' "$work/stderr.txt" || echo "(no block_reason found)"
  rm -rf "$work"
}

run_indeterminate() {
  echo "=== SCENARIO: INDETERMINATE ADVISORY  (AC-028 / BC-5.39.010 PC15b / PC26) ==="
  echo "    Fixture: BC-INDEX.md corrupted with an invalid UTF-8 byte sequence."
  echo "    Primary target (BC-5.39.010.md) decodes and version-matches fine."
  local work; work="$(_setup_work)"
  cp -r "$SCRIPT_DIR/scenarios/indeterminate-index-corrupt/factory/." "$work/.factory/"
  # Corrupt BC-INDEX.md with an invalid UTF-8 continuation byte (0xFF 0xFE),
  # simulating a mis-encoded save. host::read_file succeeds as raw bytes;
  # the decode failure happens inside extract_bc_index_version_state().
  printf '\n<!-- CORRUPTION MARKER -->\xff\xfe\n' >> "$work/.factory/specs/behavioral-contracts/BC-INDEX.md"
  set +e
  _run_dispatcher "$work" ".factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md"
  local rc=$?
  set -e
  echo "exit code: $rc"
  local log; log="$(_today_log "$work")"
  echo "internal log advisory (host::log_warn) record:"
  grep '"plugin_name":"validate-cross-site-correspondence"' "$log" 2>/dev/null \
    | grep '"level":"warn"' \
    | grep -o '"message":"[^"]*"' \
    || echo "(no advisory record found — UNEXPECTED)"
  rm -rf "$work"
}

run_continue() {
  echo "=== SCENARIO: CONTINUE  (well-formed artifacts — clean pass) ==="
  echo "    Fixture: a1-current-index — BC-INDEX.md row cites v1.6, frontmatter"
  echo "    version: \"1.6\" (consistent)."
  local work; work="$(_setup_work)"
  cp -r "$FIXTURE_BASE/a1-current-index/factory/." "$work/.factory/"
  set +e
  _run_dispatcher "$work" ".factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md"
  local rc=$?
  set -e
  echo "exit code: $rc"
  local log; log="$(_today_log "$work")"
  if grep -q '"plugin_name":"validate-cross-site-correspondence"' "$log" 2>/dev/null \
     && grep '"plugin_name":"validate-cross-site-correspondence"' "$log" | grep -q '"level":"warn"'; then
    echo "UNEXPECTED: advisory present"
  else
    echo "no block, no advisory — write proceeds normally"
  fi
  rm -rf "$work"
}

case "${1:-}" in
  block) run_block ;;
  indeterminate) run_indeterminate ;;
  continue) run_continue ;;
  all)
    run_block
    echo
    run_indeterminate
    echo
    run_continue
    ;;
  *)
    echo "usage: $0 {block|indeterminate|continue|all}" >&2
    exit 1
    ;;
esac
