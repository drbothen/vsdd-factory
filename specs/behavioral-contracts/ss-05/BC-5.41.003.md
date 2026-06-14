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

# BC-5.41.003: validate-burst-log and validate-dispatch-advance exempt commits with "PreCompact flush wave-" prefix from MULTI_COMMIT_CHAIN_NOT_ALLOWED

## Description

The `validate-burst-log` and `validate-dispatch-advance` WASM/bash hooks implement the `MULTI_COMMIT_CHAIN_NOT_ALLOWED` detector (TD-VSDD-053), which blocks sequential `factory-artifacts` commits whose subjects both contain sentinel words like "backfill", "Stage 1", "Stage 2". The PreCompact flush hook (BC-7.07.001) produces commits with subject prefix `PreCompact flush wave-<N>`, which are lifecycle-orthogonal to state-manager bursts (ADR-026 Decision 10). These commits MUST be exempt from the chain detector. Without this exemption, a PreCompact flush commit followed by a legitimate burst commit would trigger a false-positive `MULTI_COMMIT_CHAIN_NOT_ALLOWED` block, making the factory unworkable after any compaction event. This BC is MANDATORY — a factory where PreCompact flush commits block subsequent state-manager bursts is production-blocking.

## Preconditions

1. `validate-burst-log` and `validate-dispatch-advance` hooks are active and operational.
2. A `PreCompact flush wave-<N>` commit has been made to `factory-artifacts` by `precompact-flush.sh` (BC-7.07.001).
3. A state-manager burst is subsequently attempted (producing a Commit A/B/C/D/E sequence).

## Postconditions

1. **Exemption by prefix match**: Both `validate-burst-log` and `validate-dispatch-advance` treat any commit whose subject matches the pattern `^PreCompact flush wave-` as lifecycle-orthogonal. Such commits are excluded from the HEAD/HEAD^ chain comparison that detects `MULTI_COMMIT_CHAIN_NOT_ALLOWED`.

2. **No false-positive block**: After a `PreCompact flush wave-<N> <timestamp>` commit, a subsequent state-manager burst commit (e.g., `state: advance to phase X`) does NOT trigger `MULTI_COMMIT_CHAIN_NOT_ALLOWED`. The burst dispatch proceeds normally.

3. **Normal chain detection preserved**: The exemption is ONLY for commits matching `^PreCompact flush wave-`. All other commit subject patterns continue to be evaluated for chain violations. The exemption does not weaken the general TD-VSDD-053 enforcement.

4. **Bats test coverage**: A bats integration test verifies the exemption: (1) simulate a `PreCompact flush wave-2 ...` commit on factory-artifacts; (2) simulate a subsequent burst commit; (3) verify that `validate-burst-log` and `validate-dispatch-advance` return `block_intent = false` (no `MULTI_COMMIT_CHAIN_NOT_ALLOWED`).

5. **Burst-log entry clarity**: The burst-log entry for the state-manager burst MUST NOT cite the PreCompact commit as Commit A/B/C/D/E. It is not a burst commit. If the burst-log author includes it, that is an INV-019 violation.

## Invariants

1. **Prefix-based exemption, not subject-based inference**: The exemption check is a simple prefix match on the raw commit subject string (`git log --format=%s -1 <SHA>`). It is not NLP inference, not regex over the full commit body, and not subject-line sentiment analysis. The check is: `subject.starts_with("PreCompact flush wave-")`.

2. **Both hooks must implement the exemption symmetrically**: `validate-burst-log` and `validate-dispatch-advance` are both co-owners of this exemption. An implementation that exempts only one of the two leaves the other as a source of false-positive blocks. Symmetric implementation is MANDATORY.

3. **The exemption is not a general escape hatch**: Commits with subjects starting with arbitrary text (e.g., "My flush wave-") are NOT exempt. Only the exact prefix `PreCompact flush wave-` (case-sensitive, as produced by `precompact-flush.sh`) is exempt.

