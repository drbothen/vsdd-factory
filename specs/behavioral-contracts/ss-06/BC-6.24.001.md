---
document_type: behavioral-contract
level: L3
version: "1.8"
status: draft
producer: product-owner
timestamp: 2026-06-14T00:00:00Z
last_amended: "2026-06-15 (v1.8) — fix burst (product-owner): (F-P33-001 MEDIUM) §Changelog: defensive skip-marker annotation added for v1.5 absence — v1.5 was a coordinated-burst skip (only behaviorally-changed BCs were bumped at the pass-5 F-P5-002/003 burst; only BC-4.14.001 and BC-7.07.001 received behavioral changes that pass); the gap v1.4→v1.6 is intentional, not a lost row. Exhaustive sibling-sweep of all 8 E-18 BCs per F-P33-001 obligation. [Prior: 2026-06-15 (v1.7) — F-P32-006: §VP Anchors TBD-VP placeholder replaced with decided DEFERRED-VP disposition (F3, S-18.03 integration anchor; rehydrate-wave skill not yet built); §Verification Properties TBD-VP row replaced with DEFERRED-VP row with explicit property description and integration(F3) proof method; §Changelog section added (was absent). BC-6.24.001 v1.6→v1.7. [Prior: 2026-06-14 (v1.6) — F2 pass-6 fix-burst: (E-18) ADR cite convention: v1.4 version tokens dropped per ADR-026 §BC Traceability Cite Convention (TD-VSDD-091 anti-volatile-pin); stable §Decision anchors adopted (cite-only change). [Prior: 2026-06-14 (v1.4) — F2 pass-4 fix-burst: (F-P4-003) ADR cite v1.3→v1.4 (cite-only). [Prior: 2026-06-14 (v1.3) — F2 pass-3 fix-burst: ADR cite v1.1→v1.3. [Prior: 2026-06-14 (v1.2) — F2 pass-2 fix-burst: (F-P2-004/008) EC-EPIC added (final-wave EPIC-COMPLETE: HANDOFF has epic_status:complete, rehydrate-wave reads HANDOFF.md only, must not error on absent wave-state.yaml); EC-004 clarified as non-final context. [Prior: 2026-06-14 (v1.1) — F2 pass-1 fix-burst: (F-3 consistency) PC1 explicitly mandates git-sourced read from factory-artifacts (no in-context memory fallback); Invariant 1 strengthened to disallow working-tree fallback. (DI) TBD-DI replaced with DI-023. TBD-VP retained with justification per report.]"
phase: F2
inputs:
  - .factory/feature-delta/issue-173/F1-delta-analysis.md
  - .factory/specs/architecture/decisions/ADR-026-wave-boundary-checkpoint-reset-and-lossless-intra-wave-compaction.md
input-hash: "c2426d5"
traces_to: .factory/specs/prd.md
origin: greenfield
extracted_from: null
subsystem: "SS-06"
capability: "CAP-032"
lifecycle_status: draft
introduced: v1.0-feature-context-durability-E18
modified:
  - "2026-06-15 (v1.8) — fix burst (product-owner): (F-P33-001) §Changelog: defensive skip-marker annotation added for v1.5 absence — coordinated-burst skip (only behaviorally-changed BCs bumped at pass-5 F-P5-002/003 burst); gap v1.4→v1.6 is intentional, not a lost row. Exhaustive 8-BC sibling-sweep per F-P33-001."
  - "2026-06-15 (v1.7) — F-P32-006: §VP Anchors TBD-VP → DEFERRED-VP (F3, S-18.03); §Verification Properties row updated with decided property description + integration(F3) proof method; §Changelog section added."
  - "2026-06-14 (v1.6) — F2 pass-6 fix-burst: ADR cite convention: stable §Decision anchors (TD-VSDD-091); cite-only."
  - "2026-06-14 (v1.4) — F2 pass-4 fix-burst: (F-P4-003) ADR cite v1.3→v1.4 (cite-only)."
  - "2026-06-14 (v1.3) — F2 pass-3 fix-burst: ADR cite v1.1→v1.3."
  - "2026-06-14 (v1.2) — F2 pass-2 fix-burst: EC-EPIC (EPIC-COMPLETE final wave; reads HANDOFF.md only; no error on absent wave-state.yaml); EC-004 non-final context clarification."
  - "2026-06-14 (v1.1) — F2 pass-1 fix-burst: PC1 git-source mandate strengthened; working-tree fallback explicitly disallowed; TBD-DI replaced with DI-023; TBD-VP retained with justification; ADR cite v1.0→v1.1."
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-6.24.001: rehydrate-wave skill loads wave-state.yaml and injects exactly the listed specs into session context — no stale prior-wave specs

