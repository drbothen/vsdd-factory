---
document_type: domain-spec-index
level: L2
version: "1.0.7"
status: accepted
producer: business-analyst
timestamp: 2026-04-27T00:00:00
last_amended: 2026-06-15 (v1.0.7)
phase: 1.3
inputs:
  - .factory/phase-0-ingestion/pass-8-final-synthesis.md
  - .factory/phase-0-ingestion/pass-2-domain-model.md
  - .factory/phase-0-ingestion/pass-1-architecture.md
  - .factory/legacy-design-docs/2026-04-24-v1.0-factory-plugin-kit-design.md
  - .factory/specs/architecture/ARCH-INDEX.md
input-hash: "bda34a9"
traces_to: phase-1-spec-crystallization
sections:
  - core-domain-model.md
  - business-rules.md
  - capabilities.md
  - edge-cases.md
  - domain-events.md
  - invariants.md
  - glossary.md
---

# L2 Domain Specification: vsdd-factory

> **Sharded artifact (DF-021).** This is the Phase 1.3 brownfield spec backfill
> synthesis. It derives from 35 entities, 17 invariants, 22 events, and 3 state
> machines produced in Phase 0 ingestion (pass-2-domain-model.md), grounded in
> the v1.0 master design doc.

## Domain Summary

vsdd-factory is a self-orchestrating, observable, sandbox-aware development
pipeline for Claude Code users. It simultaneously ships a compiled Rust hook
dispatcher (Subsystem A) and a 119-skill orchestration framework (Subsystem B)
as a single Claude Code marketplace plugin. The domain covers: capability-gated
WASM plugin execution, multi-sink event observability, and declarative SDLC
workflow management driven by LLM agents.

## Document Map

| Section | File | Tokens | Primary Consumer | Purpose |
|---------|------|--------|-----------------|---------|
| Core Domain Model | core-domain-model.md | ~1,100 | architect, product-owner | 35 entities (22 Half A + 13 Half B), value objects, relationships, state machines |
| Business Rules | business-rules.md | ~900 | product-owner, architect, story-writer | 17 cross-cutting business rules from pass-2 §2b |
| Capabilities | capabilities.md v1.7 | ~1,200 | product-owner, architect, story-writer | CAP-001..CAP-032 user-facing capability catalog with SS-NN traceability |
| Edge Cases | edge-cases.md | ~900 | story-writer, test-writer | DEC-001..DEC-018 domain-level exception flows |
| Domain Events | domain-events.md | ~1,100 | architect, test-writer | DE-001..DE-022 event catalog with producer/consumer/schema |
| Invariants | invariants.md v1.19 | ~1,400 | product-owner, architect | DI-001..DI-025 domain invariants with SS-NN enforcement owner |
| Glossary | glossary.md | ~800 | all agents | 22 canonical domain terms for ubiquitous language |

## Cross-References

| If you need... | Read these together |
|----------------|-------------------|
| BC creation input (Phase 1.4) | capabilities.md + invariants.md + edge-cases.md |
| Architecture design input | core-domain-model.md + invariants.md + domain-events.md |
| Story decomposition input | capabilities.md + edge-cases.md |
| NFR derivation | invariants.md + edge-cases.md (from pass-4-nfr-catalog.md) |
| Full domain review (adversary/spec-reviewer) | ALL sections |
| Subsystem-to-capability mapping | capabilities.md (SS-NN column) + ARCH-INDEX.md |
| Event causality chain | domain-events.md + core-domain-model.md (§ State machines) |

## ID Registry Summary

| ID Format | Count | Section | Range |
|-----------|-------|---------|-------|
| CAP-NNN | 32 | capabilities.md | CAP-001..CAP-032 |
| DI-NNN | 25 | invariants.md | DI-001..DI-025 |
| DE-NNN | 22 | domain-events.md | DE-001..DE-022 |
| DEC-NNN | 18 | edge-cases.md | DEC-001..DEC-018 |

## Subsystem Cross-Walk

