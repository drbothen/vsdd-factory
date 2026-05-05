#!/usr/bin/env bash
# verify-sha-currency.sh — state-manager burst hygiene gate
#
# Verifies cross-branch and cross-record SHA agreement, plus narrative
# voice on the active pass. Specifically:
#   - develop SHA cited in STATE.md/HANDOFF.md matches actual develop HEAD
#     (cross-branch cite — not self-referential, no loop possible)
#   - Cross-record SHA agreement between wave-state.yaml's gate_pass_N
#     remediation_sha entries and STATE.md frontmatter
#   - Multi-commit-chain guard (chains > 2 commits on factory-artifacts)
#   - Tense-flip detection in active-pass narrative (advisory)
#
# Run as the state-manager burst hygiene gate; safe to invoke manually:
#
#   bash .factory/hooks/verify-sha-currency.sh
#   bash .factory/hooks/verify-sha-currency.sh --project-root /path/to/proj
#
# Exit codes:
#   0 — PASS (all checks clean)
#   1 — FAIL (one or more checks detected drift)
#
# WARN-level issues (tense-flip) print to stdout but do NOT fail the run.
#
# What this hook does NOT verify (TD-VSDD-053 narrowing, 2026-05-04):
#
# The hook used to verify that the *current-burst factory-artifacts HEAD* SHA
# cited inside STATE.md / SESSION-HANDOFF.md matched git HEAD. That check
# was removed because the cite was self-referential — STATE.md sits on the
# factory-artifacts branch, so committing STATE.md changes HEAD, instantly
# staling any HEAD-SHA cite inside the same content. The two-commit protocol
# (Stage 1 placeholder → commit → Stage 2 backfill SHA → commit) was a
# workaround that triggered "fix-the-fix" loops when any of 8 cite locations
# was missed (manifested 6× in one session, 5+ force-pushes). Resolved by
# removing the cite altogether: STATE.md no longer claims its own SHA;
# `git -C .factory log -1` is the source of truth. Historical SHA references
# in changelog rows, decisions log, and cycle manifests remain valid (they
# point at PAST burst SHAs which are immutable audit trail).
#
# History: This hook started as a STATE.md/HANDOFF.md HEAD-currency check
# during a wave-gate convergence cycle that hit six consecutive recurrences
# of SHA drift / narrative-staleness. It hardened across each pass and was
# narrowed in scope by TD-VSDD-053 (2026-05-04). See
# docs/lessons-learned/wave-gate-bookkeeping.md for the full case study.

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

# ---------- Read cited SHAs (cross-branch only — develop) ----------
#
# We deliberately do NOT extract the factory-artifacts HEAD SHA from STATE.md
# or SESSION-HANDOFF.md. Per TD-VSDD-053 (2026-05-04), STATE.md no longer
# cites its own branch's HEAD — that cite was self-referential and looped.
# The current factory-artifacts HEAD is `git -C .factory log -1`, not a
# string in any artifact.
#
# We DO extract the develop SHA cite because that's a cross-branch claim
# (STATE.md sits on factory-artifacts but cites develop); committing STATE.md
# does not change develop, so the cite is stable across the commit and the
# loop class doesn't apply.

if [ ! -f "$STATE_MD" ]; then
  echo "FAIL: $STATE_MD missing — state-manager has not initialized?"
  exit 1
fi

CITED_DEV_STATE=$(grep -oE 'develop_head: "?[0-9a-f]{8,40}' "$STATE_MD" 2>/dev/null \
  | head -1 | grep -oE '[0-9a-f]{8,40}' | cut -c1-8 || echo "NOT_FOUND")

if [ -f "$HANDOFF_MD" ]; then
  CITED_DEV_HANDOFF=$(grep -oE 'develop HEAD[^|`]*`?[0-9a-f]{8}' "$HANDOFF_MD" 2>/dev/null \
    | head -1 | grep -oE '[0-9a-f]{8}' | tail -1 || echo "NOT_FOUND")
else
  CITED_DEV_HANDOFF="NO_HANDOFF"
fi

echo "STATE.md    develop cited      : $CITED_DEV_STATE"
echo "HANDOFF.md  develop cited      : $CITED_DEV_HANDOFF"
echo ""

# ---------- develop SHA must match exactly (cross-branch cite, no loop) ----------

if [ "$CITED_DEV_STATE" != "NOT_FOUND" ] && [ "$ACTUAL_DEV" != "$CITED_DEV_STATE" ]; then
  echo "FAIL: develop SHA in STATE.md is stale (cited=$CITED_DEV_STATE actual=$ACTUAL_DEV)"
  FAIL=1
fi
if [ "$CITED_DEV_HANDOFF" != "NOT_FOUND" ] && [ "$CITED_DEV_HANDOFF" != "NO_HANDOFF" ] \
    && [ "$ACTUAL_DEV" != "$CITED_DEV_HANDOFF" ]; then
  echo "FAIL: develop SHA in SESSION-HANDOFF.md is stale (cited=$CITED_DEV_HANDOFF actual=$ACTUAL_DEV)"
  FAIL=1
fi

# ---------- Multi-commit chain guard (factory-artifacts) ----------
#
# The two-commit protocol is retired (TD-VSDD-053) — single-commit bursts
# only. This guard remains as a regression defense: if any future workflow
# reintroduces backfill-style chains, fail loudly so the discipline gap is
# visible immediately.

HEAD_MSG=$(git -C "$FACTORY_DIR" log -1 --format=%s 2>/dev/null || echo "")
PARENT_MSG=$(git -C "$FACTORY_DIR" log -1 --format=%s HEAD^ 2>/dev/null || echo "")
HEAD_IS_BACKFILL=0
PARENT_IS_BACKFILL=0
echo "$HEAD_MSG"   | grep -qi "backfill" && HEAD_IS_BACKFILL=1
echo "$PARENT_MSG" | grep -qi "backfill" && PARENT_IS_BACKFILL=1

if [ "$HEAD_IS_BACKFILL" -eq 1 ] && [ "$PARENT_IS_BACKFILL" -eq 1 ]; then
  echo "FAIL: MULTI_COMMIT_CHAIN_NOT_ALLOWED — HEAD and HEAD^ both contain 'backfill'."
  echo "      The single-commit protocol (TD-VSDD-053) does not use backfill commits."
  echo "      A 2+ chain of 'backfill' commits suggests reintroduction of the retired"
  echo "      two-commit pattern. Recover with: git -C .factory reset --soft HEAD~2"
  echo "      then re-author as a single commit per skills/state-burst/SKILL.md."
  FAIL=1
fi

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
