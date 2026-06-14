---
document_type: behavioral-contract
level: L3
version: "1.2"
status: draft
producer: product-owner
timestamp: 2026-06-14T00:00:00Z
last_amended: "2026-06-14 (v1.2) — F2 pass-2 fix-burst: (F-P2-004) PC3 EPIC-COMPLETE exception added (empty next_wave_stories AND all stories terminal → exit 0, HANDOFF epic_status:complete, no wave-state.yaml); BrokenSprintState hard error retained for empty AND any non-terminal story; EC-001 split into EC-001a (EPIC-COMPLETE) + EC-001b (BrokenSprintState); test vectors updated. [Prior: 2026-06-14 (v1.1) — F2 pass-1 fix-burst: (F-3) PC3 re-anchored: stories list derives from sprint-state.yaml `status: pending` OR `status: draft` entries ordered by dependency graph (not from phantom `wave:` story frontmatter field which does not exist). PC3 'no phantom' mandate explicit. Empty list is HARD ERROR per SOUL.md §4 — Postcondition 3 updated and EC-001 changed from 'valid' to hard block. (DI) TBD-DI replaced with DI-023. TBD-VP retained with justification per report.]"
phase: F2
inputs:
  - .factory/feature-delta/issue-173/F1-delta-analysis.md
  - .factory/specs/architecture/decisions/ADR-026-wave-boundary-checkpoint-reset-and-lossless-intra-wave-compaction.md
input-hash: "c2426d5"
traces_to: .factory/specs/prd.md
origin: greenfield
extracted_from: null
subsystem: "SS-05"
capability: "CAP-032"
lifecycle_status: draft
introduced: v1.0-feature-context-durability-E18
modified:
  - "2026-06-14 (v1.2) — F2 pass-2 fix-burst: PC3 EPIC-COMPLETE exception (all terminal → exit 0 + HANDOFF epic_status:complete, no wave-state.yaml); EC-001 split EC-001a+EC-001b; test vectors updated."
  - "2026-06-14 (v1.1) — F2 pass-1 fix-burst: PC3 stories derivation re-anchored (sprint-state.yaml status:pending/draft + dependency-order; no phantom wave: frontmatter); empty list → HARD ERROR; EC-001 updated; TBD-DI replaced with DI-023; ADR cite v1.0→v1.1."
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-5.41.002: wave-gate produces curated wave-state.yaml manifest listing next-wave stories and spec deps — no RAG

## Description

At wave close, alongside HANDOFF.md (BC-5.41.001), the `wave-gate` / `wave-handoff` skill produces a `wave-state.yaml` manifest on the `factory-artifacts` branch. The manifest is a deterministic, curated enumeration of the next wave's stories and the exact spec files (BC files, ADR files, SS-NN files) they depend on. It is the authoritative rehydration vehicle for the `rehydrate-wave` skill (BC-6.24.001). RAG over the spec corpus is explicitly deferred (ADR-026 Decision 4 v2 deferral).

## Preconditions

1. HANDOFF.md has been produced and verified (BC-5.41.001 postconditions satisfied).
2. `STORY-INDEX.md` is current and accessible from `factory-artifacts`.
3. Each next-wave story's frontmatter contains `spec_files:` or equivalent dependency references.
4. The operator has reviewed the manifest contents before wave-close commit (or the skill generates and surfaces them for review).

## Postconditions

1. **wave-state.yaml written**: A `wave-state.yaml` file is written to the `factory-artifacts` branch at the same commit as `HANDOFF.md` (atomic write; single commit covers both artifacts).

2. **Required fields present**:
   - `wave_id` — integer; next wave number (current wave + 1)
   - `generated_at` — ISO-8601 timestamp
   - `generated_from_handoff_sha` — 40-char hex SHA of the HANDOFF.md commit on factory-artifacts
   - `stories` — list of story objects; each with `{id, status, spec_files: [<path>, ...]}`. At minimum includes all stories assigned to wave N+1 in STORY-INDEX.md.
   - `arch_files` — list of architecture file paths always included in rehydration context (ARCH-INDEX.md, directly referenced ADRs)
   - `state_pointer` — literal string `.factory/STATE.md`

