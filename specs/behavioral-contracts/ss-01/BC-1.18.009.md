---
document_type: behavioral-contract
level: L3
version: "1.1"
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
  - .factory/specs/behavioral-contracts/ss-01/BC-1.18.005.md
  - .factory/cycles/v1.0-brownfield-backfill/S-25.02-f2-architecture-delta.md
input-hash: "14b0031"
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

# BC-1.18.009: BC-INDEX Frontmatter `changelog:` Array Auto-Rotation (Mechanism B1, Reusing `rotate_changelog`) — Single-Actor Block-and-Retry

## Description

`BC-INDEX.md`'s frontmatter `changelog:` YAML sequence is, structurally, an append-only log
embedded inside an otherwise-structured document — the SAME shape as the four mechanism-A
artifacts, just YAML-list-item-shaped instead of markdown-section-shaped, and direct measurement
(2026-09-05) shows it is BC-INDEX's DOMINANT size driver at 177,305 of 539,713 total bytes. This BC
extends the SAME native gate BC-1.18.006 defines with one additional artifact-shape case: when the
matched artifact is "frontmatter changelog array" rather than "flat append-only file," and
BC-1.18.005 Postcondition 8's item-count trigger fires, the gate performs ONLY a rotate/trim step
via the already-shipped `rotate_changelog` (`crates/last-amended-migrate/src/rotate.rs`, ADR-049
§Decision 6) — automating what ADR-049 could previously only do via manual CLI invocation — THEN
returns `HookResult::Block` with a retry instruction, per the CORRECTED single-actor contract below.
**This BC's Postconditions 2 and 6 and Invariant 1 were rewritten in this fix-burst
(F-S2502-F2-001, BLOCKER, ADR-051 v1.1 Decision 7 "CORRECTED" subsection) to withdraw the v1.0
"gate rotates AND prepends, then Continues" design, which was unsound (double-actor prepend hazard;
for `Write`/`MultiEdit`, a stale pre-rotation payload could silently re-introduce the
just-archived tail item, permanently undoing the rotation).** The corrected contract makes this BC a
strict structural mirror of BC-1.18.006's already-accepted block-and-retry contract: one actor per
write, no exception carved out for B1.

## Preconditions

