---
document_type: architecture-section
level: L3
section: "verification-architecture"
version: "1.1"
status: draft
producer: architect
timestamp: YYYY-MM-DDTHH:MM:SS
phase: 1b
inputs: [prd.md, behavioral-contracts/, module-decomposition.md]
traces_to: ARCH-INDEX.md
---

# Verification Architecture

> Defines which invariants must be formally proven, the purity boundary,
> verification tooling, and the Provable Properties Catalog.

## Provable Properties Catalog

| VP | Description | Module | Tool | Phase |
|----|-------------|--------|------|-------|
| VP-NNN | [property description] | [module-name] | [kani/proptest/fuzz/integration] | P0/P1 |

**Rules:**
- Every VP in VP-INDEX.md must appear here with matching module, tool, and phase
- VP-INDEX.md is the authoritative catalog — this table must match it exactly (Policy 9)
- Sort by VP number ascending

## P0 Properties (Inline OR reference-to-shard)

<!-- v1.1: For projects with 10+ VPs or rich architecture, this section may reference
     a separate file instead of inlining. Example:
     See architecture/purity-boundary-map.md for the full purity boundary map.
     Sharding is appropriate when this file exceeds 1,500 tokens. -->

[Comma-separated list of VP-NNN IDs with Priority=P0. These must be verified before release.]

## P1 Properties (Inline OR reference-to-shard)

[Comma-separated list of VP-NNN IDs with Priority=P1. These should be verified before release.]

## Purity Boundary (Inline OR reference-to-shard)

<!-- v1.1: May reference architecture/purity-boundary-map.md if already sharded.
     Example: "See architecture/purity-boundary-map.md for the full classification." -->

| Module | Classification | Rationale |
|--------|---------------|-----------|
| [module-name] | Pure Core / Effectful Shell | [why — what I/O boundaries exist] |

## Verification Tooling (Inline OR reference-to-shard)

<!-- v1.1: May reference architecture/tooling-selection.md if already sharded.
     Example: "See architecture/tooling-selection.md for tool versions and config." -->

| Tool | Version | Used For | Configuration |
|------|---------|----------|--------------|
| Kani | [version] | Model checking pure-core modules | [config notes] |
| Proptest | [version] | Property-based testing | [config notes] |
| cargo-fuzz | [version] | Fuzz testing parsers and input handlers | [config notes] |
| cargo-mutants | [version] | Mutation testing kill rate | [config notes] |
