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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1283
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

# Behavioral Contract BC-5.05.018: spec-steward: append-only IDs and immutable filename slugs

## Description

All VSDD identifiers (BC, CAP, VP, EC, DI, ASM, R, FM, STORY, HS) are append-only.
Reuse of retired IDs is HIGH severity. Filename slugs MUST remain stable across
title changes.

## Preconditions

1. spec-steward managing VSDD identifiers.

## Postconditions

1. No active artifact has the same ID as a retired one in any index.
2. Git log shows no rename of slug-based filenames.

## Invariants

1. ID space is monotonically growing.
2. Filename slugs are stable.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Retired ID re-introduced as active | HIGH severity finding |
| EC-002 | Title change without slug change | Acceptable |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| New BC with new ID | Accepted | happy-path |
| Reuse of BC-001 (previously retired) | Rejected (HIGH severity) | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | No retired ID is reused in any active artifact | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/spec-steward.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.06.007 — composes with (product-owner append-only IDs)
- BC-5.05.015 — composes with (governance-only writes)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#append-only-ids`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/spec-steward.md:192-201` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Append-Only ID and Slug Protection rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (indexes, git) |
| **Global state access** | reads ID registries |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure (governance check) |

#### Refactoring Notes

No refactoring needed.
