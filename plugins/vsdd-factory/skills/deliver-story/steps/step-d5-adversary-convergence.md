---
name: step-d5-adversary-convergence
description: Run the per-story adversary convergence loop to 3 clean NITPICK_ONLY passes before proceeding to demos. Writes convergence-state.json per BC-5.39.001.
---

# Step D.5: Per-Story Adversary Convergence

> **Shared context:** Read `./_shared-context.md` before executing this step — it contains the Iron Law, dispatch discipline, context discipline, model selection, and verification rules.

## Purpose

Ensures the story implementation is adversarially reviewed to convergence before demo recording begins. Blocks wave-gate dispatch (via the `validate-per-story-adversary-convergence` WASM hook) unless convergence state exists and satisfies `passes_clean >= 3 AND last_classification == "NITPICK_ONLY"`.

## Behavioral Contract Anchors

- BC-5.39.001 — convergence criterion, state file schema, loop procedure
- BC-5.39.002 — deferred-finding classification rules (cross-story, integration, system-level, architectural)
- ADR-017 — per-story adversary phasing rationale

## Convergence Criterion

`passes_clean >= 3 AND last_classification == "NITPICK_ONLY"` in
`.factory/cycles/<cycle-id>/<story-id>/adversary-convergence-state.json`.

The `passes_clean` counter increments by 1 per pass where `last_classification == "NITPICK_ONLY"`.
It RESETS to 0 if any pass produces a finding above NITPICK_ONLY. Minimum 3 clean passes — no exceptions.

## Dispatch Loop

**Step 1 — Adversary dispatch:**
Dispatch `adversary` agent (model tier: Capable) with context:
- Story worktree diff (`.worktrees/<STORY-ID>/`)
- Story spec (`.factory/stories/<STORY-ID>-*.md`)
- Anchored BCs listed in the story's `behavioral_contracts:` frontmatter field
- Current convergence state file (if it exists)

Task: "Review the story diff against the story spec and anchored BCs. Classify each finding as CRITICAL, HIGH, MEDIUM, LOW, or NITPICK_ONLY. Tag out-of-scope findings (cross-story, integration, system-level, architectural) as deferred per BC-5.39.002. Write updated convergence state JSON to `.factory/cycles/<cycle-id>/<story-id>/adversary-convergence-state.json`."

**Step 2 — State-manager backup:**
Dispatch `state-manager` to commit the updated state file to `factory-artifacts`.

**Step 3 — Check convergence:**
Read `.factory/cycles/<cycle-id>/<story-id>/adversary-convergence-state.json`.
- If `passes_clean < 3` OR `last_classification != "NITPICK_ONLY"`:
  dispatch `implementer` to fix within-story findings, then repeat from Step 1.
- If `passes_clean >= 3 AND last_classification == "NITPICK_ONLY"`: proceed to Step E.

## Deferred Findings (BC-5.39.002 PC4)

Out-of-scope findings are written to `deferred_findings[]` and do NOT block convergence.
They are surfaced at the wave-gate or Phase 5 adversary pass as appropriate.

Deferred categories:
- `cross-story` — requires context from another story → target: `wave-gate`
- `integration` — requires multi-story or subsystem context → target: `wave-gate`
- `system-level` — concerns system-wide behavior → target: `phase-5`
- `architectural` — concerns design decisions spanning the architectural boundary → target: `phase-5`

## State File Schema (BC-5.39.001 PC2)

Path: `.factory/cycles/<cycle-id>/<story-id>/adversary-convergence-state.json`

```json
{
  "passes_clean": 0,
  "last_finding_count": 3,
  "last_classification": "HIGH",
  "last_timestamp": "2026-05-07T00:00:00Z",
  "deferred_findings": []
}
```

## Exit Condition

`passes_clean >= 3 AND last_classification == "NITPICK_ONLY"` in the state file.

**Verify independently** — read the state file after the final adversary pass and confirm both fields before proceeding to Step E.

## Artifacts

- `.factory/cycles/<cycle-id>/<story-id>/adversary-convergence-state.json` — convergence state (committed to factory-artifacts by state-manager)
- Any within-story fix commits on the feature branch

## Note on Bootstrap Exemption (D-354)

The bootstrap cohort stories (S-12.01, S-12.02, S-13.01 in cycle v1.0-feature-engine-discipline-pass-1) were delivered before this gate existed and are exempt from the gate's blocking behavior for that cycle per D-354. For all stories in subsequent cycles, this step is mandatory and blocking.
