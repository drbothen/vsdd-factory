#!/usr/bin/env bash
# Demo: AC-015 — Multi-table topo-sort (BSD-portable, correct epic table selection)
# Shows: dependency-ordered stories from multi-table STORY-INDEX
set -euo pipefail

SKILL="/Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-18.01/plugins/vsdd-factory/skills/wave-handoff/wave-handoff.sh"

echo "=== wave-handoff: Multi-table topo-sort demo ==="
echo "DAG: S-18.02→root, S-18.03→[S-18.02], S-18.04a→[S-18.02,S-18.03]"
echo "sprint-state file order: S-18.04a, S-18.02, S-18.03 (NOT dep order)"

WORK="$(mktemp -d /tmp/demo-topo-XXXXXX)"
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
mkdir -p "$ARTIFACTS_WT/hooks" "$ARTIFACTS_WT/specs/behavioral-contracts/ss-05"
mkdir -p "$ARTIFACTS_WT/specs/architecture/decisions" "$ARTIFACTS_WT/stories"
echo "# BC-5.41.001" > "$ARTIFACTS_WT/specs/behavioral-contracts/ss-05/BC-5.41.001.md"
echo "# ARCH-INDEX" > "$ARTIFACTS_WT/specs/architecture/ARCH-INDEX.md"
echo "# ADR-026" > "$ARTIFACTS_WT/specs/architecture/decisions/ADR-026-wave-boundary-checkpoint-reset-and-lossless-intra-wave-compaction.md"

# Multi-table STORY-INDEX: E-0 (spaced Depends On) + E-18 (hyphenated Depends-On)
cat > "$ARTIFACTS_WT/stories/STORY-INDEX.md" << 'EOFI'
---
document_type: story-index
---
## Epic E-0 (7-col, spaced Depends On — leading table)
| Story ID | Title | Epic | Points | Priority | Depends On | Status |
|----------|-------|------|--------|----------|------------|--------|
| S-0.01 | infra | E-0 | 2 | P0 | -- | merged |
| S-0.02 | release | E-0 | 2 | P0 | S-0.01 | merged |
## Epic E-18 (9-col, hyphenated Depends-On)
| Story ID | Title | Epic | Points | Priority | Depends-On | Blocks | Status | BCs |
|----------|-------|------|--------|----------|-----------|--------|--------|-----|
| S-18.02 | Validate handoff | E-18 | 8 | P0 | [] | [S-18.03] | draft | [] |
| S-18.03 | Rehydrate wave | E-18 | 8 | P1 | [S-18.02] | [S-18.04a] | draft | [] |
| S-18.04a | Multi-dep diamond | E-18 | 5 | P1 | [S-18.02, S-18.03] | [] | draft | [] |
EOFI

for sid in S-18.02 S-18.03 S-18.04a; do
  cat > "$ARTIFACTS_WT/stories/${sid}-story-fixture.md" << STEOF
---
story_id: $sid
behavioral_contracts:
  - BC-placeholder
---
STEOF
done

# File order intentionally reversed (NOT dep order)
cat > "$WORK/sprint-state.yaml" << 'EOFI'
stories:
  - id: S-18.04a
    status: draft
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

echo "Running skill (file order: S-18.04a,S-18.02,S-18.03)..."
echo ""
bash "$SKILL" \
  --artifacts-worktree "$ARTIFACTS_WT" \
  --sprint-state "$WORK/sprint-state.yaml" \
  --state-md "$WORK/STATE.md" \
  --bc-dir "$ARTIFACTS_WT/specs/behavioral-contracts" 2>/dev/null

echo ""
echo "--- wave-state.yaml stories (COMMITTED blob, topo-sorted) ---"
git -C "$WORK" show factory-artifacts:wave-state.yaml | grep -E "^  - id:|^stories:"
echo ""
echo "Expected topo order: S-18.02 → S-18.03 → S-18.04a"
echo "=== DONE: BSD-portable topo-sort, correct E-18 table selected ==="

git -C "$WORK" worktree remove --force "$ARTIFACTS_WT" 2>/dev/null || true
rm -rf "$WORK"
