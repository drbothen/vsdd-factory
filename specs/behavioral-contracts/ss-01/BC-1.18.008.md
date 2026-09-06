---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-09-05T00:00:00Z
phase: F2
inputs:
  - .factory/specs/architecture/decisions/ADR-051-layer-2-two-mechanism-size-triggered-shard-rotation-append-logs-and-bc-index-sharding.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.18.006.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.18.007.md
  - .factory/cycles/v1.0-brownfield-backfill/S-25.02-f1-delta-analysis.md
input-hash: "ce17474"
traces_to: .factory/specs/prd.md
origin: greenfield
extracted_from: null
subsystem: "SS-01"
capability: "CAP-043"
lifecycle_status: draft
introduced: v1.0-brownfield-backfill
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-1.18.008: Mandatory One-Time Backfill-Split of the Four Pre-Existing Oversized Cycle Append-Logs

## Description

AC-002/AC-003 as worded only gate FUTURE writes; they do not, by themselves, retroactively split
the four cycle append-log files (`decision-log.md`, `burst-log.md`, `lessons.md`,
`session-checkpoints.md`) that are already far over any calibrated cap. This BC specifies a
mandatory, one-time backfill-split operation, executed once at F4 activation, that splits each
existing monolithic file into `ceil(current_bytes / shard_cap_bytes)` sealed shards plus a fresh
current file, publishing the full shard index for the pre-existing history in the same operation.
Without this BC, Layer 2 would only prevent future overflow and would never actually shrink the
four artifacts currently producing the majority of observed `plugin.indeterminate` events — an
incomplete delivery of the story's own stated purpose, forbidden under CLAUDE.md's production-grade
default (no partial/MVP delivery of a shipped feature).

## Preconditions

1. BC-1.18.005's cap formula has been F4-locked (constants are no longer provisional) for all four
   mechanism-A artifacts.
2. BC-1.18.006's seal+create+index-publish mechanism and BC-1.18.007's retention/compaction policy
   are both implemented and available for reuse (this BC does not invent new split logic — it
   applies the existing seal mechanism retroactively, in a loop, to pre-existing monolithic
   content).
3. At the time this BC's backfill operation is scheduled, direct measurement of the four
   mechanism-A artifacts (as of 2026-09-05, the most recent measurement available at F2) shows:
   `decision-log.md` 908,938 bytes, `burst-log.md` 806,198 bytes, `lessons.md` 234,731 bytes,
   `session-checkpoints.md` 830,621 bytes — every one already 5-19× over the illustrative
   provisional 49,152-byte (48 KiB) cap. Exact byte counts at the actual F4 execution moment MUST
   be re-measured (these will have grown further between F2 and F4); the counts above are cited
   for illustrative scale only, not as the literal input to the F4 backfill run.

## Postconditions

1. **The backfill-split executes exactly once per artifact, as a one-time migration task at F4
   activation — never as an ongoing per-write mechanism.** This BC is distinct from BC-1.18.006's
   per-write roll: BC-1.18.006 fires reactively on every future over-cap write; this BC fires
   proactively, once, against each artifact's PRE-EXISTING content at the moment Layer 2's cap
   check goes live for that artifact.

2. **Split algorithm: partition pre-existing content into `ceil(current_bytes / shard_cap_bytes)`
   sealed shards, preserving record boundaries.** The monolithic file's content is partitioned at
   the same structural boundaries the artifact's own format uses (e.g., `## Decisions Log` row
   boundaries for `decision-log.md`, `### <burst-heading>` boundaries for `burst-log.md`) — never
   at an arbitrary byte offset that could split a single record (a D-NNN row, a burst-log h2 block)
   across two shard files. Each resulting shard's byte size is `<= shard_cap_bytes`
   (BC-1.18.005's per-artifact effective cap, via the Cross-Validator Minimum Rule). The LAST
   partition becomes the fresh "current" file at the canonical name (per BC-1.18.006's stable-
   current-filename convention); all partitions before it are sealed with sequential `seq`
   numbers starting at 1, in chronological (original-file-order) sequence.

3. **The full shard index is published for the complete pre-existing history in the same
   operation.** The resulting `<artifact-stem>.shard-index.toml` contains one `[[shard]]` entry
   per sealed partition (not just future seals) — `sealed_at` for these backfilled entries is set
   to the backfill operation's own execution timestamp (the true historical seal moments for
   PRE-existing content are not recoverable from the monolithic file alone, since it was never
   previously sharded; this is a documented, accepted approximation, not a defect — genuinely
   time-accurate `sealed_at` values only exist for shards sealed AFTER Layer 2 goes live).

