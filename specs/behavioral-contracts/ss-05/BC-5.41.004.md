---
document_type: behavioral-contract
level: L3
version: "1.4"
status: draft
producer: product-owner
timestamp: 2026-06-28T00:00:00Z
last_amended: "(v1.4) — PC3 intra-partition ordering switched from partition-restricted edges (def a) to full-graph wave-depth (def b) per ADR-026 §Decision 3a v1.37 + human directive 'full rigor option b'; EC-010 detection unchanged, depth basis now full-graph; supersession-edge note updated to reflect edge INCLUDED in depth computation (not excluded from topo-sort). [Prior: (v1.3) — EC-010 narrowed to tolerate supersession edges (depended-on story carries superseded_by) + hard-abort genuine anomalies; PC3 cross-partition supersession-edge note added; +2 test vectors (topo-violation-genuine-anomaly, topo-violation-supersession-tolerated); per ADR-026 §Decision 3a v1.36, human directive — S-3.04 stays partial pending post-E-18 ADR-015 revisit. [Prior: (v1.2) — PC3 amended to two-partition ordering rule per ADR-026 §Decision 3a (S-18.11 PC3-vs-guard reconciliation, human-approved); TopoViolation guard obligation added as new EC-010. [Prior: (v1.1) — Architecture Anchors/Related BCs producer-authority corrected to BC-5.41.004 per AC-007 (F-P5-001); Description INV-cite INV-4→INV-2 (F-P5-002); comprehensive internal-consistency sweep: PC3 wave_id phrasing updated to wave-group-ordinal (2026-06-28). [Prior: v1.0 — initial creation (product-owner): producer-side sprint-state.yaml per-story format obligation (S-18.11 T-2; closes O-P9-001 producer arm).]]"
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
modified:
  - "2026-06-29 (v1.4) — PC3 intra-partition ordering switched from partition-restricted edges (def a) to full-graph wave-depth (def b) per ADR-026 §Decision 3a v1.37 + human directive 'full rigor option b'; EC-010 detection unchanged, depth basis now full-graph."
  - "2026-06-29 (v1.3) — EC-010 narrowed to tolerate supersession edges (depended-on story carries superseded_by) + hard-abort genuine anomalies; PC3 cross-partition supersession-edge note added; +2 test vectors (topo-violation-genuine-anomaly, topo-violation-supersession-tolerated); per ADR-026 §Decision 3a v1.36, human directive — S-3.04 stays partial pending post-E-18 ADR-015 revisit."
  - "2026-06-28 (v1.2) — PC3 amended to two-partition ordering rule per ADR-026 §Decision 3a (S-18.11 PC3-vs-guard reconciliation, human-approved); EC-010 TopoViolation guard added."
  - "2026-06-28 (v1.1) — Architecture Anchors/Related BCs producer-authority corrected to BC-5.41.004 per AC-007 (F-P5-001); Description INV-cite INV-4→INV-2 (F-P5-002); comprehensive internal-consistency sweep: PC3 wave_id phrasing updated to wave-group-ordinal."
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-5.41.004: sprint-state.yaml producer emits per-story `{id, status}` list under `stories:` root key — wave-ascending topo-sort order, status mechanically derived from STORY-INDEX.md, no RAG

## Description

The wave-scheduling skill (or any equivalent sprint-state.yaml producer) MUST write a top-level `stories:` key in `.factory/stories/sprint-state.yaml`. The value is a list of per-story objects; each object has at minimum `id: <S-N.NN>` and `status: <canonical-sprint-status>`. List order MUST be wave-ascending based on the dependency-graph topo-sort from STORY-INDEX.md `depends_on:` arrays — not from a phantom `wave:` story frontmatter field, which does not exist (INV-3). Every story that appears with a non-retired status in STORY-INDEX.md MUST appear in the list; status values MUST be read mechanically from STORY-INDEX.md catalog rows, never inferred or RAG-approximated (INV-2; producer-side complement to BC-5.41.002 INV4). This BC is the producer-side complement to the consumer-side obligations in BC-5.41.001 (PC2 P-SPRINT-STATE-WAVE-ORDER precondition) and BC-5.41.002 (PC3 stories derivation).

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

