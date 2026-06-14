---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-06-14T00:00:00Z
last_amended: 2026-06-14
phase: F2
inputs:
  - .factory/feature-delta/issue-173/F1-delta-analysis.md
  - .factory/specs/architecture/decisions/ADR-026-wave-boundary-checkpoint-reset-and-lossless-intra-wave-compaction.md
input-hash: "[to-be-computed-by-state-manager]"
traces_to: .factory/specs/prd.md
origin: greenfield
extracted_from: null
subsystem: "SS-06"
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

# BC-6.24.001: rehydrate-wave skill loads wave-state.yaml and injects exactly the listed specs into session context — no stale prior-wave specs

## Description

The `rehydrate-wave` skill is invoked at the start of a new session after a wave-boundary reset. It reads `wave-state.yaml` from the `factory-artifacts` branch and injects exactly the spec files listed therein into the session context. It does not load prior-wave specs, does not perform RAG retrieval, and does not infer additional context from in-session state. The resulting session context is scoped to precisely what the next wave requires — no more, no less. This is the consumption partner of `wave-handoff` (BC-5.41.002) which produces the manifest.

## Preconditions

1. A wave-boundary reset has occurred (the human has cleared the session and started a new one).
2. `wave-state.yaml` exists on the `factory-artifacts` branch and is readable.
3. The `factory-artifacts` branch is accessible via `git`.
4. The operator invokes `/rehydrate-wave` as the first skill in the new session.

## Postconditions

1. **wave-state.yaml read from git**: The skill fetches `wave-state.yaml` from `factory-artifacts` via `git show factory-artifacts:wave-state.yaml` (or equivalent). It does NOT read from the working tree or from in-context memory.

2. **Exactly listed specs injected**: The skill reads each path listed in `wave-state.yaml` under `stories[].spec_files` and `arch_files` and presents them as context to the session. The set of injected files is exactly the union of these two lists — no additions, no omissions.

3. **No stale prior-wave specs**: The skill does not load any spec files from prior waves that are not explicitly listed in `wave-state.yaml`. Prior-wave BC files, ADR files, or story files that are not in the manifest are not injected.

4. **STATE.md pointer injected**: The skill always injects `.factory/STATE.md` (the `state_pointer` field value) as the first context item, regardless of whether it is also in `spec_files`.

5. **Operator confirmation required before proceeding**: After presenting the injected spec list to the operator, the skill pauses and requests confirmation before the session proceeds with any pipeline work. The confirmation step ensures the operator can verify the rehydration scope.

6. **Missing spec files — warn, not block**: If a path listed in `wave-state.yaml` does not exist on the filesystem or in `factory-artifacts`, the skill emits a warning naming the missing path but continues injecting the remaining listed files. The operator is informed of any gaps.

7. **wave-state.yaml not found — hard block**: If `wave-state.yaml` does not exist on `factory-artifacts`, the skill hard-blocks with a clear error: `RehydrationError: wave-state.yaml not found on factory-artifacts; cannot rehydrate. Run /wave-handoff on wave N to produce the manifest.`

8. **No RAG fallback**: The skill must not fall back to semantic retrieval over the spec corpus if `wave-state.yaml` is missing or incomplete. RAG is an explicitly deferred capability (ADR-026 Decision 4).

## Invariants

1. **Git-sourced manifest**: `wave-state.yaml` is always read from `factory-artifacts` via git, never from in-context memory or the working tree. This prevents a stale in-memory copy from being used.

2. **Exact list semantics**: The injected file set is `Set(stories[*].spec_files) UNION Set(arch_files) UNION {state_pointer}`. Neither subset nor superset is acceptable.

3. **No RAG**: Any code path that performs vector similarity search, LLM-based retrieval, or fuzzy file-matching to extend the context beyond the manifest is a specification violation.

4. **Transparency**: The skill outputs a human-readable summary of exactly which files were injected before pausing for confirmation. Invisible injection (no output) is a specification violation.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | wave-state.yaml exists; all paths resolve | All files injected; confirmation prompt shown |
| EC-002 | wave-state.yaml missing from factory-artifacts | Hard block with RehydrationError; no injection |
| EC-003 | wave-state.yaml exists; one spec_file path missing on filesystem | Warning names missing path; remaining files injected; confirmation prompt shown |
| EC-004 | wave-state.yaml `stories: []` (empty wave) | Only `arch_files` + `state_pointer` injected; operator warned no stories are listed |
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

TBD-VP — no dedicated VP assigned at F2; story-writer assigns at F3.

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| TBD-VP | rehydrate-wave reads wave-state.yaml from factory-artifacts; injects exactly listed files; no stale prior-wave specs; no RAG | integration |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-032 ("Guarantee lossless context-window transitions via wave-boundary checkpoint and PreCompact flush") per capabilities.md §CAP-032 |
| Capability Anchor Justification | CAP-032 ("Guarantee lossless context-window transitions via wave-boundary checkpoint and PreCompact flush") per capabilities.md §CAP-032 — this BC specifies the rehydration consumption side of the wave-boundary reset mechanism; deterministic injection of exactly the listed specs (and no others) is the guarantee that the new session starts with the correct scope, completing the CAP-032 wave-boundary continuity guarantee begun by HANDOFF.md production (BC-5.41.001) |
| L2 Domain Invariants | TBD-DI — new invariant candidate for session rehydration determinism |
| Architecture Module | SS-06 (Skill Catalog) — rehydrate-wave skill in `plugins/vsdd-factory/skills/` |
| ADR | ADR-026 v1.0 Decision 3 (prompt-the-human; operator clears session), Decision 4 (curated wave-state.yaml manifest; RAG deferred) |
| Stories | S-18.03 |
| Cycle | v1.0-feature-context-durability-E18 (F2) |
| Feature | issue #173 / E-18 |
