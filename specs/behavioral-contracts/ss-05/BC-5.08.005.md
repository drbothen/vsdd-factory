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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:429
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

# Behavioral Contract BC-5.08.005: devops-engineer: never commits secrets

## Description

All secrets MUST be referenced via GitHub Secrets or equivalent secret stores;
never hardcoded in workflows, Dockerfiles, or scripts. The agent MUST never
`git add` a `.env` file with values.

## Preconditions

1. devops-engineer authoring CI/CD or environment files.

## Postconditions

1. No committed file contains `API_KEY=<actual-value>` patterns.
2. .env is gitignored.
3. `.env.example` has key names only with empty/placeholder values.

## Invariants

1. Zero hardcoded secrets in committed files.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Need to set secret in CI | Use GitHub Secrets reference |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Workflow using `${{ secrets.API_KEY }}` | Accepted | happy-path |
| Workflow with `API_KEY: sk-abc123` | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | No committed file contains hardcoded API key patterns | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/devops-engineer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.08.006 — composes with (SHA pinning)

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
| **Path** | `plugins/vsdd-factory/agents/devops-engineer.md:79, 70-75` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit no-secrets rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (CI/CD files) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
