#!/usr/bin/env bash
# check-autocompact-setting.sh — S-18.10 / BC-6.25.001
#
# Reads .claude/settings.json (project-local, preferred) then
# ~/.claude/settings.json (global fallback) and verifies that
# env.CLAUDE_AUTOCOMPACT_PCT_OVERRIDE is present and numeric and ≤ 80.
#
# Emits one check table row to stdout:
#   PASS     — key present and numeric value ≤ 80
#   ADVISORY — key absent, value > 80, non-numeric (treated as absent),
#              missing settings.json, or malformed JSON
#
# Advisory-only: exits 0 in all cases; never blocks (BC-6.25.001 INV1).
# No side effects: reads settings.json only (BC-6.25.001 INV4).
# Row always emitted (BC-6.25.001 INV5).
#
# Uses pure-bash/grep/sed JSON extraction — no python3/jq dependency.
# This eliminates the undeclared-runtime-dependency silent-failure class
# (F-P1-001) and removes the need for any 2>/dev/null suppression (F-P1-002).
#
# Usage:
#   env PROJECT_ROOT=<repo-root> bash check-autocompact-setting.sh
#
# Environment variables:
#   PROJECT_ROOT — directory containing .factory/ and .claude/
#                  (defaults to: 4 levels up from this script's lib/ dir)
#   HOME         — used to resolve ~/.claude/settings.json global fallback
#
# ADR-026 §Decision 5: canonical value 70; ceiling 80.
# ADR-026 §F-11: check-state-health MUST verify this key.

set -euo pipefail

# ---------------------------------------------------------------------------
# Resolve project root (INV2: project-local always takes precedence)
# ---------------------------------------------------------------------------

# If PROJECT_ROOT is not set by the caller, derive from script location.
# lib/ is one level below skill root; skill root is under plugins/vsdd-factory/skills/
# so 4 levels up from lib/ reaches the repo root.
if [ -z "${PROJECT_ROOT:-}" ]; then
  SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
  PROJECT_ROOT="$(cd "$SCRIPT_DIR/../../../.." && pwd)"
fi

LOCAL_SETTINGS="$PROJECT_ROOT/.claude/settings.json"
GLOBAL_SETTINGS="${HOME}/.claude/settings.json"

CHECK_NAME="CLAUDE_AUTOCOMPACT_PCT_OVERRIDE"
REMEDIATION_HINT='Missing — add env: {CLAUDE_AUTOCOMPACT_PCT_OVERRIDE: "70"} to .claude/settings.json (ADR-026 §Decision 5: proactive compaction threshold; 70% gives PreCompact flush headroom)'

# ---------------------------------------------------------------------------
# Helper: emit a PASS row (BC-6.25.001 PC3 / INV5)
# ---------------------------------------------------------------------------
emit_pass() {
  local val="$1"
  echo "| $CHECK_NAME | PASS | Present, value $val ≤ 80 (70 is canonical per ADR-026 §Decision 5) |"
}

# ---------------------------------------------------------------------------
# Helper: emit an ADVISORY row (BC-6.25.001 PC1/PC2 / INV5)
# ---------------------------------------------------------------------------
emit_advisory() {
  local details="$1"
  echo "| $CHECK_NAME | ADVISORY | $details |"
}

# ---------------------------------------------------------------------------
# Helper: pure-bash extraction of env.CLAUDE_AUTOCOMPACT_PCT_OVERRIDE
#
# Parses a simple flat settings.json that contains an "env" object with
# string-valued keys.  Value space: quoted string or bare number.
# Returns one of:
#   OK:<numeric-string>     — key found, value is a decimal integer
#   NON_NUMERIC:<raw>       — key found, value is not a decimal integer
#   ABSENT:                 — key or env block missing
#   PARSE_ERROR:<message>   — file unreadable or grossly malformed
#
# Approach:
#   1. Read the entire file into a variable (fail loudly if unreadable).
#   2. Find the "env" object with a grep-based state machine.
#   3. Within it, locate the key and extract its value.
#
# Limitations (intentional — sufficient for the value space in use):
#   - Does not handle nested objects inside "env".
#   - Does not handle values containing embedded newlines (not valid here).
#   - Does not handle escaped quotes inside the value (not expected here).
# ---------------------------------------------------------------------------
extract_autocompact_value() {
  local settings_file="$1"
  local file_content

  # Read the file; fail to PARSE_ERROR if unreadable.
  if ! file_content="$(cat "$settings_file" 2>&1)"; then
    echo "PARSE_ERROR:cannot read file: $file_content"
    return 0
  fi

  # Quick sanity check: file must contain at least one '{'.
  if ! printf '%s' "$file_content" | grep -qF '{'; then
    echo "PARSE_ERROR:not a JSON object (no '{' found)"
    return 0
  fi

  # ---------------------------------------------------------------------------
  # Locate the "env" block.
  # Strategy: find the line containing '"env"' and a '{', then collect lines
  # until a line matching the closing brace at the same nesting depth.
  #
  # We use awk for multi-line extraction — awk is a POSIX tool with no
  # undeclared dependency risk.  No 2>/dev/null needed: awk always exits 0
  # on valid input, and we already validated the file content.
  # ---------------------------------------------------------------------------
  local env_block
  env_block="$(printf '%s\n' "$file_content" | awk '
    /\"env\"[[:space:]]*:/ { in_env=1; depth=0 }
    in_env {
      for (i=1; i<=length($0); i++) {
        c = substr($0, i, 1)
        if (c == "{") depth++
        if (c == "}") {
          depth--
          if (depth == 0) { print; in_env=0; next }
        }
      }
      print
    }
  ')"

  # No "env" block found at all.
  if [ -z "$env_block" ]; then
    echo "ABSENT:"
    return 0
  fi

  # ---------------------------------------------------------------------------
  # Within the env block, find the key CLAUDE_AUTOCOMPACT_PCT_OVERRIDE and
  # extract its value.
  #
  # Match pattern (handles quoted string or bare number):
  #   "CLAUDE_AUTOCOMPACT_PCT_OVERRIDE"   :   "VALUE"
  #   "CLAUDE_AUTOCOMPACT_PCT_OVERRIDE"   :   NUMBER
  #
  # We use grep + sed to extract the raw value string (without quotes).
  # ---------------------------------------------------------------------------
  local raw_value

  # grep for the key line; sed strips surrounding quotes and trailing
  # comma/whitespace.  If the key is absent, grep exits non-zero — we
  # treat that as ABSENT (not an error).
  local key_line
  if ! key_line="$(printf '%s\n' "$env_block" | grep "\"${CHECK_NAME}\"")"; then
    echo "ABSENT:"
    return 0
  fi

  # Extract value: take everything after the first ':', strip leading/trailing
  # whitespace, strip enclosing double-quotes (if present), strip trailing comma.
  raw_value="$(printf '%s\n' "$key_line" \
    | sed 's/^[^:]*://' \
    | sed 's/^[[:space:]]*//' \
    | sed 's/[[:space:]]*,*[[:space:]]*$//' \
    | sed 's/^"//' \
    | sed 's/"$//')"

  # Guard: empty value after extraction
  if [ -z "$raw_value" ]; then
    echo "NON_NUMERIC:"
    return 0
  fi

  # Check if value is a decimal integer (optional leading minus + digits only).
  if printf '%s' "$raw_value" | grep -qE '^-?[0-9]+$'; then
    echo "OK:${raw_value}"
  else
    echo "NON_NUMERIC:${raw_value}"
  fi
  return 0
}