## Description

The `rehydrate-wave` skill is invoked at the start of a new session after a wave-boundary reset. It reads `wave-state.yaml` from the `factory-artifacts` branch and injects exactly the spec files listed therein into the session context. It does not load prior-wave specs, does not perform RAG retrieval, and does not infer additional context from in-session state. The resulting session context is scoped to precisely what the next wave requires — no more, no less. This is the consumption partner of `wave-handoff` (BC-5.41.002) which produces the manifest.

## Preconditions

1. A wave-boundary reset has occurred (the human has cleared the session and started a new one).
2. `wave-state.yaml` exists on the `factory-artifacts` branch and is readable.
3. The `factory-artifacts` branch is accessible via `git`.
4. The operator invokes `/rehydrate-wave` as the first skill in the new session.

## Postconditions

1. **wave-state.yaml read from git (exact-list injection)**: The skill fetches `wave-state.yaml` from `factory-artifacts` via `git show factory-artifacts:wave-state.yaml` (or equivalent). It does NOT read from the working tree, from in-context memory, or from a locally cached copy that may be stale. The working-tree path `.factory/wave-state.yaml` (if it exists) is NOT the authoritative source — factory-artifacts branch via git is the only valid source.

2. **Exactly listed specs injected**: The skill reads each path listed in `wave-state.yaml` under `stories[].spec_files` and `arch_files` and presents them as context to the session. The set of injected files is exactly the union of these two lists — no additions, no omissions.

3. **No stale prior-wave specs**: The skill does not load any spec files from prior waves that are not explicitly listed in `wave-state.yaml`. Prior-wave BC files, ADR files, or story files that are not in the manifest are not injected.

4. **STATE.md pointer injected**: The skill always injects `.factory/STATE.md` (the `state_pointer` field value) as the first context item, regardless of whether it is also in `spec_files`.

5. **Operator confirmation required before proceeding**: After presenting the injected spec list to the operator, the skill pauses and requests confirmation before the session proceeds with any pipeline work. The confirmation step ensures the operator can verify the rehydration scope.

6. **Missing spec files — warn, not block**: If a path listed in `wave-state.yaml` does not exist on the filesystem or in `factory-artifacts`, the skill emits a warning naming the missing path but continues injecting the remaining listed files. The operator is informed of any gaps.

7. **wave-state.yaml not found — hard block**: If `wave-state.yaml` does not exist on `factory-artifacts`, the skill hard-blocks with a clear error: `RehydrationError: wave-state.yaml not found on factory-artifacts; cannot rehydrate. Run /wave-handoff on wave N to produce the manifest.`

8. **No RAG fallback**: The skill must not fall back to semantic retrieval over the spec corpus if `wave-state.yaml` is missing or incomplete. RAG is an explicitly deferred capability (ADR-026 Decision 4).

## Invariants

1. **Git-sourced manifest (no working-tree fallback)**: `wave-state.yaml` is always read from `factory-artifacts` via git (`git show factory-artifacts:wave-state.yaml`), never from in-context memory, the working tree, or a locally cached copy. A working-tree `wave-state.yaml` that differs from the branch version is NOT authoritative. This prevents a stale in-memory or locally-edited copy from being used for rehydration.

2. **Exact list semantics**: The injected file set is `Set(stories[*].spec_files) UNION Set(arch_files) UNION {state_pointer}`. Neither subset nor superset is acceptable.

