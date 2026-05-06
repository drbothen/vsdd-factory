#!/usr/bin/env bash
# block.sh — canonical block-message helper for all PreToolUse blocking hooks.
#
# Source from each hook with:
#   source "${CLAUDE_PLUGIN_ROOT}/hooks/lib/block.sh"
#
# Provides two functions:
#   block_pre <hook> <reason> <recommendation> <code> [k=v ...]
#       Single-line stderr block in canonical Why/Fix/Code form, then exit 2.
#       Designed so the message survives legacy-bash-adapter truncation
#       (which takes only the first stderr line).
#
#   block_pre_json <hook> <reason> <recommendation> <code> [k=v ...]
#       Same canonical message but emitted via PreToolUse JSON envelope as
#       permissionDecision: deny + permissionDecisionReason. Used by
#       hooks that prefer the JSON path (protect-bc, protect-vp).
#
# Telemetry: both helpers call the project's emit-event tool if available;
# silent no-op otherwise.

# Internal: emit telemetry without ever causing the hook to fail.
_emit() {
  if [ -n "${CLAUDE_PLUGIN_ROOT:-}" ] && [ -x "${CLAUDE_PLUGIN_ROOT}/bin/emit-event" ]; then
    "${CLAUDE_PLUGIN_ROOT}/bin/emit-event" "$@" 2>/dev/null || true
  fi
  return 0
}

# Internal: trim ALL trailing periods so we can re-add exactly one consistently.
# Uses a loop to match Rust's trim_end_matches('.') greedy behavior — both strip
# every trailing period, not just the last one. Prevents double-period sequences
# when callers pass strings already ending in '.'.
_strip_trailing_period() {
  local s="$1"
  while [[ "$s" == *. ]]; do s="${s%.}"; done
  printf '%s' "$s"
}

# Format the canonical block line.
# Args: <hook> <reason> <recommendation> <code>
_format_block_line() {
  local hook="$1" reason="$2" rec="$3" code="$4"
  reason="$(_strip_trailing_period "$reason")"
  rec="$(_strip_trailing_period "$rec")"
  printf 'BLOCKED by %s: %s. Fix: %s. Code: %s.' "$hook" "$reason" "$rec" "$code"
}

# Public: bash exit-2 block.
# Usage: block_pre <hook> <reason> <recommendation> <code> [extra_emit_kv...]
block_pre() {
  local hook="${1:?block_pre: hook name required}"; shift
  local reason="${1:?block_pre: reason required}"; shift
  local rec="${1:?block_pre: recommendation required}"; shift
  local code="${1:?block_pre: reason code required}"; shift
  _emit type=hook.block hook="$hook" reason="$code" "$@"
  _format_block_line "$hook" "$reason" "$rec" "$code" >&2
  printf '\n' >&2
  exit 2
}

# Public: PreToolUse JSON envelope deny.
# Usage: block_pre_json <hook> <reason> <recommendation> <code> [extra_emit_kv...]
# Requires jq. If jq is unavailable, falls back to block_pre (stderr + exit 2)
# so the call is never silently allowed.
block_pre_json() {
  local hook="${1:?block_pre_json: hook name required}"; shift
  local reason="${1:?block_pre_json: reason required}"; shift
  local rec="${1:?block_pre_json: recommendation required}"; shift
  local code="${1:?block_pre_json: reason code required}"; shift
  _emit type=hook.block hook="$hook" reason="$code" "$@"
  # Fail-safe: if jq is absent fall back to the stderr+exit-2 path rather than
  # silently allowing the tool call (exit 0 with no JSON envelope).
  if ! command -v jq >/dev/null 2>&1; then
    block_pre "$hook" "$reason" "$rec" "$code"
    # block_pre exits 2; unreachable
  fi
  local msg
  msg="$(_format_block_line "$hook" "$reason" "$rec" "$code")"
  jq -nc --arg msg "$msg" '{
    hookSpecificOutput: {
      hookEventName: "PreToolUse",
      permissionDecision: "deny",
      permissionDecisionReason: $msg
    }
  }'
  exit 0
}
