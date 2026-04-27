---
document_type: behavioral-contract
level: L3
version: "1.1"
status: draft
producer: phase-1-4b-bc-extractor
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs:
  - .factory/phase-0-ingestion/pass-3-behavioral-contracts.md
  - .factory/specs/behavioral-contracts/bc-id-mapping.md
input-hash: "1e73fa7"
traces_to: .factory/specs/architecture/ARCH-INDEX.md
origin: brownfield
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts.md#L484"
subsystem: SS-06
capability: "CAP-007"
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

# Behavioral Contract BC-6.01.004: activate skill copies hooks.json.<platform> to hooks.json then verifies dispatcher binary

> Source: `pass-3-behavioral-contracts.md` line 484 (was `BC-AUDIT-073`)
> Subsystem: SS-06 — Skill Catalog
> Section: BC-6.01 — Skill quality-gate contracts (broad-sweep)

## Description

The activate skill invokes `apply-platform.sh <platform>`, which copies the per-platform `hooks.json.<platform>` variant to `hooks.json` and verifies the dispatcher binary. Discrete numeric exit codes signal each failure mode; stderr is surfaced verbatim.

## Preconditions

1. Platform detected.

## Postconditions

1. `apply-platform.sh <platform>` exit 0 = success; 1 = variant missing; 2 = binary missing; 3 = binary not executable; 4 = usage error. Stderr surfaced verbatim.

## Invariants

1. TBD — derive from skill SKILL.md frontmatter and acceptance criteria.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | hooks.json.<platform> missing | Exit 1 — variant missing |
| EC-002 | dispatcher binary missing | Exit 2 — binary missing |
| EC-003 | dispatcher binary present but not executable | Exit 3 — not executable |
| EC-004 | apply-platform.sh invoked without platform arg | Exit 4 — usage error |
| EC-005 | All checks pass | Exit 0 — success; hooks.json activated |

## Canonical Test Vectors

> Golden-file test inputs and expected outputs.

| Input | Expected Output | Category |
|-------|----------------|----------|
| Valid platform with all artifacts in place | Exit 0; hooks.json overwritten with platform variant | happy-path |
| Platform variant file absent | Exit 1; stderr describes missing variant | error |
| Binary chmod -x | Exit 3; stderr describes non-executable dispatcher | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | apply-platform.sh exit codes are stable: 0/1/2/3/4 with the documented meanings | manual |
| VP-002 | Stderr from apply-platform.sh is surfaced verbatim by the activate skill | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-007 ("Deploy and activate the plugin on any supported platform") per capabilities.md §CAP-007 |
| L2 Domain Invariants | TBD |
| Architecture Module | plugins/vsdd-factory/skills/activate/SKILL.md; plugins/vsdd-factory/scripts/apply-platform.sh |
| Stories | S-2.06 |

## Related BCs (Recommended)

- BC-6.01.003 — activate requires platform detection success (upstream)
- BC-6.01.005 — activate writes platform metadata to settings.local.json (downstream)

## Architecture Anchors (Recommended)

- `architecture/ss-06-skill-catalog.md#activate-apply-platform` — TBD anchor

## Story Anchor (Recommended)

TBD — assigned by story-writer

## VP Anchors (Recommended)

- (TBD — to be assigned in Phase 1.6c)
- VP-002 — stderr surfacing

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/activate/SKILL.md` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Source Line** | 484 |
| **Audit ID** | BC-AUDIT-073 |
| **Evidence (verbatim)** | `skills/activate/SKILL.md` step 6. |
| **Confidence (verbatim)** | HIGH. |

#### Evidence Types Used

- documentation: stated in SKILL.md step 6
- guard clause: apply-platform.sh exit-code branches

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads + writes (copies hooks.json variant; reads binary metadata) |
| **Global state access** | reads filesystem under .claude/ and plugins/vsdd-factory/hooks/ |
| **Deterministic** | yes — given filesystem state |
| **Thread safety** | not applicable (one-shot setup script) |
| **Overall classification** | effectful shell |

#### Refactoring Notes

apply-platform.sh is already a thin orchestration script; behavior is testable by staging fixture filesystem layouts and asserting exit codes.

#### Source Excerpt (verbatim)

```text
### BC-AUDIT-073: activate skill copies hooks.json.<platform> to hooks.json then verifies dispatcher binary
- **Preconditions:** Platform detected.
- **Postconditions:** `apply-platform.sh <platform>` exit 0 = success; 1 = variant missing; 2 = binary missing; 3 = binary not executable; 4 = usage error. Stderr surfaced verbatim.
- **Evidence:** `skills/activate/SKILL.md` step 6.
- **Confidence:** HIGH.
```
