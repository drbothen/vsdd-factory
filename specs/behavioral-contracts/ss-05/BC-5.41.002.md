---
document_type: behavioral-contract
level: L3
version: "1.18"
status: active
producer: product-owner
timestamp: 2026-06-14T00:00:00Z
last_amended: "2026-06-19 (v1.18) — EPIC-COMPLETE single-file commit explicitly permitted in PC6 (product-owner): (F-SP13-P5-001 / S-18.13 spec-cascade pass-5) PC6 atomicity note extended to clarify the EPIC-COMPLETE path: on EPIC-COMPLETE, wave-state.yaml is not produced; --commit creates a single atomic commit containing HANDOFF.md alone. This is NOT a PC6 violation — PC6 forbids splitting a wave-close into two separate commits; it does not require wave-state.yaml to exist when the epic is complete. The two-file (HANDOFF.md + wave-state.yaml) atomicity requirement applies only on the HAS-NEXT-WAVE path. BC-5.41.002 v1.17→v1.18. [Prior: 2026-06-19 (v1.17) — PC6 atomicity boundary clarification (product-owner): (S-18.13 proper fix, human-approved) clarifying note added to PC6: atomicity is defined at the git-commit boundary, not the disk-write boundary. HANDOFF.md is written by the agent Write tool; wave-state.yaml is written by bash (`--emit-wave-state`); `--commit` stages BOTH in ONE git commit. Two separate git commits remain forbidden. The two files being written at different times does NOT violate PC6. BC-5.41.002 v1.16→v1.17. [Prior: 2026-06-19 (v1.16) — POL-14 lifecycle promotion (state-manager): S-18.01 PR #193 squash-merged 8b26a0fe to develop 2026-06-19; lifecycle_status draft→active per POL-14. BC-5.41.002 v1.15→v1.16. [Prior: 2026-06-18 (v1.15) — schema-tension reconciliation (product-owner): EC-002 reconciled with PC2 plain-path-string schema — `status: missing` sub-object replaced with advisory stderr warning (`WARNING: spec_file path does not resolve on disk: <path>`); unresolved path remains a plain string in `spec_files`; PC2 `spec_files: [<path>, ...]` schema unchanged. O-P15-001 closure. BC-5.41.002 v1.14→v1.15. [Prior: 2026-06-18 (v1.14) — clarity refinement (product-owner): PC2 `generated_from_handoff_sha` first sentence rewritten to eliminate the 'most recent HANDOFF.md commit' filter implication (F-P7-001 LOW). The field is now defined as the factory-artifacts branch HEAD SHA captured via `git -C <ARTIFACTS_WT> rev-parse HEAD` immediately before staging/committing the current wave artifacts — not filtered by commit message; any interleaved commits on factory-artifacts are included in HEAD. The wave-1 null rule, step (1)-(4) sequence, and 'NOT the SHA of the commit being created' clause are preserved unchanged. No behavioral change — this resolves the spec's existing intent. BC-5.41.002 v1.13→v1.14. [Prior: 2026-06-18 (v1.13) — spec clarification (product-owner): PC2 `generated_from_handoff_sha` semantics corrected — self-referential-SHA fixed-point contradiction (AC-014 ∧ AC-017 jointly unsatisfiable as literally written) resolved. The field references the PRIOR verified HANDOFF.md commit that already exists on factory-artifacts BEFORE the current wave-close atomic commit, NOT the self-commit that contains wave-state.yaml (cryptographic fixed-point, infeasible). Sequence is: write+validate HANDOFF.md → read its pre-commit content → generate wave-state.yaml using that content → commit both atomically; at generation time only the PRIOR HANDOFF.md commit has a resolvable SHA. Value is `null` for wave 1 (no prior HANDOFF.md commit on factory-artifacts). EC-004 revised: hard-block is 'prior HANDOFF.md commit is not determinable' — not 'null is forbidden'. S-18.01 AC-014 wording requires story-writer/implementer update (report below). BC-5.41.002 v1.12→v1.13. [Prior: 2026-06-16 (v1.12) — fix burst (product-owner): DEFERRED-VP (F3, S-18.01) disposition resolved — VP-087 now exists per F2 gate human directive. §VP Anchors prose replaced with active VP-087 reference; §Verification Properties table row updated from DEFERRED-VP to VP-087 with verbatim title. Title-cite parity confirmed: cited title matches VP-087 H1 verbatim. BC-5.41.002 v1.11→v1.12. [Prior: 2026-06-15 (v1.11) — fix burst (product-owner): (F-P39-002 LOW) Status-token spelling drift: PC3 BrokenSprintState example, EC-001b example tokens, and §Canonical Test Vectors broken-sprint-state row aligned to ADR-026 §Terminal-Wave Discriminator canonical hyphenated spelling. `in_progress` (underscore) → `in-progress` (hyphen) in three normative-body sites. EC-001b `in_review` (not in ADR enum; no canonical hyphenated form exists) → `blocked` (ADR canonical non-terminal state). ADR-026 §Terminal-Wave Discriminator (line 877 normative block) is the source of truth: non-terminal active states are `pending, draft, partial, in-progress, blocked`; sprint-state.yaml confirms `in-progress` (hyphenated) as the live spelling. No behavioral change — BrokenSprintState hard error fires on ANY non-terminal status not in the pending/draft set; only the examples change. BC-5.41.002 v1.10→v1.11 (F-P39-002). Refs: F-P39-002, ADR-026 §Terminal-Wave Discriminator, sprint-state.yaml status enum. [Prior: 2026-06-15 (v1.10) — fix burst (product-owner): (F-P35-002 MEDIUM) §Changelog v1.5 skip-marker: de-enumerated explanatory clause — removed false enumeration of which BCs changed at pass-5 (the enumeration was a false premise per F-P35-002); reworded to be self-contained without citing specific changed BCs, which makes it robust against premise errors. [Prior: 2026-06-15 (v1.9) — fix burst (product-owner): (F-P33-001 MEDIUM) §Changelog: defensive skip-marker annotation added for v1.5 absence — v1.5 was a coordinated-burst skip (only behaviorally-changed BCs were bumped at the pass-5 F-P5-002/003 burst; only BC-4.14.001 and BC-7.07.001 received behavioral changes that pass); the gap v1.4→v1.6 is intentional, not a lost row. Exhaustive sibling-sweep of all 8 E-18 BCs per F-P33-001 obligation. [Prior: 2026-06-15 (v1.8) — F-P32-006: §VP Anchors TBD-VP placeholder replaced with decided DEFERRED-VP disposition (F3, S-18.01 integration anchor); §Verification Properties TBD-VP row replaced with DEFERRED-VP row with explicit property description and integration(F3) proof method. BC-5.41.002 v1.7→v1.8. [Prior: 2026-06-15 (v1.7) — fix burst (product-owner): (F-P30-003 LOW) §Postconditions PC3 BrokenSprintState-path and §Edge Cases EC-001b: human-readable error message aligned to ADR-026 §Terminal-Wave Discriminator canonical text: 'BrokenSprintState: stories in non-terminal, non-pending states exist but no next-wave stories are pending/draft. Update sprint-state.yaml to reflect actual story states.' (prior text: 'No next-wave stories found in sprint-state.yaml but non-terminal stories exist — sprint-state.yaml needs updating.'). (O-P29-002 LOW intent) §Architecture Anchors: SS-05/SS-06 subsystem-split justification note added. BC-5.41.002 v1.6→v1.7. [Prior: 2026-06-14 (v1.6) — F2 pass-6 fix-burst: (E-18) ADR cite convention: v1.4 version token dropped per ADR-026 §BC Traceability Cite Convention (TD-VSDD-091 anti-volatile-pin); stable §Decision anchor adopted (cite-only change). [Prior: 2026-06-14 (v1.4) — F2 pass-4 fix-burst: (F-P4-003) ADR cite v1.3→v1.4 (cite-only). [Prior: 2026-06-14 (v1.3) — F2 pass-3 fix-burst: (O-P3-002) PC7 added: EPIC-COMPLETE operator surfacing — on EPIC-COMPLETE (final wave), wave-handoff announces completion to the operator via stdout with concrete message format before exiting 0. ADR cite v1.1→v1.3. [Prior: 2026-06-14 (v1.2) — F2 pass-2 fix-burst: (F-P2-004) PC3 EPIC-COMPLETE exception added (empty next_wave_stories AND all stories terminal → exit 0, HANDOFF epic_status:complete, no wave-state.yaml); BrokenSprintState hard error retained for empty AND any non-terminal story; EC-001 split into EC-001a (EPIC-COMPLETE) + EC-001b (BrokenSprintState); test vectors updated. [Prior: 2026-06-14 (v1.1) — F2 pass-1 fix-burst: (F-3) PC3 re-anchored: stories list derives from sprint-state.yaml `status: pending` OR `status: draft` entries ordered by dependency graph (not from phantom `wave:` story frontmatter field which does not exist). PC3 'no phantom' mandate explicit. Empty list is HARD ERROR per SOUL.md §4 — Postcondition 3 updated and EC-001 changed from 'valid' to hard block. (DI) TBD-DI replaced with DI-023. TBD-VP retained with justification per report.]"
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
lifecycle_status: active
introduced: v1.0-feature-context-durability-E18
modified:
  - "2026-06-19 (v1.18) — EPIC-COMPLETE single-file commit explicitly permitted in PC6 (product-owner): (F-SP13-P5-001 / S-18.13 spec-cascade pass-5) PC6 atomicity note extended: on EPIC-COMPLETE path, wave-state.yaml is not produced; --commit creates a single atomic commit containing HANDOFF.md alone. This is NOT a PC6 violation — PC6 forbids two separate commits; it does not require wave-state.yaml to exist when the epic is complete."
  - "2026-06-19 (v1.17) — PC6 atomicity boundary clarification (product-owner): (S-18.13 proper fix, human-approved) clarifying note added to PC6 atomicity postcondition — atomicity is defined at the git-commit boundary (single --commit subcommand stages BOTH HANDOFF.md written by agent Write tool + wave-state.yaml written by bash), not at the disk-write boundary. Two separate git commits remain forbidden. No change to PC6 intent."
  - "2026-06-19 (v1.16) — POL-14 lifecycle promotion (state-manager): S-18.01 PR #193 squash-merged 8b26a0fe to develop 2026-06-19; lifecycle_status draft→active. No behavioral change."
  - "2026-06-18 (v1.15) — schema-tension reconciliation (product-owner): EC-002 reconciled with PC2 plain-path-string schema: `status: missing` sub-object replaced with advisory stderr warning (`WARNING: spec_file path does not resolve on disk: <path>`); unresolved path remains a plain string in `spec_files`; PC2 `spec_files: [<path>, ...]` schema unchanged. O-P15-001 closure."
  - "2026-06-18 (v1.14) — clarity refinement (product-owner): (F-P7-001 LOW) PC2 `generated_from_handoff_sha` first sentence rewritten — eliminated 'most recent HANDOFF.md commit' filter implication; field now defined as factory-artifacts HEAD SHA captured via `git -C <ARTIFACTS_WT> rev-parse HEAD` before staging/committing current wave artifacts (not filtered by commit message). Wave-1 null rule, step (1)-(4) sequence, and 'NOT the SHA of the commit being created' clause preserved. No behavioral change. Closes pass-7 F-P7-001 misread."
  - "2026-06-18 (v1.13) — spec clarification (product-owner): PC2 `generated_from_handoff_sha` semantics corrected (self-referential-SHA fixed-point contradiction); EC-004 revised (hard-block on indeterminate prior-HANDOFF-commit, not on null; wave-1 null is valid)."
  - "2026-06-16 (v1.12) — fix burst (product-owner): DEFERRED-VP (F3, S-18.01) resolved — §VP Anchors updated to active VP-087 reference; §Verification Properties row updated to VP-087 with verbatim title."
  - "2026-06-15 (v1.11) — fix burst (product-owner): (F-P39-002 LOW) PC3/EC-001b/test-vectors status-token spelling: `in_progress` → `in-progress`, `in_review` → `blocked` (ADR-026 §Terminal-Wave Discriminator canonical spelling alignment)."
  - "2026-06-15 (v1.10) — fix burst (product-owner): (F-P35-002) §Changelog v1.5 skip-marker: de-enumerated explanatory clause (removed false BC-ID enumeration per F-P35-002 false-premise finding)."
  - "2026-06-15 (v1.9) — fix burst (product-owner): (F-P33-001) §Changelog: defensive skip-marker annotation added for v1.5 absence — coordinated-burst skip (only behaviorally-changed BCs bumped at pass-5 F-P5-002/003 burst); gap v1.4→v1.6 is intentional, not a lost row. Exhaustive 8-BC sibling-sweep per F-P33-001."
  - "2026-06-15 (v1.8) — F-P32-006: §VP Anchors TBD-VP → DEFERRED-VP (F3, S-18.01); §Verification Properties row updated with decided property description + integration(F3) proof method."
  - "2026-06-15 (v1.7) — fix burst (product-owner): (F-P30-003) PC3 + EC-001b: BrokenSprintState human-readable error message aligned to ADR-026 §Terminal-Wave Discriminator canonical text. (O-P29-002) §Architecture Anchors: SS-05/SS-06 split justification note added."
  - "2026-06-14 (v1.6) — F2 pass-6 fix-burst: ADR cite convention: stable §Decision anchor (TD-VSDD-091); cite-only."
  - "2026-06-14 (v1.4) — F2 pass-4 fix-burst: (F-P4-003) ADR cite v1.3→v1.4 (cite-only)."
  - "2026-06-14 (v1.3) — F2 pass-3 fix-burst: PC7 added (EPIC-COMPLETE stdout surfacing per O-P3-002); ADR cite v1.1→v1.3."
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
   - `generated_from_handoff_sha` — the factory-artifacts branch HEAD SHA captured immediately before the current wave-close atomic commit (i.e., `git -C <ARTIFACTS_WT> rev-parse HEAD` executed before staging/committing the current wave artifacts) — normally the prior wave-close HANDOFF.md commit, but it is NOT filtered by commit message: any interleaved commits on factory-artifacts between wave closes contribute to HEAD and are fully included. This is the prior verified commit against which this manifest was generated. This is NOT the SHA of the commit that contains this wave-state.yaml (that SHA is not yet computable at generation time — it is a cryptographic fixed-point that would require the committed blob's bytes to include the hash of the tree containing those same bytes). The correct implementation sequence is: (1) write and validate HANDOFF.md content (working-tree stage, not yet committed); (2) capture `prior_handoff_sha` = current `factory-artifacts` HEAD at this moment (the last HANDOFF.md commit to have landed on the branch); (3) generate wave-state.yaml with `generated_from_handoff_sha: <prior_handoff_sha>`; (4) commit both HANDOFF.md and wave-state.yaml atomically in a single git commit. For wave 1 (no prior HANDOFF.md commit exists on factory-artifacts), the value is `null`.
   - `stories` — list of story objects; each with `{id, status, spec_files: [<path>, ...]}`. At minimum includes all stories assigned to wave N+1 in STORY-INDEX.md.
   - `arch_files` — list of architecture file paths always included in rehydration context (ARCH-INDEX.md, directly referenced ADRs)
   - `state_pointer` — literal string `.factory/STATE.md`

3. **Stories list is derived mechanically from real substrate**: The stories in `wave-state.yaml` are derived from `sprint-state.yaml` by selecting entries with `status: pending` OR `status: draft`, then applying the dependency-order graph from STORY-INDEX.md `depends_on:` arrays to produce the wave sequence. This is the SAME algorithm used by the `wave-scheduling` skill's topological sort step. No `wave:` frontmatter field on story files is referenced — that field does not exist. Each story's `spec_files` list is derived from that story's `bcs:` frontmatter array (resolved to file paths) and any explicitly declared `arch_deps:` entries. **Empty stories list handling — two cases**:
   - **EPIC-COMPLETE exception**: if `sprint-state.yaml` has no entries with `status: pending` or `status: draft` AND all entries have a terminal status (merged, withdrawn, or cancelled), this is the final wave. `wave-handoff` MUST exit 0, write HANDOFF.md with `epic_status: complete` (and `next_wave_stories: []`), and NOT write `wave-state.yaml`. This is a legitimate wave-close.
   - **BrokenSprintState hard error**: if `sprint-state.yaml` has no entries with `status: pending` or `status: draft` BUT one or more entries have a non-terminal, non-pending status (e.g., `status: in-progress` or a story that is neither merged/withdrawn/cancelled/pending/draft), `wave-handoff` MUST abort with exit 1 and an explicit error message: "BrokenSprintState: stories in non-terminal, non-pending states exist but no next-wave stories are pending/draft. Update sprint-state.yaml to reflect actual story states." A silent no-op or an empty `wave-state.yaml` with `stories: []` written silently is a SOUL.md §4 violation (SOUL.md #4: silent failures are forbidden).

4. **No RAG**: The manifest does not use semantic retrieval. Every path in `spec_files` is a literal filesystem path that must resolve on the `factory-artifacts` branch or the working tree. Paths that do not resolve produce an ADVISORY warning to stderr (`WARNING: spec_file path does not resolve on disk: <path>`) at generation time; the unresolved path is still included in `spec_files` as a plain string — this is not a hard block (some spec files may be in-progress). The `spec_files` field remains a plain list of path strings in all cases; no `status:` sub-objects are introduced (see EC-002).

5. **arch_files minimum set**: Must always include:
   - `.factory/specs/architecture/ARCH-INDEX.md`
   - `.factory/specs/architecture/decisions/ADR-026-wave-boundary-checkpoint-reset-and-lossless-intra-wave-compaction.md`
   - `.factory/specs/architecture/decisions/ADR-025-single-writer-factory-locklease-prevent-concurrent-session-races-on-factory-artifacts-orphan-branch.md`
   - Any ADR directly referenced by a story in `stories[].spec_files`

6. **Commit atomicity**: `wave-state.yaml` and `HANDOFF.md` are committed in a single git commit to `factory-artifacts` with message: `HANDOFF wave-<N> <ISO-timestamp>`. They are never committed separately.

   **Atomicity boundary clarification (agent-Write/bash-commit boundary)**: Atomicity is preserved across the agent-Write/bash-commit boundary. `HANDOFF.md` is written to the factory-artifacts working tree by the agent Write tool (BC-5.41.001 PC10, step 2); `wave-state.yaml` is written by bash (via the `--emit-wave-state` subcommand); the `--commit` bash subcommand stages BOTH files and creates ONE git commit. The two files being written at different times (Write tool vs bash) does NOT violate PC6 — atomicity is defined at the git-commit boundary, not the disk-write boundary. Two separate git commits (e.g., one commit for HANDOFF.md and a second commit for wave-state.yaml) remain forbidden.

   **EPIC-COMPLETE path — single-file commit explicitly permitted**: On the EPIC-COMPLETE path (`epic_status: complete` / `next_wave_stories: []`), `wave-state.yaml` is not produced — the epic is complete and there is no next wave to manifest. On this path, `--commit` creates a single atomic commit containing `HANDOFF.md` alone. This is NOT a PC6 violation. PC6 forbids splitting a wave-close into two separate commits; it does not require `wave-state.yaml` to exist when the epic is complete. The two-file (HANDOFF.md + wave-state.yaml) atomicity requirement applies only on the HAS-NEXT-WAVE path. Cross-reference BC-5.41.001 PC10 step 4 (EPIC-COMPLETE arm) and EC-017 (HandoffFileAbsent fires only on missing HANDOFF.md, not on intentionally absent wave-state.yaml).

7. **EPIC-COMPLETE operator surfacing**: When `wave-handoff` determines EPIC-COMPLETE (all entries in `sprint-state.yaml` have terminal status per PC3 EPIC-COMPLETE exception), before exiting 0 it MUST write the following message to stdout so the operator is explicitly notified:
   ```
   EPIC-COMPLETE: All stories in sprint-state.yaml have reached terminal status.
   Epic <epic-id> is complete. No wave-state.yaml written for next wave.
   HANDOFF.md committed to factory-artifacts with epic_status: complete.
   ```
   Where `<epic-id>` is derived from the cycle identifier in `STATE.md` `current_cycle:` field. A silent exit 0 on EPIC-COMPLETE is a specification violation — `wave-state.yaml` is intentionally absent (EC-001a), and a silent exit would make this indistinguishable from an error condition where `wave-state.yaml` was accidentally omitted.

## Invariants

1. **wave-state.yaml is the sole rehydration vehicle**: The `rehydrate-wave` skill reads ONLY from `wave-state.yaml` to determine which specs to load. No other source (in-context state, BC-INDEX grep, story list guessing) is used for rehydration.

2. **Manifest is deterministic given STORY-INDEX.md state**: Two invocations of wave-handoff on the same STORY-INDEX.md state must produce byte-identical `stories` and `arch_files` lists (modulo `generated_at` timestamp and `generated_from_handoff_sha`).

3. **No phantom stories**: Only stories with `status: pending` or `status: draft` in `sprint-state.yaml` appear in the manifest, ordered by the dependency graph. No phantom `wave:` frontmatter field on story files is used — that field does not exist. Stories whose IDs do not appear in STORY-INDEX.md must not appear in the manifest.

4. **RAG exclusion is mandatory**: Any code path that performs semantic vector retrieval over the spec corpus to populate `wave-state.yaml` is a specification violation. The manifest is curated and mechanical.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001a | `sprint-state.yaml` has no entries with `status: pending` or `status: draft`; ALL other entries have terminal status (merged, withdrawn, cancelled) — EPIC-COMPLETE | Exit 0; write HANDOFF.md with `epic_status: complete` and `next_wave_stories: []`; do NOT write `wave-state.yaml`; wave declared complete |
| EC-001b | `sprint-state.yaml` has no entries with `status: pending` or `status: draft`; one or more entries have non-terminal status (e.g., `in-progress`, `blocked`) — BrokenSprintState | HARD ERROR: exit 1; explicit error message "BrokenSprintState: stories in non-terminal, non-pending states exist but no next-wave stories are pending/draft. Update sprint-state.yaml to reflect actual story states."; no `wave-state.yaml` written; operator must correct sprint-state.yaml |
| EC-002 | A story's `bcs:` frontmatter references a BC path that does not resolve on disk | An ADVISORY warning is logged to stderr: `WARNING: spec_file path does not resolve on disk: <path>`; the unresolved path is included in `spec_files` as a plain string (consistent with PC2 `spec_files: [<path>, ...]` schema — no `status: missing` sub-object); not a hard block |
| EC-003 | Story has no `spec_files` derivable (no `bcs:` frontmatter, no arch_deps) | Story included in `stories` list with `spec_files: []`; operator warned to add dependencies |
| EC-004 | No prior HANDOFF.md commit exists on `factory-artifacts` (wave 1, first-ever wave-close; `git log --oneline factory-artifacts` shows no HANDOFF.md commit) | `generated_from_handoff_sha: null` is the correct value. This is NOT a hard block. Wave 1 is a valid state where no prior handoff SHA exists to reference; `null` explicitly documents that provenance (analogous to `precompact_flush_sha: null` on wave 1 when the flush log is absent). The skill MUST use `null` (not a fabricated SHA, not an omitted field, not a placeholder string) to preserve the anti-fabrication guarantee. |
| EC-005 | Operator adds a manual `spec_files` override for a story | Permitted if the override mechanism is explicit (e.g., story frontmatter `extra_spec_files:`); mechanically merged with derived list |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| sprint-state.yaml: S-18.02 status=pending, S-18.03 status=draft; STORY-INDEX.md has both; wave-handoff invoked | wave-state.yaml: `wave_id: <next-wave>`, `stories: [{id: S-18.02, ...}, {id: S-18.03, ...}]` (dependency-ordered) | happy-path |
| S-18.02 bcs: [BC-4.14.001, BC-5.41.001] | S-18.02 spec_files includes `.factory/specs/behavioral-contracts/ss-04/BC-4.14.001.md` and `ss-05/BC-5.41.001.md` | spec-derivation |
| No `status: pending` or `status: draft` entries in sprint-state.yaml; all entries terminal (merged/withdrawn/cancelled) | Exit 0; HANDOFF.md with `epic_status: complete`; no wave-state.yaml written | epic-complete |
| No `status: pending` or `status: draft` entries in sprint-state.yaml; one entry `status: in-progress` | HARD ERROR; exit 1; "BrokenSprintState: stories in non-terminal, non-pending states exist but no next-wave stories are pending/draft. Update sprint-state.yaml to reflect actual story states."; no wave-state.yaml written | broken-sprint-state |
| wave-state.yaml and HANDOFF.md in same commit | single git commit on factory-artifacts | atomicity |
| Wave 1 (first-ever wave-close); no prior HANDOFF.md commit on factory-artifacts | wave-state.yaml `generated_from_handoff_sha: null` (valid; no prior SHA to reference) | wave-1-null-prior-handoff |
| Wave 2 (prior HANDOFF.md commit exists at `abc123...def456` on factory-artifacts HEAD before this commit) | wave-state.yaml `generated_from_handoff_sha: "abc123...def456"` (the PRIOR commit SHA, not the SHA of the commit containing this wave-state.yaml) | wave-n-prior-handoff-sha |

## Related BCs

- BC-5.41.001 — sibling: HANDOFF.md is co-committed with wave-state.yaml in the same atomic commit
- BC-6.24.001 — depends on: rehydrate-wave skill reads wave-state.yaml produced by this BC
- BC-5.40.001 — depends on: factory lock must be held during commit; lock renewal invoked per ADR-025 D11

## Architecture Anchors

- `plugins/vsdd-factory/skills/wave-handoff/SKILL.md` — NEW skill; produces both HANDOFF.md and wave-state.yaml at wave close (S-18.01 deliverable)
- ADR-026 §Decision 4 — wave-state.yaml schema specification (minimum required fields)
- **Subsystem SS-05/SS-06 split justification (O-P29-002):** This BC's `subsystem: SS-05` (Pipeline Orchestration) anchors the orchestration LOGIC and contract (wave-close decisions, wave-state.yaml schema, BrokenSprintState/EPIC-COMPLETE discriminator); the skill FILE artifact (`wave-handoff/SKILL.md`) resides in SS-06 (Skill Catalog) per ADR-026 §Deliverables. This split is intentional: the BC governs orchestration behavior; the Architecture Anchors above locate the skill implementation files. SS-05 is the correct subsystem assignment for this behavioral contract.

## Story Anchor

S-18.01 (HANDOFF.md schema + wave-handoff skill)

## VP Anchors

- VP-087 — wave-state.yaml Is Produced Atomically With HANDOFF.md, Stories List Derives From Real Substrate, BrokenSprintState Blocks on Non-Terminal Stories

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-087 | wave-state.yaml Is Produced Atomically With HANDOFF.md, Stories List Derives From Real Substrate, BrokenSprintState Blocks on Non-Terminal Stories — wave-state.yaml is produced atomically with HANDOFF.md in a single git commit; stories list derived from sprint-state.yaml status:pending/draft entries ordered by dependency graph (no phantom wave: field; no RAG); BrokenSprintState hard error (exit 1, no file written) when non-terminal stories present but no pending/draft entries; EPIC-COMPLETE exception: no wave-state.yaml written, exit 0, HANDOFF.md with epic_status:complete | integration |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-032 ("Guarantee lossless context-window transitions via wave-boundary checkpoint and PreCompact flush") per capabilities.md §CAP-032 |
| Capability Anchor Justification | CAP-032 ("Guarantee lossless context-window transitions via wave-boundary checkpoint and PreCompact flush") per capabilities.md §CAP-032 — this BC specifies the curated rehydration manifest that enables deterministic session rehydration after a wave-boundary reset (ADR-026 Decision 4); deterministic rehydration is the direct complement to wave-boundary hard reset, together forming the complete context-durability guarantee |
| L2 Domain Invariants | DI-023 (Wave/phase identity and next-wave story lists derive from real persisted substrate fields; no phantom fields — enforced by stories derivation from sprint-state.yaml `status:pending/draft` + dependency-order, not from phantom `wave:` story frontmatter; empty list = hard error per SOUL.md §4) |
| Architecture Module | SS-05 (Pipeline Orchestration) — wave-handoff skill |
| ADR | ADR-026 §Decision 4 (wave-state.yaml curated manifest; RAG explicitly deferred; next_wave_stories derived from sprint-state.yaml status:pending/draft entries + dependency-order; empty list = hard error) |
| Stories | S-18.01 |
| Cycle | v1.0-feature-context-durability-E18 (F2) |
| Feature | issue #173 / E-18 |

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| v1.18 | 2026-06-19 | product-owner | (F-SP13-P5-001 / S-18.13 spec-cascade pass-5) PC6 atomicity note extended: EPIC-COMPLETE single-file commit explicitly permitted. On EPIC-COMPLETE path (`epic_status: complete` / `next_wave_stories: []`), `wave-state.yaml` is intentionally not produced; `--commit` creates a single atomic commit containing `HANDOFF.md` alone. This is NOT a PC6 violation — PC6 forbids splitting a wave-close into two separate commits; it does not require `wave-state.yaml` to exist when the epic is complete. Two-file atomicity applies only on the HAS-NEXT-WAVE path. Cross-reference: BC-5.41.001 PC10 step 4 (EPIC-COMPLETE arm) and EC-017 (HandoffFileAbsent scope). |
| v1.17 | 2026-06-19 | product-owner | (S-18.13 proper fix, human-approved) PC6 atomicity postcondition: clarifying note added stating atomicity is defined at the git-commit boundary, not the disk-write boundary. HANDOFF.md written by agent Write tool (BC-5.41.001 PC10 step 2); wave-state.yaml written by bash (`--emit-wave-state`); `--commit` subcommand stages BOTH in ONE atomic git commit. Two separate git commits remain forbidden. The two files being written at different times (Write tool vs bash) does NOT violate PC6 intent. No behavioral change to PC6 — clarification only. |
| v1.16 | 2026-06-19 | state-manager | POL-14 lifecycle promotion: S-18.01 PR #193 squash-merged 8b26a0fe to develop 2026-06-19; lifecycle_status draft→active. No behavioral change. |
| v1.15 | 2026-06-18 | product-owner | (O-P15-001) EC-002 schema-tension reconciliation: `status: missing` sub-object clause replaced with advisory stderr warning (`WARNING: spec_file path does not resolve on disk: <path>`); unresolved path remains a plain string in `spec_files` consistent with PC2 `spec_files: [<path>, ...]` plain-path schema. PC4 tightened to match: stderr advisory wording added, plain-string invariant made explicit with cross-reference to EC-002. PC2 `spec_files` schema unchanged — no `status:` sub-objects introduced. |
| v1.14 | 2026-06-18 | product-owner | (F-P7-001 LOW) PC2 `generated_from_handoff_sha` first sentence clarity refinement: eliminated the "most recent HANDOFF.md commit" filter implication that misread as a grep/log filter that would skip non-HANDOFF interleaved commits. Field now defined as the factory-artifacts branch HEAD SHA captured via `git -C <ARTIFACTS_WT> rev-parse HEAD` immediately before staging/committing the current wave artifacts — not filtered by commit message. Wave-1 null rule, step (1)-(4) sequence, and "NOT the SHA of the commit being created" clause preserved verbatim. No behavioral change — resolves the spec's existing intent. Closes pass-7 F-P7-001 misread. |
| v1.13 | 2026-06-18 | product-owner | Spec clarification: PC2 `generated_from_handoff_sha` semantics corrected — self-referential-SHA fixed-point contradiction (AC-014 ∧ AC-017 jointly unsatisfiable) resolved. Field now unambiguously defined as the PRIOR verified HANDOFF.md commit SHA that already exists on factory-artifacts before the current wave-close commit, NOT the self-commit (which has no knowable SHA at generation time). Correct implementation sequence stated: write+validate HANDOFF.md → capture prior_handoff_sha = current factory-artifacts HEAD → generate wave-state.yaml → atomic single commit. Wave 1 null case documented: null is valid when no prior HANDOFF.md commit exists. EC-004 revised: hard-block case changed from 'HANDOFF.md commit not yet visible' (was a misframing of the infeasible self-SHA) to 'no prior HANDOFF.md commit on factory-artifacts (wave 1)' with null as the correct value. Two new test vectors added: wave-1-null-prior-handoff and wave-n-prior-handoff-sha. |
| v1.12 | 2026-06-16 | product-owner | DEFERRED-VP (F3, S-18.01) resolved — VP-087 now exists per F2 gate human directive. §VP Anchors prose replaced with active bullet reference to VP-087; §Verification Properties row updated from DEFERRED-VP to VP-087 with verbatim H1 title "wave-state.yaml Is Produced Atomically With HANDOFF.md, Stories List Derives From Real Substrate, BrokenSprintState Blocks on Non-Terminal Stories". Title-cite parity verified against VP-087 H1 verbatim. |
| v1.11 | 2026-06-15 | product-owner | (F-P39-002 LOW) Status-token spelling drift corrected: PC3 BrokenSprintState example `status: in_progress` → `status: in-progress`; EC-001b examples `in_progress`, `in_review` → `in-progress`, `blocked`; test vector `status: in_progress` → `status: in-progress`. All aligned to ADR-026 §Terminal-Wave Discriminator canonical non-terminal-active-states list (`pending, draft, partial, in-progress, blocked`). `in_review` dropped — not in ADR canonical enum; `blocked` substituted as a second ADR-canonical example. No behavioral change: BrokenSprintState fires on any non-terminal, non-pending/draft status; only the examples change. |
| v1.10 | 2026-06-15 | product-owner | (F-P35-002) §Changelog v1.5 skip-marker: de-enumerated explanatory clause — removed false BC-ID enumeration that claimed which specific BCs changed at the pass-5 burst (premise was false per F-P35-002); reworded to be self-contained without enumerating changed BCs, which makes the marker robust against premise errors. |
| v1.9 | 2026-06-15 | product-owner | (F-P33-001 MEDIUM) §Changelog: defensive skip-marker annotation added for v1.5 absence — v1.5 was a coordinated-burst skip; the jump v1.4→v1.6 is intentional, not a lost row. Exhaustive sibling-sweep of all 8 E-18 BCs per F-P33-001 obligation. |
| v1.8 | 2026-06-15 | product-owner | (F-P32-006) §VP Anchors TBD-VP placeholder replaced with decided DEFERRED-VP disposition (F3, S-18.01 integration anchor); §Verification Properties TBD-VP row replaced with DEFERRED-VP row with explicit property description and integration(F3) proof method. |
| v1.7 | 2026-06-15 | product-owner | (F-P30-003) PC3 BrokenSprintState-path + EC-001b + canonical test vector `broken-sprint-state`: human-readable error message aligned to ADR-026 §Terminal-Wave Discriminator canonical text. (O-P29-002) §Architecture Anchors: SS-05/SS-06 split justification note added. |
| v1.6 | 2026-06-14 | product-owner | ADR cite convention: stable §Decision anchor (TD-VSDD-091); cite-only. |
| ~~v1.5~~ | — | — | **[SKIP — coordinated-burst skip (F-P33-001):** v1.5 was not produced for this BC; this BC received no behavioral change at the pass-5 (F-P5-*) coordinated fix-burst; the v1.4→v1.6 jump is intentional, not a lost changelog row. This row distinguishes a deliberate skip from a lost entry.] |
| v1.4 | 2026-06-14 | product-owner | ADR cite v1.3→v1.4 (cite-only). |
| v1.3 | 2026-06-14 | product-owner | PC7 added (EPIC-COMPLETE stdout surfacing per O-P3-002); ADR cite v1.1→v1.3. |
| v1.2 | 2026-06-14 | product-owner | PC3 EPIC-COMPLETE exception (all terminal → exit 0 + HANDOFF epic_status:complete, no wave-state.yaml); EC-001 split EC-001a+EC-001b; test vectors updated. |
| v1.1 | 2026-06-14 | product-owner | PC3 stories derivation re-anchored (sprint-state.yaml status:pending/draft + dependency-order; no phantom wave: frontmatter); empty list → HARD ERROR; EC-001 updated; TBD-DI replaced with DI-023; ADR cite v1.0→v1.1. |
| v1.0 | 2026-06-14 | product-owner | Initial creation (E-18 context-durability feature). |