3. **No RAG**: Any code path that performs vector similarity search, LLM-based retrieval, or fuzzy file-matching to extend the context beyond the manifest is a specification violation.

4. **Transparency**: The skill outputs a human-readable summary of exactly which files were injected before pausing for confirmation. Invisible injection (no output) is a specification violation.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | wave-state.yaml exists; all paths resolve | All files injected; confirmation prompt shown |
| EC-002 | wave-state.yaml missing from factory-artifacts | Hard block with RehydrationError; no injection |
| EC-003 | wave-state.yaml exists; one spec_file path missing on filesystem | Warning names missing path; remaining files injected; confirmation prompt shown |
| EC-004 | wave-state.yaml `stories: []` (empty wave) — non-final context (wave-state.yaml exists but stories is empty without EPIC-COMPLETE indicator) | Only `arch_files` + `state_pointer` injected; operator warned no stories are listed |
| EC-EPIC | EPIC-COMPLETE final wave: HANDOFF.md has `epic_status: complete` and `next_wave_stories: []`; wave-state.yaml was NOT written by wave-handoff (BC-5.41.002 EC-001a) | `rehydrate-wave` reads HANDOFF.md (not wave-state.yaml); detects `epic_status: complete`; injects STATE.md + arch_files from HANDOFF.md only; emits message "Epic complete — no next-wave stories"; does NOT error on absence of wave-state.yaml |
| EC-005 | Operator invokes /rehydrate-wave mid-session (not at start) | Skill executes normally but warns that prior in-session context may already be contaminated with stale specs |
| EC-006 | wave-state.yaml `arch_files: []` (empty) | Only `stories[*].spec_files` + `state_pointer` injected; no architectural context — operator warned |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| wave-state.yaml: 2 stories each with 2 spec_files; 3 arch_files | 7 files injected (4 story spec files + 3 arch files) + STATE.md = 8 total; confirmation prompt shown | happy-path |
| wave-state.yaml missing | `RehydrationError`; no files injected; human-readable error shown | manifest-missing |
| wave-state.yaml present; one spec_file `foo.md` missing on filesystem | Warning about `foo.md`; other 7 files injected; confirmation prompt shown | missing-spec-warn |
| wave-state.yaml: `stories: []`; 2 arch_files | 2 arch files + STATE.md = 3 injected; operator warned about empty stories | empty-stories |

## Related BCs

- BC-5.41.002 — depends on: wave-state.yaml is produced by wave-handoff; this skill consumes it
- BC-5.40.001 — sibling: factory-lock skill; rehydrate-wave does not hold or renew the lock (it is a read-only rehydration operation)

## Architecture Anchors

- `plugins/vsdd-factory/skills/rehydrate-wave/SKILL.md` — NEW skill under SS-06 (S-18.03 deliverable)
- `plugins/vsdd-factory/skills/wave-handoff/SKILL.md` — producer of the wave-state.yaml consumed here
- ADR-026 §Decision 4 (curated wave-state.yaml manifest as rehydration vehicle; RAG explicitly deferred)

## Story Anchor

S-18.03 (wave-reset skill + wave-state.yaml scoped rehydration)

## VP Anchors