# ---------------------------------------------------------------------------
# Step 1: Determine which settings.json to use (INV2: project-local first)
# ---------------------------------------------------------------------------
SETTINGS_PATH=""
SETTINGS_FOUND=false

if [ -f "$LOCAL_SETTINGS" ]; then
  SETTINGS_PATH="$LOCAL_SETTINGS"
  SETTINGS_FOUND=true
elif [ -f "$GLOBAL_SETTINGS" ]; then
  SETTINGS_PATH="$GLOBAL_SETTINGS"
  SETTINGS_FOUND=true
fi

# ---------------------------------------------------------------------------
# Step 2: No settings.json found → AC-001 + AC-004 PC4(c) ADVISORY
# ---------------------------------------------------------------------------
if [ "$SETTINGS_FOUND" = false ]; then
  emit_advisory "${REMEDIATION_HINT} (no settings.json found at .claude/settings.json or ~/.claude/settings.json)"
  exit 0
fi

# ---------------------------------------------------------------------------
# Step 3: Parse value from resolved settings.json using pure-bash extraction.
# Output format: STATUS:raw_value (see extract_autocompact_value docstring).
# ---------------------------------------------------------------------------
PARSE_OUT="$(extract_autocompact_value "$SETTINGS_PATH")"

# Parse the structured output: STATUS:value
PARSE_STATUS="${PARSE_OUT%%:*}"
PARSE_RAW="${PARSE_OUT#*:}"

# ---------------------------------------------------------------------------
# Step 4: Emit appropriate row based on parse result
# ---------------------------------------------------------------------------

case "$PARSE_STATUS" in
  OK)
    # Key present and numeric — compare against ceiling 80 (inclusive: ≤ 80 → PASS)
    VAL="$PARSE_RAW"
    if [ "$VAL" -le 80 ]; then
      # AC-003 / BC-6.25.001 PC3: value ≤ 80 → PASS
      emit_pass "$VAL"
    else
      # AC-002 / BC-6.25.001 PC2: value > 80 → ADVISORY
      emit_advisory "Value $VAL exceeds ADR-026 §Decision 5 ceiling of 80 (MEDIUM-confidence 83% harness cap); recommend 70 for safe PreCompact flush headroom"
    fi
    ;;

  NON_NUMERIC)
    # AC-006 / BC-6.25.001 INV3: non-numeric treated as absent → ADVISORY with note
    RAW="$PARSE_RAW"
    if [ -z "$RAW" ]; then
      emit_advisory "${REMEDIATION_HINT}; Value '' is not a valid integer; treating as absent"
    else
      emit_advisory "${REMEDIATION_HINT}; Value '$RAW' is not a valid integer; treating as absent"
    fi
    ;;

  ABSENT)
    # AC-001 / BC-6.25.001 PC1: key absent → ADVISORY with remediation hint
    emit_advisory "$REMEDIATION_HINT"
    ;;

  PARSE_ERROR)
    # EC-011: malformed JSON → ADVISORY
    emit_advisory "settings.json parse error: $PARSE_RAW; cannot verify $CHECK_NAME (ADR-026 §Decision 5)"
    ;;

  *)
    # Fallback: treat unknown parse state as absent (INV1: never block)
    emit_advisory "$REMEDIATION_HINT"
    ;;
esac

# Always exit 0 — advisory-only, never blocking (BC-6.25.001 INV1 / AC-005 / PC5)
exit 0