1. The dispatcher's native shard-cap gate (BC-1.18.005/BC-1.18.006) is live and has been extended
   with a "frontmatter changelog array" artifact-shape case (BC-1.18.005 Postcondition 8) that
   matches `BC-INDEX.md` (and, by the same generalized mechanism, `ARCH-INDEX.md` and `VP-INDEX.md`,
   which ADR-049's own audit finding 3 confirms carry the identical `changelog:` shape — though this
   BC's own scope, per S-25.02, is `BC-INDEX.md` specifically).
2. An `Edit`/`Write`/`MultiEdit` call targets `BC-INDEX.md`'s frontmatter, constructing its payload
   per the existing ADR-049 §Decision 2 discipline (the displaced `last_amended` entry becomes the
   new `changelog:` item) — this BC introduces no change to how the AGENT composes its own payload.
3. `rotate_changelog` (`crates/last-amended-migrate/src/rotate.rs`) and `prepend_changelog_item`
   (`crates/last-amended-migrate/src/changelog.rs`) are available as library functions callable
   from `shard_manager.rs` (a new workspace-internal dependency edge:
   `factory-dispatcher` → `last-amended-migrate`, confirmed acyclic per ADR-051 §8's dependency
   check — `last-amended-migrate` has no dependency back on `factory-dispatcher`).
4. `rotate_changelog`'s own implementation (`rewrite_source_after_rotation` in `rotate.rs`) is
   verified to be a PURE TRIM — it keeps only `keep_items` (the retained N-1 most-recent items) and
   writes a `changelog_archive:` discoverability pointer; it never calls `prepend_changelog_item`
   itself and has no parameter for a "new item to insert." This is a structural SDK/library fact,
   not a runtime state — it is what makes the corrected single-actor contract (Postcondition 2)
   the only sound design, mirroring the reasoning BC-1.18.006 Precondition 3 applies to
   `HookResult`'s three-variant contract.

## Postconditions

1. **The `changelog:` sequence is capped at N most-recent items — a config value, not a hardcoded
   constant.** `N` (the maximum retained `changelog:` item count before rotation) is read from the
   same shard-config source BC-1.18.005 Postcondition 6 introduces for mechanism A's cap constants
   — this BC's implementation MUST NOT hardcode `N` into `shard_manager.rs`.

2. **CORRECTED (fix-burst, F-S2502-F2-001, BLOCKER) — the gate performs ONLY the rotate/trim step,
   NEVER the prepend; the prepend is EXCLUSIVELY the responsibility of whichever `Edit`/`Write`/
   `MultiEdit` call ultimately lands.** On overflow (the item-count trigger fires per BC-1.18.005
   Postcondition 8: `current_item_count + 1 > N`), the gate invokes `rotate_changelog` — and ONLY
   `rotate_changelog` — to trim the live `changelog:` sequence down to N-1 items, moving the
   overflowing TAIL (the oldest items beyond the retained N-1) into a new sealed changelog shard
   under `.factory/specs/behavioral-contracts/BC-INDEX-changelog-shards/
   BC-INDEX-changelog.<seq:04>.md`. The gate NEVER calls `prepend_changelog_item` itself. Sequence
   when rotation is needed:
   1. Agent issues its `Edit`/`Write`/`MultiEdit` call against `BC-INDEX.md`'s frontmatter,
      already containing its own prepend of the new `changelog:` item (standard ADR-049 §Decision 2
      discipline, unchanged).
   2. The gate's item-count trigger evaluates `current_item_count + 1 > N` against the file's
      CURRENT (pre-write) state. If false: `Continue` — no rotation; the agent's own prepend lands
      normally, unmodified (EC-002, unchanged from v1.0).
   3. If true: the gate invokes `rotate_changelog` to trim the live sequence to N-1 items and
      publish the sealed shard, THEN returns `HookResult::Block` with an explicit retry
      instruction: "BC-INDEX.md's `changelog:` sequence was rotated to make room (oldest item(s)
      moved to `BC-INDEX-changelog-shards/BC-INDEX-changelog.<seq:04>.md`); the frontmatter now has
      N-1 items. Retry your write: if you used `Edit`, reissue as a fresh `Write` or a fresh `Edit`
      re-read against the current (post-rotation) file, since your original
      `old_string`/`new_string` pair may no longer match; if you used `Write`, recompute your
      `content` payload against the current (post-rotation) file before retrying — do not resubmit
      your original payload unchanged, since it reflects pre-rotation state." This is a
      shape-appropriate specialization of BC-1.18.006 Postcondition 2's retry-wording contract, not
      a divergent one.
   4. Agent retries, reading/recomputing against the now-rotated file; its retried prepend lands via
      `Continue` (item count is now `N-1+1 = N`, not exceeding N) — SINGLE ACTOR, exactly once.
   **This WITHDRAWS the v1.0 design** in which the gate performed BOTH the rotate/trim AND the
   prepend (via `prepend_changelog_item`), then returned `Continue`, letting the originating agent's
   own already-composed call land on top. That design was unsound for two independent reasons: (a)
   **double-actor prepend** — two writers (the gate and the agent) both perform the identical
   logical action in the same operation, producing either a literal duplicate or an order-dependent
   race; (b) **stale-payload clobber for `Write`/`MultiEdit`** — a `Write` call's `content` is the
   agent's own complete, pre-computed payload built from PRE-rotation frontmatter state; if the gate
   rotates and then returns `Continue`, the agent's stale full-file `Write` payload lands OVER the
   just-rotated file, silently RE-INTRODUCING the just-archived tail item and guaranteeing the very
   next write re-triggers rotation again (an infinite churn loop) — exactly the hazard
   BC-1.18.006's Description names as "structurally forbidden" for mechanism A (a PreToolUse hook
   cannot safely mutate a file underneath an in-flight `Edit`/`Write`).

3. **This is the SAME native gate as BC-1.18.006, with a different artifact-shape case — not a
   separate mechanism.** The dispatcher's PreToolUse handling path (BC-1.18.005 Precondition 1)
   dispatches to one of (at least) two artifact-shape handlers based on the `[[shard]]` config
   entry's declared `shape` field: `"flat"` (mechanism A, BC-1.18.006) or
   `"frontmatter-changelog-array"` (this BC, mechanism B1). Both handlers share the SAME
   config-match/no-match entry point (BC-1.18.005 Postcondition 1's zero-cost bypass for unmatched
   paths applies identically to both shapes).

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

