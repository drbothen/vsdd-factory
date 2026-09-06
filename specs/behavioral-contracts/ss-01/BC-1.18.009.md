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
  - .factory/specs/architecture/decisions/ADR-049-last-amended-write-path-durable-fix-current-entry-plus-changelog-sequence.md
  - crates/last-amended-migrate/src/rotate.rs
  - crates/last-amended-migrate/src/changelog.rs
  - .factory/specs/behavioral-contracts/ss-01/BC-1.18.006.md
input-hash: "d92ac49"
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

# BC-1.18.009: BC-INDEX Frontmatter `changelog:` Array Auto-Rotation (Mechanism B1, Reusing `rotate_changelog`)

## Description

`BC-INDEX.md`'s frontmatter `changelog:` YAML sequence is, structurally, an append-only log
embedded inside an otherwise-structured document — the SAME shape as the four mechanism-A
artifacts, just YAML-list-item-shaped instead of markdown-section-shaped, and direct measurement
(2026-09-05) shows it is BC-INDEX's DOMINANT size driver at 177,305 of 539,713 total bytes. This BC
extends the SAME native gate BC-1.18.006 defines with one additional artifact-shape case: when the
matched artifact is "frontmatter changelog array" rather than "flat append-only file," the roll
action caps the `changelog:` sequence at N most-recent items and, on overflow, invokes the
already-shipped `rotate_changelog` (`crates/last-amended-migrate/src/rotate.rs`, ADR-049 §Decision
6) rather than duplicating rotation logic — automating what ADR-049 could previously only do via
manual CLI invocation.

## Preconditions

1. The dispatcher's native shard-cap gate (BC-1.18.005/BC-1.18.006) is live and has been extended
   with a "frontmatter changelog array" artifact-shape case that matches `BC-INDEX.md` (and, by
   the same generalized mechanism, `ARCH-INDEX.md` and `VP-INDEX.md`, which ADR-049's own audit
   finding 3 confirms carry the identical `changelog:` shape — though this BC's own scope, per
   S-25.02, is `BC-INDEX.md` specifically).
2. An `Edit`/`Write`/`MultiEdit` call targets `BC-INDEX.md`'s frontmatter, prepending a new
   `changelog:` list item per the existing ADR-049 §Decision 2 discipline (the displaced
   `last_amended` entry becomes the new `changelog:` item).
3. `rotate_changelog` (`crates/last-amended-migrate/src/rotate.rs`) and `prepend_changelog_item`
   (`crates/last-amended-migrate/src/changelog.rs`) are available as library functions callable
   from `shard_manager.rs` (a new workspace-internal dependency edge:
   `factory-dispatcher` → `last-amended-migrate`, confirmed acyclic per ADR-051 §8's dependency
   check — `last-amended-migrate` has no dependency back on `factory-dispatcher`).

## Postconditions

1. **The `changelog:` sequence is capped at N most-recent items — a config value, not a hardcoded
   constant.** `N` (the maximum retained `changelog:` item count before rotation) is read from the
   same shard-config source BC-1.18.005 Postcondition 6 introduces for mechanism A's cap constants
   — this BC's implementation MUST NOT hardcode `N` into `shard_manager.rs`.

2. **On overflow (the prepend would bring the sequence past N items), `rotate_changelog` is invoked
   as a library call — no new rotation logic is written.** The overflowing TAIL of the
   `changelog:` sequence (the oldest items beyond the retained N) is moved by `rotate_changelog`
   into a new sealed changelog shard under
   `.factory/specs/behavioral-contracts/BC-INDEX-changelog-shards/BC-INDEX-changelog.<seq:04>.md`,
   THEN the just-displaced `last_amended` entry is prepended to the now-shortened `changelog:`
   sequence via `prepend_changelog_item`, per ADR-049 §Decision 2's existing discipline (this BC
   does not alter that discipline — it automates WHEN `rotate_changelog` fires, not what it does).

3. **This is the SAME native gate as BC-1.18.006, with a different artifact-shape case — not a
   separate mechanism.** The dispatcher's PreToolUse handling path (BC-1.18.005 Precondition 1)
   dispatches to one of (at least) two artifact-shape handlers based on the `[[shard]]` config
   entry's declared shape: "flat append-only file" (mechanism A, BC-1.18.006) or "frontmatter
   changelog array" (this BC, mechanism B1). Both handlers share the SAME config-match/no-match
   entry point (BC-1.18.005 Postcondition 1's zero-cost bypass for unmatched paths applies
   identically to both shapes).

