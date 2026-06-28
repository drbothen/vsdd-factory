---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-06-28T00:00:00Z
last_amended: "2026-06-28 (v1.0) — initial creation (product-owner): producer-side sprint-state.yaml per-story format obligation (S-18.11 T-2; closes O-P9-001 producer arm)."
phase: F3
inputs:
  - .factory/stories/S-18.11-sprint-state-per-story-format-producer.md
  - .factory/specs/behavioral-contracts/ss-05/BC-5.41.001.md
  - .factory/specs/behavioral-contracts/ss-05/BC-5.41.002.md
  - .factory/stories/sprint-state.yaml
  - .factory/specs/architecture/decisions/ADR-026-wave-boundary-checkpoint-reset-and-lossless-intra-wave-compaction.md
input-hash: ""
traces_to: .factory/specs/prd.md
origin: greenfield
extracted_from: null
subsystem: "SS-05"
capability: "CAP-032"
lifecycle_status: draft
introduced: v1.0-feature-context-durability-E18
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-5.41.004: sprint-state.yaml producer emits per-story `{id, status}` list under `stories:` root key — wave-ascending topo-sort order, status mechanically derived from STORY-INDEX.md, no RAG

## Description

The wave-scheduling skill (or any equivalent sprint-state.yaml producer) MUST write a top-level `stories:` key in `.factory/stories/sprint-state.yaml`. The value is a list of per-story objects; each object has at minimum `id: <S-N.NN>` and `status: <canonical-sprint-status>`. List order MUST be wave-ascending based on the dependency-graph topo-sort from STORY-INDEX.md `depends_on:` arrays — not from a phantom `wave:` story frontmatter field, which does not exist (INV-3). Every story that appears with a non-retired status in STORY-INDEX.md MUST appear in the list; status values MUST be read mechanically from STORY-INDEX.md catalog rows, never inferred or RAG-approximated (INV-4). This BC is the producer-side complement to the consumer-side obligations in BC-5.41.001 (PC2 P-SPRINT-STATE-WAVE-ORDER precondition) and BC-5.41.002 (PC3 stories derivation).

## Preconditions

1. The wave-scheduling skill (or any sprint-state.yaml writer) is invoked with intent to write or update `.factory/stories/sprint-state.yaml`.
2. STORY-INDEX.md is accessible from the current working tree and is current — its catalog rows are the authoritative source for story IDs and statuses.
3. STORY-INDEX.md `depends_on:` arrays are the authoritative source for dependency edges used to compute wave-ascending topo-sort order.
4. The producer does NOT exec any `git` command to derive story status or ordering (`git rev-parse`, `git cat-file`, etc. are forbidden for this purpose — MUST read STORY-INDEX.md directly from the working tree as a plain file).
5. No `wave:` frontmatter field exists on individual story files. Wave assignment is derived exclusively from the dependency graph.

## Postconditions

1. **`stories:` root key present**: The written `.factory/stories/sprint-state.yaml` contains a top-level `stories:` key. The value is a YAML list (not a mapping, not a scalar). An absent `stories:` key is a hard failure — the producer MUST abort with an explicit error if it cannot emit the key.

2. **Per-story entry schema**: Each entry in the `stories:` list has at minimum:
   - `id:` — string; the canonical story ID from STORY-INDEX.md (e.g., `S-18.02`). No fabricated or abbreviated IDs.
   - `status:` — string; one of the canonical sprint status values (see Invariant 1 for the exhaustive enum). The value MUST be read directly from the STORY-INDEX.md catalog row for that story. No AI-inferred, RAG-approximated, or context-estimated statuses are permitted.

3. **Wave-ascending order**: The `stories:` list is ordered by wave in ascending order. The wave assignment for each story is derived from the dependency-graph topo-sort of STORY-INDEX.md `depends_on:` arrays. Stories in wave N appear before stories in wave N+1. Within the same wave, stories are ordered by story ID (lexicographic ascending, e.g., S-18.01 before S-18.02) for determinism. This ordering satisfies the **P-SPRINT-STATE-WAVE-ORDER** precondition of BC-5.41.001 PC2 so that the leading-contiguous-terminal-run algorithm can operate unambiguously.

   **Tie-break rule**: If two stories have the same wave-level ordinal (neither depends on the other) and the topo-sort cannot discriminate, order by story ID string lexicographic ascending. This is the canonical tie-break; the SKILL.md behavioral step MUST document it.

