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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:475
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

# Behavioral Contract BC-5.09.001: dtu-validator: never use production API keys with write access

## Description

All API calls to real services MUST use test/sandbox keys (Stripe test mode,
Okta preview org, GitHub read-only PAT). Production keys with write access are
categorically forbidden — even for read operations they're avoided when test
alternatives exist.

## Preconditions

1. dtu-validator running against external services.

## Postconditions

1. Environment audit shows no `STRIPE_LIVE_KEY` or production-tier credentials
   in the validator's effective env.
2. Only `*_TEST_KEY` / `*_PREVIEW_TOKEN` / `*_SANDBOX_KEY` patterns.

## Invariants

1. Production write keys never used.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Service has no test mode | Use read-only key with explicit override audit trail |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Validator using Stripe test mode | Accepted | happy-path |
| Validator using STRIPE_LIVE_KEY | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Effective env has no production write keys | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/dtu-validator.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.09.002 — composes with (fidelity thresholds)
- BC-5.09.003 — composes with (drift detection)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#dtu-validator`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/dtu-validator.md:105-114, 134` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit API Key Management section

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (env) + network |
| **Global state access** | reads env |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
