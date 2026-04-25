#!/usr/bin/env bash
# verify-sha-currency.sh — state-manager burst hygiene gate
#
# Verifies that SHAs cited in .factory/STATE.md and .factory/SESSION-HANDOFF.md
# are current (match git HEAD), that cross-record SHAs in
# .factory/wave-state.yaml agree with STATE.md frontmatter, and that the
# active-pass narrative is in past-tense voice.
#
# Run BEFORE every push to factory-artifacts (Stage 1 of two-commit protocol)
# AND AFTER (Stage 2 verification). The dispatcher's wave-gate-prerequisite
# hook calls this automatically, but you can also run it manually:
#
#   bash .factory/hooks/verify-sha-currency.sh
#   bash .factory/hooks/verify-sha-currency.sh --project-root /path/to/proj
#
# Exit codes:
#   0 — PASS (all checks clean)
#   1 — FAIL (one or more checks detected drift)
#
# WARN-level issues (tense-flip, fabricated SHA cites) print to stdout but do
# NOT fail the run — they're advisory.
#
# History: This hook started as a STATE.md/HANDOFF.md HEAD-currency check
# during a wave-gate convergence cycle that hit six consecutive recurrences
# of SHA drift / narrative-staleness. It hardened across each pass:
#   - Multi-commit-chain guard (chains > 2 commits)
#   - Single Canonical SHA Rule + 2-commit exception
#   - Cross-record verification (STATE.md vs wave-state.yaml)
#   - Tense-flip detection
# See docs/lessons-learned/wave-gate-bookkeeping.md for the full case study.

set -euo pipefail

# ---------- Argument parsing ----------

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FACTORY_DIR="$(dirname "$SCRIPT_DIR")"
PROJECT_ROOT="$(dirname "$FACTORY_DIR")"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --project-root)
      PROJECT_ROOT="$2"
      FACTORY_DIR="$PROJECT_ROOT/.factory"
      shift 2
      ;;
    *)
      echo "Unknown argument: $1" >&2
      exit 1
      ;;
  esac
done

STATE_MD="$FACTORY_DIR/STATE.md"
HANDOFF_MD="$FACTORY_DIR/SESSION-HANDOFF.md"
WAVE_STATE="$FACTORY_DIR/wave-state.yaml"

echo "=== SHA Currency + Burst Hygiene Check ==="
echo "Project root  : $PROJECT_ROOT"
echo "factory dir   : $FACTORY_DIR"
echo ""

FAIL=0
WARN=0

# ---------- Resolve actual SHAs ----------

ACTUAL_DEV_FULL=$(git -C "$PROJECT_ROOT" rev-parse develop 2>/dev/null || echo "ERR_NO_DEVELOP")
ACTUAL_FA_FULL=$(git -C "$FACTORY_DIR" rev-parse HEAD 2>/dev/null || echo "ERR_NO_FA")

ACTUAL_DEV="${ACTUAL_DEV_FULL:0:8}"
ACTUAL_FA="${ACTUAL_FA_FULL:0:8}"

echo "Actual develop HEAD      : $ACTUAL_DEV"
echo "Actual factory-artifacts : $ACTUAL_FA"
echo ""

# ---------- Read cited SHAs ----------

if [ ! -f "$STATE_MD" ]; then
  echo "FAIL: $STATE_MD missing — state-manager has not initialized?"
  exit 1
fi

CITED_DEV_STATE=$(grep -oE 'develop_head: "?[0-9a-f]{8,40}' "$STATE_MD" 2>/dev/null \
  | head -1 | grep -oE '[0-9a-f]{8,40}' | cut -c1-8 || echo "NOT_FOUND")
CITED_FA_STATE=$(grep -oE 'factory-artifacts HEAD[^0-9a-f]*[0-9a-f]{8}' "$STATE_MD" 2>/dev/null \
  | head -1 | grep -oE '[0-9a-f]{8}' | tail -1 || echo "NOT_FOUND")

if [ -f "$HANDOFF_MD" ]; then
  CITED_DEV_HANDOFF=$(grep -oE 'develop HEAD[^|`]*`?[0-9a-f]{8}' "$HANDOFF_MD" 2>/dev/null \
    | head -1 | grep -oE '[0-9a-f]{8}' | tail -1 || echo "NOT_FOUND")
  CITED_FA_HANDOFF=$(grep -oE 'factory-artifacts HEAD[^0-9a-f]*[0-9a-f]{8}' "$HANDOFF_MD" 2>/dev/null \
    | head -1 | grep -oE '[0-9a-f]{8}' | tail -1 || echo "NOT_FOUND")
else
  CITED_DEV_HANDOFF="NO_HANDOFF"
  CITED_FA_HANDOFF="NO_HANDOFF"
fi