6. **CORRECTED (fix-burst, F-S2502-F2-001, BLOCKER) — the block-and-retry observable outcome
   (BC-1.18.006 Postcondition 2) applies IDENTICALLY to this artifact shape; there is NO
   "Continue after rotation" divergence.** If the rotation itself cannot complete (e.g.,
   `rotate_changelog` errors), the gate returns `HookResult::Error`, mirroring BC-1.18.006's EC-003
   (seal-rename-failure) contract for mechanism A — unchanged from v1.0. If the rotation completes
   successfully, the gate returns `HookResult::Block` (Postcondition 2 above) — NEVER `Continue` —
   because `Continue` after a successful rotation would let the agent's own already-composed,
   PRE-rotation payload land on top of the just-rotated file, which is exactly the double-actor /
   stale-payload hazard Postcondition 2 withdraws. **This supersedes the v1.0 text of this
   postcondition, which incorrectly asserted that mechanism A's block-and-retry contract does not
   apply to B1 ("this write CAN proceed after rotation makes room... a deliberate divergence") — that
   assertion is WITHDRAWN.** B1's block-and-retry wording differs from mechanism A's only in the
   artifact-name and rotation-mechanism details (Postcondition 2 step 3's exact text), never in the
   actor-ownership shape.

## Invariants

1. **CORRECTED (fix-burst, F-S2502-F2-001, BLOCKER) — the gate's rotation step calls
   `rotate_changelog` ONLY; `prepend_changelog_item` is NEVER invoked by the gate — only by the
   agent's own write (original or retried).** No duplicate changelog-rotation logic exists in
   `shard_manager.rs`; `rotate_changelog` is imported from `crates/last-amended-migrate` as a
   library call. Post-fix, `prepend_changelog_item` appears in NEITHER `shard_manager.rs` NOR any
   other dispatcher-native code path this BC introduces — it remains exclusively a function the
   AGENT's own tool-call composition logic follows (per ADR-049 §Decision 2), never a function the
   gate calls on the agent's behalf. **This supersedes the v1.0 invariant text ("`rotate_changelog`
   and `prepend_changelog_item` are called, never reimplemented"), which incorrectly implied the
   gate itself calls both functions.**

2. **Existing `changelog:` items are never mutated in place, only relocated.** Rotation moves the
   oldest N+1th-and-beyond items out of the live frontmatter into a sealed shard file; it never
   edits the content of a surviving item, and it never edits the content of a rotated-out item
   (the rotated-out items are moved byte-for-byte into the sealed shard).

3. **`ARCH-INDEX.md`/`VP-INDEX.md`'s identical `changelog:` shape is NOT auto-rotated by this BC.**
   This BC's scope is `BC-INDEX.md` specifically, per S-25.02's own scope. Extending the same
   mechanism to `ARCH-INDEX.md`/`VP-INDEX.md` is a future BC-authorship decision, not silently
   implied by this BC's implementation reusing the same underlying `rotate_changelog` function.

4. **Block-and-retry is the ONLY observable outcome of a successful rotation; `Continue`-after-
   rotation is structurally forbidden (fix-burst, F-S2502-F2-001).** No implementation of this BC
   may return `Continue` after `rotate_changelog` completes successfully — doing so reintroduces
   the withdrawn double-actor/stale-payload hazard Postcondition 2 and Postcondition 6 describe.
   This invariant makes BC-1.18.009 a structural mirror of BC-1.18.006 Invariant 1's "the gate
   returns `Block` on every over-cap/over-N-item write, never `Continue`" contract.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `changelog:` sequence is exactly at N items; a new prepend would make N+1 | Rotation triggers: the oldest item (previously item N) is moved to a new sealed shard, live sequence trimmed to N-1 items, THEN `HookResult::Block` returned with a retry instruction (Postcondition 2) — the agent's retried write lands the new item, bringing the sequence back to N |
| EC-002 | `changelog:` sequence is well under N items | No rotation; the new item is simply prepended by the agent's own original call (`Continue`, standard ADR-049 §Decision 2 discipline, unmodified) |
| EC-003 | `rotate_changelog` itself fails (e.g., cannot write the sealed shard file — disk full) | `HookResult::Error`; the triggering prepend is NOT applied; the frontmatter is left in its pre-rotation state (fail-loud, matching BC-1.18.006 EC-003's mechanism-A precedent) — unchanged from v1.0 |
| EC-004 | A single `changelog:` item is itself extremely large (e.g., the 16,521-byte line ADR-051 directly measured) | That item still counts as exactly ONE item toward the N-item cap — rotation is item-count-triggered for this mechanism, per BC-1.18.005 Postcondition 8, not byte-size-triggered per item |
| EC-005 | Two concurrent sessions both attempt to prepend a `changelog:` item near the N-item boundary | TD-VSDD-053 single-commit-per-burst and the project's factory-lock discipline (ADR-025) prevent concurrent factory-artifacts commits from landing interleaved; the second session's dispatch re-reads the (now-rotated) frontmatter state before its own prepend is evaluated |
| EC-006 (fix-burst, F-S2502-F2-001) | An implementer mistakenly has the gate call `prepend_changelog_item` after a successful `rotate_changelog`, then returns `Continue` | This is the withdrawn v1.0 design and a direct violation of Postcondition 2/6 and Invariant 1/4 — a static-analysis check (VP-126) MUST detect any `prepend_changelog_item` call site inside `shard_manager.rs`'s B1 handler as a defect, not a valid implementation choice |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `changelog:` at N=50 items (config), agent's `Edit` call already contains its own new-item prepend | Item #50 (oldest) rotated into `BC-INDEX-changelog-shards/BC-INDEX-changelog.0001.md`; live sequence trimmed to 49 items; gate returns `Block{reason: "...rotated to make room... Retry your write..."}`; agent's retry lands the new item, sequence returns to 50 total | happy-path |
| `changelog:` at 10 items, N=50, agent's `Write` call already contains its own new-item prepend | No rotation; `Continue`; the agent's original call's new item is simply prepended (11 items total) — single actor, no retry needed | happy-path |
| `rotate_changelog` fails mid-operation (simulated disk-full) | `HookResult::Error`; frontmatter `changelog:` unchanged from its pre-attempt state | error |
| Agent retries a blocked `Write` WITHOUT recomputing its payload against the post-rotation file (resubmits stale pre-rotation content) | The stale `Write` either fails at the tool layer (content mismatch with a validator expecting post-rotation shape) or, if it lands, re-introduces the just-rotated tail item — this is a caller-compliance failure, not a gate defect; the gate's own retry-instruction text (Postcondition 2 step 3) explicitly warns against this | error |
| Reading full changelog history: sealed shard `BC-INDEX-changelog.0001.md` + current frontmatter `changelog:` sequence | Concatenation (sealed shards in `seq` order, oldest-to-newest, followed by current sequence, newest-first per existing convention) reproduces full history with no gaps or duplicates | edge-case |
| Static scan of `shard_manager.rs`'s B1 handler for `prepend_changelog_item` call sites | ZERO call sites found (Invariant 1) — the function is imported/used ONLY by agent-side tooling, never by the gate | happy-path |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-126 | No-reimplementation-and-no-gate-side-prepend invariant — `shard_manager.rs`'s B1 handler contains no changelog-rotation logic other than a call into `rotate_changelog`, and contains ZERO call sites for `prepend_changelog_item` (fix-burst-strengthened per F-S2502-F2-001) | code-review / static-analysis check (grep for duplicated rotation logic AND for any `prepend_changelog_item` call site inside the B1 handler) |
| VP-125 | Bounded-live-sequence invariant — after any sequence of prepends, the live frontmatter `changelog:` sequence never exceeds N items | proptest (arbitrary prepend sequences; property: `len(changelog) <= N` after every operation) |
| VP-125 | No-history-loss invariant — every `changelog:` item ever prepended remains recoverable (live or in a sealed shard) | proptest (arbitrary prepend sequences; property: total recoverable item count is monotonically non-decreasing and equals the total prepend count) |
| VP-131 | Fail-loud rotate_changelog-failure invariant — a `rotate_changelog` invocation failure returns `HookResult::Error` (`E-SHD-004`), never `Block`/`Continue`, and leaves the live `changelog:` sequence byte-identical to its pre-rotation state (EC-003, Postcondition 6) | unit test (injected `rotate_changelog`-failure FS — disk-full/permission; assert `Error` variant naming artifact + failing op, and pre-rotation state preserved) |

**Fix-burst note (F-S2502-F2-001):** formal-verifier should review VP-125/VP-126 against this BC's
corrected contract — the "bounded-live-sequence" and "no-history-loss" properties still hold
structurally, but any proptest/static-analysis harness must now model TWO tool-call events (block,
then retry) for the over-N case, not one, and VP-126's static scan must additionally assert zero
`prepend_changelog_item` call sites in the gate's own code (not yet re-verified against VP bodies
in this burst — VP body edits are formal-verifier's domain). VP-131 (new in the S-25.02 F2
verification-property fix-burst, VP-INDEX v3.03) closes the EC-003/Postcondition-6 fail-loud gap
symmetric with VP-120/VP-122/VP-124's mechanism-A fail-loud legs (F-S2502-F2-004).

## Related BCs

- BC-1.18.006 — this BC extends the SAME native gate with a different artifact-shape case, and (post-fix) is a strict structural mirror of its block-and-retry actor-ownership contract (depends on)
- BC-1.18.005 — owns the item-count trigger condition (Postcondition 8) this BC's rotation responds to (depends on)
- BC-1.18.010 — sibling mechanism B2 (per-subsystem body sharding) targets BC-INDEX's OTHER size driver; both mechanisms are triggered by the same gate (composes with)
- BC-1.18.011 — the B2 migration BC's governed one-time-migration pattern is architecturally parallel to (but independent of) this BC's ongoing per-write rotation (related to)
- BC-10.13.001 — the sanctioned migration/rotation tool whose `rotate_changelog` primitive this BC automates the invocation of (depends on)

## Architecture Anchors

- `crates/factory-dispatcher/src/shard_manager.rs` — the "frontmatter changelog array" artifact-shape handler, calling ONLY `rotate_changelog` from `last-amended-migrate` (never `prepend_changelog_item`)
- `crates/last-amended-migrate/src/rotate.rs` — `rotate_changelog` / `rewrite_source_after_rotation` (reused, not reimplemented; verified pure-trim, no prepend call)
- `crates/last-amended-migrate/src/changelog.rs` — `prepend_changelog_item` (agent-side tooling only, per the corrected contract — never called by the gate)
- `crates/hook-sdk/src/result.rs` — `HookResult` enum, whose `Block`/`Continue`/`Error` three-variant contract is what makes the single-actor correction structurally necessary (same SDK fact BC-1.18.006 Precondition 3 cites)

## SDK Grounding Evidence

Literal stable-anchor greps substantiating this BC's external-artifact claims (POLICY 5;
no `grep -n` / no file:line citations per TD-VSDD-091):

