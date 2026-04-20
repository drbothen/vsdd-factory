---
name: create-architecture
description: Create sharded architecture documents from PRD and behavioral contracts. Designs system architecture with ADR-style decisions, component diagrams, and purity boundaries. Writes to .factory/specs/architecture/.
disable-model-invocation: true
allowed-tools: Read, Write, Edit, Bash, AskUserQuestion
---

## The Iron Law

> **NO ARCHITECTURE WITHOUT VERIFICATION FEASIBILITY ASSESSMENT**

Every architecture decision must include a verification feasibility assessment: can the chosen approach be proven correct? Where are the purity boundaries? What is provable vs. testable-only? Do NOT skip to story decomposition or implementation — purity boundaries MUST be drawn and verification properties MUST be defined before proceeding. Architecture decisions are the most expensive to reverse.

## Red Flags

| Thought | Reality |
|---|---|
| "The architecture is obvious from the requirements" | Obvious architectures still need purity boundaries and VP assignments. Document them. |
| "Verification properties can be defined later during implementation" | VPs inform architecture. A design that can't be verified must be redesigned NOW, not after implementation. |
| "This module doesn't need a purity classification" | Every module must be classified (pure core, boundary, infrastructure). No exceptions. |
| "The ADR rationale is clear from context" | ADR rationale must be explicit. Future readers don't have your context. |
| "Let me skip the feasibility report — the architecture is straightforward" | The feasibility report catches verification gaps before they become implementation problems. |
| "One big architecture document is easier to write" | Architecture is sharded into ARCH-NN section files. Monoliths drift and overwhelm context. |
| "I can use the reference repo's architecture directly" | Reference architectures inform decisions but divergences must be documented in ADRs with rationale. |
| "Module criticality can default to MEDIUM everywhere" | Default MEDIUM masks CRITICAL modules (auth, crypto, state machines). Classify deliberately. |

# Create Architecture

Design the system architecture based on the PRD and behavioral contracts. Architecture is sharded into numbered section files, not a single monolith.

## Templates

Read and follow the output format in:
- `${CLAUDE_PLUGIN_ROOT}/templates/architecture-template.md` — full architecture document
- `${CLAUDE_PLUGIN_ROOT}/templates/architecture-index-template.md` — ARCH-INDEX format
- `${CLAUDE_PLUGIN_ROOT}/templates/architecture-section-template.md` — per-section ARCH-NN format
- `${CLAUDE_PLUGIN_ROOT}/templates/L4-verification-property-template.md` — VP-NNN format

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

## Visual Tooling

When visual content would help the human understand architecture options, use the best available tool. No hard dependency on any single tool.

| Tier | Tool | Check | Best for |
|------|------|-------|----------|
| 1 | `/vsdd-factory:visual-companion` | Node.js available, user accepts | Interactive component diagrams, side-by-side architecture comparisons |
| 1 | `/vsdd-factory:visual-companion` (excalidraw) | Setup completed | Architecture diagrams, dependency graphs, interactive editing |
| 2 | `/vsdd-factory:create-excalidraw` | Always available | Generate .excalidraw files for offline viewing in excalidraw.com or VS Code |
| 3 | Mermaid code blocks | Always available | Sequence diagrams, component diagrams, state machines |
| 4 | ASCII/text | Always available | Simple dependency trees, layer diagrams |

Before using Tier 1, ask the human once:
> "I can show architecture diagrams in a browser for this. Want to try it? (Requires Node.js and opening a local URL)"

If they decline or Node.js isn't available, fall back to the next tier.

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

## Self-Review (before adversarial review)

Before routing to adversarial review, check your own work:

1. **Placeholder scan:** Any "TBD", "TODO", incomplete sections, or vague decisions? Fix them now.
2. **Internal consistency:** Do ARCH section cross-references resolve? Do VP traces point to real BCs? Is the dependency graph acyclic?
3. **Scope check:** Does every module have a clear single responsibility? Are purity boundaries drawn for all modules?
4. **Ambiguity check:** Could any architecture decision be interpreted two different ways? Make the chosen option and rationale explicit.

Fix issues inline. This is a cheap filter — catch obvious gaps before spending tokens on the adversary.

## After Writing

1. Commit all files to factory-artifacts.
2. Tell the user: "Architecture created with <N> sections and <N> verification properties. Recommended next: `/adversarial-review` to stress-test the full spec set before moving to stories."