3. **Two-partition wave-order**: Partition A (terminal prefix) — all terminal stories (merged/withdrawn/cancelled) form a contiguous leading block. Partition B (non-terminal suffix) — all non-terminal stories follow. This satisfies BC-5.41.001 PC2 P-SPRINT-STATE-WAVE-ORDER (the WaveOrderUnverifiable guard requires only that no terminal entry appears after the first non-terminal entry; two-partition guarantees this structurally). Per ADR-026 §Decision 3a.

   **Intra-partition sort key (both partitions):** Within Partition A, order terminal stories by **(full-graph wave-depth ASC, story-ID lexicographic ASC)**. Within Partition B, order non-terminal stories by the same sort key. The **full-graph wave-depth** of story S is: root stories (no `depends_on` entries) have depth 1; all others have depth = `1 + max(depth(P) for all P in S.depends_on)`, computed over the FULL `depends_on` graph including cross-partition edges. This is a global graph property independent of partition placement. Per ADR-026 §Decision 3a (v1.37).

   **Tie-break rule**: If two stories within the same partition have the same full-graph wave-depth (neither depends on the other transitively) and the depth cannot discriminate, order by story ID string lexicographic ascending. This is the canonical tie-break; the SKILL.md behavioral step MUST document it.

   **Cross-partition supersession edges — EXPECTED, TOLERATED, and INCLUDED in depth computation**: When a terminal story (Partition A) has a `depends_on:` entry pointing to a non-terminal story (Partition B) that carries `superseded_by:` frontmatter, this is a legitimate abandoned-by-supersession edge (e.g., merged S-3.01/02/03/S-4.07/08 → partial S-3.04 which carries `superseded_by: ADR-015`). This edge INCLUDES that edge in the full-graph wave-depth computation, but the terminal story still appears in Partition A before its superseded non-terminal dependency in Partition B (partition boundary is determined by terminal/non-terminal status, not dependency direction). This does NOT affect consumers: they read `status:` and list position, never `depends_on:`. The EC-010 guard allows this case (see Edge Cases for the two-case split distinguishing tolerated supersession edges from genuine anomalies). Per ADR-026 §Decision 3a (v1.37).

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
   - **Next-wave selector** (consumed by BC-5.41.002 PC3): only `draft`. The statuses `ready`, `in-progress`, `partial`, and `blocked` are non-terminal but are NOT next-wave selectors — if `sprint-state.yaml` contains no `draft` stories (and no stories are `pending` per BC-5.41.002's reserved arm), BC-5.41.002 PC3 raises BrokenSprintState. BC-5.41.001 PC2 classifies terminal vs. non-terminal for the wave-group-ordinal algorithm (derive_wave_id); this BC's producer obligation is to reflect STORY-INDEX.md statuses faithfully so those consumer-side algorithms can operate correctly.

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
| EC-010 | A terminal story (merged/withdrawn/cancelled) has a `depends_on:` edge pointing to a non-terminal story. Two sub-cases: (a) **Supersession edge — TOLERATE**: the depended-on non-terminal story carries `superseded_by:` frontmatter (e.g., `superseded_by: ADR-015`). This is a legitimate abandoned-by-supersession edge (examples: merged S-3.01/02/03 or S-4.07/08 depending on partial S-3.04 which has `superseded_by: ADR-015`). Detection condition for tolerate: STORY-INDEX shows the dep-story as non-terminal AND the dep-story's file frontmatter contains `superseded_by:` with a non-null value. The edge IS INCLUDED in the full-graph wave-depth computation; the terminal story appears in Partition A before its superseded non-terminal dependency in Partition B (partition boundary is determined by terminal/non-terminal status, not dependency direction); no abort. (b) **Genuine anomaly — HARD ABORT**: the depended-on non-terminal story does NOT carry `superseded_by:` frontmatter (a done story depends on an actively-in-progress story — genuine STORY-INDEX inconsistency). Producer aborts with `TopoViolation: terminal story <ID> depends_on non-terminal story <dep-ID>`; no partial sprint-state.yaml write occurs. Detection: for each terminal story's `depends_on:` entries, check (i) the referenced story is non-terminal in STORY-INDEX AND (ii) the referenced story's file frontmatter lacks `superseded_by:` (null or absent) — only (i)+(ii) together trigger the hard abort. Tolerate vs abort is governed exclusively by `superseded_by:` presence; the edge is always included in depth computation for the tolerated case. Per ADR-026 §Decision 3a (v1.37). |

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
| STORY-INDEX: S-A.01 status=merged, depends_on: [S-B.01]; S-B.01 status=partial, NO `superseded_by:` in S-B.01 frontmatter | Hard abort: `TopoViolation: terminal story S-A.01 depends_on non-terminal story S-B.01`; no file written | topo-violation-genuine-anomaly |
| STORY-INDEX: S-A.01 status=merged, depends_on: [S-B.01]; S-B.01 status=partial, WITH `superseded_by: ADR-015` in S-B.01 frontmatter | Emit normally: Partition A=[S-A.01 merged]; Partition B=[S-B.01 partial]; supersession edge INCLUDED in full-graph wave-depth computation; terminal story S-A.01 appears in Partition A before non-terminal S-B.01 in Partition B; no abort | topo-violation-supersession-tolerated |

## Related BCs

- BC-5.41.001 — depends on: BC-5.41.001 PC2 (wave_id derivation from sprint-state.yaml) requires the P-SPRINT-STATE-WAVE-ORDER precondition; this BC is the producer obligation that establishes that precondition
- BC-5.41.002 — consumed by (BC-5.41.002 PC3 is the CONSUMER-side obligation; it reads the `stories:` list this BC mandates and derives `next_wave_stories` + EPIC-COMPLETE signal from `status: draft` / `status: pending` entries ordered by dependency graph). The SKILL.md producer step cites **BC-5.41.004 PC1–PC3** as the producer format authority; BC-5.41.002 PC3 is the consumer expectation that the producer output satisfies — not the authority the producer step must cite (per AC-007)
- BC-5.41.003 — sibling: both are SS-05 behavioral contracts in the context-durability family

## Architecture Anchors

- `plugins/vsdd-factory/skills/wave-scheduling/SKILL.md` — the producer skill; behavioral step mandating per-story `stories:` list emission (S-18.11 T-5 deliverable; MUST cite **BC-5.41.004 PC1–PC3** as the producer format authority; BC-5.41.002 PC3 is the consumer-side obligation that this producer satisfies — it is the consumer counterpart, not the producer authority)
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
| v1.4 | 2026-06-29 | product-owner | PC3 intra-partition ordering switched from partition-restricted edges (def a) to full-graph wave-depth (def b) per ADR-026 §Decision 3a v1.37 + human directive "full rigor, option b". Intra-partition sort key for both partitions is now (full-graph wave-depth ASC, story-ID lex ASC); full-graph wave-depth is computed over the FULL `depends_on` graph including cross-partition edges (depth = 1 for roots; 1 + max(depth(parents)) for all others). Cross-partition supersession-edge note updated: edge INCLUDED in depth computation (not excluded from topo-sort); terminal story still appears in Partition A before its superseded non-terminal dependency in Partition B (partition placement governed by terminal/non-terminal status). EC-010 detection logic unchanged (tolerate-superseded / abort-genuine-anomaly via `superseded_by:` presence); EC-010 wording reconciled to def-b phrasing — no residual "exclude from topo-sort" language; tolerate vs abort governed exclusively by `superseded_by:` presence. Test vector `topo-violation-supersession-tolerated` updated to reflect "supersession edge INCLUDED in depth computation." |
| v1.3 | 2026-06-29 | product-owner | EC-010 narrowed to tolerate supersession edges (depended-on story carries `superseded_by:`) and hard-abort genuine anomalies; PC3 cross-partition supersession-edge note added (tolerated-by-supersession edges excluded from Partition-A intra-partition topo-sort, emit normally); +2 canonical test vectors (topo-violation-genuine-anomaly, topo-violation-supersession-tolerated); per ADR-026 §Decision 3a v1.36, human directive — S-3.04 stays partial pending post-E-18 ADR-015 revisit. |
| v1.2 | 2026-06-28 | product-owner | PC3 amended to two-partition ordering rule per ADR-026 §Decision 3a (S-18.11 PC3-vs-guard reconciliation, human-approved): Partition A — all terminal stories (merged/withdrawn/cancelled) form a contiguous leading block, wave-ascending topo-sort + lex tie-break within partition; Partition B — all non-terminal stories follow, wave-ascending topo-sort + lex tie-break within partition. Satisfies BC-5.41.001 PC2 P-SPRINT-STATE-WAVE-ORDER and the WaveOrderUnverifiable guard structurally. derive_wave_id and wave_id = 2 unaffected. EC-010 TopoViolation guard added: if a terminal story depends_on a non-terminal story (STORY-INDEX inconsistency), producer MUST hard-abort with `TopoViolation: terminal story <ID> depends_on non-terminal story <dep-ID>` (ADR-026 §Decision 3a caveat). EC-003 tie-break rule clarified to apply within each partition. |
| v1.1 | 2026-06-28 | product-owner | Architecture Anchors/Related BCs producer-authority corrected to BC-5.41.004 PC1–PC3 per AC-007 (F-P5-001): SKILL.md step now cites BC-5.41.004 as producer format authority; BC-5.41.002 PC3 repositioned as consumer-side counterpart (not authority). Description INV-cite corrected INV-4→INV-2 (F-P5-002): no-fabrication/no-RAG clause is INV-2 (not INV-4 which is the no-git-exec rule); phrased as "INV-2; producer-side complement to BC-5.41.002 INV4". Comprehensive internal-consistency sweep: PC3 "leading-contiguous-terminal-run algorithm" updated to "wave-group-ordinal algorithm (derive_wave_id: completed terminal WAVE GROUPS + 1)" per S-18.11 v1.3/v1.4 wave-group-ordinal semantics. Related BCs BC-5.41.002 direction clarified to "consumed by" (consumer counterpart, not format authority). |
| v1.0 | 2026-06-28 | product-owner | Initial creation (S-18.11 T-2; closes O-P9-001 producer arm). BC-5.41.004: producer-side sprint-state.yaml per-story {id, status} format obligation. INV-1 canonical status enum corrected to STORY-INDEX-grounded 8-value set (architect-adjudicated, S-18.11): `draft`, `ready`, `in-progress`, `partial`, `blocked`, `merged`, `withdrawn`, `cancelled`. Removed `completed` and `pending` (never used in STORY-INDEX.md); added `ready` (observed STORY-INDEX value). Terminal = {merged, withdrawn, cancelled}; non-terminal active = {draft, ready, in-progress, partial, blocked}; next-wave selector = {draft} only. EC-007 updated to cite the corrected 8-value enum and explicitly name `completed`, `closed`, `pending`, and `tier-*` tokens as hard-abort triggers. Two additional test vectors added (completed-is-unknown-hard-abort, pending-is-unknown-hard-abort). |
