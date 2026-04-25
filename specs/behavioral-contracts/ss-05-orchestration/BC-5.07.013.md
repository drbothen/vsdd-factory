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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:529
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

# Behavioral Contract BC-5.07.013: dx-engineer: tool installation requires security-reviewer audit

## Description

Before installing any tool (vhs, kani, semgrep, etc.), the dx-engineer MUST spawn
security-reviewer for a CVE/NVD/OSV check + Perplexity recent-compromise search +
SHA verification. Any finding blocks installation pending human approval.

## Preconditions

1. dx-engineer about to install a tool.

## Postconditions

1. Each tool install is preceded by a security-reviewer dispatch in the agent log.
2. Install proceeds only after CLEAN verdict or explicit human override.

## Invariants

1. No tool install without security audit.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Security finding even if low severity | BLOCK pending human override |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| dx-engineer installing kani | security-reviewer dispatch precedes install | happy-path |
| Install without prior security audit | Audit failure | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every tool install has a preceding security-reviewer dispatch | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/dx-engineer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.07.042 — composes with (security-reviewer supply chain audit)

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
| **Path** | `plugins/vsdd-factory/agents/dx-engineer.md:37, 93-101` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Supply Chain Security Audit section

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads + writes |
| **Global state access** | none |
| **Deterministic** | no (depends on registry state) |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
