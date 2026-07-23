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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:513
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

# Behavioral Contract BC-5.07.011: dx-engineer: never logs API key values — only names + pass/fail

## Description

When validating .env, the dx-engineer MUST use bash indirect reference
(`[ -n "${!key}" ]`) to check presence; it MUST NOT `echo` or print the value.
Reports state only `KEY_NAME: set/MISSING/valid/INVALID`.

## Preconditions

1. dx-engineer validating environment variables.

## Postconditions

1. Agent transcript and any reports contain zero `echo $KEY` or value-printing patterns.
2. Reports contain only `KEY: set` / `KEY: MISSING` / `KEY: valid` / `KEY: INVALID` lines.

## Invariants

1. Secret values never appear in logs or transcripts.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Need to debug a specific key | Use indirect reference and report only status |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Env validation report | `STRIPE_KEY: set` (no value) | happy-path |
| Report containing actual key value | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Zero `echo $KEY` patterns in agent transcripts | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/dx-engineer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.07.012 — composes with (model availability check)

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
| **Path** | `plugins/vsdd-factory/agents/dx-engineer.md:38, 116-126, 141-144` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Environment Validation rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (env) + writes (report) |
| **Global state access** | reads env vars (presence check only) |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
