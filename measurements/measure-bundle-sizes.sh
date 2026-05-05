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
  # LC_ALL=C: consistent number formatting across locales.
  # tr -d removes any whitespace wc -c may emit (BSD wc pads with leading spaces).
  # Result must match ^[0-9]+$ — a plain decimal integer.
  DISPATCHER_BYTES=$(LC_ALL=C wc -c < "$DISPATCHER_BINARY" | tr -d ' \t\n')
else
  echo "WARNING: factory-dispatcher binary not found at $REPO_ROOT/target/release/factory-dispatcher" >&2
  echo "Run: cargo build --release -p factory-dispatcher" >&2
fi

# ---------------------------------------------------------------------------
# Sum ALL .wasm files in the bundle directory for unaccounted detection.
# all_hook_plugins_wasm_bytes = frozen-17 sum (= sum(per_plugin)).
# unaccounted_wasm_bytes = bytes from non-frozen .wasm files
#   (e.g. hello-hook.wasm, underscore-named stubs not in frozen enumeration).
# grand_total_bytes = all_hook_plugins_wasm_bytes + unaccounted + dispatcher.
# ---------------------------------------------------------------------------
TOTAL_WASM_BYTES=0
for f in "$BUNDLE_DIR"/*.wasm; do
  [ -f "$f" ] || continue
  # LC_ALL=C: consistent number formatting across locales.
  # tr -d removes any whitespace wc -c may emit. Result is ^[0-9]+$.
  sz=$(LC_ALL=C wc -c < "$f" | tr -d ' \t\n')
  TOTAL_WASM_BYTES=$((TOTAL_WASM_BYTES + sz))
done

# ---------------------------------------------------------------------------
# Build per_plugin map for the 17 frozen plugins (AC-2, AC-5)
# Only counts present files; absent plugins record 0.
# Also accumulates ALL_WASM_BYTES = sum of frozen-17 (= sum(per_plugin)).
# ---------------------------------------------------------------------------
per_plugin_json="{"
first=1
ALL_WASM_BYTES=0
for plugin in "${FROZEN_PLUGINS[@]}"; do
  wasm_file="$BUNDLE_DIR/${plugin}.wasm"
  if [ -f "$wasm_file" ]; then
    # LC_ALL=C: consistent number formatting across locales.
    # tr -d removes any whitespace wc -c may emit. Result is ^[0-9]+$.
    sz=$(LC_ALL=C wc -c < "$wasm_file" | tr -d ' \t\n')
    ALL_WASM_BYTES=$((ALL_WASM_BYTES + sz))
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

# Bytes from non-frozen .wasm files (hello-hook.wasm, underscore stubs, etc.)
# If non-zero, these warrant review — they are not part of the frozen enumeration.
UNACCOUNTED_WASM_BYTES=$((TOTAL_WASM_BYTES - ALL_WASM_BYTES))

# ---------------------------------------------------------------------------
# Compute grand total (AC-3)
# grand_total = all_hook_plugins_wasm_bytes + unaccounted_wasm_bytes + dispatcher
# ---------------------------------------------------------------------------
GRAND_TOTAL=$((ALL_WASM_BYTES + UNACCOUNTED_WASM_BYTES + DISPATCHER_BYTES))

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
    # Run cold-start measurement: --warmup 0 to capture first-invocation cost.
    # N=30 for p95 reliability (N=10 is too noisy; p95 index only 9 samples).
    # LC_ALL=C ensures wc -c emits plain digits; tr removes any stray whitespace.
    # Use a temp file so hyperfine JSON is captured correctly; writing to
    # /dev/stderr and redirecting 2>&1 was previously inverted, producing an
    # empty hyperfine_out and silently falling back to cold_start_p95_measured_ms=0.
    tmp_json=$(mktemp "${TMPDIR:-/tmp}/hyperfine.XXXXXX")
    mv "$tmp_json" "$tmp_json.json"
    tmp_json="$tmp_json.json"
    trap "rm -f $tmp_json" EXIT INT TERM
    # Redirect both stdout and stderr to /dev/null: hyperfine prints its
    # human-readable benchmark summary to stdout; the JSON result goes to
    # the temp file only, so suppressing stdout is safe.
    hyperfine --warmup 0 --runs 30 --export-json "$tmp_json" \
      "${DISPATCHER_BINARY} < ${FIXTURE}" >/dev/null 2>&1 || true
    hyperfine_out=$(cat "$tmp_json")
    # Parse p95 from JSON output (times array is in seconds; convert to ms).
    # Asserts hyperfine_out contains valid JSON with a "times" array.
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
printf '  "unaccounted_wasm_bytes": %d,\n' "$UNACCOUNTED_WASM_BYTES"
printf '  "dispatcher_bytes": %d,\n' "$DISPATCHER_BYTES"
printf '  "grand_total_bytes": %d,\n' "$GRAND_TOTAL"
printf '  "cold_start_p95_measured_ms": %s,\n' "$COLD_START_P95_MS"
printf '  "per_plugin": %s\n' "$per_plugin_json"
printf '}\n'