echo "STATE.md    develop cited      : $CITED_DEV_STATE"
echo "STATE.md    factory-arts cited : $CITED_FA_STATE"
echo "HANDOFF.md  develop cited      : $CITED_DEV_HANDOFF"
echo "HANDOFF.md  factory-arts cited : $CITED_FA_HANDOFF"
echo ""

# ---------- develop SHA must match exactly ----------

if [ "$CITED_DEV_STATE" != "NOT_FOUND" ] && [ "$ACTUAL_DEV" != "$CITED_DEV_STATE" ]; then
  echo "FAIL: develop SHA in STATE.md is stale (cited=$CITED_DEV_STATE actual=$ACTUAL_DEV)"
  FAIL=1
fi
if [ "$CITED_DEV_HANDOFF" != "NOT_FOUND" ] && [ "$CITED_DEV_HANDOFF" != "NO_HANDOFF" ] \
    && [ "$ACTUAL_DEV" != "$CITED_DEV_HANDOFF" ]; then
  echo "FAIL: develop SHA in SESSION-HANDOFF.md is stale (cited=$CITED_DEV_HANDOFF actual=$ACTUAL_DEV)"
  FAIL=1
fi

# ---------- Two-commit protocol exception (factory-artifacts SHA) ----------

# Allow 1-commit drift between cited factory-artifacts SHA and HEAD ONLY when:
#   (a) HEAD's commit message contains "backfill" (Stage 2 marker), AND
#   (b) HEAD^'s commit message does NOT contain "backfill" (chain is exactly 2)
# Without (b), the exception masks 3+ commit chain bursts.
HEAD_MSG=$(git -C "$FACTORY_DIR" log -1 --format=%s 2>/dev/null || echo "")
PARENT_MSG=$(git -C "$FACTORY_DIR" log -1 --format=%s HEAD^ 2>/dev/null || echo "")
HEAD_IS_BACKFILL=0
PARENT_IS_BACKFILL=0
echo "$HEAD_MSG"   | grep -qi "backfill" && HEAD_IS_BACKFILL=1
echo "$PARENT_MSG" | grep -qi "backfill" && PARENT_IS_BACKFILL=1

if [ "$HEAD_IS_BACKFILL" -eq 1 ] && [ "$PARENT_IS_BACKFILL" -eq 1 ]; then
  echo "FAIL: MULTI_COMMIT_CHAIN_NOT_ALLOWED — HEAD and HEAD^ both contain 'backfill'."
  echo "      The two-commit protocol permits exactly 1 fix commit + 1 backfill commit."
  echo "      Recover with: git -C .factory reset --soft HEAD~2 && redo Stage 1 + Stage 2."
  FAIL=1
fi

# Fabrication check: verify cited SHAs actually exist as git objects.
for label in "STATE.md:$CITED_FA_STATE" "SESSION-HANDOFF.md:$CITED_FA_HANDOFF"; do
  doc="${label%%:*}"
  sha="${label#*:}"
  [ "$sha" = "NOT_FOUND" ] || [ "$sha" = "NO_HANDOFF" ] && continue
  if ! git -C "$FACTORY_DIR" cat-file -e "${sha}^{commit}" 2>/dev/null; then
    echo "WARN: $doc cites factory-artifacts SHA $sha but it does not exist as a git object (FABRICATED?)"
    WARN=1
  fi
done

PARENT_FA=$(git -C "$FACTORY_DIR" rev-parse HEAD^ 2>/dev/null | cut -c1-8 || echo "NO_PARENT")

check_fa_currency() {
  local doc="$1" cited="$2"
  [ "$cited" = "NOT_FOUND" ] || [ "$cited" = "NO_HANDOFF" ] && return 0
  if [ "$ACTUAL_FA" = "$cited" ]; then
    return 0
  fi
  if [ "$PARENT_FA" = "$cited" ] && [ "$HEAD_IS_BACKFILL" -eq 1 ] && [ "$PARENT_IS_BACKFILL" -eq 0 ]; then
    echo "NOTE: $doc cites HEAD^ ($cited) — within two-commit protocol exception (HEAD is backfill; HEAD^ is not)"
    return 0
  fi
  echo "FAIL: factory-artifacts SHA in $doc is stale (cited=$cited actual=$ACTUAL_FA parent=$PARENT_FA head_is_backfill=$HEAD_IS_BACKFILL parent_is_backfill=$PARENT_IS_BACKFILL)"
  return 1
}

check_fa_currency "STATE.md"           "$CITED_FA_STATE"   || FAIL=1
check_fa_currency "SESSION-HANDOFF.md" "$CITED_FA_HANDOFF" || FAIL=1

# ---------- Cross-record SHA verification (STATE.md ↔ wave-state.yaml) ----------

