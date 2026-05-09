---
document_type: behavioral-contract
level: L3
version: "1.2"
status: draft
producer: product-owner
timestamp: 2026-05-06T00:00:00Z
phase: 1a
inputs:
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/F1-delta-analysis.md
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md
input-hash: "40a6fb6"
traces_to: .factory/cycles/v1.0-feature-engine-discipline-pass-1/F1-delta-analysis.md
origin: greenfield
subsystem: "SS-05"
capability: "CAP-005"
lifecycle_status: active
introduced: v1.0-feature-engine-discipline-pass-1
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-5.39.001
section: "5.39"
last_amended: 2026-05-07
---

# BC-5.39.001: Per-story adversarial convergence loop MUST achieve minimum 3 clean passes (NITPICK_ONLY) before demo recording

## Description

Every story delivered through the per-story-delivery Lobster workflow MUST complete a
minimum of 3 consecutive adversarial review passes that each classify as `NITPICK_ONLY`
before the demo recording step (Step 5) is executed. The convergence state is persisted
in a per-story JSON file at `.factory/cycles/<cycle-id>/<story-id>/adversary-convergence-state.json`.
The adversary agent (defined in `agents/adversary.md`) owns convergence assessment; no
separate convergence-checker agent is introduced. This gate is inserted between Step 4
(Implement) and Step 5 (Record demos) in the per-story-delivery workflow.

## Preconditions

1. A story has been dispatched through the per-story-delivery Lobster workflow.
2. Step 4 (Implement) has completed successfully and the story implementation is in the story worktree.
3. The cycle directory `.factory/cycles/<cycle-id>/` exists and is writable.
4. The story-scoped subdirectory `.factory/cycles/<cycle-id>/<story-id>/` has been created by the workflow.
5. The adversary agent (`agents/adversary.md`) is available and invocable as a subagent.
6. The story worktree diff against `develop` is available for adversary scope (BC-5.39.002 precondition).

## Postconditions

1. The adversary agent runs against the story scope (per BC-5.39.002 scope constraints) and
   records each pass result to `.factory/cycles/<cycle-id>/<story-id>/adversary-convergence-state.json`.
2. The JSON state file satisfies the following schema after each pass:
   ```json
   {
     "passes_clean": <int>,
     "last_finding_count": <int>,
     "last_classification": "CRITICAL" | "HIGH" | "MEDIUM" | "LOW" | "NITPICK_ONLY",
     "last_timestamp": "<ISO-8601 UTC>",
     "deferred_findings": [
       {
         "finding_id": "<string>",
         "category": "cross-story" | "integration" | "system-level" | "architectural",
         "target": "wave-gate" | "phase-5",
         "note": "<string>"
       }
     ]
   }
   ```
3. The `passes_clean` counter increments by 1 for each pass where `last_classification == "NITPICK_ONLY"`.
4. The `passes_clean` counter RESETS to 0 if any pass produces `last_classification != "NITPICK_ONLY"`.
5. The loop continues until `passes_clean >= 3 AND last_classification == "NITPICK_ONLY"`.
6. Step 5 (Record demos) MUST NOT execute while `passes_clean < 3` or `last_classification != "NITPICK_ONLY"`.
7. Deferred findings (categorized per BC-5.39.002) do NOT reset `passes_clean` and do NOT block convergence.
8. The adversary convergence loop is owned by the `agents/adversary.md` agent — no separate
   convergence-checker agent is created for this purpose.

## Invariants

1. `passes_clean` is a non-negative integer that only increases when `last_classification == "NITPICK_ONLY"`
   and resets to 0 otherwise. It never decreases without a reset.
2. The convergence criterion is strict binary: `passes_clean >= 3 AND last_classification == "NITPICK_ONLY"`.
   No partial credit, no exception for "almost clean" passes.
3. The state file path is `.factory/cycles/<cycle-id>/<story-id>/adversary-convergence-state.json`.
   The path is computed from runtime cycle-id and story-id — it is never hardcoded or guessed.
4. The `last_timestamp` field is always an ISO-8601 UTC timestamp representing when the
   most recent pass completed, not when the BC was authored.
5. The `deferred_findings` array is append-only within a single story's convergence run.
   Deferred findings are never removed retroactively.
6. The adversary agent operates with information asymmetry: it reads only the story worktree
   diff, the story spec, and anchored BCs — not the full conversation context of the implementer.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | First story in a new cycle — no state file exists | Adversary creates the file on first pass with `passes_clean: 0`. The missing file is not an error. |
