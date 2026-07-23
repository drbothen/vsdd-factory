---
document_type: behavioral-contract
level: L3
version: "1.1"
status: draft
producer: codebase-analyzer
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs: [.factory/phase-0-ingestion/pass-3-deep-agents.md]
input-hash: "595f07d"
traces_to: .factory/specs/architecture/ARCH-INDEX.md
origin: brownfield
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:537
subsystem: SS-05
capability: CAP-TBD
lifecycle_status: active
introduced: v1.0.0-beta.4
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# Behavioral Contract BC-5.07.014: dx-engineer: SHA pinning of dependencies and Docker images

## Description

Where possible, the dx-engineer pins all dependencies: cargo `--locked`,
`.tool-versions` for asdf/mise, Docker images by SHA256, GitHub Actions by
commit SHA.

## Preconditions

1. dx-engineer authoring or installing dependencies.

## Postconditions

1. `.tool-versions` exists.
2. cargo installs use `--locked`.
3. No floating tags in Docker compose files.

## Invariants

1. SHA pinning is mandatory wherever possible.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Tool source does not support SHA pinning | TBD — document deviation |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `.tool-versions` present + `cargo install --locked` used | Accepted | happy-path |
| Docker `image: nginx:latest` | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | No floating tags in Docker compose; cargo --locked used | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/dx-engineer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.08.006 — composes with (devops-engineer GitHub Actions SHA pinning)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#dx-engineer`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/dx-engineer.md:207-213` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit SHA Pinning rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads + writes |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
