#!/usr/bin/env bash
# measure-bundle-sizes.sh — S-9.00 reproducible WASM bundle size measurement
#
# Usage: measure-bundle-sizes.sh <bundle-dir>
#   <bundle-dir>  Directory containing .wasm plugin files
#                 (e.g. plugins/vsdd-factory/hook-plugins/)
#
# Emits JSON to stdout with:
#   all_hook_plugins_wasm_bytes  — sum of ALL .wasm files in bundle-dir
#   grand_total_bytes            — all_hook_plugins_wasm_bytes + dispatcher_bytes
#   dispatcher_bytes             — factory-dispatcher binary size (0 if not found)
#   per_plugin                   — object with one entry per frozen-list plugin
#   cold_start_p95_measured_ms   — p95 cold-start latency for handoff-validator
#   methodology_version          — pinned version for this script
#   measurement_timestamp        — ISO-8601 UTC timestamp at measurement time
#   host_platform                — uname-derived platform string
#
# Cross-platform: macOS (BSD tools) + Linux (GNU tools) + Windows-via-Git-Bash
#   - Uses wc -c < <file> for byte counts (POSIX portable)
#   - Does NOT use du -sb (GNU-only -b flag)
#   - Does NOT use readarray (bash 4+ only)
#   - Does NOT use stat -c (GNU-only)
#   - Does NOT use realpath (Linux-only)
#
# Prerequisites:
#   - .wasm artifacts must already be built (cargo build --release --target wasm32-wasip1)
#   - factory-dispatcher binary must be built (cargo build --release -p factory-dispatcher)
#   - jq must be installed (for JSON output)
#   - hyperfine must be installed (for cold-start measurement)
#
# Methodology version: 1 (pinned; bump when measurement logic changes)

set -euo pipefail

METHODOLOGY_VERSION=1

# ---------------------------------------------------------------------------
# Frozen 17-plugin enumeration from AC-2 (names without .wasm extension)
# Do NOT change to a directory glob — enumeration must be frozen per spec.
# ---------------------------------------------------------------------------
FROZEN_PLUGINS=(
  block-ai-attribution
  capture-commit-activity
  capture-pr-activity
  handoff-validator
  legacy-bash-adapter
  pr-manager-completion-guard
  regression-gate
  session-end-telemetry
  session-learning
  session-start-telemetry
  tool-failure-hooks
  track-agent-start
  track-agent-stop
  update-wave-state-on-merge
  validate-pr-review-posted
  warn-pending-wave-gate
  worktree-hooks
)

