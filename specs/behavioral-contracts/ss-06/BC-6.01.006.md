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
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts.md#L496"
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

# Behavioral Contract BC-6.01.006: activate drift warns on cross-host re-activation

> Source: `pass-3-behavioral-contracts.md` line 496 (was `BC-AUDIT-075`)
> Subsystem: SS-06 — Skill Catalog
> Section: BC-6.01 — Skill quality-gate contracts (broad-sweep)

## Description

When `.vsdd-factory.activated_platform` exists in settings.local.json and differs from the currently detected platform, the activate skill prints a warning, continues activation, and updates the persisted platform string.

## Preconditions

1. `.vsdd-factory.activated_platform` exists and ≠ currently detected platform.

## Postconditions

1. Warning printed; activation continues; persisted platform updated.

## Invariants

1. TBD — derive from skill SKILL.md frontmatter and acceptance criteria.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | persisted platform == detected platform | No warning; activation proceeds normally |
| EC-002 | persisted platform absent (first activation) | No warning; persisted platform written |
| EC-003 | persisted platform differs from detected (e.g., repo cloned across darwin-arm64 → linux-x64) | Warning printed; activation continues; persisted platform updated to the new detected value |

## Canonical Test Vectors

> Golden-file test inputs and expected outputs.

| Input | Expected Output | Category |
|-------|----------------|----------|
| settings.local.json has `activated_platform: darwin-arm64`; current host detects `linux-x64` | Drift warning printed; activation continues; persisted platform updated to linux-x64 | happy-path (intended drift handling) |
| settings.local.json has `activated_platform: darwin-arm64`; current host detects `darwin-arm64` | No warning; normal activation | edge-case |
| settings.local.json has no `activated_platform` key | No warning; key written for the first time | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | Drift detection compares persisted vs detected platform; mismatch emits a warning but does not abort | manual |
| (TBD — to be assigned in Phase 1.6c) | Persisted platform is updated to the newly detected value after a drift warning | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-007 ("Deploy and activate the plugin on any supported platform") per capabilities.md §CAP-007 |
| L2 Domain Invariants | TBD |
| Architecture Module | plugins/vsdd-factory/skills/activate/SKILL.md |
| Stories | S-2.06 |

## Related BCs (Recommended)

- BC-6.01.003 — activate requires platform detection success (provides current detected platform)
- BC-6.01.005 — activate writes activated_platform to settings.local.json (writes the value being compared)

## Architecture Anchors (Recommended)

- `architecture/ss-06-skill-catalog.md#activate-drift-warning` — TBD anchor

## Story Anchor (Recommended)

TBD — assigned by story-writer

## VP Anchors (Recommended)

- (TBD — to be assigned in Phase 1.6c)
- (TBD — to be assigned in Phase 1.6c) — Persisted platform update on drift (VP-002 placeholder mis-anchor removed; real VP-002 is SS-01 wasmtime invariant)

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/activate/SKILL.md` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Source Line** | 496 |
| **Audit ID** | BC-AUDIT-075 |
| **Evidence (verbatim)** | `skills/activate/SKILL.md` step 4. |
| **Confidence (verbatim)** | HIGH. |

#### Evidence Types Used

- documentation: stated in SKILL.md step 4

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads + writes (.claude/settings.local.json + stderr warning) |
| **Global state access** | reads + mutates project-local Claude settings |
| **Deterministic** | yes — given persisted and detected platform values |
| **Thread safety** | not applicable (one-shot activation) |
| **Overall classification** | mixed (pure comparison core + I/O shell for read/warn/update) |

#### Refactoring Notes

The drift comparison can be extracted as a pure function (persisted, detected) → (warn?, new_persisted), leaving I/O at the boundary. Easy to unit-test with table-driven cases.

#### Source Excerpt (verbatim)

```text
### BC-AUDIT-075: activate drift warns on cross-host re-activation
- **Preconditions:** `.vsdd-factory.activated_platform` exists and ≠ currently detected platform.
- **Postconditions:** Warning printed; activation continues; persisted platform updated.
- **Evidence:** `skills/activate/SKILL.md` step 4.
- **Confidence:** HIGH.
```
