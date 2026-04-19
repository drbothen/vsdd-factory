---
name: step-b-broad-sweep
description: Run 7 broad analysis passes (Inventory, Architecture, Domain Model, Behavioral Contracts, NFRs, Conventions, Synthesis) over the reference codebase.
---

# Step A: Broad Sweep (Passes 0-6)

> **Shared context:** Read `./_shared-context.md` before executing this step — it contains the Iron Law, Red Flags, subagent delivery protocol, sandbox considerations, and file naming convention.

Run 7 sequential passes over the codebase. Each pass builds on prior pass outputs. Launch the `codebase-analyzer` agent for each pass.

**One agent per project — always.** Never combine multiple projects into a single agent.

## Passes

### Pass 0: Inventory
- File tree, dependency graph, tech stack
- File prioritization scoring (entry points → configs → core → API → tests → utils)
- Output: `.factory/semport/<project>/<project>-pass-0-inventory.md`

### Pass 1: Architecture
- Module boundaries, layers, component relationships
- Deployment topology, cross-cutting concerns
- Mermaid architecture and data flow diagrams
- Output: `.factory/semport/<project>/<project>-pass-1-architecture.md`

### Pass 2: Domain Model
- Two-sub-pass approach:
  - 2a: Structural (entities, relationships, value objects, enums)
  - 2b: Behavioral (operations, business rules, state machines, events)
- Output: `.factory/semport/<project>/<project>-pass-2-domain-model.md`

### Pass 3: Behavioral Contracts
- Extract from test files (first-class spec inputs), function signatures, validation logic
- Draft BCs with confidence levels (HIGH/MEDIUM/LOW)
- Output: `.factory/semport/<project>/<project>-pass-3-behavioral-contracts.md`

### Pass 4: NFR Extraction
- Performance, security, observability, reliability, scalability patterns
- Configuration values encoding NFR decisions
- Output: `.factory/semport/<project>/<project>-pass-4-nfr-catalog.md`

### Pass 5: Convention Catalog
- Naming, module organization, error handling, test patterns
- Design patterns with locations and consistency assessment
- Output: `.factory/semport/<project>/<project>-pass-5-conventions.md`

### Pass 6: Synthesis
- Cross-reference all passes for inconsistencies
- Unified knowledge doc with confidence assessment
- Gap report identifying orphaned modules and under-documented subsystems
- Output: `.factory/semport/<project>/<project>-pass-6-synthesis.md`

## Artifacts

7 pass files in `.factory/semport/<project>/`

## Commit

`factory(phase-0): brownfield ingest of <project>`

## Success Criteria

- All 7 pass files exist and are non-empty
- Pass 6 synthesis cross-references prior passes
- Gap report identifies areas needing convergence deepening