| SS-ID | Name | CAPs supported |
|-------|------|----------------|
| SS-01 | Hook Dispatcher Core | CAP-001, CAP-002, CAP-008, CAP-010, CAP-011 |
| SS-02 | Hook SDK and Plugin ABI | CAP-002, CAP-009 |
| SS-03 | Event Emission (OTel-Aligned) | CAP-003, CAP-010 |
| SS-04 | Plugin Ecosystem | CAP-002, CAP-008, CAP-013 |
| SS-05 | Pipeline Orchestration | CAP-001, CAP-004, CAP-005, CAP-006, CAP-012, CAP-014, CAP-016, CAP-018, CAP-032 |
| SS-06 | Skill Catalog | CAP-001, CAP-004, CAP-005, CAP-006, CAP-014, CAP-015, CAP-016, CAP-017, CAP-018, CAP-019, CAP-020, CAP-021, CAP-022, CAP-023, CAP-024, CAP-025, CAP-026, CAP-028, CAP-032 |
| SS-07 | Hook Bash Layer | CAP-008, CAP-013, CAP-027, CAP-032 |
| SS-08 | Templates and Rules | CAP-014, CAP-016, CAP-025 |
| SS-09 | Configuration and Activation | CAP-007, CAP-028 |
| SS-10 | CLI Tools and Bin | CAP-003, CAP-010, CAP-027 |

## Priority Distribution

| Priority | Count | CAPs |
|----------|-------|------|
| P0 (must-have) | 14 | CAP-001, CAP-002, CAP-003, CAP-007, CAP-008, CAP-009, CAP-010, CAP-013, CAP-014, CAP-016, CAP-028, CAP-029, CAP-031, CAP-032 |
| P1 (should-have) | 11 | CAP-004, CAP-005, CAP-006, CAP-011, CAP-012, CAP-015, CAP-017, CAP-018, CAP-019, CAP-020, CAP-030 |
| P2 (nice-to-have) | 7 | CAP-021, CAP-022, CAP-023, CAP-024, CAP-025, CAP-026, CAP-027 |

## Changelog

| Version | Date | Author | Notes |
|---------|------|--------|-------|
| 1.0.7 | 2026-06-15 | state-manager | D-583 F2 E-18 ADV PASS-20 NOT-CLEAN FIX BURST: (F-P20-001 MED) Document Map capabilities.md row CAP range corrected CAP-001..CAP-028→CAP-001..CAP-032 (stale by 4 capabilities; line-72 ID Registry Summary already correctly listed CAP-001..CAP-032 with count 32; Document Map description was the only stale site). L2-INDEX v1.0.6→v1.0.7. Refs: F-P20-001, D-583, E-18, issue-173. |
| 1.0.6 | 2026-06-15 | state-manager | D-581 F2 E-18 ADV PASS-18 NOT-CLEAN FIX BURST: capabilities.md bumped v1.6→v1.7 (F-P18-O1 LOW: §CHANGELOG reordered monotonic descending; all version rows v1.0–v1.6 confirmed present; no row content altered). L2-INDEX Document Map section updated to reference capabilities.md v1.7. |
| 1.0.5 | 2026-06-15 | state-manager | D-579 F2 E-18 ADV PASS-16 NOT-CLEAN FIX BURST: invariants.md bumped v1.18→v1.19 (F-P16-005 LOW: DI-022 lock-renewal made conditional — mandatory WHEN lock held; skipped no-op when absent; per BC-7.07.001 PC3/Inv3/EC-009 + ADR-025 opt-in). L2-INDEX Document Map section updated to reference invariants.md v1.19. |
| 1.0.4 | 2026-06-14 | business-analyst | F-14 fix (POLICY 2 gap — E-18 BCs TBD-DI): authored DI-020..DI-025 in invariants.md v1.12. ID Registry DI-NNN range updated DI-001..DI-017 → DI-001..DI-025. CAP-NNN count updated 28→32 (CAP-029..CAP-032 already existed in capabilities.md; index count corrected). Priority distribution corrected (P0/P1/P2 now matches capabilities.md content including CAP-029..CAP-032). Subsystem Cross-Walk: CAP-032 added to SS-05, SS-06, SS-07. invariants.md Document Map token estimate updated (~950→~1,400). |
| 1.0.3 | 2026-05-14 | architect | D-468 F-PASS13-001 closure: corrected citation D-350 → D-466 in v1.0.2 row (D-350 is F5-cycle S-13.01 merge decision; D-466 is brownfield E-10 pass-12 fix burst). |
| 1.0.2 | 2026-05-13 | architect | D-466 E-10 pass-12 fix burst F-3+F-6 closure (HH-4 regex-alternation discipline): SS-03 subsystem name `Observability Sinks` → `Event Emission (OTel-Aligned)` per POLICY 6 canonical-name SoT (ARCH-INDEX Subsystem Registry). |
| 1.0.1 | (prior) | business-analyst | Prior version. |
| 1.0 | (initial) | business-analyst | Initial L2 domain spec index. |
