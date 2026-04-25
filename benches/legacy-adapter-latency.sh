#!/usr/bin/env bash
# legacy-adapter-latency.sh — S-2.7 latency bench for the v0.79.x → v1.0
# regression validation. Times five representative bash hooks invoked
# directly vs. via the dispatcher + legacy-bash-adapter, and reports the
# median of N runs each.
#
# This is a measurement tool, not an optimization target. If the
# dispatcher path is broken (as observed in S-2.7-beta), the "adapter"
# numbers will reflect the cost of an early-exit path (wasm
# instantiation + a host call that errors before bash runs) rather than
# real round-trip-through-bash work — read the report carefully.
#
# Usage:
#   benches/legacy-adapter-latency.sh [N]
#
# N defaults to 10. Output is plain text suitable for pasting into a PR
# description; no third-party dependencies beyond bash, python3 (for
# microsecond timestamps), and the dispatcher binary.
#
# Environment:
#   DISPATCHER     path to factory-dispatcher (default: target/release/factory-dispatcher)
#   PLUGIN_ROOT    plugins/vsdd-factory/ (default: plugins/vsdd-factory under repo root)
#
# Exit code: 0 always. The bench is informational; CI gates on the
# numbers via a separate readability check, not on the script's exit.

set -u

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
N="${1:-10}"
DISPATCHER="${DISPATCHER:-$REPO_ROOT/target/release/factory-dispatcher}"
PLUGIN_ROOT="${PLUGIN_ROOT:-$REPO_ROOT/plugins/vsdd-factory}"

if [ ! -x "$DISPATCHER" ]; then
  echo "ERROR: dispatcher not found at $DISPATCHER" >&2
  echo "Build first: cargo build --workspace --release" >&2
  exit 0
fi

if [ ! -d "$PLUGIN_ROOT" ]; then
  echo "ERROR: plugin root not found at $PLUGIN_ROOT" >&2
  exit 0
fi

now_us() {
  python3 -c 'import time; print(int(time.time()*1000000))'
}

median_us() {
  # stdin: one integer per line. stdout: median (lower-mid for even N).
  sort -n | awk -v n="$N" 'BEGIN{i=0} {a[i++]=$1} END{print a[int(n/2)]}'
}

# Hook | direct script path | event envelope
HOOKS=(
  "capture-commit-activity|hooks/capture-commit-activity.sh|{\"event_name\":\"PostToolUse\",\"tool_name\":\"Bash\",\"session_id\":\"b\",\"tool_input\":{\"command\":\"git commit -m x\"},\"tool_response\":{\"exit_code\":0,\"stdout\":\"[main abc1234] x\",\"stderr\":\"\"}}"
  "block-ai-attribution|hooks/block-ai-attribution.sh|{\"event_name\":\"PreToolUse\",\"tool_name\":\"Bash\",\"session_id\":\"b\",\"tool_input\":{\"command\":\"echo hi\"}}"
  "convergence-tracker|hooks/convergence-tracker.sh|{\"event_name\":\"PostToolUse\",\"tool_name\":\"Edit\",\"session_id\":\"b\",\"tool_input\":{\"file_path\":\"/tmp/x.md\"},\"tool_response\":{\"success\":true}}"
  "validate-changelog-monotonicity|hooks/validate-changelog-monotonicity.sh|{\"event_name\":\"PostToolUse\",\"tool_name\":\"Edit\",\"session_id\":\"b\",\"tool_input\":{\"file_path\":\"/tmp/CHANGELOG.md\"},\"tool_response\":{\"success\":true}}"
  "destructive-command-guard|hooks/destructive-command-guard.sh|{\"event_name\":\"PreToolUse\",\"tool_name\":\"Bash\",\"session_id\":\"b\",\"tool_input\":{\"command\":\"echo hi\"}}"
)

WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT
mkdir -p "$WORK/proj/.factory/logs"

printf '== legacy-adapter-latency.sh — N=%s, dispatcher=%s ==\n\n' "$N" "$DISPATCHER"
printf '%-36s %12s %12s %10s\n' "hook" "direct_us" "adapter_us" "ratio"
printf '%-36s %12s %12s %10s\n' "----" "---------" "----------" "-----"

cold_runs=()
for i in $(seq 1 "$N"); do
  t1="$(now_us)"
  CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT" \
  CLAUDE_PROJECT_DIR="$WORK/proj" \
    "$DISPATCHER" >/dev/null 2>&1 \
    < <(printf '%s' '{"event_name":"NoSuch","tool_name":"X","session_id":"b"}')
  t2="$(now_us)"
  cold_runs+=( "$((t2 - t1))" )
done
cold_median=$(printf '%s\n' "${cold_runs[@]}" | median_us)
printf 'dispatcher cold-start (no plugins matched): %s us median (n=%s)\n\n' "$cold_median" "$N"

for spec in "${HOOKS[@]}"; do
  IFS='|' read -r name script env <<<"$spec"
  printf '%s\n' "$env" > "$WORK/env.json"

  # Direct
  direct_runs=()
  for i in $(seq 1 "$N"); do
    t1="$(now_us)"
    CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT" \
    CLAUDE_PROJECT_DIR="$WORK/proj" \
    VSDD_LOG_DIR="$WORK/proj/.factory/logs" \
      bash "$PLUGIN_ROOT/$script" < "$WORK/env.json" >/dev/null 2>&1
    t2="$(now_us)"
    direct_runs+=( "$((t2 - t1))" )
  done
  direct_median=$(printf '%s\n' "${direct_runs[@]}" | median_us)

  # Adapter (whole dispatcher invocation — note: also runs other plugins
  # that happen to match the same event/tool, so this is an upper bound
  # on the per-plugin adapter cost).
  adapter_runs=()
  for i in $(seq 1 "$N"); do
    t1="$(now_us)"
    CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT" \
    CLAUDE_PROJECT_DIR="$WORK/proj" \
      "$DISPATCHER" < "$WORK/env.json" >/dev/null 2>&1
    t2="$(now_us)"
    adapter_runs+=( "$((t2 - t1))" )
  done
  adapter_median=$(printf '%s\n' "${adapter_runs[@]}" | median_us)

  if [ "$direct_median" -gt 0 ]; then
    # ratio in percent — adapter / direct
    ratio=$(( adapter_median * 100 / direct_median ))
    printf '%-36s %12s %12s %9s%%\n' "$name" "$direct_median" "$adapter_median" "$ratio"
  else
    printf '%-36s %12s %12s %10s\n' "$name" "$direct_median" "$adapter_median" "n/a"
  fi
done

echo
echo "Notes:"
echo "  - 'adapter' is the full dispatcher run, which fans out to every"
echo "    matching plugin. It is an upper bound on the per-plugin cost."
echo "  - The 30% adapter-overhead budget in S-2.7 is a per-plugin"
echo "    target. Divide adapter_us by the number of matched plugins"
echo "    (visible in the dispatcher's stderr line) for an apples-to-"
echo "    apples comparison."
