#!/usr/bin/env bash
# postcompact-reanchor.sh — PostCompact re-anchor advisory hook (S-18.05)
#
# BC-7.07.002 / VP-089 / SS-07 / DI-024
#
# Fires on PostCompact. Reads current_cycle and current_step from
# factory-artifacts:.factory/STATE.md via `git show`. Sources the develop
# SHA from `git rev-parse origin/develop` executed at hook invocation time
# (read-only; advisory; no STATE.md schema change required — ADR-026 §Decision 7
# / F-P1-001). Emits a structured re-anchor block to stdout so the LLM
# session can re-ground itself after compaction.
#
# KEY invariants:
#   Inv1: NEVER commits, pushes, or adds to factory-artifacts (read-only).
#   Inv2: current_cycle/current_step sourced ONLY from git show (factory-artifacts).
#         develop SHA sourced from git rev-parse origin/develop at hook invocation.
#         Never from env vars or in-context reasoning.
#   PC3:  Cannot block compaction (PostCompact is advisory-only).
#   PC5:  Exits 0 on ALL code paths (fail-open; set -e errors are trapped).
#   PC6:  on_error=continue in hooks-registry.toml (harness-level fail-open).
#   No current_wave field — STATE.md has no such field; MUST NOT emit one.
#
# JSONL log schema (exactly 6 fields — BC-7.07.002 PC2):
#   event, current_cycle, current_step, develop_sha, timestamp, status
#   NO wave_id field. Field renamed last_verified_develop_sha → develop_sha (F-P1-001).

set -euo pipefail

# ---------------------------------------------------------------------------
# Trap ALL errors (set -e exits) and ensure we always exit 0 (PC5/DI-024).
# Inline string form avoids SC2329 (named function not directly invoked).
# ---------------------------------------------------------------------------
trap 'exit 0' ERR

# ---------------------------------------------------------------------------
# Consume stdin (JSON PostCompact envelope) — we don't need its content but
# some shells / pipe partners require it to be consumed to avoid SIGPIPE.
# ---------------------------------------------------------------------------
if [ ! -t 0 ]; then
  read -r -d '' _envelope 2>/dev/null || true
fi

# ---------------------------------------------------------------------------
# Helper: get current timestamp in ISO-8601 format (UTC where possible).
# Falls back to date without %Z on systems that don't support it.
# ---------------------------------------------------------------------------
_timestamp() {
  date -u +"%Y-%m-%dT%H:%M:%SZ" 2>/dev/null \
    || date +"%Y-%m-%dT%H:%M:%S" 2>/dev/null \
    || echo "unknown"
}

# ---------------------------------------------------------------------------
# Helper: get today's date for the daily log filename (YYYY-MM-DD).
# ---------------------------------------------------------------------------
_today() {
  date -u +"%Y-%m-%d" 2>/dev/null \
    || date +"%Y-%m-%d" 2>/dev/null \
    || echo "0000-00-00"
}

# ---------------------------------------------------------------------------
# Helper: append a JSONL entry to .factory/logs/postcompact-reanchor-DATE.jsonl
#
# Args:
#   $1 — current_cycle  (string; "UNKNOWN" if absent)
#   $2 — current_step   (string; "UNKNOWN" if absent)
#   $3 — develop_sha    (string; "UNKNOWN" if absent) — F-P1-001 renamed field
#   $4 — status         ("ok" or "warn")
# ---------------------------------------------------------------------------
_append_log() {
  local cycle="$1"
  local step="$2"
  local sha="$3"
  local status_val="$4"
  local ts
  ts=$(_timestamp)
  local today
  today=$(_today)

  local log_dir=".factory/logs"
  local log_file="${log_dir}/postcompact-reanchor-${today}.jsonl"

  # Attempt to create log directory if absent (EC-005).
  if [ ! -d "$log_dir" ]; then
    mkdir -p "$log_dir" 2>/dev/null || {
      # Log dir creation failed — stdout-only advisory, exit 0 (EC-005).
      return 0
    }
  fi

  # Build the JSONL line with exactly 6 fields (no wave_id — BC-7.07.002 PC2).
  # Field name: develop_sha (renamed from last_verified_develop_sha per F-P1-001).
  # Use printf for portability (no jq dependency required by spec).
  local json_line
  json_line=$(printf '{"event":"PostCompact","current_cycle":"%s","current_step":"%s","develop_sha":"%s","timestamp":"%s","status":"%s"}' \
    "$cycle" "$step" "$sha" "$ts" "$status_val")

  # Append to log file (fail-open: ignore write errors).
  printf '%s\n' "$json_line" >> "$log_file" 2>/dev/null || true
}