4. **The rotation is triggered by the SAME `Edit`/`Write`/`MultiEdit` PreToolUse gate, not a
   separate scheduled or manually-invoked process.** Unlike ADR-049's pre-this-BC state (a manual,
   operator-invoked safety net), this BC's rotation fires automatically, size-triggered, as part
   of the SAME dispatcher-native check that mechanism A's roll-before-write uses — closing the gap
   ADR-051's Context section identifies ("that tool is operator-invoked, not size-triggered or
   dispatcher-mediated").

5. **Sealed changelog shards are enumerable and never lose history.** Every `rotate_changelog`
   invocation appends to the `BC-INDEX-changelog-shards/` directory; no prior sealed changelog
   shard is ever overwritten or deleted by a subsequent rotation. The full changelog history
   remains reconstructable by reading all sealed shards in `seq` order followed by the current
   frontmatter `changelog:` sequence.

6. **The block-and-retry observable outcome (BC-1.18.006 Postcondition 2) applies identically to
   this artifact shape, with shape-appropriate wording.** If the rotation itself cannot complete
   (e.g., `rotate_changelog` errors), the gate returns `HookResult::Error`, mirroring BC-1.18.006's
   EC-003 (seal-rename-failure) contract for mechanism A. If the rotation completes successfully,
   the gate does NOT block the triggering write in this case — the rotation makes ROOM in the
   frontmatter for the write to proceed, so (unlike mechanism A's block-and-retry, which fires
   because the WRITE itself doesn't fit) this write CAN proceed after rotation makes room, and the
   gate returns `Continue` after the rotation completes, allowing the original prepend to land in
   the same tool-call invocation. This is a deliberate divergence from BC-1.18.006's block-and-retry
   contract, justified because a `changelog:` item prepend is bounded in size (one new list item)
   and rotation ALWAYS succeeds in making room (the rotated-out tail is unconditionally removed
   from the live frontmatter), unlike mechanism A's append blocks, which may themselves be
   arbitrarily large and cannot be guaranteed to fit even after one roll.

## Invariants

1. **`rotate_changelog` and `prepend_changelog_item` are called, never reimplemented.** No
   duplicate changelog-rotation logic exists in `shard_manager.rs`; both functions are imported
   from `crates/last-amended-migrate` as library calls.

2. **Existing `changelog:` items are never mutated in place, only relocated.** Rotation moves the
   oldest N+1th-and-beyond items out of the live frontmatter into a sealed shard file; it never
   edits the content of a surviving item, and it never edits the content of a rotated-out item
   (the rotated-out items are moved byte-for-byte into the sealed shard).

3. **`ARCH-INDEX.md`/`VP-INDEX.md`'s identical `changelog:` shape is NOT auto-rotated by this BC.**
   This BC's scope is `BC-INDEX.md` specifically, per S-25.02's own scope. Extending the same
   mechanism to `ARCH-INDEX.md`/`VP-INDEX.md` is a future BC-authorship decision, not silently
   implied by this BC's implementation reusing the same underlying `rotate_changelog` function.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `changelog:` sequence is exactly at N items; a new prepend would make N+1 | Rotation triggers: the oldest item (previously item N) is moved to a new sealed shard; the sequence returns to N items after the new item is prepended |
| EC-002 | `changelog:` sequence is well under N items | No rotation; the new item is simply prepended (`Continue`, standard ADR-049 §Decision 2 discipline, unmodified) |
| EC-003 | `rotate_changelog` itself fails (e.g., cannot write the sealed shard file — disk full) | `HookResult::Error`; the triggering prepend is NOT applied; the frontmatter is left in its pre-rotation state (fail-loud, matching BC-1.18.006 EC-003's mechanism-A precedent) |
| EC-004 | A single `changelog:` item is itself extremely large (e.g., the 16,521-byte line ADR-051 directly measured) | That item still counts as exactly ONE item toward the N-item cap — rotation is item-count-triggered for this mechanism, not byte-size-triggered per item (distinguishing B1's trigger condition from mechanism A's byte-size trigger; B1's OWN overall byte contribution is what BC-1.18.005's per-artifact cap formula bounds for BC-INDEX.md as a whole, but the ROTATION decision within B1 is item-count-based, matching `rotate_changelog`'s existing, already-shipped trigger semantics) |
| EC-005 | Two concurrent sessions both attempt to prepend a `changelog:` item near the N-item boundary | TD-VSDD-053 single-commit-per-burst and the project's factory-lock discipline (ADR-025) prevent concurrent factory-artifacts commits from landing interleaved; the second session's dispatch re-reads the (now-rotated) frontmatter state before its own prepend is evaluated |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `changelog:` at N=50 items (config), new prepend arrives | Item #50 (oldest) rotated into `BC-INDEX-changelog-shards/BC-INDEX-changelog.0001.md`; sequence becomes items #2-50 (49 items) + new item (50 items total) | happy-path |
| `changelog:` at 10 items, N=50 | No rotation; new item simply prepended (11 items total) | happy-path |
| `rotate_changelog` fails mid-operation (simulated disk-full) | `HookResult::Error`; frontmatter `changelog:` unchanged from its pre-attempt state | error |
| Reading full changelog history: sealed shard `BC-INDEX-changelog.0001.md` + current frontmatter `changelog:` sequence | Concatenation (sealed shards in `seq` order, oldest-to-newest, followed by current sequence, newest-first per existing convention) reproduces full history with no gaps or duplicates | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (pending) | No-reimplementation invariant — `shard_manager.rs`'s B1 handler contains no changelog-rotation logic other than calls into `rotate_changelog`/`prepend_changelog_item` | code-review / static-analysis check (grep for duplicated rotation logic) |
| (pending) | Bounded-live-sequence invariant — after any sequence of prepends, the live frontmatter `changelog:` sequence never exceeds N items | proptest (arbitrary prepend sequences; property: `len(changelog) <= N` after every operation) |
| (pending) | No-history-loss invariant — every `changelog:` item ever prepended remains recoverable (live or in a sealed shard) | proptest (arbitrary prepend sequences; property: total recoverable item count is monotonically non-decreasing and equals the total prepend count) |

## Related BCs

- BC-1.18.006 — this BC extends the SAME native gate with a different artifact-shape case (depends on)
- BC-1.18.010 — sibling mechanism B2 (per-subsystem body sharding) targets BC-INDEX's OTHER size driver; both mechanisms are triggered by the same gate (composes with)
- BC-10.13.001 — the sanctioned migration/rotation tool whose `rotate_changelog` primitive this BC automates the invocation of (depends on)

## Architecture Anchors

- `crates/factory-dispatcher/src/shard_manager.rs` — the "frontmatter changelog array" artifact-shape handler, calling into `last-amended-migrate`
- `crates/last-amended-migrate/src/rotate.rs` — `rotate_changelog` (reused, not reimplemented)
- `crates/last-amended-migrate/src/changelog.rs` — `prepend_changelog_item` (reused, not reimplemented)

## Story Anchor

S-25.02 — Artifact Sharding Layer 2: Size-Triggered Shard Rotation for Cycle Artifacts

## VP Anchors

- (pending) — VP IDs pending VP-INDEX allocation by formal-verifier/state-manager per the existing `(pending)` placeholder convention.

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-043 |
| Capability Anchor Justification | CAP-043 ("Artifact Sharding Layer 2: Size-Triggered Shard Rotation for Cycle Append-Logs and BC-INDEX Structured-Catalog Sharding") per capabilities.md §CAP-043 — this BC specifies mechanism B1 exactly as CAP-043 describes it: "B1 reuses the already-shipped `rotate_changelog` primitive (CAP-042) to automatically rotate the frontmatter `changelog:` array (BC-INDEX's dominant size driver...) once it overflows a configured item count." |
| L2 Domain Invariants | none (dispatcher runtime architectural invariant, not an L2 domain-spec DI-NNN) |
| Architecture Module | SS-01 (Hook Dispatcher Core — `shard_manager.rs` B1 artifact-shape handler) |
| ADR | ADR-051 §Decision 7 (BC-INDEX two-level structured-catalog sharding, B1 sub-mechanism); ADR-049 §Decision 6 (`rotate_changelog` primitive, reused); §Decision 2 (`last_amended`/`changelog:` prepend discipline, respected unmodified) |
| Stories | S-25.02 |
| Cycle | v1.0-brownfield-backfill (F2 — product-owner spec-evolution burst) |
| Feature | E-25 — Validation Integrity and Large-Artifact Resilience |

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| 1.0 | 2026-09-05 | product-owner | Initial creation (NEW BC, not in the original F1 enumeration — mechanism B1, added per D-1166 human widest-scope decision covering BC-INDEX.md). Automatic size-triggered invocation of the already-shipped `rotate_changelog`/`prepend_changelog_item` primitives via the same native gate BC-1.18.006 defines, with a "Continue after rotation" divergence from mechanism A's block-and-retry (justified: rotation always succeeds in making room for a bounded-size changelog item prepend). CAP-043 capability anchor. ADR-051 §D7 + ADR-049 §D2/§D6 citations. |
