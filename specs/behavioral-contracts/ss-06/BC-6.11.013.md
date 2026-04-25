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
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-skills-batch-1.md#L1621"
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

# Behavioral Contract BC-6.11.013: factory-obs: env override port allowlist

> Source: `pass-3-deep-skills-batch-1.md` line 1621 (was `BC-AUDIT-356`)
> Subsystem: SS-06 — Skill Catalog
> Section: Factory operations and dashboards skills

## Description

factory-obs: env override port allowlist. Honors VSDD_OBS_GRAFANA_PORT (3000), VSDD_OBS_LOKI_PORT (3100), VSDD_OBS_OTLP_HTTP_PORT (4318), VSDD_OBS_PROMETHEUS_PORT (9090), VSDD_OBS_RENDERER_PORT (8081), VSDD_FACTORY_LOGS (legacy single-path), VSDD_OBS_REGISTRY (test override), VSDD_OBS_OPEN_BROWSER (1=force, 0=suppress, unset=auto-detect TTY).

## Preconditions

1. Port collision recovery

## Postconditions

1. Each env var produces the documented effect when set.

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
| VP-001 | TBD — assertion derived from acceptance: "Each env var produces the documented effect when set." | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD |
| L2 Domain Invariants | TBD |
| Architecture Module | plugins/vsdd-factory/skills/factory-obs/SKILL.md |
| Stories | TBD |

## Related BCs (Recommended)

- TBD — link to sibling BCs in same skill (cross-reference once full SS-06 catalog written)

## Architecture Anchors (Recommended)

- `architecture/ss-06-skill-catalog.md#factory-obs-env-override-port-allowlist` — TBD anchor

## Story Anchor (Recommended)

TBD — assigned by story-writer

## VP Anchors (Recommended)

- VP-001 — TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/factory-obs/SKILL.md` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Source line(s) within skill** | 69-87 |

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
#### BC-AUDIT-356 — factory-obs: env override port allowlist

**Skill:** `plugins/vsdd-factory/skills/factory-obs/SKILL.md`
**Confidence:** HIGH
**Source line(s):** 69-87
**Trigger:** Port collision recovery
**Behavior:** Honors VSDD_OBS_GRAFANA_PORT (3000), VSDD_OBS_LOKI_PORT (3100), VSDD_OBS_OTLP_HTTP_PORT (4318), VSDD_OBS_PROMETHEUS_PORT (9090), VSDD_OBS_RENDERER_PORT (8081), VSDD_FACTORY_LOGS (legacy single-path), VSDD_OBS_REGISTRY (test override), VSDD_OBS_OPEN_BROWSER (1=force, 0=suppress, unset=auto-detect TTY).
**Acceptance:** Each env var produces the documented effect when set.
```