# For every wave declared in wave-state.yaml's `waves:` map, find every
# `gate_pass_N: { ..., remediation_sha: <sha> }` record. For each, check that
# the corresponding STATE.md frontmatter entry
# `adversary_<wave>_pass_N_*.remediation_sha` agrees. Drift between these is
# the Pass 6 H-001 defect class (single-canonical-SHA discipline applied only
# to the current burst, not to historical pass records that needed re-cite).
#
# Implementation: python3 + yaml because awk+sed couldn't scope per-wave
# without hardcoding wave names. The script falls back to "no-op with WARN"
# if python3 or yaml.safe_load are unavailable, so the hook degrades
# gracefully on minimal hosts.

if [ -f "$WAVE_STATE" ]; then
  CROSS_DRIFT=$(python3 - "$WAVE_STATE" "$STATE_MD" 2>/dev/null <<'PY' || echo "PY_UNAVAILABLE"
import re
import sys

try:
    import yaml
except Exception:
    print("PY_UNAVAILABLE")
    sys.exit(0)

wave_state_path, state_md_path = sys.argv[1], sys.argv[2]

with open(wave_state_path) as f:
    state = yaml.safe_load(f) or {}

waves = state.get("waves") or {}

with open(state_md_path) as f:
    state_md = f.read()

drift_lines = []
for wave_name, wave_data in waves.items():
    if not isinstance(wave_data, dict):
        continue
    for key, val in wave_data.items():
        m = re.match(r"^gate_pass_(\d+)$", key)
        if not m or not isinstance(val, dict):
            continue
        pass_n = m.group(1)
        yaml_sha = val.get("remediation_sha")
        if not yaml_sha or yaml_sha == "null":
            continue
        # Look up the STATE.md frontmatter record for this wave + pass.
        # Convention: adversary_<wave>_pass_<N>_*.remediation_sha
        wave_id = wave_name  # already snake_case in canonical layout
        pat = re.compile(
            rf"adversary_{re.escape(wave_id)}_(?:gate_)?pass_{pass_n}_[^:]+:.*?remediation_sha:\s*([0-9a-f]+)",
            re.DOTALL,
        )
        match = pat.search(state_md)
        if not match:
            continue  # No STATE.md record for this pass — out of scope
        state_sha = match.group(1)
        if state_sha != str(yaml_sha):
            drift_lines.append(f"{wave_id} pass_{pass_n}: STATE={state_sha} YAML={yaml_sha}")

for line in drift_lines:
    print(f"DRIFT {line}")
PY
)
  if [ "$CROSS_DRIFT" = "PY_UNAVAILABLE" ]; then
    echo "WARN: cross-record SHA check skipped (python3+yaml unavailable; install via 'pip install pyyaml')"
    WARN=1
  elif [ -n "$CROSS_DRIFT" ]; then
    while IFS= read -r line; do
      [ -z "$line" ] && continue
      echo "FAIL: cross-record SHA drift — $line"
      FAIL=1
    done <<< "$CROSS_DRIFT"
  fi
fi

# ---------- Tense-flip / narrative-staleness check ----------

# Pre-burst Stage 1 commits MUST write narrative in past-tense voice
# ("REMEDIATED — Awaiting Pass N+1") so that no Stage 2 step is needed to
# rewrite the tense. The case study (see lessons-learned doc) recorded
# tense-flip findings recurring across three consecutive passes before
# the rule was codified.
#
# WARN-level: doesn't fail the gate, just surfaces. Operators can configure
# their pre-push hook to FAIL on WARN if they want strict enforcement.

TENSE_FLIP_PATTERNS='IN_PROGRESS|in progress|REMEDIATION_IN_PROGRESS|this burst remediates|burst remediates|REMEDIATION IN PROGRESS'
for doc in "$STATE_MD" "$HANDOFF_MD" "$WAVE_STATE"; do
  [ -f "$doc" ] || continue
  hits=$(grep -E "$TENSE_FLIP_PATTERNS" "$doc" 2>/dev/null | head -3)
  if [ -n "$hits" ]; then
    echo "WARN: $(basename "$doc") contains in-progress voice — Stage 1 commits should be past-tense:"
    echo "$hits" | sed 's/^/      /'
    WARN=1
  fi
done

# ---------- Summary ----------

echo ""
if [ "$FAIL" -eq 0 ] && [ "$WARN" -eq 0 ]; then
  echo "PASS: all SHA currency + burst hygiene checks pass"
  exit 0
elif [ "$FAIL" -eq 0 ]; then
  echo "PASS (with WARN): $(echo "$WARN") warnings — review and decide if blocking"
  exit 0
else
  echo "FAIL: SHA drift or burst-hygiene issues detected — fix before pushing"
  exit 1
fi
