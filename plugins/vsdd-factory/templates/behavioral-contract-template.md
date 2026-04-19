---
document_type: behavioral-contract
level: L3
version: "1.1"
status: draft
producer: "[agent-id]"
timestamp: YYYY-MM-DDTHH:MM:SS
phase: 1a
inputs: [domain-spec/L2-INDEX.md, research/RESEARCH-INDEX.md]
input-hash: "[md5]"              # advisory — used for drift detection, not gating
traces_to: domain-spec/L2-INDEX.md
origin: greenfield|brownfield    # metadata-only — does not affect BC semantics or downstream consumption
extracted_from: "[source file path -- brownfield only, omit for greenfield]"
subsystem: "[L2 subsystem name]"
capability: "CAP-NNN"
# Lifecycle fields (DF-030)
lifecycle_status: active        # active | deprecated | retired | removed
introduced: vX.Y.Z             # cycle that created this artifact
modified: []                    # cycles that modified it (e.g., [v1.1.0, v1.3.0])
deprecated: null                # cycle that deprecated (null if active)
deprecated_by: null             # which cycle deprecated it (e.g., v1.2.0-feature-auth)
replacement: null               # replacement artifact ID (e.g., BC-4.01.001)
retired: null                   # cycle that retired (null if active)
removed: null                   # cycle that removed (null if not removed)
removal_reason: null             # why removed
---

# Behavioral Contract BC-S.SS.NNN: [Title]

> **One-per-file:** Each behavioral contract lives in its own file.
> Filename convention: `BC-S.SS.NNN.md` (e.g., `BC-2.3.045.md`)
> Numbering: BC-S.SS.NNN where S = PRD section, SS = subsection (L2 subsystem), NNN = sequential.

## Description

[2-3 sentence summary of what this behavioral contract covers. Provides quick
context without reading the full preconditions/postconditions. Especially useful
when scanning BC-INDEX or reviewing anchor justifications.]

## Preconditions

1. [What must be true before this behavior is invoked]
2. [Input constraints, state requirements]

## Postconditions

1. [What will be true after successful execution]
2. [Output guarantees, state changes]

## Invariants

1. [What must remain true throughout execution]
2. [Properties that are never violated]

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | [boundary condition] | [what happens] |
| EC-002 | [boundary condition] | [what happens] |

## Canonical Test Vectors

> Golden-file test inputs and expected outputs. Used for regression testing
> and agent validation. Include at minimum: one happy-path, one edge-case,
> one error-case vector.

| Input | Expected Output | Category |
|-------|----------------|----------|
| [input description] | [expected result] | happy-path |
| [input description] | [expected result] | edge-case |
| [input description] | [expected result] | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-001 | [property description] | kani/proptest/fuzz/manual |
| VP-002 | [property description] | kani/proptest/fuzz/manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-NNN |
| L2 Domain Invariants | DI-NNN (if applicable) |
| Architecture Module | [module name] (filled by architect) |
| Stories | STORY-NNN (filled by story-writer) |

## Related BCs (Recommended)

<!-- v1.1: Added for machine-extractable cross-references. Supplements the Traceability table. -->

- [BC-S.SS.NNN] — [relationship: composes with / depends on / supersedes / related to]

## Architecture Anchors (Recommended)

<!-- v1.1: Added for direct links to architecture section files. -->

- `architecture/[section-name].md#[anchor]` — [what aspect this BC relates to]

## Story Anchor (Recommended)

<!-- v1.1: Added for direct link to implementing story. -->

[STORY-NNN] — [short story title]

## VP Anchors (Recommended)

<!-- v1.1: Added for direct links to verification properties. -->

- [VP-NNN] — [property description]

---

### Brownfield-Specific Sections (include only when `origin: brownfield`)

> The sections below apply when this contract was extracted from existing code
> during Phase 0d. They capture source evidence and refactoring notes.

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `[full/path/to/module]` |
| **Confidence** | [high / medium / low] (based on code evidence quality) |
| **Extraction Date** | [YYYY-MM-DD] |

#### Evidence Types Used

- **guard clause**: explicit validation check in code
- **type constraint**: enforced by the type system
- **assertion**: runtime assertion statement
- **documentation**: stated in doc comments but not enforced in code
- **inferred**: no explicit check, but calling code always provides valid input

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | [none / reads only / reads + writes / network calls] |
| **Global state access** | [none / reads global / mutates global] |
| **Deterministic** | [yes / no -- depends on time/random/external state] |
| **Thread safety** | [Send + Sync / Send only / not thread-safe / unknown] |
| **Overall classification** | [pure / effectful shell / mixed / opaque] |

#### Refactoring Notes

[If mixed: describe what I/O could be extracted to make the core logic pure.
If already pure: note "No refactoring needed -- suitable for formal verification."
If opaque: describe what prevents classification.]