| EC-002 | Adversary finds only NITPICK findings on first pass | `passes_clean` becomes 1. Loop continues until `passes_clean >= 3`. |
| EC-003 | Pass 3 produces a MEDIUM finding (regression) | `passes_clean` resets to 0. The loop restarts from 0. |
| EC-004 | All adversary findings are classified as cross-story/architectural | All findings deferred to `deferred_findings`; `last_classification` is `NITPICK_ONLY`; `passes_clean` increments. |
| EC-005 | Adversary agent is unavailable (timeout / crash) | The pass is not counted; `passes_clean` is not incremented; the story is not advanced. Escalate to orchestrator. |
| EC-006 | Story has zero anchored BCs | Adversary still runs — scope is the diff vs `develop`. A story with zero BCs may still have implementation findings. |
| EC-007 | `passes_clean` reaches exactly 3 but a new finding surfaces in pass 4 | Pass 4 resets `passes_clean` to 0. The criterion is 3 *consecutive* clean passes measured at the moment of gate evaluation. |

## Canonical Test Vectors

| Scenario | Pass Sequence | Final State | Decision |
|----------|--------------|-------------|----------|
| Clean convergence | NITPICK, NITPICK, NITPICK | `passes_clean: 3`, `last_classification: NITPICK_ONLY` | PASS — proceed to Step 5 |
| Regression on pass 2 | NITPICK, HIGH, NITPICK, NITPICK, NITPICK | `passes_clean: 3`, `last_classification: NITPICK_ONLY` | PASS (3 consecutive clean after reset) |
| Stuck convergence | CRITICAL, HIGH, MEDIUM, NITPICK, NITPICK | `passes_clean: 2`, `last_classification: NITPICK_ONLY` | BLOCK — only 2 clean passes |
| Deferred-only findings | defer(cross-story), NITPICK, NITPICK, NITPICK | `passes_clean: 3`, `deferred_findings: [...]` | PASS — deferred does not block |
| State file absent (first run) | (file created) NITPICK, NITPICK, NITPICK | `passes_clean: 3` | PASS |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-071 | Block Invariant — kani harness verifies HookResult::Block on non-converged input (canonical block_with_fix form per VP-071 v1.2) | kani |
| (static-check) | per-story-delivery.md Step 4.5 references BC-5.39.001 and BC-5.39.002 verbatim | grep-based static check |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-005 |
| Capability Anchor Justification | CAP-005 ("Run adversarial review with information asymmetry") per capabilities.md §CAP-005 — this BC formalizes the per-story adversary review gate that CAP-005 defines: the adversary agent uses a different model family and fresh context window (no prior conversation) to review story artifacts, and convergence (3 clean passes) is the quantifiable threshold that operationalizes "information asymmetry until clean." |
| L2 Domain Invariants | none |
| Architecture Module | plugins/vsdd-factory/workflows/phases/per-story-delivery.md (Step 4.5); agents/adversary.md; .factory/cycles/<cycle-id>/<story-id>/adversary-convergence-state.json |
| Stories | S-12.01, S-14.01 |
| FR | FR-047 (per-story adversarial convergence + artifact path discipline — to be added in PRD delta) |

## Related BCs

- BC-5.39.002 — sibling (defines adversary scope limits and deferred finding classification; this BC governs the loop mechanics)
- BC-4.10.001 — composes with (WASM hook that enforces the wave-gate block when stories lack convergence clearance)
- BC-4.10.002 — composes with (graceful degrade of the WASM hook outside wave-gate context)
- BC-5.36.005 — depends on (adversary partial-fix-regression check — applies within each pass of this loop)
- BC-5.36.006 — depends on (adversary fix propagation scope — applies within each pass)

## Architecture Anchors

- `plugins/vsdd-factory/workflows/phases/per-story-delivery.md` — Step 4.5 insertion point (between Step 4 Implement and Step 5 Record demos)
- `agents/adversary.md` — adversary agent definition; convergence assessment role
- `.factory/cycles/<cycle-id>/<story-id>/adversary-convergence-state.json` — per-story convergence state (schema defined in Postcondition 2)

## Story Anchor

Story A — v1.0-feature-engine-discipline-pass-1 (F3 story decomposition)

## VP Anchors

- VP-071 — adversary convergence gate enforcement

## Changelog

| Version | Date | Description |
|---------|------|-------------|
| 1.0 | 2026-05-06 | Initial authoring (product-owner; F2 phase of v1.0-feature-engine-discipline-pass-1). OQ3 resolution applied: per-story state file schema and convergence criterion codified. |
| 1.1 | 2026-05-07 | VP-071 traceability row amended (product-owner; F-P2-002, F2-amendment): stale "advisory-block output always emitted when per-story gate has not been cleared" description replaced with canonical "Block Invariant — kani harness verifies HookResult::Block on non-converged input (canonical block_with_fix form per VP-071 v1.2)". Proof method column corrected to "kani" (was "kani / adversarial review"). Fixes sibling-file regression of F-CRIT-3 — BC-5.39.001 was the second sibling BC missed in the B1 pass-1 fix. |
| 1.2 | 2026-05-09 | F-P45-001 — Traceability Stories row propagated from BC-INDEX v1.57: "Story A" placeholder → S-12.01, S-14.01. BC-INDEX was updated in fix-burst-39 (v1.55) for S-12.01 and fix-burst-40 (v1.56) for S-14.01; body was not updated in those bursts. Refs: F-P45-001, fix-burst-42. |
