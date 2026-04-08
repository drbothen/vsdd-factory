---
name: create-architecture
description: Create sharded architecture documents from PRD and behavioral contracts. Designs system architecture with ADR-style decisions, component diagrams, and purity boundaries. Writes to .factory/specs/architecture/.
disable-model-invocation: true
allowed-tools: Read, Write, Edit, Bash, AskUserQuestion
---

# Create Architecture

Design the system architecture based on the PRD and behavioral contracts. Architecture is sharded into numbered section files, not a single monolith.

## Templates

Read and follow the output format in:
- `.claude/templates/architecture-template.md` — full architecture document
- `.claude/templates/architecture-index-template.md` — ARCH-INDEX format
- `.claude/templates/architecture-section-template.md` — per-section ARCH-NN format
- `.claude/templates/L4-verification-property-template.md` — VP-NNN format

## Prerequisites

- `.factory/specs/prd.md` must exist.
- `.factory/specs/behavioral-contracts/` should have contracts defined.
- Read the product brief and domain spec if they exist.
- Check `.factory/specs/research/RESEARCH-INDEX.md` for general (technology) research. Read `general-*` reports — they contain library evaluations, architecture pattern research, and technical feasibility assessments that should inform architecture decisions.

### Reference Repos (conditional)

If `.factory/reference-manifest.yaml` exists, ingested codebases informed the requirements. When making architecture decisions:
- Read `.factory/semport/<project>/` architecture and convention passes to understand how reference repos solved similar problems.
- When adopting or deliberately diverging from a reference repo's approach, document it in the ADR: `Reference: <project> uses <approach>. We chose <alternative> because <rationale>.`
- Use `.reference/<project>/` to inspect actual implementations when the semport summary doesn't cover enough detail for a decision.

## Process

### 1. Identify Architecture Sections

Based on the subsystems in the PRD, determine which architecture sections are needed:

| File | Section | When Needed |
|------|---------|-------------|
| ARCH-00-overview.md | System overview, principles | Always |
| ARCH-01-core-services.md | Core service architecture | Always |
| ARCH-02-data-layer.md | Data models, storage | If persistent data |
| ARCH-03-api-layer.md | API design, endpoints | If has API surface |
| ARCH-04-agent-system.md | Agent architecture | If multi-agent |
| ARCH-05-workflow-engine.md | Workflow/pipeline design | If has workflows |
| ARCH-06-integration.md | External integrations | If has integrations |

Add or rename sections as needed. Ask the user to validate.

### 2. For Each Section, Make Decisions

Use ADR (Architecture Decision Record) style:

```markdown
### Decision: <title>

**Status:** accepted
**Context:** <why this decision is needed>
**Options considered:**
1. <option A> — <pros/cons>
2. <option B> — <pros/cons>
**Decision:** <chosen option>
**Rationale:** <why this option>
**Consequences:** <what this means for implementation>
```

### 3. Define Component Architecture

For each section:
- Components and their responsibilities (single responsibility)
- Interfaces between components (explicit contracts)
- Data flow (how data moves through the system)
- Dependency direction (strictly acyclic — SOUL.md #11)

### 4. Define Purity Boundaries

Identify which modules are pure (no side effects) vs impure (I/O, state):
- Pure core: domain logic, validation, transformation
- Impure shell: I/O, network, database, user interaction
- Boundary: where pure meets impure (this is where most bugs live)

### 5. Map BCs to Architecture

Trace each behavioral contract to the component(s) responsible for implementing it. This becomes the traceability link.

### 6. Define Verification Properties

Based on architecture decisions, create verification properties (VP-NNN) in `.factory/specs/verification-properties/`:
- Invariants that the architecture must maintain
- Safety properties (bad things that must not happen)
- Liveness properties (good things that must eventually happen)

## Output

Write sharded files to `.factory/specs/architecture/` following `spec-format.md` ARCH format.

Write ARCH-INDEX.md linking all sections.

Write VP files to `.factory/specs/verification-properties/` with VP-INDEX.md.

## After Writing

1. Commit all files to factory-artifacts.
2. Tell the user: "Architecture created with <N> sections and <N> verification properties. Recommended next: `/adversarial-review` to stress-test the full spec set before moving to stories."