4. **The backfill-split composes with BC-1.18.007's retention policy immediately.** If the number
   of shards a backfill-split produces for a given artifact already exceeds `retention_count`
   (plausible for `decision-log.md` at 908,938 bytes ÷ ~49,152-byte cap ≈ 19 shards, versus a
   default `retention_count` of 10), the retention/compaction archival move (BC-1.18.007
   Postcondition 2) applies to the OLDEST backfilled shards in the SAME operation — the backfill
   does not first produce an over-retention active set and defer archival to a later event.

5. **Atomicity: the backfill-split for a given artifact is all-or-nothing.** If the split
   operation is interrupted partway (e.g., process crash after writing 3 of 19 shard files), the
   operation MUST be safely re-runnable from scratch: it does not corrupt the original monolithic
   file until every resulting shard file AND the shard-index have been written to a staging
   location and validated (total byte count across all shards + fresh current file equals the
   original monolithic file's byte count, no record duplicated or dropped), at which point the
   staged results atomically replace the original monolithic file's role via the same
   temp-file-then-rename discipline BC-1.18.006 already establishes.

6. **Content-preservation verification is mandatory before the original monolithic file's role is
   retired.** The backfill-split MUST verify, before completing: (a) the concatenation of all
   sealed shards in `seq` order plus the final current file reproduces the original monolithic
   file's content byte-for-byte (modulo the shard/index metadata itself, which is new), and (b)
   every record (D-NNN row, burst-log h2 block, lessons entry, session-checkpoint entry) that
   existed in the original file is present in exactly one resulting shard — never zero, never two.
   This verification is a hard gate: if it fails, the backfill-split aborts and the original
   monolithic file is left untouched (fail-loud, not partial-and-silent).

## Invariants

1. **This BC's split logic is a caller of BC-1.18.006's atomic-write primitives, not a
   reimplementation.** The backfill-split reuses the SAME temp-file-then-rename atomic-write
   pattern and the SAME shard-index schema BC-1.18.006 defines for the ongoing per-write case —
   this BC differs only in WHEN it runs (once, at F4 activation) and WHAT it operates on
   (pre-existing monolithic content, iteratively partitioned, rather than a single new block).

2. **No record is ever split across a shard boundary.** The partition points (Postcondition 2) are
   always at structural record boundaries native to the artifact's own format, never mid-record.

3. **The backfill-split is idempotent against a shard-index that already exists for that
   artifact.** If a partial or complete backfill-split has already run for an artifact (e.g., a
   prior interrupted attempt left a valid partial shard-index), re-running the backfill MUST
   either resume from the last verified-complete shard or detect the already-sharded state and
   skip re-splitting (never double-split an already-sharded artifact into redundant shards).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | An artifact's byte count at F4 execution time is smaller than `shard_cap_bytes` (e.g., a fresh cycle's `lessons.md` that hasn't yet grown large) | `ceil(current_bytes / shard_cap_bytes) = 1` — no sealing occurs; the file remains at the canonical name unchanged, but a shard-index IS still created (with zero `[[shard]]` entries and `current_shard` pointing at the unchanged file), so the artifact is "shard-index-registered" even though no split was structurally necessary |
| EC-002 | A single record (one D-NNN decision-log row) is itself larger than `shard_cap_bytes` | The backfill-split MUST NOT truncate or split the oversized record; that shard is allowed to exceed `shard_cap_bytes` for that ONE record only, and this is flagged in the shard-index entry (e.g., an `oversized_record: true` field) as a known, documented exception — consistent with `MAX_SINGLE_RECORD_BYTES`'s role in the cap formula as a margin allowance, not an absolute ceiling on any conceivable record |
| EC-003 | Backfill process crashes after writing shard files 1-3 of an expected 19 | Postcondition 5's atomicity guarantee: the original monolithic file is untouched (staging was incomplete), and the partial staged output is discarded on the next backfill attempt, which restarts cleanly from the original file |
| EC-004 | Content-preservation verification (Postcondition 6) finds a byte-count or record-count mismatch | Backfill aborts; original file left untouched; fail-loud error surfaced to the operator running the F4 migration (analogous to ADR-049's own one-time migration pattern, which this BC is explicitly modeled on) |
| EC-005 | The four artifacts have grown further between F2 (this BC's authoring) and F4 (its execution) — the illustrative byte counts in Precondition 3 are stale by then | Not a defect: Precondition 3 explicitly requires re-measurement at actual F4 execution time; the illustrative F2-era counts exist only to establish scale (5-19× over cap), not as literal backfill inputs |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `decision-log.md` at 908,938 bytes, cap 49,152 bytes, records never spanning a boundary | `ceil(908938/49152) = 19` shards produced (`decision-log.0001.md`..`decision-log.0018.md` sealed + `decision-log.md` fresh current); shard-index with 18 `[[shard]]` entries | happy-path |
| `lessons.md` at 234,731 bytes, cap 49,152 bytes | `ceil(234731/49152) = 5` shards (4 sealed + 1 current) | happy-path |
| Artifact at 40,000 bytes, cap 49,152 bytes (under cap) | 1 "shard" total = the unchanged current file; shard-index created with 0 sealed `[[shard]]` entries | edge-case |
| A single decision-log row of 60,000 bytes (exceeds 49,152-byte cap alone) | That shard's `bytes_at_seal = 60000 > shard_cap_bytes`, flagged `oversized_record: true`; NOT split mid-record | edge-case |
| Backfill interrupted after 3/19 shards written, restarted | Original file byte-identical to pre-crash state; restart produces the same 19-shard result as an uninterrupted run | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (pending) | Content-preservation invariant — concatenation of all resulting shards (in `seq` order) plus the final current file reproduces the original monolithic file byte-for-byte | property test / golden-file round-trip against real (or synthetic fixture) monolithic files |
| (pending) | Record-integrity invariant — every structural record present in the original file appears in EXACTLY ONE resulting shard | property test (record-count-conservation check against synthetic fixtures with known record counts) |
| (pending) | Atomicity-under-interruption invariant — a simulated crash at any point during the split leaves the original file either fully intact or the split fully complete, never a partial/corrupt intermediate state | fault-injection test (simulated crash at each of N write steps; assert post-recovery state is one of the two valid states) |
| (pending) | Idempotency invariant — running the backfill-split twice against an already-sharded artifact does not produce duplicate or additional shards | unit test (double-invocation against a fixture with a pre-existing shard-index) |

## Related BCs

- BC-1.18.005 — this BC applies BC-1.18.005's cap formula retroactively (depends on)
- BC-1.18.006 — this BC reuses BC-1.18.006's atomic-write and shard-index-schema primitives (depends on)
- BC-1.18.007 — this BC's output composes immediately with the retention policy if the backfill produces more shards than `retention_count` (depends on)
- BC-7.08.001 — the Cohort B fail-closed flip is gated on THIS BC completing (the existing oversized files must be split before flipping fail-closed, or the flip would immediately re-trigger the exact INDETERMINATE loop Layer 2 exists to eliminate) (depended on by)

## Architecture Anchors

- `crates/factory-dispatcher/src/shard_manager.rs` — backfill-split entry point, reusing the seal/index-publish primitives
- `crates/last-amended-migrate/` — the crate's existing one-time-migration pattern (ADR-049's precedent) this BC's operational model follows

## Story Anchor

S-25.02 — Artifact Sharding Layer 2: Size-Triggered Shard Rotation for Cycle Artifacts

## VP Anchors

- (pending) — VP IDs pending VP-INDEX allocation by formal-verifier/state-manager per the existing `(pending)` placeholder convention.

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-043 |
| Capability Anchor Justification | CAP-043 ("Artifact Sharding Layer 2: Size-Triggered Shard Rotation for Cycle Append-Logs and BC-INDEX Structured-Catalog Sharding") per capabilities.md §CAP-043 — this BC specifies the "mandatory one-time backfill-split" the capability's own description names explicitly: "without it, this capability would only prevent future overflow and never shrink the artifacts already producing the majority of observed INDETERMINATE events, an incomplete delivery of its own stated purpose." |
| L2 Domain Invariants | none (dispatcher runtime architectural invariant, not an L2 domain-spec DI-NNN) |
| Architecture Module | SS-01 (Hook Dispatcher Core — one-time backfill migration logic in `shard_manager.rs`) |
| ADR | ADR-051 §Decision 2 ("Immediate consequence the story draft did not anticipate — a one-time backfill split is required") |
| Stories | S-25.02 |
| Cycle | v1.0-brownfield-backfill (F2 — product-owner spec-evolution burst) |
| Feature | E-25 — Validation Integrity and Large-Artifact Resilience |

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| 1.0 | 2026-09-05 | product-owner | Initial creation (NEW BC, not in the original F1 enumeration — required per ADR-051 Decision 2's finding that AC-002/AC-003 only gate future writes). One-time backfill-split of the four pre-existing oversized cycle append-log files, record-boundary-safe partitioning, content-preservation verification gate, composition with the retention policy. CAP-043 capability anchor. ADR-051 §D2 citation. |
