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
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts.md#L490"
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

# Behavioral Contract BC-6.01.005: activate skill writes platform + plugin version + activated_at to .claude/settings.local.json

> Source: `pass-3-behavioral-contracts.md` line 490 (was `BC-AUDIT-074`)
> Subsystem: SS-06 — Skill Catalog
> Section: BC-6.01 — Skill quality-gate contracts (broad-sweep)

## Description

On successful activation, the activate skill merges activation metadata (`activated_platform`, `activated_at`, `activated_plugin_version`) and the default agent into `.claude/settings.local.json`, preserving all other keys.

## Preconditions

1. Successful activation.

## Postconditions

1. `.claude/settings.local.json` merged with `{ agent: "vsdd-factory:orchestrator:orchestrator", "vsdd-factory": { activated_platform, activated_at, activated_plugin_version } }` preserving all other keys.

## Invariants

1. TBD — derive from skill SKILL.md frontmatter and acceptance criteria.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | settings.local.json does not yet exist | File is created with the activation payload |
| EC-002 | settings.local.json contains unrelated keys | Unrelated keys preserved verbatim; only the documented keys are merged |
| EC-003 | settings.local.json already has a different `agent` | Overwritten to `vsdd-factory:orchestrator:orchestrator` |

## Canonical Test Vectors

> Golden-file test inputs and expected outputs.

| Input | Expected Output | Category |
|-------|----------------|----------|
| Empty / absent settings.local.json | File written with merged keys; other keys absent (none to preserve) | happy-path |
| Pre-existing settings.local.json with unrelated keys | All non-vsdd-factory keys preserved; activation keys merged | edge-case |
| Pre-existing settings.local.json with stale `vsdd-factory` block | Block updated with new platform/version/timestamp | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | settings.local.json after activation contains agent + vsdd-factory.{activated_platform, activated_at, activated_plugin_version} | manual |
| VP-002 | All keys present before activation that are not in the merge payload remain unchanged | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-007 ("Deploy and activate the plugin on any supported platform") per capabilities.md §CAP-007 |
| L2 Domain Invariants | TBD |
| Architecture Module | plugins/vsdd-factory/skills/activate/SKILL.md |
| Stories | S-2.06 |

## Related BCs (Recommended)

- BC-6.01.003 — activate requires platform detection success (upstream)
- BC-6.01.004 — activate copies hooks.json.<platform> + verifies dispatcher (upstream)
- BC-6.01.006 — activate drift warns on cross-host re-activation (uses persisted activated_platform)

## Architecture Anchors (Recommended)

- `architecture/ss-06-skill-catalog.md#activate-settings-merge` — TBD anchor

## Story Anchor (Recommended)

TBD — assigned by story-writer

## VP Anchors (Recommended)

- (TBD — to be assigned in Phase 1.6c)
- VP-002 — non-payload keys preserved

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/skills/activate/SKILL.md` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Source Line** | 490 |
| **Audit ID** | BC-AUDIT-074 |
| **Evidence (verbatim)** | `skills/activate/SKILL.md` step 5. |
| **Confidence (verbatim)** | HIGH. |

#### Evidence Types Used

- documentation: stated in SKILL.md step 5

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads + writes (.claude/settings.local.json) |
| **Global state access** | reads + mutates project-local Claude settings |
| **Deterministic** | yes — given inputs and current settings file |
| **Thread safety** | not applicable (one-shot activation) |
| **Overall classification** | mixed (settings merge with preservation logic; refactor-friendly to a pure JSON-merge core + I/O shell) |

#### Refactoring Notes

The merge logic can be extracted as a pure JSON-merge function (input: existing settings + payload → output: merged settings) with the file I/O confined to the shell. Suitable for unit tests against fixture JSON inputs.

#### Source Excerpt (verbatim)

```text
### BC-AUDIT-074: activate skill writes platform + plugin version + activated_at to .claude/settings.local.json
- **Preconditions:** Successful activation.
- **Postconditions:** `.claude/settings.local.json` merged with `{ agent: "vsdd-factory:orchestrator:orchestrator", "vsdd-factory": { activated_platform, activated_at, activated_plugin_version } }` preserving all other keys.
- **Evidence:** `skills/activate/SKILL.md` step 5.
- **Confidence:** HIGH.
```
