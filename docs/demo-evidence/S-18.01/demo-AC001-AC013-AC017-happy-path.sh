#!/usr/bin/env bash
# Demo: AC-001/AC-013/AC-015/AC-017 — Happy-path has-next-wave
# Shows: committed HANDOFF.md (9 base fields) + wave-state.yaml (6 fields) atomically
set -euo pipefail

SKILL="/Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-18.01/plugins/vsdd-factory/skills/wave-handoff/wave-handoff.sh"

echo "=== wave-handoff: happy-path has-next-wave demo ==="

WORK="$(mktemp -d /tmp/demo-happy-XXXXXX)"
ARTIFACTS_WT="$WORK/factory-wt"

# Init hermetic git fixture
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
mkdir -p "$ARTIFACTS_WT/hooks" "$ARTIFACTS_WT/specs/behavioral-contracts/ss-05"
mkdir -p "$ARTIFACTS_WT/specs/architecture/decisions" "$ARTIFACTS_WT/stories"
echo "# BC-5.41.001" > "$ARTIFACTS_WT/specs/behavioral-contracts/ss-05/BC-5.41.001.md"
echo "# ARCH-INDEX" > "$ARTIFACTS_WT/specs/architecture/ARCH-INDEX.md"
echo "# ADR-026" > "$ARTIFACTS_WT/specs/architecture/decisions/ADR-026-wave-boundary-checkpoint-reset-and-lossless-intra-wave-compaction.md"
cat > "$ARTIFACTS_WT/stories/STORY-INDEX.md" << 'EOFI'
---
document_type: story-index
---
## Epic E-0 (7-col spaced)
| Story ID | Title | Epic | Points | Priority | Depends On | Status |
|----------|-------|------|--------|----------|------------|--------|
| S-0.01 | infra | E-0 | 2 | P0 | -- | merged |
## Epic E-18 (9-col hyphenated)
| Story ID | Title | Epic | Points | Priority | Depends-On | Blocks | Status | BCs |
|----------|-------|------|--------|----------|-----------|--------|--------|-----|
| S-18.02 | Validate handoff | E-18 | 8 | P0 | [] | [S-18.03] | draft | [] |
| S-18.03 | Rehydrate wave   | E-18 | 8 | P1 | [S-18.02] | [] | pending | [] |
EOFI
cat > "$ARTIFACTS_WT/stories/S-18.02-validate.md" << 'EOFI'
---
story_id: S-18.02
behavioral_contracts:
  - BC-4.14.001
---
EOFI
cat > "$ARTIFACTS_WT/stories/S-18.03-rehydrate.md" << 'EOFI'
---
story_id: S-18.03
behavioral_contracts:
  - BC-6.24.001
---
EOFI
cat > "$WORK/sprint-state.yaml" << 'EOFI'
stories:
  - id: S-18.02
    status: pending
  - id: S-18.03
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

echo "Fixture ready. Invoking skill..."
echo ""
bash "$SKILL" \
  --artifacts-worktree "$ARTIFACTS_WT" \
  --sprint-state "$WORK/sprint-state.yaml" \
  --state-md "$WORK/STATE.md" \
  --bc-dir "$ARTIFACTS_WT/specs/behavioral-contracts" 2>/dev/null

echo ""
echo "--- COMMITTED HANDOFF.md (9 base fields) ---"
git -C "$WORK" show factory-artifacts:HANDOFF.md
echo ""
echo "--- Commit: $(git -C "$WORK" log --oneline factory-artifacts | head -1) ---"
echo "--- Files: $(git -C "$WORK" diff-tree --no-commit-id -r --name-only factory-artifacts | tr '\n' ' ') ---"
echo "=== DONE: exit 0, atomic commit, 9 fields in HANDOFF.md ==="

git -C "$WORK" worktree remove --force "$ARTIFACTS_WT" 2>/dev/null || true
rm -rf "$WORK"
