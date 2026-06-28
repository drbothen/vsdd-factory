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
# Step 3: Parse value from resolved settings.json using python3.
# Output format: STATUS:raw_value
#   OK:<numeric-string>     — key found, value is a valid decimal integer
#   NON_NUMERIC:<raw>       — key found, value is not a decimal integer
#   ABSENT:                 — key or env block missing
#   PARSE_ERROR:<message>   — JSON file could not be parsed
# ---------------------------------------------------------------------------
PARSE_OUT=$(python3 -c "
import sys, json
settings_path = '$SETTINGS_PATH'
check_name = '$CHECK_NAME'
try:
    with open(settings_path) as f:
        data = json.load(f)
except Exception as e:
    print('PARSE_ERROR:' + str(e))
    sys.exit(0)
env_block = data.get('env', None)
if env_block is None or not isinstance(env_block, dict):
    print('ABSENT:')
    sys.exit(0)
val = env_block.get(check_name, None)
if val is None:
    print('ABSENT:')
    sys.exit(0)
val_str = str(val).strip()
if not val_str:
    print('NON_NUMERIC:' + val_str)
    sys.exit(0)
try:
    int(val_str)
except ValueError:
    print('NON_NUMERIC:' + val_str)
    sys.exit(0)
print('OK:' + val_str)
" 2>/dev/null)

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
