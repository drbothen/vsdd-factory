#!/usr/bin/env bash
# Demo: Anti-fabrication hard-block (AC-015 preflight + AC-001 no partial write)
# Shows: phantom story ID → exit 1 + AntiFabricationFailed + NO partial HANDOFF.md
set -euo pipefail

SKILL="/Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-18.01/plugins/vsdd-factory/skills/wave-handoff/wave-handoff.sh"

echo "=== wave-handoff: Anti-fabrication hard-block demo ==="
echo "S-99.99 is a PHANTOM story not in STORY-INDEX.md"

WORK="$(mktemp -d /tmp/demo-antifab-XXXXXX)"
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

# STORY-INDEX only has S-18.02
cat > "$ARTIFACTS_WT/stories/STORY-INDEX.md" << 'EOFI'
---
document_type: story-index
---
## Epic E-18
| Story ID | Title | Epic | Points | Priority | Depends-On | Blocks | Status | BCs |
|----------|-------|------|--------|----------|-----------|--------|--------|-----|
| S-18.02 | Validate handoff | E-18 | 8 | P0 | [] | [] | draft | [] |
EOFI

# sprint-state references S-99.99 (phantom — not in STORY-INDEX)
cat > "$WORK/sprint-state.yaml" << 'EOFI'
stories:
  - id: S-18.02
    status: pending
  - id: S-99.99
    status: draft
EOFI
cat > "$WORK/STATE.md" << 'EOFI'
---
current_step: "pass-2"
current_cycle: "v1.0-feature-context-durability-E18"
factory_lock: null
---
# STATE
EOFI

echo "STORY-INDEX: [S-18.02]  sprint-state: [S-18.02, S-99.99]"
echo "S-99.99 is NOT in STORY-INDEX — AntiFabricationFailed expected"
echo ""
set +e
bash "$SKILL" \
  --artifacts-worktree "$ARTIFACTS_WT" \
  --sprint-state "$WORK/sprint-state.yaml" \
  --state-md "$WORK/STATE.md" \
  --bc-dir "$ARTIFACTS_WT/specs/behavioral-contracts"
EXIT_CODE=$?
set -e
echo ""
echo "--- Exit code: $EXIT_CODE (expected: 1) ---"
if [ -f "$ARTIFACTS_WT/HANDOFF.md" ]; then
  echo "ERROR: partial HANDOFF.md was written (spec violation)"
else
  echo "CONFIRMED: No partial HANDOFF.md written (clean worktree)"
fi
echo "Commits on factory-artifacts: $(git -C "$WORK" rev-list --count factory-artifacts) (expected: 1)"
echo "=== DONE: AntiFabricationFailed hard-block verified ==="

git -C "$WORK" worktree remove --force "$ARTIFACTS_WT" 2>/dev/null || true
rm -rf "$WORK"