# ---------------------------------------------------------------------------
# Main: read STATE.md from factory-artifacts via git show (Inv2).
# ---------------------------------------------------------------------------

# Attempt to read STATE.md from factory-artifacts branch.
# Any git error → warn + exit 0 (EC-002 / PC5).
state_md=""
if ! state_md=$(git show factory-artifacts:.factory/STATE.md 2>/dev/null); then
  # factory-artifacts unreachable (EC-002)
  ts=$(_timestamp)
  echo "[PostCompact Re-anchor] WARN: factory-artifacts unreachable; re-anchor skipped"
  echo "Source: factory-artifacts STATE.md (verified at ${ts})"
  _append_log "UNKNOWN" "UNKNOWN" "UNKNOWN" "warn"
  exit 0
fi

# Parse current_cycle and current_step from the YAML frontmatter using grep
# (no jq/yq needed for simple key: value).
# Pattern: "^fieldname: value" — strips surrounding whitespace.
current_cycle=""
current_step=""

current_cycle=$(printf '%s\n' "$state_md" \
  | grep -E '^current_cycle:' \
  | sed 's/^current_cycle:[[:space:]]*//' \
  | tr -d '\r' \
  | head -1 \
  || true)

current_step=$(printf '%s\n' "$state_md" \
  | grep -E '^current_step:' \
  | sed 's/^current_step:[[:space:]]*//' \
  | tr -d '\r' \
  | head -1 \
  || true)

# Source develop SHA from git rev-parse at hook invocation time (F-P1-001 /
# ADR-026 §Decision 7). This is the live authoritative develop HEAD;
# requires no STATE.md schema change. Falls back to "UNKNOWN" on any error.
# Use refs/remotes/origin/develop (canonical full refspec) to avoid git printing
# the partial ref name to stdout when the ref doesn't resolve.
#
# BC-7.07.002 PC1 / Invariant 2: when git rev-parse fails, develop_sha=UNKNOWN
# and sha_status=warn. status=ok ONLY when rev-parse succeeds with a real SHA.
develop_sha=""
sha_status="ok"
if develop_sha=$(git rev-parse refs/remotes/origin/develop 2>/dev/null) && [ -n "$develop_sha" ]; then
  : # develop_sha set above; sha_status remains "ok"
else
  develop_sha="UNKNOWN"
  sha_status="warn"
fi

# Determine context label and status (EC-003 / AC-006).
ts=$(_timestamp)

if [ -z "$current_cycle" ] && [ -z "$current_step" ]; then
  # Both absent — context=UNKNOWN, status=warn (EC-003)
  echo "[PostCompact Re-anchor] context=UNKNOWN sha=${develop_sha}"
  echo "Source: factory-artifacts STATE.md (verified at ${ts})"
  _append_log "UNKNOWN" "UNKNOWN" "${develop_sha}" "warn"
  exit 0
fi

if [ -z "$current_cycle" ] || [ -z "$current_step" ]; then
  # One absent — context=UNKNOWN for the missing part, status=warn (EC-003)
  echo "[PostCompact Re-anchor] context=UNKNOWN sha=${develop_sha}"
  echo "Source: factory-artifacts STATE.md (verified at ${ts})"
  _append_log "${current_cycle:-UNKNOWN}" "${current_step:-UNKNOWN}" "${develop_sha}" "warn"
  exit 0
fi

# Happy path (EC-001): cycle and step present.
# Status is "ok" only when develop_sha is a real SHA (rev-parse succeeded).
# Status is "warn" when develop_sha=UNKNOWN (rev-parse failed) even though
# context fields are valid — BC-7.07.002 PC1 / Invariant 2 / VP-089 §1.
echo "[PostCompact Re-anchor] context=${current_cycle}/${current_step} sha=${develop_sha}"
echo "Source: factory-artifacts STATE.md (verified at ${ts})"
_append_log "$current_cycle" "$current_step" "${develop_sha}" "${sha_status}"

exit 0
