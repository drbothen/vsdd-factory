#!/usr/bin/env bash
# Demo: AC-012 — EPIC-COMPLETE path
# Shows: verbatim 3-line announcement + epic_status:complete + no wave-state.yaml
set -euo pipefail

SKILL="/Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-18.01/plugins/vsdd-factory/skills/wave-handoff/wave-handoff.sh"

echo "=== wave-handoff: EPIC-COMPLETE demo (all stories terminal) ==="

WORK="$(mktemp -d /tmp/demo-epic-XXXXXX)"
ARTIFACTS_WT="$WORK/factory-wt"

git -C "$WORK" init -q -b feature-test 2>/dev/null
git -C "$WORK" config user.email "demo@vsdd-factory"
git -C "$WORK" config user.name "Demo"
echo "root" > "$WORK/root.txt"
git -C "$WORK" add root.txt
git -C "$WORK" commit -q -m "feature-test root"
git -C "$WORK" update-ref refs/remotes/origin/develop "$(git -C "$WORK" rev-parse HEAD)"
git -C "$WORK" checkout --orphan factory-artifacts -q
git -C "$WORK" rm -rf . -q 2>/dev/null || true
echo "init" > "$WORK/.gitkeep"
git -C "$WORK" add .gitkeep
git -C "$WORK" commit -q -m "factory-artifacts init"
git -C "$WORK" checkout -q feature-test
mkdir -p "$ARTIFACTS_WT"
git -C "$WORK" worktree add -q "$ARTIFACTS_WT" factory-artifacts
mkdir -p "$ARTIFACTS_WT/hooks" "$ARTIFACTS_WT/specs/behavioral-contracts/ss-05" "$ARTIFACTS_WT/stories"
echo "# BC-5.41.001" > "$ARTIFACTS_WT/specs/behavioral-contracts/ss-05/BC-5.41.001.md"

cat > "$WORK/sprint-state.yaml" << 'EOFI'
stories:
  - id: S-18.01
    status: merged
  - id: S-18.02
    status: merged
  - id: S-18.03
    status: cancelled
EOFI
cat > "$WORK/STATE.md" << 'EOFI'
---
current_step: "pass-3"
current_cycle: "v1.0-feature-context-durability-E18"
factory_lock: null
---
# STATE
EOFI

echo "sprint-state: all stories terminal (merged/cancelled)"
echo "Invoking skill..."
echo ""
bash "$SKILL" \
  --artifacts-worktree "$ARTIFACTS_WT" \
  --sprint-state "$WORK/sprint-state.yaml" \
  --state-md "$WORK/STATE.md" \
  --bc-dir "$ARTIFACTS_WT/specs/behavioral-contracts"
echo ""
echo "--- Checking wave-state.yaml (must be ABSENT) ---"
if git -C "$WORK" show factory-artifacts:wave-state.yaml 2>/dev/null; then
  echo "ERROR: wave-state.yaml written (should be absent on EPIC-COMPLETE)"
else
  echo "CONFIRMED: wave-state.yaml absent from committed tree (AC-012)"
fi
echo "--- epic_status in HANDOFF.md ---"
git -C "$WORK" show factory-artifacts:HANDOFF.md | grep "epic_status"
echo "=== DONE: EPIC-COMPLETE path verified ==="

git -C "$WORK" worktree remove --force "$ARTIFACTS_WT" 2>/dev/null || true
rm -rf "$WORK"
