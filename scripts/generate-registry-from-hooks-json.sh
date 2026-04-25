#!/usr/bin/env bash
# generate-registry-from-hooks-json.sh — convert v0.79.x bash-hook inventory
# into the v1.0 hooks-registry.toml that factory-dispatcher (S-1.2 / S-2.1)
# loads at startup.
#
# Why this exists: v0.79.x routed each bash hook directly via Claude Code's
# hooks.json. v1.0 routes everything through a single dispatcher binary and
# the dispatcher consults `hooks-registry.toml` for the per-hook plugin /
# capability declarations. Every existing bash hook becomes one
# `[[hooks]]` entry that points at `legacy-bash-adapter.wasm`; the adapter
# (S-2.1) reads `[hooks.config] script_path` and execs the underlying
# `*.sh`. This is the migration tool that produces that registry from the
# pre-templating hooks.json (committed at 7b4b774^ in git history; the
# current hooks.json is a per-event dispatcher pointer and no longer
# carries per-hook detail).
#
# Usage:
#   scripts/generate-registry-from-hooks-json.sh
#       Read git show 7b4b774^:plugins/vsdd-factory/hooks/hooks.json
#       Write plugins/vsdd-factory/hooks-registry.toml
#
#   scripts/generate-registry-from-hooks-json.sh path/to/hooks.json
#       Read the supplied hooks.json instead. Useful for testing the
#       generator on a synthetic input.
#
# Idempotence: running twice in a row produces byte-identical output.
# Entries are sorted within each event by name; LC_ALL=C is enforced
# throughout so locale never reorders output.
#
# Failure modes (loud, never silent):
#   * hooks.json entry references a script that no longer exists on disk
#   * script on disk has no entry in hooks.json
#   * jq missing
#   * git missing (when reading the historical default)

set -euo pipefail
export LC_ALL=C

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
HOOKS_DIR="$REPO_ROOT/plugins/vsdd-factory/hooks"
OUT_FILE="$REPO_ROOT/plugins/vsdd-factory/hooks-registry.toml"
HISTORICAL_REF="7b4b774^:plugins/vsdd-factory/hooks/hooks.json"

# Allow-list of hook basenames that MUST block on plugin error rather
# than continuing. These are the v0.79.x "gate" hooks: their job is to
# stop a tool call dead, so a crash or timeout has to be treated as a
# failed gate. Everything outside the list defaults to on_error =
# "continue" (telemetry hooks, validators that warn but don't block).
GATE_HOOKS=(
  block-ai-attribution
  destructive-command-guard
  protect-secrets
  red-gate
  factory-branch-guard
  protect-vp
  protect-bc
  brownfield-discipline
  validate-wave-gate-prerequisite
  validate-pr-merge-prerequisites
  verify-git-push
  check-factory-commit
  validate-template-compliance
  validate-input-hash
  validate-factory-path-root
  pr-manager-completion-guard
  handoff-validator
)

# --------- helpers -----------------------------------------------------

require_cmd() {
  command -v "$1" >/dev/null 2>&1 || {
    echo "error: required command not found: $1" >&2
    exit 1
  }
}

is_gate() {
  local name="$1"
  local g
  for g in "${GATE_HOOKS[@]}"; do
    [[ "$name" == "$g" ]] && return 0
  done
  return 1
}

# Compute the band (lower bound) for an entry given its event and name.
band_for() {
  local event="$1"
  local name="$2"
  case "$event" in
    PreToolUse) echo 10 ;;
    PostToolUse)
      case "$name" in
        capture-*) echo 100 ;;
        *)        echo 200 ;;
      esac
      ;;
    SubagentStop|Stop) echo 900 ;;
    *)
      echo "error: unknown event for priority assignment: $event ($name)" >&2
      exit 1
      ;;
  esac
}

# Emit binary_allow union for one script: always bash, plus git/gh/jq/curl
# when the script invokes them. Conservative on purpose — every binary
# is a capability-surface expansion the operator has to audit.
binary_allow_for_script() {
  local script="$1"
  local out=("bash")
  # Use word-boundary grep so "github" doesn't trigger "gh" and "jquery"
  # doesn't trigger "jq". -F is too loose; -E with \b is the right call.
  if grep -qE '\bgit\b' "$script"; then
    out+=("git")
  fi
  if grep -qE '\bgh\b' "$script"; then
    out+=("gh")
  fi
  if grep -qE '\bjq\b' "$script"; then
    out+=("jq")
  fi
  if grep -qE '\bcurl\b' "$script"; then
    out+=("curl")
  fi
  printf '%s\n' "${out[@]}"
}

# --------- main --------------------------------------------------------

require_cmd jq

