---
document_type: domain-spec-index    # canonical value — NOT domain-spec-section
# Migration: accept document_type: domain-spec-section with section: "index"
# as legacy-acceptable. conform-to-template will normalize to domain-spec-index.
level: L2
version: "1.1"
status: draft
producer: business-analyst
timestamp: YYYY-MM-DDTHH:MM:SS
phase: 1a
inputs: [product-brief.md, research/RESEARCH-INDEX.md]
input-hash: "[md5]"
traces_to: product-brief.md
sections:                          # REQUIRED — enumerated list of shard files
  - capabilities.md                # Each entry is a section file in domain-spec/
  - entities.md                    # The business-analyst must populate this list
  - invariants.md                  # with ALL produced section files.
  - events.md
  - edge-cases.md
  - assumptions.md
  - risks.md
  - failure-modes.md
  - differentiators.md
  - event-flow.md
---

# L2 Domain Specification: [Product Name]

> **Sharded artifact (DF-021).** This index provides navigation and summary.
> Detail lives in per-section files listed below. Each section targets
> 800-1,200 tokens for optimal LLM consumption.

## Domain Summary

[1-2 sentences describing the product domain]

## Document Map

| Section | File | Tokens | Primary Consumer | Purpose |
|---------|------|--------|-----------------|---------|
| Domain Capabilities | capabilities.md | ~NNN | product-owner, architect, story-writer | CAP-NNN capability catalog |
| Domain Entities | entities.md | ~NNN | architect, product-owner, ux-designer | Entity model with attributes and invariants |
| Domain Invariants | invariants.md | ~NNN | product-owner, architect | DI-NNN business rules |
| Domain Events | events.md | ~NNN | architect | Event triggers, preconditions, outcomes |
| Edge Cases | edge-cases.md | ~NNN | story-writer, test-writer | DEC-NNN domain-level edge cases |
| Assumptions | assumptions.md | ~NNN | product-owner, test-writer | ASM-NNN with validation methods |
| Risks | risks.md | ~NNN | product-owner, architect | R-NNN risk register |
| Failure Modes | failure-modes.md | ~NNN | architect, test-writer | FM-NNN runtime failure catalog |
| Differentiators | differentiators.md | ~NNN | product-owner | Competitive differentiator → CAP-NNN mapping |
| Event Flow | event-flow.md | ~NNN | (human reference) | Optional causality chain narrative |

## Cross-References

| If you need... | Read these together |
|----------------|-------------------|
| BC creation input | capabilities.md + invariants.md + edge-cases.md + assumptions.md + risks.md + differentiators.md |
| Architecture design input | capabilities.md + entities.md + invariants.md + events.md + risks.md + failure-modes.md |
| Story decomposition input | capabilities.md + edge-cases.md |
| Holdout scenario generation | assumptions.md + risks.md + failure-modes.md |
| NFR derivation | risks.md + failure-modes.md |
| Full domain review (adversary/spec-reviewer) | ALL sections |

## ID Registry Summary

| ID Format | Count | Section |
|-----------|-------|---------|
| CAP-NNN | [n] | capabilities.md |
| DI-NNN | [n] | invariants.md |
| DEC-NNN | [n] | edge-cases.md |
| ASM-NNN | [n] | assumptions.md |
| R-NNN | [n] | risks.md |
| FM-NNN | [n] | failure-modes.md |

## Priority Distribution

| Priority | Count | Items |
|----------|-------|-------|
| P0 (must-have) | [n] | [CAP-NNN list] |
| P1 (should-have) | [n] | [CAP-NNN list] |
| P2 (nice-to-have) | [n] | [CAP-NNN list] |
