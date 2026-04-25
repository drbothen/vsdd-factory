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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:117
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

# Behavioral Contract BC-5.04.006: adversary: returns findings as chat text, never writes files

## Description

The adversary MUST NOT use Write or Edit tools. State-manager owns persistence
to `.factory/cycles/<cycle>/adversarial-reviews/`. Read-only is structurally
enforced via the tool profile.

## Preconditions

1. adversary dispatched.

## Postconditions

1. Tool profile excludes Write/Edit/Bash/exec/process.
2. Effective allowed tools = {Read, Grep, Glob}.
3. Findings are returned as chat text; state-manager persists them.

## Invariants

1. The adversary's read-only tool profile is structural — not advisory.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Agent attempts to use Write | Tool denied at runtime |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Adversary dispatch | Tools = {Read, Grep, Glob} | happy-path |
| Attempt to Write | Denied | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Tool profile excludes Write/Edit/Bash | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/adversary.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.04.001 — composes with (information wall)
- BC-5.10.001 — composes with (state-manager persistence)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#adversary`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/adversary.md:188-189` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit tool-profile rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads only |
| **Global state access** | none |
| **Deterministic** | yes (given fixed inputs) |
| **Thread safety** | unknown |
| **Overall classification** | pure (read-only) |

#### Refactoring Notes

No refactoring needed.
