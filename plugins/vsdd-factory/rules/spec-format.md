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

- `S` = PRD section / top-level subsystem grouping (1–9)
- `SS` = subsection within the section (matches an L2 subsystem; 01–99)
- `NNN` = contract number (001–999)
- Example: `BC-1.01.001` — Section 1, Subsection 01, Contract 001

### Sharded Layout

BCs are organized into per-subsystem shard directories under
`behavioral-contracts/`, one shard per subsystem registered in ARCH-INDEX:

```
.factory/specs/behavioral-contracts/
├── BC-INDEX.md
├── ss-01/
│   ├── BC-1.01.001.md
│   ├── BC-1.01.002.md
│   └── ...
├── ss-02/
│   ├── BC-2.02.001.md
│   └── ...
└── ss-NN/
    └── ...
```

The shard directory name is the **bare `SS-NN` identifier** (lowercased,
e.g., `ss-01/`) — NOT the descriptive subsystem name (NOT `ss-01-hook-dispatcher/`).
Subsystem descriptive names live authoritatively in ARCH-INDEX Subsystem
Registry and in `architecture/SS-NN-<name>.md` section files.

The sharded layout scales to projects with hundreds or thousands of BCs
without making the directory unbrowsable. Small projects (under ~50 BCs)
may keep a flat layout, but the canonical convention is sharded.

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
- Stories: <S-N.MM references>
- VPs: <VP-NNN references>
```

### Index Format (BC-INDEX.md)

```markdown
# Behavioral Contract Index

| ID | Title | Subsystem | Status | Stories |
|----|-------|-----------|--------|---------|
| BC-1.01.001 | ... | ... | draft/reviewed/green | S-1.01 |
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

## Story Format (S-N.MM)

### ID Format

- **Stories** use `S-N.MM` (section.story, zero-padded):
  - `N` = section / epic-grouping (single digit, matches the parent epic `E-N`)
  - `MM` = zero-padded story number within that section (e.g., `01`, `15`)
  - Examples: `S-1.01`, `S-3.15`, `S-0.02`
  - Filename: `S-N.MM-<short-description>.md` (e.g., `S-1.01-foundational-types.md`)
- **Epics** use `E-N` (single digit). Epic `E-N` directly contains all `S-N.*`
  stories. Filename: `E-N-<short-description>.md` under `.factory/stories/epics/`.
- **Story `N` and BC `S` are different hierarchies.** BC `S` (in
  BC-S.SS.NNN) is the subsystem number; story `N` (in S-N.MM) is the
  epic/release-milestone grouping. A single story can implement BCs from
  multiple subsystems via the `subsystems: [SS-NN, ...]` frontmatter array.

```markdown
# S-N.MM: <Title>

## Epic
<E-N reference>

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
- <S-N.MM references>

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
