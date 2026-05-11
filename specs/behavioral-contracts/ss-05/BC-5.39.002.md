---
document_type: behavioral-contract
level: L3
version: "1.1"
status: draft
producer: product-owner
timestamp: 2026-05-06T00:00:00Z
phase: 1a
inputs:
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/F1-delta-analysis.md
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md
input-hash: "[pending-recompute]"
traces_to: .factory/cycles/v1.0-feature-engine-discipline-pass-1/F1-delta-analysis.md
origin: greenfield
extracted_from: null
subsystem: "SS-05"
capability: "CAP-005"
lifecycle_status: active
introduced: v1.0-feature-engine-discipline-pass-1
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-5.39.002
section: "5.39"
last_amended: 2026-05-09
---

# BC-5.39.002: Per-story adversary scope MUST be limited to story diff, spec, and anchored BCs; out-of-scope findings MUST be deferred

## Description

The adversary agent's per-story review scope is bounded to exactly three information sources:
(a) the story worktree diff against `develop`, (b) the story spec file, and (c) the BCs
anchored to the story via the story's `bcs:` frontmatter array. Any finding that requires
knowledge outside this scope — including cross-story implications, integration concerns,
system-level design decisions, or architectural choices — MUST be tagged as a deferred
finding in the `deferred_findings` array of the convergence state file and routed to either
wave-gate or Phase-5 review. Deferred findings MUST NOT block per-story convergence.

## Preconditions

1. The adversary agent has been dispatched for a per-story convergence loop pass (BC-5.39.001).
2. The story worktree diff against `develop` is computable via `git diff develop...HEAD` in the story worktree.
3. The story spec file is available at `.factory/specs/stories/<story-id>.md` (or equivalent path).
4. The story's `bcs:` frontmatter array lists the BC files anchored to this story.
5. The convergence state file at `.factory/cycles/<cycle-id>/<story-id>/adversary-convergence-state.json` is writable.

## Postconditions

1. The adversary agent ONLY consumes: (a) the story diff, (b) the story spec, and (c) each
   BC file listed in the story's `bcs:` frontmatter array. The adversary does NOT load:
   full codebase context, other stories' specs, PRD sections not referenced in the story spec,
   or architecture documents not directly cited by the anchored BCs.
2. For each finding the adversary generates, it MUST classify the finding's scope:
   - `in-scope`: finding applies solely to the story's diff + spec + anchored BCs.
   - `cross-story`: finding requires context from another story's scope.
   - `integration`: finding requires knowledge of how multiple stories or subsystems interact.
   - `system-level`: finding concerns system-wide behavior not representable in a single story diff.
   - `architectural`: finding concerns design decisions that span the architectural boundary.
3. In-scope findings are included in `last_finding_count` and contribute to `last_classification`.
4. Out-of-scope findings (categories: `cross-story`, `integration`, `system-level`, `architectural`)
   are placed in `deferred_findings` with:
   - `finding_id`: a stable identifier (e.g., `F-<story-id>-<seq>`)
   - `category`: one of the four out-of-scope categories
   - `target`: `"wave-gate"` (for cross-story/integration) or `"phase-5"` (for system-level/architectural)
   - `note`: a one-sentence description of the finding
5. Out-of-scope findings are NOT counted in `last_finding_count` and do NOT affect `last_classification`.
6. `passes_clean` increments when ALL in-scope findings are `NITPICK_ONLY`, regardless of
   the number of deferred out-of-scope findings.
7. At wave-gate, the `deferred_findings` arrays from all stories in the wave are aggregated
   and reviewed as a batch (enforced by BC-4.10.001).

## Invariants

1. The adversary agent MUST NOT load files outside the three defined scope sources. Scope
   creep (loading architecture files, PRD sections, other story specs) is a BC violation.
2. The four out-of-scope categories are exhaustive. A finding that does not fit any of the
   four categories MUST be treated as in-scope (conservative default).
3. The `target` field in `deferred_findings` has exactly two valid values:
   - `"wave-gate"` — cross-story and integration findings that affect the current wave
   - `"phase-5"` — system-level and architectural findings for cycle-end review
4. The `finding_id` in `deferred_findings` is unique within the story's convergence state file.
   Duplicate finding IDs are a schema violation.
5. Deferred findings are never retroactively removed from the `deferred_findings` array,
   even if a subsequent pass determines the finding no longer applies.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | All findings in a pass are cross-story deferred | `last_finding_count: 0`, `last_classification: NITPICK_ONLY`, `passes_clean` increments. Deferred findings appended. |