```
$ grep -oE "^pub fn rotate_changelog" crates/last-amended-migrate/src/rotate.rs
pub fn rotate_changelog
```

```
$ grep -oE "^fn rewrite_source_after_rotation" crates/last-amended-migrate/src/rotate.rs
fn rewrite_source_after_rotation
```

```
$ grep -oE "^pub fn prepend_changelog_item|^pub fn ensure_changelog_field" crates/last-amended-migrate/src/changelog.rs
ensure_changelog_field
prepend_changelog_item
```

(second line shown without the `pub fn` prefix reflects grep's own match ordering; both functions
are confirmed present as separate, independently-callable library functions — `rotate_changelog`
has no parameter for inserting a new item, and `rewrite_source_after_rotation` performs the pure
trim, grounding this BC's Precondition 4 and the corrected Postcondition 2's actor-separation
argument.)

```
$ grep -oE "^pub enum HookResult" crates/hook-sdk/src/result.rs
pub enum HookResult
```

```
$ grep -oE "^\s*(Continue|Block \{[^}]*\}|Error \{[^}]*\})" crates/hook-sdk/src/result.rs | sed -E 's/^\s+//' | sort -u
Block { reason: String }
Continue
Error { message: String }
```

Confirms `HookResult`'s three-variant contract, grounding this BC's corrected Postcondition 6 claim
that no fourth "mutate-then-continue" variant exists — `Continue` and mutation-before-return are
mutually exclusive only insofar as `Continue` cannot itself carry the agent's own pending payload,
which is the actual structural reason a second gate-side write is unsafe.

