---
document_type: holdout-scenario-index
level: ops
version: "1.0"
status: draft
producer: product-owner
timestamp: YYYY-MM-DDTHH:MM:SS
phase: 2
inputs: [behavioral-contracts/, prd.md]
traces_to: prd.md
---

# Holdout Scenario Index

> **One-per-file rule:** Each holdout scenario lives in its own HS-NNN.md file.
> HS-NNN IDs are lifecycle-scoped (append-only, never reused).
> Wave holdout scenarios (WHS-W[N]-NNN) are cycle-scoped and reset per cycle.

## Scenario Catalog

| HS ID | Title | Category | Priority | Source BCs | Status |
|-------|-------|----------|----------|-----------|--------|
| HS-001 | [title] | [category] | must-pass / should-pass | BC-S.SS.NNN | active / stale / retired |

## Category Distribution

| Category | Count |
|----------|-------|
| security | [N] |
| edge-case | [N] |
| integration | [N] |
| real-world-corpus | [N] |
| **Total** | **[N]** |

## Wave Holdout Scenarios (cycle-scoped)

| WHS ID | Title | Wave | Target Stories | Status |
|--------|-------|------|---------------|--------|
| WHS-W1-001 | [title] | Wave 1 | STORY-NNN | active |

Wave holdout scenarios reset per convergence cycle. They are NOT tracked
in the lifecycle HS-NNN sequence.
