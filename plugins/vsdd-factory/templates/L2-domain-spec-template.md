<!-- DEPRECATED: This monolithic template has been replaced by the sharded
     L2-domain-spec-index-template.md + L2-domain-spec-section-template.md.
     Use the sharded templates for new projects. Retained for reference. -->
---
document_type: domain-spec
level: L2
version: "1.0"
status: draft
producer: business-analyst
timestamp: YYYY-MM-DDTHH:MM:SS
phase: 1a
inputs: [product-brief.md, research/RESEARCH-INDEX.md]
traces_to: product-brief.md
---

# Domain Specification: [Product Name]

> L2 Domain Spec — defines domain capabilities, entities, invariants, and
> constraints independent of implementation architecture. This document feeds
> into the L3 Behavioral Contracts (PRD) and Architecture.

## 1. Domain Capabilities

| ID | Capability | Description | Business Rule | Priority |
|----|-----------|-------------|---------------|----------|
| CAP-001 | | | | P0/P1/P2 |

> For pipeline-oriented projects (CLI tools, data processors), capabilities
> are processing stages: "Parse input", "Transform data", "Validate output".

## 2. Domain Entities

| Entity | Description | Key Attributes | Invariants |
|--------|-------------|---------------|------------|
| | | | |

> For pipeline-oriented projects, entities are key data structures flowing
> through the pipeline (e.g., LinkRecord, ValidationResult).

## 3. Domain Invariants (Business Rules)

| ID | Rule | Scope | Violation Behavior |
|----|------|-------|--------------------|
| DI-001 | | | |

> For pipeline-oriented projects, invariants are pipeline constraints
> (e.g., "URL extraction preserves source position").

## 4. Domain Events / Processing Stages

| Event/Stage | Trigger | Preconditions | Outcomes |
|-------------|---------|---------------|----------|
| | | | |

> For pipeline-oriented projects, this describes the processing pipeline
> as numbered stages with inputs and outputs.

## 5. Domain-Level Edge Cases

| ID | Capability | Edge Case | Expected Behavior |
|----|-----------|-----------|-------------------|
| DEC-001 | CAP-NNN | | |

## 6. Assumptions Requiring Validation

| ID | Assumption | Confidence | Validation Method | Impact if Wrong | Status | Traced To |
|----|------------|------------|-------------------|-----------------|--------|-----------|
| ASM-001 | | High/Medium/Low | | | unvalidated | |

> **Status values:** `unvalidated` | `validated` | `invalidated`
> **Traced To:** artifact IDs that consume or validate this assumption (HS-NNN, BC-S.SS.NNN, STORY-NNN, R-NNN)

> Every assumption is a risk. High-confidence assumptions still need validation
> methods documented. If impact-if-wrong is HIGH, consider adding to Risk Register.

## 7. Risk Register

| ID | Risk | Likelihood | Impact | Mitigation | Status | Category | Traced To |
|----|------|-----------|--------|-----------|--------|----------|-----------|
| R-001 | | High/Medium/Low | High/Medium/Low | | open | | |

> **Status values:** `open` | `mitigated` | `accepted` | `closed`
> **Category values:** `security` | `performance` | `reliability` | `business`
> **Traced To:** artifact IDs that address this risk (NFR-NNN, HS-NNN, architecture section, SEC-NNN)

## 8. Failure Modes

| ID | Subsystem | Failure Mode | Impact | Detection | Recovery |
|----|-----------|-------------|--------|-----------|----------|
| FM-001 | | | | | |

> Distinct from edge cases. Edge cases = unusual input. Failure modes = what
> breaks at runtime (DNS failure, TLS errors, disk full, OOM).

## 9. Competitive Differentiator Traceability

| Differentiator | Why It Matters | Supporting Capabilities |
|----------------|---------------|----------------------|
| | | CAP-NNN, CAP-NNN |

> Maps each product differentiator to the domain capabilities that deliver it.
> If a differentiator has no capability backing it, it's marketing not engineering.
> **PRD Seeding Note:** This section seeds PRD Section 6 (Competitive Differentiator
> Traceability). The product-owner must map each differentiator here to BC-NNN contracts
> in the PRD, ensuring every claimed differentiator has verifiable behavioral backing.

## 10. Domain Event Flow (Optional)

> **Human-reference only.** This section provides context for human readers but is
> not machine-consumed by downstream agents. Agents should use the structured tables
> in Sections 1-9 for their inputs.

> For domain-rich projects with complex temporal flows, describe causality chains
> referencing CAP-NNN IDs. Not required for pipeline-oriented projects.