VP allocation: DEFERRED to F3 (story S-18.03 implementation). Rationale: the rehydrate-wave skill (S-18.03 deliverable) does not yet exist; a VP harness cannot be authored against an unbuilt implementation vehicle. At F3, test-writer will assign a VP-NNN to cover: (a) the git-sourced manifest read (git show factory-artifacts:wave-state.yaml — bats-testable even without a live Claude session); (b) exact-list injection semantics (union of stories[*].spec_files + arch_files + state_pointer — verifiable by asserting the skill enumerates the correct file set); and (c) EPIC-COMPLETE path (EC-EPIC: reads HANDOFF.md, not wave-state.yaml, when epic_status:complete detected). The context-injection presentation to the LLM may require a manual session test; this will be determined at F3. Interim: no holistic VP covers BC-6.24.001 at this time.

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| DEFERRED-VP (F3, S-18.03) | rehydrate-wave reads wave-state.yaml from factory-artifacts via git (not working tree); injects exactly the union of stories[*].spec_files + arch_files + state_pointer; no stale prior-wave specs; hard-blocks with RehydrationError when wave-state.yaml missing; no RAG fallback; EPIC-COMPLETE path: reads HANDOFF.md only when epic_status:complete detected; context-injection presentation may require manual session test at F3 | integration (F3) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-032 ("Guarantee lossless context-window transitions via wave-boundary checkpoint and PreCompact flush") per capabilities.md §CAP-032 |
| Capability Anchor Justification | CAP-032 ("Guarantee lossless context-window transitions via wave-boundary checkpoint and PreCompact flush") per capabilities.md §CAP-032 — this BC specifies the rehydration consumption side of the wave-boundary reset mechanism; deterministic injection of exactly the listed specs (and no others) is the guarantee that the new session starts with the correct scope, completing the CAP-032 wave-boundary continuity guarantee begun by HANDOFF.md production (BC-5.41.001) |
| L2 Domain Invariants | DI-023 (Wave/phase identity and next-wave story lists derive from real persisted substrate fields; no phantom fields — enforced by git-sourced manifest read (no working-tree or in-context fallback) and exact-list injection semantics (no additions from RAG or in-context inference)) |
| Architecture Module | SS-06 (Skill Catalog) — rehydrate-wave skill in `plugins/vsdd-factory/skills/` |
| ADR | ADR-026 §Decision 3 (prompt-the-human; operator clears session), §Decision 4 (curated wave-state.yaml manifest; RAG deferred; reads from factory-artifacts via git; working-tree not authoritative) |
| Stories | S-18.03 |
| Cycle | v1.0-feature-context-durability-E18 (F2) |
| Feature | issue #173 / E-18 |

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| v1.8 | 2026-06-15 | product-owner | (F-P33-001 MEDIUM) §Changelog: defensive skip-marker annotation added for v1.5 absence — v1.5 was a coordinated-burst skip (only behaviorally-changed BCs were bumped at the pass-5 F-P5-002/003 burst; only BC-4.14.001 and BC-7.07.001 received behavioral changes that pass); the jump v1.4→v1.6 is intentional, not a lost row. Exhaustive sibling-sweep of all 8 E-18 BCs per F-P33-001 obligation. |
| v1.7 | 2026-06-15 | product-owner | (F-P32-006) §VP Anchors TBD-VP placeholder replaced with decided DEFERRED-VP disposition (F3, S-18.03 integration anchor; rehydrate-wave skill not yet built); §Verification Properties TBD-VP row replaced with DEFERRED-VP row with explicit property description and integration(F3) proof method; §Changelog section added. |
| v1.6 | 2026-06-14 | product-owner | ADR cite convention: stable §Decision anchors (TD-VSDD-091); cite-only. |
| ~~v1.5~~ | — | — | **[SKIP — coordinated-burst skip (F-P33-001):** v1.5 was not produced for this BC because only behaviorally-changed BCs (BC-4.14.001, BC-7.07.001) were bumped at the pass-5 F-P5-002/003 burst; the jump v1.4→v1.6 is intentional, not a lost row. This row distinguishes a deliberate skip from a lost entry.] |
| v1.4 | 2026-06-14 | product-owner | ADR cite v1.3→v1.4 (cite-only). |
| v1.3 | 2026-06-14 | product-owner | ADR cite v1.1→v1.3. |
| v1.2 | 2026-06-14 | product-owner | EC-EPIC added (EPIC-COMPLETE final wave; rehydrate-wave reads HANDOFF.md only; no error on absent wave-state.yaml); EC-004 non-final context clarification. |
| v1.1 | 2026-06-14 | product-owner | PC1 git-source mandate strengthened; working-tree fallback explicitly disallowed; TBD-DI replaced with DI-023; TBD-VP retained with justification; ADR cite v1.0→v1.1. |
| v1.0 | 2026-06-14 | product-owner | Initial creation (E-18 context-durability feature). |
