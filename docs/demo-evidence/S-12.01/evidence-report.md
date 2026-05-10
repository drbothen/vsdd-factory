---
story_id: S-12.01
title: Per-Story Adversary Convergence — Step 4.5, Lobster Wiring, Scope Contract
recorded_by: demo-recorder
recorded_at: 2026-05-07
bcs:
  - BC-5.39.001
  - BC-5.39.002
status: complete
---

# Demo Evidence Report — S-12.01

## Summary

9 demos recorded (8 per-AC + 1 cross-cutting bats suite).
All 8 acceptance criteria have coverage. The bats suite recording confirms
31/31 tests green across all ACs simultaneously. No demos skipped.

## AC Coverage Table

| AC ID | Demo File | Description | Status | BC Trace |
|-------|-----------|-------------|--------|----------|
| AC-001 | [AC-001-step-4.5-inserted.gif](AC-001-step-4.5-inserted.gif) | per-story-delivery.md showing Step 4.5 inserted between Step 4 and Step 5 | PASS | BC-5.39.001 PC1 |
| AC-002 | [AC-002-convergence-schema.gif](AC-002-convergence-schema.gif) | Step 4.5 convergence criterion (passes_clean >= 3, NITPICK_ONLY) and adversary-convergence-state.json schema | PASS | BC-5.39.001 PC2, PC5 |
| AC-003 | [AC-003-three-perimeter-contract.gif](AC-003-three-perimeter-contract.gif) | adversary.md Three-Perimeter Scope Contract section with ADR-017 citation | PASS | BC-5.39.002 PC1 |
| AC-004 | [AC-004-deferred-categories.gif](AC-004-deferred-categories.gif) | adversary.md showing all 4 deferred-finding categories (cross-story, integration, system-level, architectural) | PASS | BC-5.39.002 PC2, INV2 |
| AC-005 | [AC-005-lobster-steps.gif](AC-005-lobster-steps.gif) | phase-3-tdd-implementation.lobster showing adversary-convergence + backup-adversary-convergence steps (18 steps total) | PASS | BC-5.39.001 PC1 |
| AC-006 | [AC-006-wave-gate-narrowed.gif](AC-006-wave-gate-narrowed.gif) | wave-gate SKILL.md Gate 3 narrowed to cross-story/integration scope with deferred_findings input | PASS | BC-5.39.002 PC1, PC7 |
| AC-007 | [AC-007-mandatory-steps-reconciled.gif](AC-007-mandatory-steps-reconciled.gif) | orchestrator.md MANDATORY STEPS referencing Step 4.5 per-story adversary convergence | PASS | BC-5.39.001 |
| AC-008 | [AC-008-orch-shim-updated.gif](AC-008-orch-shim-updated.gif) | orchestrator/per-story-delivery.md parallel Step 4.5 (c2) adversary convergence loop reference | PASS | BC-5.39.001 |
| cross-cutting | [bats-suite-green.gif](bats-suite-green.gif) | bats per-story-adversary-workflow.bats — 31/31 ok covering all ACs | PASS | BC-5.39.001, BC-5.39.002 |

## Artifact Inventory

| File | Type | Size |
|------|------|------|
| AC-001-step-4.5-inserted.tape | VHS script | 937 B |
| AC-001-step-4.5-inserted.gif | Recording | 118 KB |
| AC-002-convergence-schema.tape | VHS script | 968 B |
| AC-002-convergence-schema.gif | Recording | 228 KB |
| AC-003-three-perimeter-contract.tape | VHS script | 951 B |
| AC-003-three-perimeter-contract.gif | Recording | 194 KB |
| AC-004-deferred-categories.tape | VHS script | 987 B |
| AC-004-deferred-categories.gif | Recording | 195 KB |
| AC-005-lobster-steps.tape | VHS script | 991 B |
| AC-005-lobster-steps.gif | Recording | 239 KB |
| AC-006-wave-gate-narrowed.tape | VHS script | 975 B |
| AC-006-wave-gate-narrowed.gif | Recording | 505 KB |
| AC-007-mandatory-steps-reconciled.tape | VHS script | 987 B |
| AC-007-mandatory-steps-reconciled.gif | Recording | 103 KB |
| AC-008-orch-shim-updated.tape | VHS script | 987 B |
| AC-008-orch-shim-updated.gif | Recording | 139 KB |
| bats-suite-green.tape | VHS script | 693 B |
| bats-suite-green.gif | Recording | 1.8 MB |

Total directory: ~4.0 MB

## Behavioral Contract Traceability

### BC-5.39.001 — Per-Story Adversary Convergence Loop Mechanics

- AC-001: Step 4.5 appears between Step 4 and Step 5 in per-story-delivery.md
- AC-002: Convergence criterion (`passes_clean >= 3 AND last_classification == "NITPICK_ONLY"`) and state JSON schema present
- AC-005: Lobster workflow wires adversary-convergence step after backup-implement; record-demos depends on backup-adversary-convergence
- AC-007: orchestrator.md MANDATORY STEPS references Step 4.5 with 3-clean-pass minimum
- AC-008: orchestrator/per-story-delivery.md step c2 documents adversary convergence loop

### BC-5.39.002 — Three-Perimeter Scope Constraints

- AC-003: adversary.md Three-Perimeter Scope Contract section present, cites ADR-017 and BC-5.39.002
- AC-004: adversary.md documents all 4 deferred categories with routing targets (wave-gate / phase-5)
- AC-006: wave-gate SKILL.md Gate 3 narrowed to cross-story and integration concerns only; reads deferred_findings from per-story convergence state files

## Skipped Demos

None. All 8 ACs have coverage.

## Notes

- VHS 0.10.0 used; only `.gif` output (not `.webm`) — VHS 0.10 single-output constraint noted in story context
- Font: FiraCode Nerd Font Mono (installed at /Users/jmagady/Library/Fonts/)
- All demos show grep/awk read-only queries against the story worktree — no production code modified
- bats-suite-green recording used 12s Sleep to allow bats to complete 31 tests before hold frame