# Resolve input hooks.json — default to the historical commit; allow an
# explicit path for tests / local experimentation.
HOOKS_JSON_INPUT=""
if [ $# -ge 1 ]; then
  if [ ! -f "$1" ]; then
    echo "error: hooks.json path not found: $1" >&2
    exit 1
  fi
  HOOKS_JSON_INPUT="$(cat "$1")"
else
  require_cmd git
  HOOKS_JSON_INPUT="$(git -C "$REPO_ROOT" show "$HISTORICAL_REF" 2>/dev/null || true)"
  if [ -z "$HOOKS_JSON_INPUT" ]; then
    echo "error: failed to read $HISTORICAL_REF — is the repo shallow-cloned?" >&2
    exit 1
  fi
fi

# Cross-check: every script referenced must exist on disk; every script
# on disk must be referenced. Either side mismatching = data drift,
# which is the operator's problem to resolve before regenerating.
REFERENCED_SCRIPTS="$(printf '%s\n' "$HOOKS_JSON_INPUT" \
  | jq -r '.. | objects | .command? // empty' \
  | sed 's|.*/||; s|\.sh$||' \
  | sort -u)"
ON_DISK_SCRIPTS="$(find "$HOOKS_DIR" -maxdepth 1 -name '*.sh' -type f \
  | xargs -n1 basename \
  | sed 's|\.sh$||' \
  | sort -u)"

MISSING_ON_DISK="$(comm -23 <(printf '%s\n' "$REFERENCED_SCRIPTS") <(printf '%s\n' "$ON_DISK_SCRIPTS") || true)"
ORPHAN_SCRIPTS="$(comm -13 <(printf '%s\n' "$REFERENCED_SCRIPTS") <(printf '%s\n' "$ON_DISK_SCRIPTS") || true)"

if [ -n "$MISSING_ON_DISK" ]; then
  echo "error: hooks.json references scripts that no longer exist on disk:" >&2
  printf '  %s\n' $MISSING_ON_DISK >&2
  exit 1
fi
if [ -n "$ORPHAN_SCRIPTS" ]; then
  echo "error: scripts on disk have no entry in hooks.json:" >&2
  printf '  %s\n' $ORPHAN_SCRIPTS >&2
  echo "       (add them to hooks.json or remove the file)" >&2
  exit 1
fi

# Flatten hooks.json into TSV: event \t matcher \t name \t timeout_s.
# Preserve all rows (including duplicate names with different matchers,
# e.g. protect-secrets fires on both Bash and Read).
#
# Empty matcher is encoded as the literal sentinel `__NONE__` rather than
# left empty: bash `read -r` collapses adjacent whitespace delimiters
# (tab is whitespace), so a row like `Event\t\tname\t5` parses as if it
# had three columns instead of four. The sentinel gives `read` a
# non-empty token in column two; the emit pass strips it back out.
ROWS="$(printf '%s\n' "$HOOKS_JSON_INPUT" | jq -r '
  .hooks
  | to_entries[]
  | .key as $event
  | .value[]
  | (.matcher // "" | if . == "" then "__NONE__" else . end) as $matcher
  | .hooks[]
  | [$event, $matcher, (.command | sub(".*/"; "") | sub("\\.sh$"; "")), (.timeout|tostring)]
  | @tsv
')"

# Pre-compute, for each event, the name-sorted list of unique names so
# priorities are deterministic regardless of hooks.json key order.
# Within an event the band determines the starting offset; entries
# inside the same band step by 10. Same-name-different-matcher entries
# share a priority (they're the same plugin, just different routing).

# Map: "<event>\t<name>" -> priority
declare -A PRIORITY

assign_priorities_for_event() {
  local event="$1"
  # Names in this event, sorted, deduped:
  local names_sorted
  names_sorted="$(printf '%s\n' "$ROWS" | awk -F'\t' -v e="$event" '$1==e {print $3}' | sort -u)"
  # Group by band, then within-band sort by name and step by 10.
  declare -A counter_by_band=()
  local n band p
  while IFS= read -r n; do
    [ -z "$n" ] && continue
    band="$(band_for "$event" "$n")"
    # Re-walk per band so per-band counters are local.
    : "${counter_by_band[$band]:=0}"
  done <<< "$names_sorted"

  # Two passes: first collect names per band sorted, then assign p = band + (idx+1)*10.
  declare -A names_by_band=()
  while IFS= read -r n; do
    [ -z "$n" ] && continue
    band="$(band_for "$event" "$n")"
    names_by_band[$band]+="$n"$'\n'
  done <<< "$names_sorted"

  for band in "${!names_by_band[@]}"; do
    local idx=0
    # Sort the band's names again for safety (they were already sorted but
    # explicit > implicit when idempotence is the goal).
    local band_names
    band_names="$(printf '%s' "${names_by_band[$band]}" | sed '/^$/d' | sort)"
    while IFS= read -r n; do
      [ -z "$n" ] && continue
      idx=$((idx + 1))
      p=$((band + idx * 10))
      PRIORITY["$event"$'\t'"$n"]="$p"
    done <<< "$band_names"
  done
}

# Unique events present in this hooks.json, in canonical order (so the
# output groups events the same way every run).
EVENTS="$(printf '%s\n' "$ROWS" | awk -F'\t' '{print $1}' | sort -u)"
while IFS= read -r e; do
  [ -z "$e" ] && continue
  assign_priorities_for_event "$e"
done <<< "$EVENTS"

# --------- emit registry ----------------------------------------------

{
  cat <<'HEADER'
# hooks-registry.toml — auto-generated by scripts/generate-registry-from-hooks-json.sh
#
# DO NOT HAND-EDIT during the v0.79.x → v1.0 migration: the generator
# is idempotent and re-running it on a hand-edited file will revert
# the changes silently. Once the migration is complete (post-1.0.0),
# this file becomes the human-edited source of truth and the generator
# is retired.
#
# Every entry routes through hook-plugins/legacy-bash-adapter.wasm,
# which reads `[hooks.config] script_path` and execs the underlying
# bash script. Native-WASM ports (S-2.5 onward) replace individual
# entries one-by-one; legacy entries can coexist with native ones in
# this same file.

schema_version = 1

HEADER

  # Iterate events in canonical (alphabetical) order. Within each event,
  # iterate (matcher, name) pairs sorted by (priority, matcher, name)
  # so the output is fully deterministic.
  while IFS= read -r event; do
    [ -z "$event" ] && continue
    printf '# ---------- %s ----------\n\n' "$event"

    # Build the entry list for this event: priority \t matcher \t name \t timeout_s
    EVENT_ROWS="$(printf '%s\n' "$ROWS" | awk -F'\t' -v e="$event" '$1==e {
      print $0
    }')"

    # Each row needs its priority looked up; produce sortable output:
    # priority<TAB>matcher<TAB>name<TAB>timeout_s
    ENRICHED=""
    while IFS=$'\t' read -r ev matcher name timeout_s; do
      [ -z "$ev" ] && continue
      key="$ev"$'\t'"$name"
      entry_p="${PRIORITY[$key]:-}"
      if [ -z "$entry_p" ]; then
        echo "error: no priority assigned for event=$ev name=$name" >&2
        exit 1
      fi
      ENRICHED+="$(printf '%s\t%s\t%s\t%s\n' "$entry_p" "$matcher" "$name" "$timeout_s")"$'\n'
    done <<< "$EVENT_ROWS"

    # Sort by priority, then matcher, then name. Numeric on column 1.
    SORTED="$(printf '%s' "$ENRICHED" | sed '/^$/d' | sort -t $'\t' -k1,1n -k2,2 -k3,3)"

    while IFS=$'\t' read -r priority matcher name timeout_s; do
      [ -z "$priority" ] && continue
      script_path="$HOOKS_DIR/$name.sh"

      # Tool field rules: omit when matcher is empty, "*", or missing.
      # `__NONE__` is the sentinel encoded by the jq flattening pass.
      tool_line=""
      if [ -n "$matcher" ] && [ "$matcher" != "*" ] && [ "$matcher" != "__NONE__" ]; then
        tool_line="tool = \"$matcher\""
      fi

      # on_error: "block" iff in gate-hooks list, else "continue".
      if is_gate "$name"; then
        on_error_line='on_error = "block"'
      else
        on_error_line='on_error = "continue"'
      fi

      timeout_ms=$((timeout_s * 1000))

      # Capabilities: union driven by grep over the script.
      mapfile -t binaries < <(binary_allow_for_script "$script_path")
      # Format as TOML array string: ["bash", "git", ...]
      bin_array=""
      for i in "${!binaries[@]}"; do
        if [ "$i" -gt 0 ]; then
          bin_array+=", "
        fi
        bin_array+="\"${binaries[$i]}\""
      done

      printf '[[hooks]]\n'
      printf 'name = "%s"\n' "$name"
      printf 'event = "%s"\n' "$event"
      if [ -n "$tool_line" ]; then
        printf '%s\n' "$tool_line"
      fi
      printf 'plugin = "hook-plugins/legacy-bash-adapter.wasm"\n'
      printf 'priority = %s\n' "$priority"
      printf 'timeout_ms = %s\n' "$timeout_ms"
      printf '%s\n' "$on_error_line"
      printf '\n'
      printf '[hooks.config]\n'
      printf 'script_path = "hooks/%s.sh"\n' "$name"
      printf '\n'
      printf '[hooks.capabilities]\n'
      printf 'env_allow = ["PATH", "HOME", "TMPDIR", "CLAUDE_PROJECT_DIR", "VSDD_SESSION_ID"]\n'
      printf '\n'
      printf '[hooks.capabilities.exec_subprocess]\n'
      printf 'binary_allow = [%s]\n' "$bin_array"
      printf 'shell_bypass_acknowledged = "legacy-bash-adapter runs unported hooks"\n'
      printf 'env_allow = ["PATH", "HOME", "TMPDIR", "CLAUDE_PROJECT_DIR", "VSDD_SESSION_ID"]\n'
      printf '\n'
    done <<< "$SORTED"
  done <<< "$EVENTS"
} > "$OUT_FILE"

echo "wrote $OUT_FILE"
