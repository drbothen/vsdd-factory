---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-04-26T00:00:00
phase: 1a
inputs: [.factory/stories/S-7.03-tdd-discipline-hardening.md]
input-hash: ""
traces_to: .factory/stories/S-7.03-tdd-discipline-hardening.md
origin: brownfield
extracted_from: ".factory/stories/S-7.03-tdd-discipline-hardening.md#AC-011"
subsystem: "SS-08"
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
bc_id: BC-8.30.002
section: "8.30"
---

# BC-8.30.002: tdd_mode=facade modifies per-story-delivery semantics and mandates mutation testing at wave gate

## Description

When a story has `tdd_mode: facade`, the per-story-delivery workflow operates in facade mode: Step 2 may combine scaffold and implementation (no stub discipline violation), Step 3 writes spec-anchored fidelity tests post-hoc (rather than red tests against a stub), Step 4 may be a no-op if implementation is already complete, and mutation testing is REQUIRED at the wave gate as the compensating quality control. Facade mode exists specifically for DTU clone work (third-party API shape replication) and structural facade patterns where the "implementation" IS the scaffold.

## Preconditions

1. A story file has `tdd_mode: facade` in frontmatter (explicitly set; default does NOT activate facade mode).
2. The orchestrator has confirmed the story is a legitimate facade use case: DTU clone, external API shape replication, or structural facade with no novel business logic.
3. The wave gate skill is configured to process this story's crate.

## Postconditions

**Step 2 (facade-mode):**
1. Stub-architect (or implementer-as-stub-architect) may write full implementations in the "stub" commit — no `todo!()` obligation.
2. Stub commit message must include `[facade-mode: tdd_mode=facade]` tag.

**Step 3 (facade-mode):**
1. Test-writer writes spec-anchored fidelity tests that verify the scaffold matches its behavioral contract (shape, error codes, field names).
2. Tests may be GREEN immediately (no red phase required).
3. Red Gate density check (BC-8.29.001) is BYPASSED for facade-mode stories.

**Step 4 (facade-mode):**
1. May be a no-op if Step 2 produced a complete implementation.
2. Implementer reviews fidelity tests and confirms correctness.

**Wave gate:**
1. `cargo mutants -p <crate> --jobs N --timeout 300` is executed for ALL stories with `tdd_mode: facade` in the wave (BC-6.21.001).
2. Kill rate must be ≥ 80% (BC-6.21.002).
3. Failure to meet kill rate blocks wave gate merge.

## Invariants

1. `tdd_mode: facade` does NOT exempt a story from mutation testing — it changes WHEN mutation testing runs (wave gate) and WHAT it tests (the complete implementation).
2. The facade mode bypass of Red Gate density check (BC-8.29.001) is ONLY valid because mutation testing is the compensating control. If mutation testing is skipped for any reason, the bypass retroactively violates this BC.
3. Facade-mode test-writer must anchor every test to a behavioral contract (BC-NNN) — "fidelity tests" are not a license for unanchored shape-checking.
4. Facade mode is appropriate for: DTU API clones, mock server implementations, structural fakes, config parsing wrappers. It is NOT appropriate for: algorithmic business logic, domain rule implementations, data transformation pipelines.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Story is a DTU clone with `tdd_mode: strict` (misconfigured) | Pipeline applies strict mode. DTU clone will fail Red Gate because the stub has full impl. Orchestrator must correct `tdd_mode` to `facade` or roll back per BC-8.29.003. |
| EC-002 | Mutation testing at wave gate reveals kill rate = 60% (below 80%) | Wave gate BLOCKS. Surviving mutants must be addressed per BC-6.21.002. |
| EC-003 | Wave contains 4 facade stories and 2 strict stories | Mutation testing runs for the 4 facade crates. Strict stories' Red Gate density already validated at Step 3. |
| EC-004 | tdd_mode: facade story contains algorithmic business logic mixed with facade code | The facade designation is suspect. Adversary must flag this. Mutation testing at wave gate is still required. |

## Canonical Test Vectors

| Story Type | tdd_mode | Step 2 | Step 3 | Step 4 | Wave Gate |
|------------|----------|--------|--------|--------|-----------|
| Jira DTU clone | facade | full impl in commit | fidelity tests (GREEN ok) | no-op | cargo mutants ≥80% kill |
| Algorithmic rule engine | strict | todo!() only | red tests | implementer fills | (n/a unless Option B chosen) |
| PagerDuty mock server | facade | full impl in commit | fidelity tests (GREEN ok) | no-op | cargo mutants ≥80% kill |
| Auth middleware | strict | todo!() only | red tests | implementer fills | (n/a) |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-064 | facade-mode mutation gate is enforced at wave gate for all facade stories | manual (procedural verification) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-016 |
| Capability Anchor Justification | CAP-016 ("Drive TDD delivery with red/green/refactor gate enforcement") per capabilities.md §CAP-016 — this BC specifies how CAP-016's gate enforcement adapts for structural facade work where the traditional red/green/refactor cycle is inappropriate, while preserving the quality assurance intent through mutation testing. |
| L2 Domain Invariants | none |
| Architecture Module | plugins/vsdd-factory/workflows/phases/per-story-delivery.md, plugins/vsdd-factory/skills/wave-gate/SKILL.md |
| Stories | S-7.03 |
| Source AC | S-7.03 §AC-011 |
| FR | FR-043 |

## Related BCs

- BC-8.30.001 — parent (tdd_mode field definition)
- BC-8.29.001 — supersedes for facade-mode stories (Red Gate density bypass)
- BC-6.21.001 — depends on (mutation testing execution at wave gate)
- BC-6.21.002 — depends on (80% kill rate floor)

## Architecture Anchors

- `plugins/vsdd-factory/workflows/phases/per-story-delivery.md` — facade-mode step semantics
- `plugins/vsdd-factory/skills/wave-gate/SKILL.md` — mutation testing execution

## Story Anchor

S-7.03

## VP Anchors

- VP-064 — facade-mode mutation gate enforcement
