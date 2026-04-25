---
document_type: domain-spec-index
level: L2
version: "1.0"
status: accepted
producer: business-analyst
timestamp: 2026-04-25T00:00:00
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
> synthesis. It derives from 35 entities, 18 invariants, 22 events, and 3 state
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
| Capabilities | capabilities.md | ~1,200 | product-owner, architect, story-writer | CAP-001..CAP-028 user-facing capability catalog with SS-NN traceability |
| Edge Cases | edge-cases.md | ~900 | story-writer, test-writer | DEC-001..DEC-018 domain-level exception flows |
| Domain Events | domain-events.md | ~1,100 | architect, test-writer | DE-001..DE-022 event catalog with producer/consumer/schema |
| Invariants | invariants.md | ~950 | product-owner, architect | DI-001..DI-018 domain invariants with SS-NN enforcement owner |
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
| CAP-NNN | 28 | capabilities.md | CAP-001..CAP-028 |
| DI-NNN | 18 | invariants.md | DI-001..DI-018 |
| DE-NNN | 22 | domain-events.md | DE-001..DE-022 |
| DEC-NNN | 18 | edge-cases.md | DEC-001..DEC-018 |

## Subsystem Cross-Walk

| SS-ID | Name | CAPs supported |
|-------|------|----------------|
| SS-01 | Hook Dispatcher Core | CAP-001, CAP-002, CAP-007, CAP-008, CAP-009, CAP-010, CAP-011 |
| SS-02 | Hook SDK and Plugin ABI | CAP-002, CAP-009 |
| SS-03 | Observability Sinks | CAP-003, CAP-010 |
| SS-04 | Plugin Ecosystem | CAP-002, CAP-008, CAP-013 |
| SS-05 | Pipeline Orchestration | CAP-001, CAP-004, CAP-005, CAP-006, CAP-012, CAP-014, CAP-016 |
| SS-06 | Skill Catalog | CAP-001, CAP-004, CAP-005, CAP-006, CAP-014, CAP-015, CAP-016, CAP-017, CAP-018, CAP-019, CAP-020, CAP-021, CAP-022, CAP-023, CAP-024, CAP-025, CAP-026 |
| SS-07 | Hook Bash Layer | CAP-008, CAP-013, CAP-027 |
| SS-08 | Templates and Rules | CAP-014, CAP-016, CAP-025 |
| SS-09 | Configuration and Activation | CAP-007, CAP-028 |
| SS-10 | CLI Tools and Bin | CAP-003, CAP-010, CAP-027 |

## Priority Distribution

| Priority | Count | CAPs |
|----------|-------|------|
| P0 (must-have) | 11 | CAP-001, CAP-002, CAP-003, CAP-007, CAP-008, CAP-009, CAP-010, CAP-013, CAP-014, CAP-016, CAP-028 |
| P1 (should-have) | 11 | CAP-004, CAP-005, CAP-006, CAP-011, CAP-012, CAP-015, CAP-017, CAP-018, CAP-019, CAP-020, CAP-021 |
| P2 (nice-to-have) | 6 | CAP-022, CAP-023, CAP-024, CAP-025, CAP-026, CAP-027 |