| EC-002 | Adversary finds both in-scope and cross-story findings | In-scope finding determines `last_classification`. Cross-story finding is deferred. Both recorded. |
| EC-003 | Adversary attempts to load the full PRD | VIOLATION. The adversary must be constrained to the story spec + anchored BCs. If a PRD section is needed, it must be cited explicitly in the story spec or an anchored BC. |
| EC-004 | A deferred finding from pass 1 is resolved by the implementer in pass 2 | The finding remains in `deferred_findings` (append-only). The adversary notes the resolution in a new pass 2 entry with a note. |
| EC-005 | Adversary cannot classify a finding as in-scope or out-of-scope | Default to in-scope (conservative). Annotate with `category: in-scope` and note the ambiguity. |
| EC-006 | `finding_id` collision in `deferred_findings` | Schema violation. Adversary must generate a unique ID (append sequence suffix). The state file is malformed if duplicates are written. |

## Canonical Test Vectors

| Input State | Finding Types | Expected deferred_findings | passes_clean delta | Decision |
|-------------|--------------|---------------------------|-------------------|----------|
| Story diff has a trivial comment typo | In-scope NITPICK | [] | +1 | PASS (if 3rd consecutive) |
| Story diff has a missing invariant enforcement | In-scope HIGH | [] | reset to 0 | BLOCK |
| Story diff exposes a cross-story data contract gap | cross-story, target: wave-gate | [{finding_id: "F-S-A-001", category: "cross-story", target: "wave-gate", note: "..."}] | +1 (in-scope was NITPICK_ONLY) | Deferred; PASS if 3rd |
| Story diff has an architectural concern about registry design | architectural, target: phase-5 | [{finding_id: "F-S-A-002", category: "architectural", target: "phase-5", note: "..."}] | +1 | Deferred; PASS if 3rd |
| Adversary loads PRD instead of story spec | Out-of-scope context load | (invariant violation — test asserts adversary did not load PRD) | N/A — violation | SCOPE VIOLATION |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (static-check) | Adversary agent system prompt cites BC-5.39.002 scope constraints | grep-based static check on agents/adversary.md |
| (integration) | Deferred findings do not affect `last_classification` or `passes_clean` | integration test: inject cross-story finding; assert `passes_clean` increments |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-005 |
| Capability Anchor Justification | CAP-005 ("Run adversarial review with information asymmetry") per capabilities.md §CAP-005 — this BC operationalizes the "information asymmetry" aspect of CAP-005: the adversary's bounded scope (diff + spec + anchored BCs only) is the mechanism that ensures the adversary operates with asymmetric context relative to the implementer. Without scope bounding, the adversary would accumulate the same context as the implementer, defeating the information asymmetry guarantee. |
| L2 Domain Invariants | none |
| Architecture Module | agents/adversary.md (scope constraints); .factory/cycles/<cycle-id>/<story-id>/adversary-convergence-state.json (deferred_findings schema) |
| Stories | S-12.01 |
| FR | FR-047 (per-story adversarial convergence + artifact path discipline — to be added in PRD delta) |

## Related BCs

- BC-5.39.001 — parent (loop mechanics; this BC governs the scope constraint that BC-5.39.001's loop enforces)
- BC-4.10.001 — composes with (WASM hook that aggregates deferred_findings at wave-gate)
- BC-5.36.005 — sibling (partial-fix-regression check within each pass; complementary to scope bounding)
- BC-5.36.006 — sibling (fix propagation scope check; complementary)

## Architecture Anchors

- `agents/adversary.md` — adversary agent definition; scope constraint paragraph must be added citing this BC
- `.factory/cycles/<cycle-id>/<story-id>/adversary-convergence-state.json` — `deferred_findings` array schema
- `plugins/vsdd-factory/workflows/phases/per-story-delivery.md` — Step 4.5 scope constraint definition

## Story Anchor

Story A — v1.0-feature-engine-discipline-pass-1 (F3 story decomposition)

## VP Anchors

(integration-test — deferred finding non-interference)

## Changelog

| Version | Date | Description |
|---------|------|-------------|
| 1.1 | 2026-05-09 | F-P45-001 — Traceability Stories row propagated from BC-INDEX v1.57: "Story A" placeholder → S-12.01. BC-INDEX was updated in fix-burst-39 (v1.55) to replace TBD; body was not updated in that burst. Refs: F-P45-001, fix-burst-42. |
| 1.0 | 2026-05-06 | Initial authoring (product-owner; F2 phase of v1.0-feature-engine-discipline-pass-1). OQ6 resolution applied: adversary agent owns convergence assessment. |
