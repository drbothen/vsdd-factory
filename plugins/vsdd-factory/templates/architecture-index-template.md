---
document_type: architecture-index
level: L3
version: "1.1"
status: draft
producer: architect
timestamp: YYYY-MM-DDTHH:MM:SS
phase: 1b
inputs: [domain-spec/L2-INDEX.md, prd.md]
traces_to: prd.md
deployment_topology: single-service   # single-service | multi-service
# If multi-service, system-overview.md MUST include a Service Boundaries section
# listing each service, its tech stack, inter-service contracts, and dependency order.
# The orchestrator reads this field to determine single-repo vs multi-repo routing.
---

# Architecture Index: [Project Name]

> **Context Engineering:** This is a lightweight index (~200-400 tokens). Agents load
> ONLY the section files they need, not the full architecture. See the Document Map
> for per-section consumer guidance.

## Document Map

| Section | File | Tokens | Primary Consumer | Purpose |
|---------|------|--------|-----------------|---------|
| System Overview | system-overview.md | ~800 | orchestrator, all agents | Architecture vision, principles, constraints |
| Module Decomposition | module-decomposition.md | ~1,000 | story-writer, implementer | Module catalog, boundaries, responsibilities |
| Dependency Graph | dependency-graph.md | ~600 | story-writer, consistency-validator | Inter-module dependencies, topological order |
| API Surface | api-surface.md | ~800 | test-writer, implementer | Public APIs, integration points |
| Verification Architecture | verification-architecture.md | ~1,000 | formal-verifier, architect | Provable Properties Catalog, proof strategy |
| Purity Boundary Map | purity-boundary-map.md | ~600 | implementer, formal-verifier | Pure core / effectful shell classification |
| Tooling Selection | tooling-selection.md | ~400 | formal-verifier | Kani, proptest, fuzz tool versions and config |
| Verification Coverage | verification-coverage-matrix.md | ~600 | consistency-validator | VP-to-module coverage mapping |

## Cross-References

| If you need... | Read these together |
|----------------|-------------------|
| Implementation plan for a module | module-decomposition.md + dependency-graph.md + api-surface.md |
| Verification plan for a module | verification-architecture.md + purity-boundary-map.md + tooling-selection.md |
| Full module picture | module-decomposition.md + purity-boundary-map.md + verification-coverage-matrix.md |
| Story decomposition input | module-decomposition.md + dependency-graph.md |

## Subsystem Registry

> **Source of truth** for subsystem names and IDs. BC frontmatter `subsystem:`,
> BC-INDEX subsystem column, story `subsystems:` fields, and PRD subsystem
> references MUST all use the exact Name from this table (Policy 6:
> `architecture_is_subsystem_name_source_of_truth`).
>
> The `validate-subsystem-names.sh` hook enforces this automatically.

| SS ID | Name | Architecture Doc | Implementing Modules | Phase Introduced |
|-------|------|-----------------|---------------------|-----------------|
| SS-01 | [subsystem name] | [section-file.md] | [module/package/crate names] | Phase 1 |
| SS-02 | [subsystem name] | [section-file.md] | [module/package/crate names] | Phase 1 |

**ID format:** `SS-NN` (two-digit sequential, append-only per Policy 1).

**Naming rules:**
- Names are human-readable, title-case (e.g., "Sensor Adapters", "Query Execution")
- Names are stable — once assigned, a subsystem name does not change
- If a subsystem is retired, mark it `(retired)` in the Name column; do not remove the row
- New subsystems added in later phases get the next sequential SS-ID

**Lifecycle:**
- **Phase 1a (PO):** BCs are grouped by L2 domain capabilities. Subsystem names at this stage are preliminary domain names.
- **Phase 1b (Architect):** The architect creates this registry, mapping domain groupings to architecture subsystems. This is the moment subsystem names become canonical. The architect may rename domain groupings to match architecture boundaries.
- **Phase 2+ (Story Writer):** Stories reference subsystem names from this registry via `subsystems:` frontmatter. Must use exact canonical Name.

## Architecture Decisions

| ID | Decision | Rationale |
|----|----------|-----------|
| AD-001 | [decision] | [why] |
