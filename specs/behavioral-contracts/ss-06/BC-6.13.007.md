---
document_type: behavioral-contract
level: L3
version: "1.1"
status: draft
producer: phase-1-4b-bc-extractor
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs:
  - .factory/phase-0-ingestion/pass-3-deep-skills-batch-1.md
  - .factory/specs/behavioral-contracts/bc-id-mapping.md
input-hash: "TBD"
traces_to: .factory/specs/architecture/ARCH-INDEX.md
origin: brownfield
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-skills-batch-1.md#L542"
subsystem: SS-06
capability: "TBD"
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

# Behavioral Contract BC-6.13.007: claude-telemetry: prunes legacy temporality key

> Source: `pass-3-deep-skills-batch-1.md` line 542 (was `BC-AUDIT-249`)
> Subsystem: SS-06 — Skill Catalog
> Section: Telemetry and analytics integration skills

## Description

claude-telemetry: prunes legacy temporality key. As of v0.76.0 the deltatocumulative collector processor handles cumulative conversion. The legacy 6th env var is no longer needed. `on` prunes it via `del(.env.OTEL_EXPORTER_OTLP_METRICS_TEMPORALITY_PREFERENCE)` after merging the 5 current keys. `off` deletes it as well.

## Preconditions

1. `on` mode (and `off` mode)

## Postconditions

1. After `on`, the legacy key is absent from settings.local.json regardless of prior state.

## Invariants

1. TBD — derive from skill SKILL.md frontmatter and acceptance criteria.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | TBD | TBD |

## Canonical Test Vectors

> Golden-file test inputs and expected outputs.

| Input | Expected Output | Category |
|-------|----------------|----------|
| TBD — happy path from skill acceptance | TBD | happy-path |
| TBD — edge case | TBD | edge-case |
| TBD — error case | TBD | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | TBD — assertion derived from acceptance: "After `on`, the legacy key is absent from settings.local.json regardless of prior state." | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD |
| L2 Domain Invariants | TBD |
| Architecture Module | plugins/vsdd-factory/skills/claude-telemetry/SKILL.md |
| Stories | TBD |

## Related BCs (Recommended)

- TBD — link to sibling BCs in same skill (cross-reference once full SS-06 catalog written)

## Architecture Anchors (Recommended)

- `architecture/ss-06-skill-catalog.md#claude-telemetry-prunes-legacy-temporality-key` — TBD anchor

## Story Anchor (Recommended)

TBD — assigned by story-writer

## VP Anchors (Recommended)

- (TBD — to be assigned in Phase 1.6c)

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/claude-telemetry/SKILL.md` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Source line(s) within skill** | 24-29, 73-86, 96-114 |

#### Evidence Types Used

- documentation: stated in SKILL.md frontmatter and prose
- inferred: behavior derived from skill acceptance criteria

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | TBD |
| **Global state access** | TBD |
| **Deterministic** | TBD |
| **Thread safety** | TBD |
| **Overall classification** | TBD |

#### Refactoring Notes

TBD — assess once architecture mapping is complete.

#### Source Excerpt (verbatim)

```text
#### BC-AUDIT-249 — claude-telemetry: prunes legacy temporality key

**Skill:** `plugins/vsdd-factory/skills/claude-telemetry/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 24-29, 73-86, 96-114
**Trigger:** `on` mode (and `off` mode)
**Behavior:** As of v0.76.0 the deltatocumulative collector processor handles cumulative conversion. The legacy 6th env var is no longer needed. `on` prunes it via `del(.env.OTEL_EXPORTER_OTLP_METRICS_TEMPORALITY_PREFERENCE)` after merging the 5 current keys. `off` deletes it as well.
**Acceptance:** After `on`, the legacy key is absent from settings.local.json regardless of prior state.
```