4. **Completeness — no non-retired story omitted**: Every story that appears in STORY-INDEX.md with a non-retired status MUST appear in the `stories:` list. A story with `status: retired` in STORY-INDEX.md MUST be omitted from the list (retired stories are not part of the active sprint corpus). If a story appears in STORY-INDEX.md but its status cannot be determined (parse failure on the catalog row), the producer MUST abort with a hard error listing the unresolvable story IDs rather than silently omit or invent a status.

5. **Legacy `epics:` section compatibility**: Any existing `epics:` top-level section in sprint-state.yaml MAY co-exist with the new `stories:` section. The `epics:` section is NOT the format consumed by wave-gate/wave-handoff consumers (BC-5.41.001 PC2/PC3, BC-5.41.002 PC3). If an `epics:` section is present, the producer MUST NOT mutate it as part of the `stories:` list write (preserve backward-compatibility). The resulting YAML MUST remain well-formed and parseable.

6. **YAML well-formedness**: The written sprint-state.yaml MUST be syntactically valid YAML. The producer MUST NOT write a `stories:` block that breaks YAML parsing of the rest of the file (including any legacy `epics:` section). A YAML-invalid output is a hard failure.

## Invariants

1. **Canonical status enum**: The only permitted values for `status:` fields in the `stories:` list are the canonical sprint statuses as authoritative-sourced from STORY-INDEX.md catalog rows — exactly these **8 values**:
   `draft`, `ready`, `in-progress`, `partial`, `blocked`, `merged`, `withdrawn`, `cancelled`.
   Rationale: the enum mirrors STORY-INDEX.md's documented per-story taxonomy — `merged`, `draft`, `partial`, `withdrawn` are actual observed values; `ready`, `in-progress`, `blocked` are documented non-terminal states; `cancelled` is an ADR-026-recognized terminal state (forward-compatible). `completed` and `pending` are NOT in this enum because STORY-INDEX.md never uses them. `retired` stories are omitted from the `stories:` list entirely (PC4) — `retired` is not a status value in the list.

   **Terminal vs. non-terminal annotation:**
   - **Terminal statuses** (story is done; wave-close eligible): `merged`, `withdrawn`, `cancelled`.
   - **Non-terminal active statuses** (story is still in flight): `draft`, `ready`, `in-progress`, `partial`, `blocked`.
   - **Next-wave selector** (consumed by BC-5.41.002 PC3): only `draft`. The statuses `ready`, `in-progress`, `partial`, and `blocked` are non-terminal but are NOT next-wave selectors — if `sprint-state.yaml` contains no `draft` stories (and no stories are `pending` per BC-5.41.002's reserved arm), BC-5.41.002 PC3 raises BrokenSprintState. BC-5.41.001 PC2 classifies terminal vs. non-terminal for the leading-contiguous-terminal-run algorithm; this BC's producer obligation is to reflect STORY-INDEX.md statuses faithfully so those consumer-side algorithms can operate correctly.

   Any value outside this 8-value set that appears in STORY-INDEX.md constitutes an unknown status token; the producer MUST abort with a hard error identifying the story ID and the unknown status token, not silently pass it through (see EC-007). This BC treats STORY-INDEX.md as the authority; if STORY-INDEX.md ever shows a status not in this enum, that is a STORY-INDEX defect surfaced as a hard abort.

2. **No fabricated statuses**: Each `status:` value in the `stories:` list MUST be a direct read from the STORY-INDEX.md catalog row for that story — specifically the `| S-NNN | ... | <status> |` pipe-delimited column. No AI-inferred, context-estimated, RAG-retrieved, or environment-variable-based status values are permitted. This rule is the producer-side complement to BC-5.41.002 INV4 (no RAG on the consumer side).

3. **No phantom `wave:` field**: The topo-sort ordering MUST be derived from STORY-INDEX.md `depends_on:` dependency edges, NOT from a `wave:` frontmatter field on individual story files. No such field exists on story specs. Consulting `wave:` frontmatter produces undefined/absent values that silently corrupt the ordering. The producer MUST implement the Kahn/DFS dependency-order topo-sort over STORY-INDEX.md `depends_on:` arrays.

4. **No git exec for status or ordering**: The producer MUST derive all `status:` values and topo-sort edges by reading STORY-INDEX.md as a plain working-tree file. Commands such as `git log`, `git cat-file`, or `git rev-parse` MUST NOT be used to derive story status or ordering. Shell POSIX tools (`awk`, `grep -E`, `sort`) are permitted for YAML key extraction and sorting.

5. **Idempotent re-generation**: Given the same STORY-INDEX.md state, two invocations of the producer MUST produce a `stories:` list with byte-identical story IDs, statuses, and ordering (modulo any `last_updated:` or timestamp fields that are explicitly allowed to drift). Idempotency ensures that a round-trip verification script can reliably compare producer output to STORY-INDEX.md without false positives.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | A story in STORY-INDEX.md is absent from the `stories:` list (but has a non-retired status) | Hard failure: producer aborts with a list of all missing story IDs before writing anything; AC-003 round-trip verification fails this story |
| EC-002 | `status:` value in a `stories:` entry does not match the STORY-INDEX.md catalog row for that story | Hard failure: producer aborts with a report of mismatched story IDs (expected vs actual); AC-003 round-trip verification catches this |
| EC-003 | Wave-ordering is ambiguous — two stories in the same wave, neither depends on the other, tie in dependency ordinal | Use story ID lexicographic ascending as the canonical tie-break; SKILL.md behavioral step MUST document the tie-break rule so it is reproducible |
| EC-004 | Legacy `epics:` section is present in sprint-state.yaml after migration | Permitted; consumers (BC-5.41.001 PC2/PC3, BC-5.41.002 PC3) read only the `stories:` root key; the `epics:` section is ignored by those consumers; producer MUST NOT mutate `epics:`; YAML must remain parseable |
| EC-005 | `stories:` list is empty — all stories have terminal status (merged/withdrawn/cancelled) | Valid EPIC-COMPLETE signal per BC-5.41.002 PC3 EC-001a; consumer wave-handoff MUST exit 0 and write HANDOFF.md with `epic_status: complete` (ADR-026 §Decision 8); producer writes `stories: []` without error |
| EC-006 | One or more stories have `status: in-progress` (non-terminal, active-but-not-next-wave) with no `draft` stories remaining | Producer writes the `stories:` list including the `in-progress` story as-is (producer obligation is correct reflection of STORY-INDEX.md state); the BrokenSprintState hard error is the CONSUMER'S responsibility (BC-5.41.002 PC3 EC-001b) — the producer does NOT pre-filter or block on consumer-side semantic rules |
| EC-007 | STORY-INDEX.md catalog row for a story has an unknown status value (outside the canonical 8-value enum: `draft`, `ready`, `in-progress`, `partial`, `blocked`, `merged`, `withdrawn`, `cancelled`) | Hard failure: producer aborts with `UnknownStatusToken: story <ID> has status '<value>' not in canonical enum`; producer does NOT pass through unknown tokens or substitute a default. Specifically: `completed`, `closed`, `pending`, and malformed `tier-*` tokens all trigger EC-007 hard-abort — these are not in the STORY-INDEX.md taxonomy. If STORY-INDEX.md ever emits one of these values, that is a STORY-INDEX defect surfaced as a producer hard abort, not a silently passed-through value. |
| EC-008 | `depends_on:` array for a story references a story ID not present in STORY-INDEX.md | Hard failure: producer aborts with `UnresolvableDependency: story <ID> depends_on <missing-ID> which is not in STORY-INDEX.md`; ordering cannot be computed safely without resolving all edges |
| EC-009 | Producer is invoked with no STORY-INDEX.md accessible from the working tree | Hard failure: producer aborts immediately with `StoryIndexNotFound: STORY-INDEX.md not found at expected path`; no partial sprint-state.yaml write occurs |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| STORY-INDEX.md with 3 non-retired stories: S-1.01 (merged, no deps), S-1.02 (draft, depends_on: S-1.01), S-1.03 (ready, depends_on: S-1.01) | sprint-state.yaml `stories:` list: `[{id: S-1.01, status: merged}, {id: S-1.02, status: draft}, {id: S-1.03, status: ready}]` (S-1.01 wave 1, S-1.02 and S-1.03 wave 2; within wave 2, S-1.02 < S-1.03 lexicographic) | happy-path |
| All stories in STORY-INDEX.md have terminal status: all merged | `stories: []` (empty list, valid EPIC-COMPLETE); no BrokenSprintState from producer | epic-complete-empty-list |
| STORY-INDEX.md has story S-99.01 with `status: in-progress` and no `draft` stories | `stories: [{id: S-99.01, status: in-progress}]` produced correctly; BrokenSprintState is consumer's concern (BC-5.41.002 PC3 EC-001b), not producer's | in-progress-producer-passthrough |
| Existing sprint-state.yaml has a top-level `epics:` section; producer writes new `stories:` list | YAML file contains both `epics:` section (unchanged) and new `stories:` list; YAML remains parseable | legacy-epics-coexistence |
| STORY-INDEX.md has story S-18.02 with `status: xyzzy` (unknown token, not in the canonical 8-value enum) | Hard abort: `UnknownStatusToken: story S-18.02 has status 'xyzzy' not in canonical enum`; no file written | unknown-status-hard-abort |
| STORY-INDEX.md has story S-18.02 with `status: completed` (removed from enum per STORY-INDEX-grounded correction) | Hard abort: `UnknownStatusToken: story S-18.02 has status 'completed' not in canonical enum`; no file written | completed-is-unknown-hard-abort |
| STORY-INDEX.md has story S-18.03 with `status: pending` (not a STORY-INDEX value; reserved in BC-5.41.002 only) | Hard abort: `UnknownStatusToken: story S-18.03 has status 'pending' not in canonical enum`; no file written | pending-is-unknown-hard-abort |
| Producer invoked; STORY-INDEX.md not found | Hard abort: `StoryIndexNotFound: STORY-INDEX.md not found`; no file written | story-index-missing-hard-abort |
| Two stories in same wave (S-2.01 and S-2.02, neither depends on the other) | Ordered S-2.01 before S-2.02 (lexicographic tie-break); order is deterministic across two invocations | tie-break-determinism |
| Round-trip verification: producer writes `stories:` list; verification script compares each `stories[*].status` to STORY-INDEX.md catalog row | All status values match; verification exits 0 | round-trip-verification |

## Related BCs

- BC-5.41.001 — depends on: BC-5.41.001 PC2 (wave_id derivation from sprint-state.yaml) requires the P-SPRINT-STATE-WAVE-ORDER precondition; this BC is the producer obligation that establishes that precondition
- BC-5.41.002 — depends on: BC-5.41.002 PC3 (stories list derived from sprint-state.yaml `status: pending` or `status: draft` entries ordered by dependency graph) requires the per-story `{id, status}` format this BC mandates on the producer
- BC-5.41.003 — sibling: both are SS-05 behavioral contracts in the context-durability family

## Architecture Anchors

- `plugins/vsdd-factory/skills/wave-scheduling/SKILL.md` — the producer skill; behavioral step mandating per-story `stories:` list emission (S-18.11 T-5 deliverable; MUST cite BC-5.41.002 PC3 as format authority)
- `.factory/stories/sprint-state.yaml` — the artifact written by this BC's producer obligation (S-18.11 T-4 deliverable)
- ADR-026 §Decision 3 — wave-state.yaml is derived from sprint-state.yaml; this BC governs the sprint-state.yaml format that makes that derivation possible
- ADR-026 §Decision 8 — EPIC-COMPLETE detection via `next_wave_stories: []`; the EPIC-COMPLETE case (EC-005) arises when the producer writes `stories: []`
- **Subsystem SS-05/SS-06 split (following O-P29-002 pattern documented in BC-5.41.001/BC-5.41.002):** This BC's `subsystem: SS-05` (Pipeline Orchestration) anchors the orchestration LOGIC and format contract (sprint-state.yaml schema, topo-sort obligation, status-fidelity rule). The skill FILE artifact (`wave-scheduling/SKILL.md`) resides in SS-06 (Skill Catalog). This split is intentional and consistent with sibling BCs in this group.

## Story Anchor

S-18.11 (sprint-state.yaml producer migration to per-story {id, status} format)

## VP Anchors

- VP-TBD — sprint-state.yaml producer emits conformant per-story `{id, status}` list and wave-handoff can parse it (integration VP; to be authored by architect/test-writer as part of S-18.11 T-1/T-6)

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | sprint-state.yaml producer emits per-story `{id, status}` list with wave-ascending topo-sort order, statuses matching STORY-INDEX.md catalog rows, and wave-handoff consumers (BC-5.41.001 PC2, BC-5.41.002 PC3) can parse the output without error | integration (bats round-trip; S-18.11 test file `sprint-state-format.bats`) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-032 ("Guarantee lossless context-window transitions via wave-boundary checkpoint and PreCompact flush") per capabilities.md §CAP-032 |
| Capability Anchor Justification | CAP-032 ("Guarantee lossless context-window transitions via wave-boundary checkpoint and PreCompact flush") per capabilities.md §CAP-032 — this BC governs the sprint-state.yaml producer format that enables the wave-gate consumers (BC-5.41.001 PC2, BC-5.41.002 PC3) to mechanically derive `wave_id` and `next_wave_stories` without a bespoke format adapter; a conformant sprint-state.yaml producer is a prerequisite for lossless wave-boundary handoffs (Part A of the CAP-032 context-durability guarantee) |
| L2 Domain Invariants | DI-020 (Wave/phase boundary transitions must not lose load-bearing pipeline state — enforced here by the producer-side obligation: a non-conformant sprint-state.yaml breaks the wave-handoff chain, losing pipeline state at wave boundaries); DI-023 (Wave/phase identity and next-wave story lists derive from real persisted substrate fields; no phantom fields — enforced here by the topo-sort-from-STORY-INDEX obligation: ordering derived from `depends_on:` real dependency edges, NOT from phantom `wave:` frontmatter) |
| Architecture Module | SS-05 (Pipeline Orchestration) — wave-scheduling skill producer |
| ADR | ADR-026 §Decision 3 (wave-state.yaml derived from sprint-state.yaml; requires per-story format this BC mandates); ADR-026 §Decision 8 (EPIC-COMPLETE via `next_wave_stories: []`; arises from EC-005 empty `stories:` list on this BC) |
| Stories | S-18.11 |
| Cycle | v1.0-feature-context-durability-E18 (F3) |
| Feature | issue #173 / E-18 |

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| v1.0 | 2026-06-28 | product-owner | Initial creation (S-18.11 T-2; closes O-P9-001 producer arm). BC-5.41.004: producer-side sprint-state.yaml per-story {id, status} format obligation. INV-1 canonical status enum corrected to STORY-INDEX-grounded 8-value set (architect-adjudicated, S-18.11): `draft`, `ready`, `in-progress`, `partial`, `blocked`, `merged`, `withdrawn`, `cancelled`. Removed `completed` and `pending` (never used in STORY-INDEX.md); added `ready` (observed STORY-INDEX value). Terminal = {merged, withdrawn, cancelled}; non-terminal active = {draft, ready, in-progress, partial, blocked}; next-wave selector = {draft} only. EC-007 updated to cite the corrected 8-value enum and explicitly name `completed`, `closed`, `pending`, and `tier-*` tokens as hard-abort triggers. Two additional test vectors added (completed-is-unknown-hard-abort, pending-is-unknown-hard-abort). |
