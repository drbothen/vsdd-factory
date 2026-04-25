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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:437
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

# Behavioral Contract BC-5.08.006: devops-engineer: all GitHub Actions pinned to SHA, never tag

## Description

Every `uses:` reference in a workflow MUST be pinned to a 40-char commit SHA.
Version tags (`@v3`, `@latest`) are forbidden. Dockerfile FROM lines must use
SHA digests, not `latest`.

## Preconditions

1. devops-engineer authoring GitHub Actions or Dockerfile.

## Postconditions

1. Every workflow's `uses:` line matches `actions/<repo>@[a-f0-9]{40}`.
2. Dockerfiles use `image@sha256:...` or pinned tags (no `latest`).

## Invariants

1. Supply-chain pinning is mandatory.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Action source uses moving tag | Pin to current SHA + comment with intended tag |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `uses: actions/checkout@a81bbbf...` (40-char SHA) | Accepted | happy-path |
| `uses: actions/checkout@v3` | Rejected | error |
| `FROM nginx:latest` | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every workflow uses: line is SHA-pinned | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/devops-engineer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.07.014 — composes with (dx-engineer SHA pinning)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#devops-engineer`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/devops-engineer.md:75, 81, 80` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit SHA pinning rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (workflows) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
