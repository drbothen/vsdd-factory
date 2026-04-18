<!-- Specification format standards. Defines how behavioral contracts, verification properties, and architecture docs are structured. -->

# Specification Format Standards

## Spec Hierarchy

| Level | Artifact | Format | Lifecycle |
|-------|----------|--------|-----------|
| **L1** | Product Brief | Markdown, mutable | Human input, can change |
| **L2** | Domain Specification | Markdown, living | Updated as domain understanding deepens |
| **L3** | Behavioral Contracts (BC) | Markdown, accumulating | New BCs per story, existing ones versioned |
| **L4** | Verification Properties (VP) | Markdown, **immutable once green** | Enforced by spec-steward hook |

## Behavioral Contracts (BC-S.SS.NNN)

### Numbering

- `S` = subsystem (01–99)
- `SS` = section within subsystem (01–99)
- `NNN` = contract number (001–999)
- Example: `BC-1.01.001` — Subsystem 1, Section 01, Contract 001

### File Format

```markdown
# BC-S.SS.NNN: <Title>

## Subsystem
<Subsystem name and identifier>

## Section
<Section name>

## Contract
<Clear, testable statement of behavior>

## Preconditions
- <What must be true before this behavior applies>

## Postconditions
- <What must be true after this behavior executes>

## Error Cases
- <What happens when preconditions are violated>

## Verification
- <How to test this contract — specific, not vague>

## Traceability
- PRD: <requirement ID or section>
- Stories: <STORY-NNN references>
- VPs: <VP-NNN references>
```

### Index Format (BC-INDEX.md)

```markdown
# Behavioral Contract Index

| ID | Title | Subsystem | Status | Stories |
|----|-------|-----------|--------|---------|
| BC-1.01.001 | ... | ... | draft/reviewed/green | STORY-001 |
```

## Verification Properties (VP-NNN)

### Numbering

Sequential: VP-001, VP-002, etc.

### File Format

```markdown
# VP-NNN: <Title>

## Property
<Formal statement of what must hold>

## Type
<invariant | precondition | postcondition | safety | liveness>

## Scope
<Which modules/subsystems this applies to>

## Verification Method
<unit-test | property-test | fuzzing | kani-proof | manual>

## Status
<draft | red | green>

## Traceability
- BCs: <BC references>
- Architecture: <ARCH section>
```

### Immutability Rule

Once a VP reaches `green` status, it **cannot be modified** without explicit approval. The spec-steward hook enforces this. To change a green VP, create a new VP that supersedes it and retire the old one.

## Architecture Documents (ARCH-NN)

### Sharding

Architecture is split into numbered sections, not a single monolith:

| File | Section |
|------|---------|
| `ARCH-INDEX.md` | Index with links to all sections |
| `ARCH-00-overview.md` | System overview, principles, constraints |
| `ARCH-01-core-services.md` | Core service architecture |
| `ARCH-02-data-layer.md` | Data models, storage, access patterns |
| `ARCH-03-api-layer.md` | API design, endpoints, contracts |
| `ARCH-04-agent-system.md` | Agent architecture, communication |
| `ARCH-05-workflow-engine.md` | Workflow state machine, execution |
| `ARCH-06-integration.md` | External integrations, plugins |

Sections are added as needed — not all are required for every project. Each section follows this structure:

```markdown
# ARCH-NN: <Section Title>

## Overview
<What this section covers>

## Decisions
<Architectural decisions with rationale (ADR-style)>

## Components
<Component descriptions, responsibilities, interfaces>

## Data Flow
<How data moves through this section>

## Constraints
<Technical constraints, performance budgets, security requirements>

## Dependencies
<What this section depends on, what depends on it>
```

## PRD Supplements

Supplementary documents live in `.factory/specs/prd-supplements/`:

| File | Purpose |
|------|---------|
| `interface-definitions.md` | API contracts, type definitions |
| `error-taxonomy.md` | Domain-specific error codes and handling |
| `data-models.md` | Entity definitions, relationships |
| `integration-points.md` | External service contracts |
| `module-criticality.md` | CRITICAL/HIGH/MEDIUM/LOW classification |

## Story Format (STORY-NNN)

```markdown
# STORY-NNN: <Title>

## Epic
<Epic reference>

## Description
<User story format: As a..., I want..., so that...>

## Acceptance Criteria
- [ ] <Testable criterion>

## Behavioral Contracts
- BC-S.SS.NNN

## Verification Properties
- VP-NNN

## Tasks
1. <Implementation task>

## Implementation Strategy
<from-scratch | gene-transfusion>

## Dependencies
- <STORY-NNN references>

## Wave
<Wave number for scheduling>
```

## BC Retirement Checklist

When retiring a BC (replacing with a new BC or removing scope), ALL of these artifacts must be updated in the SAME burst:

1. **BC-INDEX.md** — mark as `status: retired` with `replaces:` or `replaced_by:` reference
2. **STORY-INDEX.md** — update traceability matrix: remove old BC, add replacement BC
3. **Implementing story frontmatter** — update `behavioral_contracts:` array
4. **Implementing story AC prose** — rewrite acceptance criteria referencing the retired BC
5. **Replacement BC's Related BCs section** — add `replaces: BC-X.XX.XXX` with rationale

Partial propagation causes multi-pass rework. The adversary will find each missing update as a separate finding.
