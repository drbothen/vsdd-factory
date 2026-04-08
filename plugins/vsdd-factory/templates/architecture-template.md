---
document_type: architecture
level: L3
version: "1.0"
status: draft
producer: "[agent-id]"
timestamp: YYYY-MM-DDTHH:MM:SS
phase: 1b
inputs: []
input-hash: "[md5]"              # advisory — used for drift detection, not gating
traces_to: ""
prd_version: "1.0"
---

# Architecture Document: [Product Name]

> **Context Engineering Principle — Extended ToC Pattern:**
> Each section provides a concise summary with references to full detail. The
> Component Map (Section 3b) is machine-readable YAML for automated agent consumption.
> Prose sections should frontload structural decisions and place implementation details
> in referenced sub-documents when they exceed ~500 tokens.

## 1. System Overview
[High-level description of the system architecture]

## 1b. L2 Domain Capability Mapping

| CAP-NNN | Capability | Subsystem | Components | Notes |
|---------|-----------|-----------|------------|-------|
| CAP-001 | [capability name] | [subsystem] | COMP-NNN, COMP-NNN | [mapping notes] |

## 2. Architecture Patterns
- [ ] Microservices / Monolith / Modular Monolith
- [ ] Event-Driven / Request-Response / Hybrid
- [ ] Synchronous / Asynchronous

## 3. System Components

| ID | Component | Responsibility | Technology | Dependencies |
|----|-----------|---------------|------------|--------------|
| COMP-001 | | | | |

## 3b. Component Map (Machine-Readable)

A structured YAML map consumed by downstream agents (story-writer, implementer,
consistency-validator) for automated cross-referencing. This improves multi-agent
consistency by 15+ percentage points over prose-only architecture.

```yaml
components:
  - id: COMP-001
    name: "[component name]"
    layer: "[presentation | business-logic | data-access | infrastructure | shared]"
    purity: "[pure-core | effectful-shell | mixed]"
    criticality: "[CRITICAL | HIGH | MEDIUM | LOW]"
    dependencies: []
    interfaces_provided: []
    interfaces_consumed: []
```

## 4. Interfaces

| Interface ID | From | To | Protocol | SLA |
|-------------|------|-----|----------|-----|
| IF-001 | | | | |

## 5. Data Models

| Entity | Storage | Primary Key | Access Patterns |
|--------|---------|-------------|-----------------|

## 6. Integration Contracts

> **DEPRECATED:** Integration contracts are now maintained in the sharded
> `architecture/api-surface.md` file. This section is retained for backward
> compatibility but should not be populated for new projects. Redirect all
> integration contract content to `api-surface.md`.

| System | Protocol | Authentication | Error Handling |
|--------|----------|---------------|----------------|

## 7. Non-Functional Architecture

| NFR | Target | Architecture Decision | Validation |
|-----|--------|----------------------|-----------|

## 8. Architecture Decision Records

### ADR-001: [Decision Title]
- **Status:** Accepted
- **Context:** [Why this decision was needed]
- **Decision:** [What was decided]
- **Consequences:** [What follows from this decision]

## 9. Deployment Topology
[Describe deployment targets, scaling strategy, failover]

---

# Part 2: Verification Architecture

## 10. Provable Properties Catalog

| ID | Property | Module | Method | Feasibility | Priority |
|----|----------|--------|--------|-------------|----------|
| VP-CAT-001 | [property description] | [module path] | [kani/proptest/fuzz] | [ready/needs-refactoring] | [P0/P1/P2] |

## 11. Purity Boundary Map

| Module | Classification | I/O Boundary | Refactoring Needed |
|--------|---------------|-------------|-------------------|
| [module path] | [pure-core / effectful-shell / mixed] | [where I/O enters] | [yes: description / no] |

### Purity Strategy

[Describe the overall approach to separating pure core logic from effectful shell.
How does the architecture ensure maximum verifiability?]

## 12. Verification Tooling

| Tool | Purpose | Configuration | Integration Point |
|------|---------|---------------|-------------------|
| Kani | Formal proofs | [config file] | [CI step] |
| cargo-fuzz | Fuzz testing | [config file] | [CI step] |
| proptest | Property-based testing | [config file] | [CI step] |
| Semgrep | SAST | [config file] | [CI step] |

## 13. Property Specifications

### Module: [module_name]

| Property | Type | Formal Statement | Proof Complexity |
|----------|------|-----------------|-----------------|
| [property name] | [safety/liveness/functional] | [formal or semi-formal statement] | [low/medium/high] |

## 14. Verification Coverage Matrix

| Module | Criticality | Kani Proofs | Proptest | Fuzz Targets | Coverage Target |
|--------|------------|-------------|----------|-------------|----------------|
| [module] | [CRITICAL/HIGH/MEDIUM/LOW] | [n planned] | [n planned] | [n planned] | [pct]% |

---

# Part 3: Module Specifications

## 15. Module Specifications

### Module: [module_name]

| Property | Value |
|----------|-------|
| **Path** | [module path] |
| **Layer** | [presentation / business-logic / data-access / infrastructure] |
| **Purity** | [pure-core / effectful-shell / mixed] |
| **Criticality** | [CRITICAL / HIGH / MEDIUM / LOW] |
| **Public API** | [list of public functions/types] |
| **Dependencies** | [list of module dependencies] |

---

## 16. Concurrency Architecture

### Threading Model
[Describe the concurrency approach: async runtime, thread pool, actors, etc.]

### Shared State

| State | Protection | Access Pattern | Contention Risk |
|-------|-----------|---------------|-----------------|
| [shared resource] | [Mutex/RwLock/atomic/channel] | [read-heavy/write-heavy/mixed] | [low/medium/high] |

### Deadlock Prevention
[Strategy for preventing deadlocks: lock ordering, timeouts, lock-free structures]

### Concurrency Invariants

| ID | Invariant | Enforcement |
|----|-----------|-------------|
| CI-001 | [concurrency invariant] | [how enforced: type system / runtime / convention] |