3. **Stories list is derived mechanically from real substrate**: The stories in `wave-state.yaml` are derived from `sprint-state.yaml` by selecting entries with `status: pending` OR `status: draft`, then applying the dependency-order graph from STORY-INDEX.md `depends_on:` arrays to produce the wave sequence. This is the SAME algorithm used by the `wave-scheduling` skill's topological sort step. No `wave:` frontmatter field on story files is referenced — that field does not exist. Each story's `spec_files` list is derived from that story's `bcs:` frontmatter array (resolved to file paths) and any explicitly declared `arch_deps:` entries. **Empty stories list handling — two cases**:
   - **EPIC-COMPLETE exception**: if `sprint-state.yaml` has no entries with `status: pending` or `status: draft` AND all entries have a terminal status (merged, withdrawn, or cancelled), this is the final wave. `wave-handoff` MUST exit 0, write HANDOFF.md with `epic_status: complete` (and `next_wave_stories: []`), and NOT write `wave-state.yaml`. This is a legitimate wave-close.
   - **BrokenSprintState hard error**: if `sprint-state.yaml` has no entries with `status: pending` or `status: draft` BUT one or more entries have a non-terminal, non-pending status (e.g., `status: in_progress` or a story that is neither merged/withdrawn/cancelled/pending/draft), `wave-handoff` MUST abort with exit 1 and an explicit error message: "No next-wave stories found in sprint-state.yaml but non-terminal stories exist — sprint-state.yaml needs updating." A silent no-op or an empty `wave-state.yaml` with `stories: []` written silently is a SOUL.md §4 violation (SOUL.md #4: silent failures are forbidden).

4. **No RAG**: The manifest does not use semantic retrieval. Every path in `spec_files` is a literal filesystem path that must resolve on the `factory-artifacts` branch or the working tree. Paths that do not resolve produce a warning at generation time (not a hard block, since some spec files may be in-progress).

5. **arch_files minimum set**: Must always include:
   - `.factory/specs/architecture/ARCH-INDEX.md`
   - `.factory/specs/architecture/decisions/ADR-026-wave-boundary-checkpoint-reset-and-lossless-intra-wave-compaction.md`
   - `.factory/specs/architecture/decisions/ADR-025-single-writer-factory-locklease-prevent-concurrent-session-races-on-factory-artifacts-orphan-branch.md`
   - Any ADR directly referenced by a story in `stories[].spec_files`

6. **Commit atomicity**: `wave-state.yaml` and `HANDOFF.md` are committed in a single git commit to `factory-artifacts` with message: `HANDOFF wave-<N> <ISO-timestamp>`. They are never committed separately.

## Invariants

1. **wave-state.yaml is the sole rehydration vehicle**: The `rehydrate-wave` skill reads ONLY from `wave-state.yaml` to determine which specs to load. No other source (in-context state, BC-INDEX grep, story list guessing) is used for rehydration.

2. **Manifest is deterministic given STORY-INDEX.md state**: Two invocations of wave-handoff on the same STORY-INDEX.md state must produce byte-identical `stories` and `arch_files` lists (modulo `generated_at` timestamp and `generated_from_handoff_sha`).

3. **No phantom stories**: Only stories with `status: pending` or `status: draft` in `sprint-state.yaml` appear in the manifest, ordered by the dependency graph. No phantom `wave:` frontmatter field on story files is used — that field does not exist. Stories whose IDs do not appear in STORY-INDEX.md must not appear in the manifest.

4. **RAG exclusion is mandatory**: Any code path that performs semantic vector retrieval over the spec corpus to populate `wave-state.yaml` is a specification violation. The manifest is curated and mechanical.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001a | `sprint-state.yaml` has no entries with `status: pending` or `status: draft`; ALL other entries have terminal status (merged, withdrawn, cancelled) — EPIC-COMPLETE | Exit 0; write HANDOFF.md with `epic_status: complete` and `next_wave_stories: []`; do NOT write `wave-state.yaml`; wave declared complete |
| EC-001b | `sprint-state.yaml` has no entries with `status: pending` or `status: draft`; one or more entries have non-terminal status (e.g., `in_progress`, `in_review`) — BrokenSprintState | HARD ERROR: exit 1; explicit error message "No next-wave stories found in sprint-state.yaml but non-terminal stories exist — sprint-state.yaml needs updating."; no `wave-state.yaml` written; operator must correct sprint-state.yaml |
| EC-002 | A story's `bcs:` frontmatter references a BC path that does not exist | Warning logged; path included in `spec_files` with `status: missing`; not a hard block |
| EC-003 | Story has no `spec_files` derivable (no `bcs:` frontmatter, no arch_deps) | Story included in `stories` list with `spec_files: []`; operator warned to add dependencies |
| EC-004 | `generated_from_handoff_sha` cannot be computed (HANDOFF.md commit not yet visible) | Hard block; wave-state.yaml must not be written without the HANDOFF.md SHA |
| EC-005 | Operator adds a manual `spec_files` override for a story | Permitted if the override mechanism is explicit (e.g., story frontmatter `extra_spec_files:`); mechanically merged with derived list |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| sprint-state.yaml: S-18.02 status=pending, S-18.03 status=draft; STORY-INDEX.md has both; wave-handoff invoked | wave-state.yaml: `wave_id: <next-wave>`, `stories: [{id: S-18.02, ...}, {id: S-18.03, ...}]` (dependency-ordered) | happy-path |
| S-18.02 bcs: [BC-4.14.001, BC-5.41.001] | S-18.02 spec_files includes `.factory/specs/behavioral-contracts/ss-04/BC-4.14.001.md` and `ss-05/BC-5.41.001.md` | spec-derivation |
| No `status: pending` or `status: draft` entries in sprint-state.yaml; all entries terminal (merged/withdrawn/cancelled) | Exit 0; HANDOFF.md with `epic_status: complete`; no wave-state.yaml written | epic-complete |
| No `status: pending` or `status: draft` entries in sprint-state.yaml; one entry `status: in_progress` | HARD ERROR; exit 1; BrokenSprintState error message; no wave-state.yaml written | broken-sprint-state |
| wave-state.yaml and HANDOFF.md in same commit | single git commit on factory-artifacts | atomicity |

## Related BCs

- BC-5.41.001 — sibling: HANDOFF.md is co-committed with wave-state.yaml in the same atomic commit
- BC-6.24.001 — depends on: rehydrate-wave skill reads wave-state.yaml produced by this BC
- BC-5.40.001 — depends on: factory lock must be held during commit; lock renewal invoked per ADR-025 D11

## Architecture Anchors

- `plugins/vsdd-factory/skills/wave-handoff/SKILL.md` — NEW skill; produces both HANDOFF.md and wave-state.yaml at wave close (S-18.01 deliverable)
- ADR-026 §Decision 4 — wave-state.yaml schema specification (minimum required fields)

## Story Anchor

S-18.01 (HANDOFF.md schema + wave-handoff skill)

## VP Anchors

TBD-VP — no dedicated VP assigned at F2 for wave-state.yaml production. Justification for deferral: this BC's core atomicity property (wave-state.yaml + HANDOFF.md in a single commit) is already covered by VP-081 (which verifies the wave-gate close preconditions holistically). A separate VP for wave-state.yaml production would overlap VP-081's integration scope. Story-writer and test-writer assign a standalone integration VP at F3 if the BC-5.41.002 tests require a distinct VP ID for traceability. Flagged to architect for VP allocation decision.

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| TBD-VP | wave-state.yaml is produced atomically with HANDOFF.md in a single commit; stories list derived from sprint-state.yaml `status:pending/draft` entries ordered by dependency graph (no phantom `wave:` field; no RAG); empty list → hard error (non-zero exit, no file written) | integration |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-032 ("Guarantee lossless context-window transitions via wave-boundary checkpoint and PreCompact flush") per capabilities.md §CAP-032 |
| Capability Anchor Justification | CAP-032 ("Guarantee lossless context-window transitions via wave-boundary checkpoint and PreCompact flush") per capabilities.md §CAP-032 — this BC specifies the curated rehydration manifest that enables deterministic session rehydration after a wave-boundary reset (ADR-026 Decision 4); deterministic rehydration is the direct complement to wave-boundary hard reset, together forming the complete context-durability guarantee |
| L2 Domain Invariants | DI-023 (Wave/phase identity and next-wave story lists derive from real persisted substrate fields; no phantom fields — enforced by stories derivation from sprint-state.yaml `status:pending/draft` + dependency-order, not from phantom `wave:` story frontmatter; empty list = hard error per SOUL.md §4) |
| Architecture Module | SS-05 (Pipeline Orchestration) — wave-handoff skill |
| ADR | ADR-026 v1.1 Decision 4 (wave-state.yaml curated manifest; RAG explicitly deferred; next_wave_stories derived from sprint-state.yaml status:pending/draft entries + dependency-order; empty list = hard error) |
| Stories | S-18.01 |
| Cycle | v1.0-feature-context-durability-E18 (F2) |
| Feature | issue #173 / E-18 |
