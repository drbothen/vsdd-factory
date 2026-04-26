---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-04-26T00:00:00
phase: 1a
inputs: [.factory/stories/S-7.03-tdd-discipline-hardening.md]
input-hash: "a3187d9"
traces_to: .factory/stories/S-7.03-tdd-discipline-hardening.md
origin: brownfield
extracted_from: ".factory/stories/S-7.03-tdd-discipline-hardening.md#AC-006"
subsystem: "SS-05"
capability: "CAP-016"
lifecycle_status: active
introduced: v1.0-brownfield-backfill
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-8.29.003
section: "8.29"
---

# BC-8.29.003: on RED_RATIO < 0.5 without GREEN-BY-DESIGN justification, orchestrator must choose remediation option A or B

## Description

When the Red Gate density check (BC-8.29.001) blocks Step 4 and the Red Gate log (BC-8.29.002) contains UNJUSTIFIED GREEN tests, the orchestrator must take one of exactly two remediation actions before any implementation work proceeds. This is not an advisory — it is a required decision gate. The options are: (A) roll back the stub commit and re-dispatch stub-architect with a stricter prompt; or (B) accept the pattern with mandatory mutation testing at wave gate plus PR description disclosure. No other path forward is permissible.

## Preconditions

1. RED_RATIO < 0.5 at Step 3 Red Gate.
2. Red Gate log (BC-8.29.002) contains at least one UNJUSTIFIED GREEN test entry.
3. `tdd_mode: strict` (or absent) — facade stories have a different path (BC-8.30.002).
4. Orchestrator has received the blocking gate report.

## Postconditions

**Option A chosen (rollback + re-dispatch):**
1. The stub commit is reverted (git revert or force-reset to pre-stub state).
2. Stub-architect is re-dispatched with: (a) reinforced anti-precedent guard (BC-5.38.004), (b) explicit list of UNJUSTIFIED functions from the log, (c) instruction to replace those functions with `todo!()`.
3. Step 3 runs again with the corrected stub.
4. RED_RATIO is recomputed. If still < 0.5, Option A or Option B applies again.

**Option B chosen (accept with mutation obligation):**
1. The story proceeds to Step 4 implementation with the current stub.
2. A mutation testing obligation is registered: `mutation_testing_required: true` in the story frontmatter.
3. The wave gate must run `cargo mutants -p <crate> --jobs N --timeout 300` for this story's crate (BC-6.21.001).
4. The PR description must disclose: "RED_RATIO was <value> at Step 3 Red Gate for story <S-NNN>. Mutation testing applied at wave gate as compensating control."

## Invariants

1. The only two remediation options are A and B. No other path (e.g., "proceed anyway without mutation", "defer to next wave") is permissible.
2. The orchestrator's choice must be documented in the Red Gate log alongside the blocking event.
3. Option B does not waive the mutation testing — it changes WHEN it occurs (wave gate vs. step gate) but not WHETHER it occurs.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Orchestrator is an automated agent (no human available) | Automated orchestrator MUST default to Option A unless story frontmatter explicitly declares `mutation_testing_required: true` as pre-authorized. |
| EC-002 | Option A rollback produces a stub that also fails RED_RATIO < 0.5 after re-dispatch | Apply BC-8.29.003 again. If third attempt still fails, escalate to human with full log chain. |
| EC-003 | All UNJUSTIFIED tests are later determined to be testing infrastructure (not story logic) | Reclassify them in the log and recompute RED_RATIO. If new ratio ≥ 0.5, gate passes without remediation. |

## Canonical Test Vectors

| Scenario | Required Action | Category |
|----------|-----------------|----------|
| RED_RATIO = 0.3, 2 UNJUSTIFIED GREEN tests, tdd_mode: strict | Orchestrator chooses A or B before Step 4 | blocking gate scenario |
| RED_RATIO = 0.3, all GREEN tests are GREEN-BY-DESIGN, no UNJUSTIFIED | Re-compute: if UNJUSTIFIED = 0, gate becomes a full-exemption pass per BC-8.29.001 | exemption path |
| RED_RATIO = 0.3, tdd_mode: facade | BC-8.30.002 applies; this BC does not block | facade bypass |
| Option B chosen; PR description omits RED_RATIO disclosure | VIOLATION of this BC's Option B postconditions | negative |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-063 | Remediation option selection is correctly branched on RED_RATIO and UNJUSTIFIED count | integration (bats) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-016 |
| Capability Anchor Justification | CAP-016 ("Drive TDD delivery with red/green/refactor gate enforcement") per capabilities.md §CAP-016 — this BC closes the loop on CAP-016's gate enforcement by ensuring that when the gate fires, the pipeline has a defined and mandatory decision protocol rather than an advisory that can be silently ignored. |
| L2 Domain Invariants | none |
| Architecture Module | plugins/vsdd-factory/workflows/phases/per-story-delivery.md (SS-05 territory) |
| Stories | S-7.03 |
| Source AC | S-7.03 §AC-006, AC-007 |
| FR | FR-043 |

## Historical Evidence

The absence of this remediation gate in Prism Wave 2 meant that even if RED_RATIO < 0.5 had been detected, there was no specified response — the pipeline would have continued. Both options are formalized here to prevent silent bypass.

## Related BCs

- BC-8.29.001 — parent (the gate that triggers this decision protocol)
- BC-8.29.002 — sibling (the log that informs the remediation decision)
- BC-6.21.001 — depends on (mutation testing executed under Option B)
- BC-8.30.002 — supersedes for facade-mode stories

## Architecture Anchors

- `plugins/vsdd-factory/workflows/phases/per-story-delivery.md` — remediation decision protocol after Step 3 gate

## Story Anchor

S-7.03

## VP Anchors

- VP-063 — RED_RATIO decision logic

## Notes

**Advisory (non-blocking):** Repeated Option B choices (>2 consecutive stories in the same wave) SHOULD trigger human escalation with root cause analysis. This is process guidance, not a machine-verifiable invariant. A future story may add cross-wave Option B tracking via sprint-state.yaml augmentation.

**Subsystem Historical Artifact:** The BC-ID prefix `8.29` embeds the original subsystem assignment of SS-08. After adversarial pass-1, this BC was authoritatively re-anchored to **SS-05 (Pipeline Orchestration)** because its Architecture Module (`per-story-delivery.md`) governs workflow-phase behavior in SS-05 territory. The `subsystem: SS-05` frontmatter is authoritative; the BC-ID prefix is a historical artifact. Per append_only_numbering policy, the ID is preserved rather than renumbered.
