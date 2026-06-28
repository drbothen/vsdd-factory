#!/usr/bin/env bash
# check-autocompact-setting.sh — S-18.10 / BC-6.25.001
#
# Reads .claude/settings.json (project-local, preferred) then
# ~/.claude/settings.json (global fallback) and verifies that
# env.CLAUDE_AUTOCOMPACT_PCT_OVERRIDE is present, numeric, in the
# valid range 1–100, and not exceeding the ADR-026 §Decision 5 ceiling (80).
#
# Emits one check table row to stdout:
#   PASS     — key present and numeric value in [1, 80]
#   ADVISORY — key absent, value ≤ 0, value > 80, non-numeric (treated as absent),
#              missing settings.json, or malformed JSON
#
# Advisory-only: exits 0 in all cases; never blocks (BC-6.25.001 INV1).
# No side effects: reads settings.json only (BC-6.25.001 INV4).
# Row always emitted (BC-6.25.001 INV5).
#
# Uses jq (required tool per setup-env/SKILL.md; many repo hooks already use
# command -v jq guards).  When jq is absent the helper degrades gracefully to
# a single ADVISORY row and exits 0 — never fatal (INV1/INV5).
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
# Step 0: Guard — jq required.  Degrade gracefully if absent (INV1/INV5).
# ---------------------------------------------------------------------------
if ! command -v jq > /dev/null 2>&1; then
  emit_advisory "settings.json cannot be verified — jq is required but not found; install with: brew install jq"
  exit 0
fi

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
# Step 3: Parse value from resolved settings.json using jq.
# jq stderr captured into a shell variable (no temp file — INV4 compliance).
# jq is format-agnostic — handles both single-line and multi-line JSON.
# ---------------------------------------------------------------------------

# Capture jq stderr for EC-011 parse-error advisory (variable, not file — INV4).
# STDERR-EXEMPT: jq stderr captured for EC-011 parse-error advisory (variable, not file)
JQ_ERR="$(jq -r '.env.CLAUDE_AUTOCOMPACT_PCT_OVERRIDE // empty' "$SETTINGS_PATH" 2>&1 1>/dev/null)" || true

# Determine key presence separately from value extraction.
# jq -e '.env | objects | has("KEY")' exits 0 + prints "true" when .env is an object
# and the key exists; exits 1 + prints "false" or no output when .env is absent or
# the key is missing.  Using `|| true` so set -e does not abort on exit 1 (key absent).
# STDERR-EXEMPT: parse validity already confirmed by the preceding jq guard; suppress residual diagnostic noise
KEY_PRESENT="$(jq -r '.env | objects | if has("CLAUDE_AUTOCOMPACT_PCT_OVERRIDE") then "true" else "false" end' "$SETTINGS_PATH" 2>/dev/null)" || { # STDERR-EXEMPT: parse validity already confirmed by the preceding jq guard; suppress residual diagnostic noise on key-presence probe
  # jq parse failure → EC-011 ADVISORY with jq's actual error message.
  emit_advisory "settings.json parse error: ${JQ_ERR}; cannot verify $CHECK_NAME"
  exit 0
}

# Extract the raw value WITHOUT `// empty` so an empty string "" is preserved
# as a distinct value from a truly absent key (which yields "null" from jq -r).
# STDERR-EXEMPT: parse validity already confirmed by the preceding jq guard; suppress residual diagnostic noise on value extraction
RAW_VALUE="$(jq -r '.env.CLAUDE_AUTOCOMPACT_PCT_OVERRIDE' "$SETTINGS_PATH" 2>/dev/null)" || { # STDERR-EXEMPT: parse validity already confirmed by the preceding jq guard; suppress residual diagnostic noise on value extraction
  # jq parse failure → EC-011 ADVISORY with jq's actual error message.
  emit_advisory "settings.json parse error: ${JQ_ERR}; cannot verify $CHECK_NAME"
  exit 0
}

# ---------------------------------------------------------------------------
# Step 4: Emit appropriate row based on extracted value
# ---------------------------------------------------------------------------

# Case A: key absent or env block absent.
# KEY_PRESENT is "false" (env block missing or key not present) or empty (env is not
# an object — e.g. "env": null).  In all these cases treat as key-absent (PC1).
if [ "$KEY_PRESENT" != "true" ]; then
  emit_advisory "$REMEDIATION_HINT"
  exit 0
fi

# Key is present.  RAW_VALUE is now the literal JSON string value; "null" means the
# JSON key exists but is JSON null (treat as non-numeric per INV3).

# Case B: key present but value is non-numeric (empty string "", "auto", null, etc.)
# — BC-6.25.001 INV3: emit the note "Value '<raw>' is not a valid integer; treating as absent".
# For the null case jq -r returns the string "null"; treat that as non-numeric too.
# Use grep pattern: optional leading minus + one or more digits.
if ! printf '%s' "$RAW_VALUE" | grep -qE '^-?[0-9]+$'; then
  # AC-006 / EC-008 / EC-009 / BC-6.25.001 INV3: non-numeric (incl. empty string) treated
  # as absent → ADVISORY with note.  <raw> is the actual value (may be empty for "").
  emit_advisory "${REMEDIATION_HINT}; Value '$RAW_VALUE' is not a valid integer; treating as absent"
  exit 0
fi

# Case C: numeric — classify the value.
# Use POSIX [ ] arithmetic comparison, not $(( )) — avoids octal pitfalls.
VAL="$RAW_VALUE"

# EC-012 lower-bound: value ≤ 0 is not a valid compaction percentage.
# Compare with test/[ ] arithmetic: use -le / -gt for base-10 semantics.
if [ "$VAL" -le 0 ]; then
  emit_advisory "Value $VAL is not a valid compaction percentage (must be in range 1–100); treating as misconfigured — recommend 70 per ADR-026 §Decision 5"
  exit 0
fi

if [ "$VAL" -le 80 ]; then
  # AC-003 / BC-6.25.001 PC3: value in [1, 80] → PASS
  emit_pass "$VAL"
else
  # AC-002 / BC-6.25.001 PC2: value > 80 → ADVISORY
  emit_advisory "Value $VAL exceeds ADR-026 §Decision 5 ceiling of 80 (MEDIUM-confidence 83% harness cap); recommend 70 for safe PreCompact flush headroom"
fi

# Always exit 0 — advisory-only, never blocking (BC-6.25.001 INV1 / AC-005 / PC5)
exit 0