## Story Anchor

S-25.02 — Artifact Sharding Layer 2: Size-Triggered Shard Rotation for Cycle Artifacts

## VP Anchors

- VP-125, VP-126, VP-131 — allocated by formal-verifier (VP-125/126: S-25.02 F2 verification-property extension burst, VP-INDEX v3.02; VP-131: S-25.02 F2 verification-property fix-burst, VP-INDEX v3.03, F-S2502-F2-004). VP-125 (proptest; bounded live changelog: sequence + no-history-loss), VP-126 (static-check; rotate_changelog reuse — no reimplemented rotation logic, and post-fix-burst, no gate-side `prepend_changelog_item` call site), VP-131 (unit-test; EC-003/Postcondition 6 fail-loud rotate_changelog failure, `E-SHD-004` — symmetric with VP-120/VP-122/VP-124's mechanism-A fail-loud legs). VP-125/126 bodies flagged for formal-verifier re-review against this BC's v1.1 corrected contract (fix-burst note above) — not yet actioned in this burst.

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-043 |
| Capability Anchor Justification | CAP-043 ("Artifact Sharding Layer 2: Size-Triggered Shard Rotation for Cycle Append-Logs and BC-INDEX Structured-Catalog Sharding") per capabilities.md §CAP-043 — this BC specifies mechanism B1 exactly as CAP-043 describes it: "B1 reuses the already-shipped `rotate_changelog` primitive (CAP-042) to automatically rotate the frontmatter `changelog:` array (BC-INDEX's dominant size driver...) once it overflows a configured item count." |
| L2 Domain Invariants | none (dispatcher runtime architectural invariant, not an L2 domain-spec DI-NNN) |
| Architecture Module | SS-01 (Hook Dispatcher Core — `shard_manager.rs` B1 artifact-shape handler) |
| ADR | ADR-051 §Decision 7 (BC-INDEX two-level structured-catalog sharding, B1 sub-mechanism, CORRECTED single-actor block-and-retry subsection); ADR-049 §Decision 6 (`rotate_changelog` primitive, reused); §Decision 2 (`last_amended`/`changelog:` prepend discipline, respected unmodified) |
| Stories | S-25.02 |
| Cycle | v1.0-brownfield-backfill (F2 — product-owner spec-evolution burst) |
| Feature | E-25 — Validation Integrity and Large-Artifact Resilience |

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| 1.1 | 2026-09-05 | product-owner | **BLOCKER fix-burst amendment (F-S2502-F2-001, ADR-051 v1.1 Decision 7 "CORRECTED" subsection).** Postconditions 2 and 6 REWRITTEN and Invariant 1 REWRITTEN (with a new Invariant 4) to withdraw the v1.0 double-actor "gate rotates AND prepends, then Continues" design, which was unsound: (a) double-actor prepend hazard (gate and agent both perform the identical insert action); (b) stale-payload clobber for `Write`/`MultiEdit` (a pre-rotation full-file payload landing over a just-rotated file silently re-introduces the archived tail, causing infinite rotation churn) — grounded in `rotate_changelog`'s own verified pure-trim signature (no `prepend_changelog_item` call, no new-item parameter). CORRECTED contract: the gate performs ONLY the rotate/trim step, THEN returns `HookResult::Block` with a retry instruction; the new item's prepend is EXCLUSIVELY the responsibility of whichever `Edit`/`Write`/`MultiEdit` call ultimately lands (original or retried) — a strict structural mirror of BC-1.18.006's block-and-retry contract. Added EC-006 (static-detection of a reintroduced gate-side prepend as a defect) and updated Canonical Test Vectors/VP table notes accordingly. H1 title amended to "... — Single-Actor Block-and-Retry" to make the corrected contract load-bearing in the title per BC H1 Title Authority. Added `## SDK Grounding Evidence` section (F-S2502-F2-007). VP-125/126 flagged for formal-verifier re-review (not actioned this burst — VP bodies are formal-verifier's domain). |
| 1.0 | 2026-09-05 | product-owner | Initial creation (NEW BC, not in the original F1 enumeration — mechanism B1, added per D-1166 human widest-scope decision covering BC-INDEX.md). Automatic size-triggered invocation of the already-shipped `rotate_changelog`/`prepend_changelog_item` primitives via the same native gate BC-1.18.006 defines, with a "Continue after rotation" divergence from mechanism A's block-and-retry (justified: rotation always succeeds in making room for a bounded-size changelog item prepend). CAP-043 capability anchor. ADR-051 §D7 + ADR-049 §D2/§D6 citations. **[WITHDRAWN by v1.1 — see above; this row is retained per POLICY 1 append-only history, not as a currently-valid contract description.]** |