4. **TD-VSDD-053 baseline is unchanged**: The `MULTI_COMMIT_CHAIN_NOT_ALLOWED` rule for "backfill", "Stage 1", "Stage 2" sentinel words is unaffected by this exemption. The exemption adds a conditional skip; it does not remove or weaken the baseline detector.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `PreCompact flush wave-2 2026-06-14T00:00:00Z` followed by burst commit | No block; exemption fires; burst proceeds |
| EC-002 | Two consecutive `PreCompact flush wave-` commits (rapid double-compaction) | Neither triggers MULTI_COMMIT_CHAIN_NOT_ALLOWED; both are individually exempt |
| EC-003 | `PreCompact flush wave-` commit followed by "backfill" burst commit | The "backfill" commit still triggers MULTI_COMMIT_CHAIN_NOT_ALLOWED (HEAD is "backfill"; HEAD^ is "PreCompact flush wave-" — only HEAD^ is exempt from being the TRIGGER; the TRIGGER is HEAD which contains "backfill" and HEAD^^ may also contain "backfill"). Note: exact chain detection logic is in validate-burst-log; this BC mandates the PreCompact exemption, not a blanket weakening of chain detection. |
| EC-004 | Subject starts with "precompact flush wave-" (lowercase) | NOT exempt; prefix match is case-sensitive. Only `PreCompact flush wave-` (capitalized) as produced by the canonical hook is exempt. |
| EC-005 | validate-burst-log implements exemption; validate-dispatch-advance does not | validate-dispatch-advance fires a false-positive block on the burst dispatch. Specification violation — both must be symmetric. |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| HEAD = `PreCompact flush wave-2 2026-06-14T00:00:00Z`; HEAD^ = `state: burst-23 Commit D` | validate-burst-log: Continue (PreCompact HEAD is exempt from being the backfill-chain trigger); validate-dispatch-advance: Continue | happy-path-exempt |
| HEAD = `state: burst-24 Commit A`; HEAD^ = `PreCompact flush wave-2 2026-06-14T00:00:00Z` | No MULTI_COMMIT_CHAIN violation (HEAD^ is PreCompact-exempt; HEAD is a normal burst commit) | burst-after-precompact |
| HEAD = `stage 1 backfill`; HEAD^ = `stage 2 backfill` | MULTI_COMMIT_CHAIN_NOT_ALLOWED (normal TD-VSDD-053 detection; PreCompact exemption not triggered) | normal-chain-detection-preserved |
| bats: simulate PreCompact flush commit + burst commit sequence | validate-burst-log exits 0 (Continue); validate-dispatch-advance exits 0 (Continue) | bats-integration |

## Related BCs

- BC-7.07.001 — depends on: precompact-flush.sh produces commits with `PreCompact flush wave-` prefix that this BC exempts
- BC-5.39.001 — sibling: 3-CLEAN convergence protocol; TD-VSDD-053 single-commit-per-burst; this BC defines the PreCompact lifecycle boundary that keeps those protocols intact

## Architecture Anchors

- `plugins/vsdd-factory/hook-plugins/validate-burst-log.wasm` (or bash equivalent) — must be amended with `PreCompact flush wave-` prefix exemption
- `plugins/vsdd-factory/hook-plugins/validate-dispatch-advance.wasm` (or bash equivalent) — must be amended symmetrically
- ADR-026 §Decision 10 — PreCompact flush lifecycle is distinct from state-manager burst lifecycle; exemption rationale

## Story Anchor

S-18.04 (precompact-flush.sh shell hook + registry; includes validate-burst-log + validate-dispatch-advance exemption as mandatory deliverable)

## VP Anchors

- VP-084 — PreCompact Flush Commit Is Lifecycle-Distinct From State-Manager Burst Commit

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-084 | validate-burst-log exempts commits with "PreCompact flush wave-" prefix from MULTI_COMMIT_CHAIN_NOT_ALLOWED detector; validate-dispatch-advance applies same exemption symmetrically | integration |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-032 ("Guarantee lossless context-window transitions via wave-boundary checkpoint and PreCompact flush") per capabilities.md §CAP-032 |
| Capability Anchor Justification | CAP-032 ("Guarantee lossless context-window transitions via wave-boundary checkpoint and PreCompact flush") per capabilities.md §CAP-032 — this BC is a MANDATORY enabler of the PreCompact flush (CAP-032 Part B); without the MULTI_COMMIT_CHAIN exemption, the flush hook would produce commits that block subsequent state-manager bursts, making the flush a production-blocking regression rather than a durability improvement; this BC closes ADR-026 §Decision 10 and F1 regression risk §4.1 R5 |
| L2 Domain Invariants | TBD-DI — PreCompact lifecycle orthogonality invariant; new invariant candidate |
| Architecture Module | SS-05 (Pipeline Orchestration) — burst-log and dispatch-advance validation logic is orchestration-layer governance (SS-05 behavioral contract) even though the hook implementations may live in SS-04 WASM or SS-07 bash |
| ADR | ADR-026 v1.0 Decision 10 (PreCompact flush lifecycle distinct from state-manager burst lifecycle; validate-burst-log + validate-dispatch-advance must be configured to ignore PreCompact flush commits) |
| Stories | S-18.04 |
| Cycle | v1.0-feature-context-durability-E18 (F2) |
| Feature | issue #173 / E-18 |