# ---------------------------------------------------------------------------
# Argument parsing
# ---------------------------------------------------------------------------
if [ $# -lt 1 ]; then
  echo "Usage: $0 <bundle-dir>" >&2
  echo "  <bundle-dir>  path to directory containing .wasm plugin files" >&2
  exit 1
fi

BUNDLE_DIR="$1"

if [ ! -d "$BUNDLE_DIR" ]; then
  echo "ERROR: bundle directory not found: $BUNDLE_DIR" >&2
  echo "Run: cargo build --release --target wasm32-wasip1 to build .wasm artifacts" >&2
  exit 1
fi

# ---------------------------------------------------------------------------
# Locate repo root (the directory containing the .factory/ subtree)
# Resolve from this script's location: .factory/measurements/measure-bundle-sizes.sh
# ---------------------------------------------------------------------------
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# ---------------------------------------------------------------------------
# Locate factory-dispatcher binary
# ---------------------------------------------------------------------------
DISPATCHER_BINARY=""
if [ -f "$REPO_ROOT/target/release/factory-dispatcher" ]; then
  DISPATCHER_BINARY="$REPO_ROOT/target/release/factory-dispatcher"
elif [ -f "$REPO_ROOT/target/release/factory-dispatcher.exe" ]; then
  DISPATCHER_BINARY="$REPO_ROOT/target/release/factory-dispatcher.exe"
fi

# ---------------------------------------------------------------------------
# Locate the cold-start fixture
# ---------------------------------------------------------------------------
FIXTURE="$SCRIPT_DIR/fixtures/handoff-validator-input.json"

# ---------------------------------------------------------------------------
# Measure dispatcher binary size
# ---------------------------------------------------------------------------
DISPATCHER_BYTES=0
if [ -n "$DISPATCHER_BINARY" ] && [ -f "$DISPATCHER_BINARY" ]; then
  DISPATCHER_BYTES=$(wc -c < "$DISPATCHER_BINARY")
  # Trim leading whitespace (wc -c may add it on some platforms)
  DISPATCHER_BYTES="${DISPATCHER_BYTES##* }"
  DISPATCHER_BYTES="${DISPATCHER_BYTES%% *}"
else
  echo "WARNING: factory-dispatcher binary not found at $REPO_ROOT/target/release/factory-dispatcher" >&2
  echo "Run: cargo build --release -p factory-dispatcher" >&2
fi

# ---------------------------------------------------------------------------
# Sum ALL .wasm files in the bundle directory (AC-1)
# ---------------------------------------------------------------------------
ALL_WASM_BYTES=0
for f in "$BUNDLE_DIR"/*.wasm; do
  [ -f "$f" ] || continue
  sz=$(wc -c < "$f")
  # Trim whitespace
  sz="${sz##* }"
  sz="${sz%% *}"
  ALL_WASM_BYTES=$((ALL_WASM_BYTES + sz))
done

# ---------------------------------------------------------------------------
# Build per_plugin map for the 17 frozen plugins (AC-2, AC-5)
# Only counts present files; absent plugins record 0.
# ---------------------------------------------------------------------------
per_plugin_json="{"
first=1
for plugin in "${FROZEN_PLUGINS[@]}"; do
  wasm_file="$BUNDLE_DIR/${plugin}.wasm"
  if [ -f "$wasm_file" ]; then
    sz=$(wc -c < "$wasm_file")
    sz="${sz##* }"
    sz="${sz%% *}"
  else
    sz=0
  fi
  if [ "$first" -eq 1 ]; then
    per_plugin_json="${per_plugin_json}\"${plugin}\": ${sz}"
    first=0
  else
    per_plugin_json="${per_plugin_json}, \"${plugin}\": ${sz}"
  fi
done
per_plugin_json="${per_plugin_json}}"

# ---------------------------------------------------------------------------
# Compute grand total (AC-3)
# ---------------------------------------------------------------------------
GRAND_TOTAL=$((ALL_WASM_BYTES + DISPATCHER_BYTES))

# ---------------------------------------------------------------------------
# Cold-start p95 measurement via hyperfine (AC-7, B.1)
# Uses handoff-validator (SubagentStop) — NOT legacy-bash-adapter
# ---------------------------------------------------------------------------
COLD_START_P95_MS=0
if [ -n "$DISPATCHER_BINARY" ] && [ -f "$DISPATCHER_BINARY" ] && [ -f "$FIXTURE" ]; then
  if command -v hyperfine >/dev/null 2>&1; then
    CLAUDE_PLUGIN_ROOT="${REPO_ROOT}/plugins/vsdd-factory"
    CLAUDE_PROJECT_DIR="${REPO_ROOT}"
    export CLAUDE_PLUGIN_ROOT CLAUDE_PROJECT_DIR
    # Run cold-start measurement: --warmup 0 to capture first-invocation cost
    # Redirect hyperfine's human-readable output to stderr; capture JSON
    hyperfine_out=$(hyperfine --warmup 0 --runs 10 --export-json /dev/stderr \
      "${DISPATCHER_BINARY} < ${FIXTURE}" 2>&1 1>/dev/null) || true
    # Parse p95 from JSON output (times array is in seconds; convert to ms)
    if echo "$hyperfine_out" | grep -q '"times"'; then
      COLD_START_P95_MS=$(echo "$hyperfine_out" | python3 -c "
import json, sys
data = json.load(sys.stdin)
times_ms = sorted([t * 1000 for t in data['results'][0]['times']])
n = len(times_ms)
p95_idx = max(0, int(n * 0.95) - 1)
print(round(times_ms[p95_idx], 1))
" 2>/dev/null || echo 0)
    fi
  else
    echo "WARNING: hyperfine not found; cold-start measurement skipped" >&2
    echo "Install hyperfine: cargo install hyperfine" >&2
  fi
else
  echo "WARNING: Skipping cold-start measurement (missing dispatcher or fixture)" >&2
fi

# ---------------------------------------------------------------------------
# Provenance fields
# ---------------------------------------------------------------------------
MEASUREMENT_TIMESTAMP=$(date -u "+%Y-%m-%dT%H:%M:%SZ" 2>/dev/null || date -u)
HOST_PLATFORM=$(uname -s)-$(uname -m 2>/dev/null || echo unknown)

# ---------------------------------------------------------------------------
# Emit JSON to stdout
# ---------------------------------------------------------------------------
printf '{\n'
printf '  "methodology_version": %d,\n' "$METHODOLOGY_VERSION"
printf '  "measurement_timestamp": "%s",\n' "$MEASUREMENT_TIMESTAMP"
printf '  "host_platform": "%s",\n' "$HOST_PLATFORM"
printf '  "all_hook_plugins_wasm_bytes": %d,\n' "$ALL_WASM_BYTES"
printf '  "dispatcher_bytes": %d,\n' "$DISPATCHER_BYTES"
printf '  "grand_total_bytes": %d,\n' "$GRAND_TOTAL"
printf '  "cold_start_p95_measured_ms": %s,\n' "$COLD_START_P95_MS"
printf '  "per_plugin": %s\n' "$per_plugin_json"
printf '}\n'
