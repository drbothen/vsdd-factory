---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-07-22T00:00:00
phase: 1a
inputs: []
input-hash: "2181a12"
traces_to: "PR #714"
origin: greenfield
extracted_from: "PR #714 (fix(templates): add mandatory CHANGELOG delivery task to story template)"
subsystem: "SS-08"
capability: "CAP-001"
lifecycle_status: draft
introduced: v1.0-feature-changelog-delivery
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-8.31.001
section: "8.31"
---

# BC-8.31.001: story template must carry a mandatory CHANGELOG-delivery task and CHANGELOG.md must maintain exactly one top-of-file ## [Unreleased] section drained at release

## Description

The story template (`plugins/vsdd-factory/templates/story-template.md`) must include task 12 in its `## Tasks (MANDATORY)` section: "Add a CHANGELOG entry under [Unreleased] > Fixed/Changed/Added describing the shipped behavior, before creating the PR." This ensures every implementer who follows the template ships the CHANGELOG row inside the story PR rather than post-hoc, preventing documentation debt caught adversarially late in F5 convergence (the pattern recurred at least twice before this contract was established, most recently motivating PR #580).

All three story-pipeline authoring artifacts — the story template, the story-writer agent prompt, and the create-story skill — must carry this requirement with consistent wording, anchored on the stable token pair `CHANGELOG` + `[Unreleased]`. Anchoring on the token pair (rather than exact phrasing) means a harmless future rephrase that preserves both tokens does not break the regression guards, but a dropped anchor in any of the three files does.

The CHANGELOG.md entry point referenced by the task — `## [Unreleased]` — must exist as the **first H2 heading** in CHANGELOG.md at all times. This anchoring invariant is enforced by a bats regression guard and must be preserved by `scripts/bump-version.sh` across releases: the script inserts new version stubs directly above the first `## N.N.N` heading (below `## [Unreleased]`), leaving the accumulation section permanently at the top. At release time, RELEASING.md Step 2 documents the drain: accumulated entries under `## [Unreleased]` are moved into the new `## <version>` section, and the `## [Unreleased]` heading is left in place, empty, for the next cycle.

## Preconditions

1. The story template (`plugins/vsdd-factory/templates/story-template.md`) is being authored, reviewed, or validated.
2. `CHANGELOG.md` exists at the repository root with `## [Unreleased]` as the first H2 heading.
3. `scripts/bump-version.sh` is run to prepare a new release stub.

## Postconditions

**Story template and pipeline-authoring artifacts (task presence):**

1. `plugins/vsdd-factory/templates/story-template.md` `## Tasks (MANDATORY)` section contains, as the last standard task: `12. [ ] Add a CHANGELOG entry under [Unreleased] > Fixed/Changed/Added describing the shipped behavior, before creating the PR`
2. `plugins/vsdd-factory/agents/story-writer.md` "Each story includes:" list contains a CHANGELOG-delivery bullet anchored on the token pair `CHANGELOG` + `[Unreleased]`.
3. `plugins/vsdd-factory/skills/create-story/SKILL.md` Tasks example contains a step anchored on the token pair `CHANGELOG` + `[Unreleased]`.
4. A case-insensitive grep for `CHANGELOG.+\[Unreleased\]` matches all three files.

**CHANGELOG.md structural invariant:**

5. `grep -m1 -E '^## ' CHANGELOG.md` outputs exactly `## [Unreleased]` — the bracketed form is the first H2.
6. At most one `## [Unreleased]` heading exists in CHANGELOG.md at any time.
7. The `## [Unreleased]` section contains an HTML comment describing its accumulation role and drain procedure.

**bump-version.sh release-anchor preservation:**

8. When run against a CHANGELOG.md with a top-of-file `## [Unreleased]` section, the new version stub is inserted directly above the first `## N.N.N` heading — not at the absolute top of the file. After the script runs, `grep -m1 -E '^## ' CHANGELOG.md` still returns `## [Unreleased]`.
9. `RELEASING.md` Step 2 documents the drain procedure: move accumulated entries from `## [Unreleased]` into the new `## <version>` section, then leave `## [Unreleased]` in place, empty.

## Invariants

1. The token pair `CHANGELOG` + `[Unreleased]` is the stable identifier. A rephrase that preserves both tokens is valid. A rephrase that drops either token is a violation.
2. The `## [Unreleased]` heading uses the bracketed form exactly. The unbracketed form `## Unreleased` is non-canonical and must not serve as the accumulation anchor.
3. bump-version.sh must NOT prepend the new stub before `## [Unreleased]`. The invariant: after any bump-version.sh run, `grep -m1 -E '^## ' CHANGELOG.md` must still return `## [Unreleased]`.
4. After a release drain, the `## [Unreleased]` section is left in place, empty. Removing it causes the bats invariant test to fail on the next CI run.
5. Exactly one `## [Unreleased]` section exists in CHANGELOG.md. A second occurrence (whether via an agent creating a divergent heading or a manual edit) is a structural defect.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Dual-section: a second `## [Unreleased]` heading is created mid-file (e.g., an agent follows the task literally and creates a new section rather than using the canonical top-of-file anchor) | The bats first-H2 test passes (canonical section is still first), but CHANGELOG.md contains two accumulation headings. Story PRs may file entries under the wrong one, fragmenting the drain. Resolution: delete the duplicate; all entries accumulate only under the canonical top-of-file section. |
| EC-002 | Missing section: `## [Unreleased]` is absent (e.g., accidentally deleted during a release drain) | bats test "CHANGELOG.md carries the top-of-file [Unreleased] section" fails — `grep -m1 -E '^## ' CHANGELOG.md` returns a versioned heading. Resolution: restore `## [Unreleased]` as the first H2 with the canonical accumulation comment before the next story PR merges. |
| EC-003 | Legacy unbracketed form `## Unreleased` remains mid-file | The bats test passes (bracketed form is first H2). The unbracketed form is dead markup. Resolution: remove the unbracketed entry. This was the pre-PR #714 state (line 656 in the old CHANGELOG.md). |
| EC-004 | bump-version.sh run when no `## N.N.N` heading exists in CHANGELOG.md | Script falls back to appending the stub at end of file (the else-branch in bump-version.sh:L131-133). The `## [Unreleased]` structural invariant is not maintained by this fallback path; the caller must restore the anchor manually after the stub is added. |
| EC-005 | Stub insertion position — `## [Unreleased]` at top, first `## 1.0.0-rc.X` at line N | bump-version.sh's head/tail split: `head -n "$((N - 1))"` captures everything above line N (including `## [Unreleased]` and any accumulated entries); stub is appended; `tail -n "+${N}"` emits the rest. The `## [Unreleased]` section remains at the top. Accumulated entries sit adjacent to the new stub for the operator to drain (RELEASING.md Step 2). |
| EC-006 | Story PR merged without the implementer adding a CHANGELOG entry | Template task 12 was available but not executed. No automated gate enforces the row at PR-merge time — validate-template-compliance.sh operates at `##`-heading granularity; row-level enforcement is not yet implemented (deferred — ADVISORY per PR #714 reviewer). The defect is caught at release drain time (empty `## [Unreleased]` section). Remediation: a CHANGELOG-only follow-up PR before the release. |

## Canonical Test Vectors

| Artifact | Check Command | Expected Result |
|----------|--------------|----------------|
| `plugins/vsdd-factory/templates/story-template.md` | `grep -Ei 'CHANGELOG.+\[Unreleased\]'` | match (task 12) |
| `plugins/vsdd-factory/agents/story-writer.md` | `grep -Ei 'CHANGELOG.+\[Unreleased\]'` | match (CHANGELOG-delivery bullet) |
| `plugins/vsdd-factory/skills/create-story/SKILL.md` | `grep -Ei 'CHANGELOG.+\[Unreleased\]'` | match (Tasks example step) |
| CHANGELOG.md first H2 | `grep -m1 -E '^## ' CHANGELOG.md` | `## [Unreleased]` |
| CHANGELOG.md after bump-version.sh | `grep -m1 -E '^## ' CHANGELOG.md` | `## [Unreleased]` (unchanged) |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (bats-guard) | All three story-pipeline authoring artifacts carry the `CHANGELOG.+[Unreleased]` token pair | bats: `plugins/vsdd-factory/tests/template-compliance.bats` — "all three story-pipeline artifacts carry the CHANGELOG delivery task" |
| (bats-guard) | `## [Unreleased]` is the first H2 heading in CHANGELOG.md | bats: `plugins/vsdd-factory/tests/template-compliance.bats` — "CHANGELOG.md carries the top-of-file [Unreleased] section the task points at" |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-001 |
| Capability Anchor Justification | CAP-001 ("Run a self-orchestrating LLM-driven SDLC pipeline") — this BC establishes the CHANGELOG delivery discipline ensuring every story PR ships its documentation artifact as part of the pipeline's completeness contract, preventing the recurring pattern where CHANGELOG entries are missed and caught adversarially post-hoc (issues #580, #714). |
| L2 Domain Invariants | none |
| Architecture Module | plugins/vsdd-factory/templates/story-template.md (SS-08), plugins/vsdd-factory/agents/story-writer.md (SS-05), plugins/vsdd-factory/skills/create-story/SKILL.md (SS-06), scripts/bump-version.sh (SS-10) |
| Stories | S-7.12 — CHANGELOG delivery discipline — retroactive BC-8.31.001 traceability and two cosmetic follow-up fixes |
| Source AC | PR #714 |
| FR | none (process governance) |

## Related BCs

- BC-8.30.001 — sibling (story template `tdd_mode` field — same template, parallel governance contract)
- BC-8.28.001 — sibling (process codification rule in rules/lessons-codification.md — same CAP-001 class)
- BC-8.28.002 — sibling (orchestrator cycle-closing checklist — same CAP-001 class)

## Architecture Anchors

- `plugins/vsdd-factory/templates/story-template.md` — task 12 in `## Tasks (MANDATORY)`
- `plugins/vsdd-factory/agents/story-writer.md` — "Each story includes:" CHANGELOG-delivery bullet
- `plugins/vsdd-factory/skills/create-story/SKILL.md` — Tasks example CHANGELOG step
- `plugins/vsdd-factory/tests/template-compliance.bats` — bats regression guards (tests 15 and 16 as of PR #714)
- `scripts/bump-version.sh` — stub inserted below `## [Unreleased]`, above first `## N.N.N` heading
- `RELEASING.md` — Step 2 drain procedure
- `CHANGELOG.md` — canonical `## [Unreleased]` top-of-file accumulation anchor

## Story Anchor

S-7.12 — CHANGELOG delivery discipline — retroactive BC-8.31.001 traceability and two cosmetic follow-up fixes

## VP Anchors

(bats guards in `plugins/vsdd-factory/tests/template-compliance.bats` — no VP-NNN IDs assigned; static-check / procedural class)

## Notes

**Gap-numbered ID:** BC-8.31.001 and BC-8.31.002 were reserved as gap-numbered expansion slots per the Wave 8 pass-1 capabilities.md F-009 annotation (`<!-- ... BC-8.31.001-002 gap-numbered for future expansion ... -->`). This BC fills the BC-8.31.001 slot as the first concrete expansion. BC-8.31.003–BC-8.31.008 remain registered as story-table candidates (no on-disk files) for docs-stories S-0.05/S-5.05/S-5.06.

**Row-level enforcement deferred:** validate-template-compliance.sh enforces at `##`-heading granularity. Enforcing the specific task-12 row would require new row-level warn-vs-block policy semantics. The PR #714 reviewer legitimately surfaced this as an ADVISORY deferral (CLAUDE.md Rule 5: "surface the decision"). This BC contracts the required behavior; a dedicated enforcement story should add a row-level CI check as the durable anti-recurrence fix.

**Cross-subsystem scope:** The contracted behavior spans SS-08 (story template), SS-05 (story-writer agent), SS-06 (create-story skill), and SS-10 (bump-version.sh script). The authoritative subsystem is SS-08 because the primary artifact being governed is a template file and the primary invariant is story-template content. The cross-subsystem modules are listed in the Architecture Module row of Traceability.

**Pre-PR #714 state:** The legacy `## Unreleased` section (unbracketed, mid-file at CHANGELOG.md line 656) was removed by PR #714 and replaced with the canonical `## [Unreleased]` anchored at the top of the file as the first H2.
