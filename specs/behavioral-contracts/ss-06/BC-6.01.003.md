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
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts.md#L478"
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

# Behavioral Contract BC-6.01.003: activate skill requires platform detection success

> Source: `pass-3-behavioral-contracts.md` line 478 (was `BC-AUDIT-072`)
> Subsystem: SS-06 — Skill Catalog
> Section: BC-6.01 — Skill quality-gate contracts (broad-sweep)

## Description

The activate skill requires that `detect-platform.sh` succeed. Exit 0 yields one of {darwin-arm64, darwin-x64, linux-x64, linux-arm64, windows-x64}; exit 1 means unsupported and aborts activation.

## Preconditions

1. `/vsdd-factory:activate` invoked.

## Postconditions

1. `detect-platform.sh` exit 0 → one of {darwin-arm64, darwin-x64, linux-x64, linux-arm64, windows-x64}; exit 1 → unsupported, abort activation.

## Invariants

1. TBD — derive from skill SKILL.md frontmatter and acceptance criteria.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | detect-platform.sh exits 0 with darwin-arm64 | Activation proceeds with that platform string |
| EC-002 | detect-platform.sh exits 1 (unsupported OS/arch) | Activation aborts; user receives unsupported-platform error |
| EC-003 | detect-platform.sh emits a string outside the 5-platform set | Treated as detection failure; activation aborts |

## Canonical Test Vectors

> Golden-file test inputs and expected outputs.

| Input | Expected Output | Category |
|-------|----------------|----------|
| Run on macOS Apple Silicon → detect-platform.sh prints `darwin-arm64`, exit 0 | Activation proceeds | happy-path |
| Run on FreeBSD → detect-platform.sh exit 1 | Activation aborted with unsupported error | error |
| Detector emits `linux-arm64`, exit 0 | Activation proceeds | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | Activation proceeds iff detect-platform.sh exits 0 with a platform in the 5-platform allowlist | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-007 ("Deploy and activate the plugin on any supported platform") per capabilities.md §CAP-007 |
| L2 Domain Invariants | TBD |
| Architecture Module | plugins/vsdd-factory/skills/activate/SKILL.md |
| Stories | S-0.03 |

## Related BCs (Recommended)

- BC-6.01.004 — activate copies hooks.json.<platform> + verifies dispatcher binary (downstream of platform detection)
- BC-6.01.005 — activate writes platform metadata to settings.local.json
- BC-6.01.006 — activate drift warns on cross-host re-activation

## Architecture Anchors (Recommended)

- `architecture/ss-06-skill-catalog.md#activate-platform-detection` — TBD anchor

## Story Anchor (Recommended)

TBD — assigned by story-writer

## VP Anchors (Recommended)

- (TBD — to be assigned in Phase 1.6c)

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/activate/SKILL.md` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Source Line** | 478 |
| **Audit ID** | BC-AUDIT-072 |
| **Evidence (verbatim)** | `skills/activate/SKILL.md` step 2. |
| **Confidence (verbatim)** | HIGH. |

#### Evidence Types Used

- documentation: stated in SKILL.md step 2
- guard clause: detect-platform.sh exit-code branch

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads only (uname / OS detection) |
| **Global state access** | none |
| **Deterministic** | yes — given a host, detection is stable |
| **Thread safety** | not applicable |
| **Overall classification** | effectful shell (host detection) |

#### Refactoring Notes

Detection is already isolated in `detect-platform.sh`; the gating behavior is testable by mocking the script's exit code in an activation harness.

#### Source Excerpt (verbatim)

```text
### BC-AUDIT-072: activate skill requires platform detection success
- **Preconditions:** `/vsdd-factory:activate` invoked.
- **Postconditions:** detect-platform.sh exit 0 → one of {darwin-arm64, darwin-x64, linux-x64, linux-arm64, windows-x64}; exit 1 → unsupported, abort activation.
- **Evidence:** `skills/activate/SKILL.md` step 2.
- **Confidence:** HIGH.
```
