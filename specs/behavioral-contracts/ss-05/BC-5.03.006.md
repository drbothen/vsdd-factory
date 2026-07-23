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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:383
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

# Behavioral Contract BC-5.03.006: demo-recorder: output strictly to docs/demo-evidence/<STORY-ID>/

## Description

All demo outputs MUST go under `docs/demo-evidence/<STORY-ID>/` in the story
worktree (committed to feature branch). Forbidden destinations: `docs/demo-evidence/*.md`
(flat), `.factory-demos/`, `.factory/demo-recordings/`. The `<STORY-ID>` matches
the story frontmatter's `story_id:` field verbatim.

## Preconditions

1. demo-recorder dispatched against a story with a valid `story_id`.

## Postconditions

1. Git diff adds files only under `docs/demo-evidence/<STORY-ID>/`.
2. No flat demo files at repo root or under `.factory/`.
3. `<STORY-ID>` matches the story frontmatter exactly.

## Invariants

1. Per-story subdirectory is mandatory — no flat layout.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Demo written to `docs/demo-evidence/foo.gif` (flat) | Rejected |
| EC-002 | Demo written to `.factory/demo-recordings/` | Rejected |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Story STORY-001 demo | Files under `docs/demo-evidence/STORY-001/` | happy-path |
| Flat output attempt | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Demo files always live under `docs/demo-evidence/<STORY-ID>/` | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/demo-recorder.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.03.007 — composes with (VHS/Playwright tooling)
- BC-5.03.008 — composes with (success + error path recording)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#demo-recorder`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/demo-recorder.md:35-38, 158-160` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Constraints and Output sections

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (scoped to demo-evidence/STORY-ID/) |
| **Global state access** | reads story frontmatter |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
